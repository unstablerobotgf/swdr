//! P25 control-channel aggregator for the swdr standalone decoder.
//!
//! live:   swdr-p25 live [--port COM11] [--csv events.csv] [--every 60]
//!         Reads decoded TSBK lines from the board's VCP, appends events to a CSV and prints a report.
//! report: swdr-p25 report events.csv
//!         Replays a CSV through the same aggregator.
//!
//! Events come from GRP_VCH_GRANT (talk), GRP_VCH_GRANT_UPD, GRP_AFFIL_RESPONSE and
//! LOC_REG_RESPONSE (listen), UNIT_REG_RESPONSE (radio on) and DEREG_ACK (radio off).

use anyhow::{bail, Context, Result};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Read, Write};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
struct Event {
    t: f64,
    kind: String,
    tg: Option<u32>,
    unit: Option<u32>,
    ch: String,
}

impl Event {
    fn csv(&self) -> String {
        let o = |v: Option<u32>| v.map(|x| x.to_string()).unwrap_or_default();
        format!("{:.3},{},{},{},{}", self.t, self.kind, o(self.tg), o(self.unit), self.ch)
    }

    fn from_csv(line: &str) -> Option<Self> {
        let p: Vec<&str> = line.split(',').collect();
        if p.len() < 5 || p[0] == "t" {
            return None;
        }
        Some(Event {
            t: p[0].parse().ok()?,
            kind: p[1].to_string(),
            tg: p[2].parse().ok(),
            unit: p[3].parse().ok(),
            ch: p[4].to_string(),
        })
    }
}

/// Parse one decoded line ("nac=00a NAME k=v ... [hex] e=0 nid_e=0") into zero or more events.
fn parse(line: &str, t: f64) -> Vec<Event> {
    let mut it = line.split_whitespace();
    if !it.next().is_some_and(|w| w.starts_with("nac=")) {
        return vec![];
    }
    let Some(name) = it.next() else { return vec![] };
    let kv: HashMap<&str, &str> = it.filter_map(|w| w.split_once('=')).collect();
    let num = |k: &str| kv.get(k).and_then(|v| v.parse::<u32>().ok());
    let ok = || num("result") == Some(0);
    let ev = |kind: &str, tg, unit, ch: &str| Event { t, kind: kind.into(), tg, unit, ch: ch.into() };
    match name {
        "GRP_VCH_GRANT" => vec![ev("talk", num("tg"), num("src"), kv.get("ch").copied().unwrap_or(""))],
        "GRP_VCH_GRANT_UPD" => {
            let mut v = vec![ev("active", num("tg1"), None, kv.get("ch1").copied().unwrap_or(""))];
            if num("tg2") != num("tg1") {
                v.push(ev("active", num("tg2"), None, kv.get("ch2").copied().unwrap_or("")));
            }
            v
        }
        "GRP_AFFIL_RESPONSE" | "LOC_REG_RESPONSE" if ok() => vec![ev("listen", num("tg"), num("unit"), "")],
        "UNIT_REG_RESPONSE" if ok() => vec![ev("on", None, num("src"), "")],
        "DEREG_ACK" => vec![ev("off", None, num("src"), "")],
        _ => vec![],
    }
}

#[derive(Default)]
struct TgAgg {
    grants: u32,
    talkers: BTreeSet<u32>,
    listeners: BTreeSet<u32>,
    last_t: f64,
}

#[derive(Default)]
struct UnitAgg {
    talk: BTreeMap<u32, u32>,
    listen: BTreeSet<u32>,
    on: Vec<f64>,
    off: Vec<f64>,
    first_t: f64,
    last_t: f64,
}

#[derive(Default)]
struct Agg {
    tgs: BTreeMap<u32, TgAgg>,
    units: BTreeMap<u32, UnitAgg>,
    events: u64,
    t0: f64,
    t1: f64,
}

impl Agg {
    fn add(&mut self, e: &Event) {
        if self.events == 0 {
            self.t0 = e.t;
        }
        self.events += 1;
        self.t1 = e.t;
        if let Some(u) = e.unit {
            let ua = self.units.entry(u).or_default();
            if ua.first_t == 0.0 {
                ua.first_t = e.t;
            }
            ua.last_t = e.t;
        }
        match e.kind.as_str() {
            "talk" => {
                let (Some(tg), Some(u)) = (e.tg, e.unit) else { return };
                let ta = self.tgs.entry(tg).or_default();
                ta.grants += 1;
                ta.talkers.insert(u);
                ta.last_t = e.t;
                *self.units.entry(u).or_default().talk.entry(tg).or_default() += 1;
            }
            "active" => {
                if let Some(tg) = e.tg {
                    self.tgs.entry(tg).or_default().last_t = e.t;
                }
            }
            "listen" => {
                let (Some(tg), Some(u)) = (e.tg, e.unit) else { return };
                let ta = self.tgs.entry(tg).or_default();
                ta.listeners.insert(u);
                ta.last_t = e.t;
                self.units.entry(u).or_default().listen.insert(tg);
            }
            "on" => {
                if let Some(u) = e.unit {
                    self.units.entry(u).or_default().on.push(e.t);
                }
            }
            "off" => {
                if let Some(u) = e.unit {
                    self.units.entry(u).or_default().off.push(e.t);
                }
            }
            _ => {}
        }
    }

    /// Primary talkgroup of a radio: most talk grants, else first listened group.
    fn primary(u: &UnitAgg) -> Option<u32> {
        u.talk.iter().max_by_key(|(_, &n)| n).map(|(&tg, _)| tg).or_else(|| u.listen.iter().next().copied())
    }

    fn report(&self, top: usize) -> String {
        let mut s = String::new();
        let span = (self.t1 - self.t0).max(0.0);
        s += &format!(
            "=== {} events over {:.0} s: {} talkgroups, {} radios ===\n",
            self.events,
            span,
            self.tgs.len(),
            self.units.len()
        );
        let mut tgs: Vec<_> = self.tgs.iter().collect();
        tgs.sort_by_key(|(_, a)| std::cmp::Reverse((a.grants, a.listeners.len())));
        s += "talkgroups (grants, talkers, listeners, last seen):\n";
        for (tg, a) in tgs.iter().take(top) {
            let last = if a.last_t > 0.0 { format!("{:.0} s ago", self.t1 - a.last_t) } else { "-".into() };
            s += &format!(
                "  tg {:>6}  grants {:>5}  talkers {:>4}  listeners {:>4}  last {}\n",
                tg,
                a.grants,
                a.talkers.len(),
                a.listeners.len(),
                last
            );
        }
        // Radios grouped by primary talkgroup.
        let mut groups: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
        for (u, ua) in &self.units {
            if let Some(tg) = Self::primary(ua) {
                groups.entry(tg).or_default().push(*u);
            }
        }
        let mut g: Vec<_> = groups.into_iter().collect();
        g.sort_by_key(|(_, v)| std::cmp::Reverse(v.len()));
        s += "radios grouped by primary talkgroup:\n";
        for (tg, units) in g.iter().take(top) {
            let shown: Vec<String> = units.iter().take(12).map(|u| u.to_string()).collect();
            let more = if units.len() > 12 { format!(" (+{})", units.len() - 12) } else { String::new() };
            s += &format!("  tg {:>6}: {} radios: {}{}\n", tg, units.len(), shown.join(" "), more);
        }
        // Registration activity.
        let mut reg: Vec<_> = self.units.iter().filter(|(_, u)| !u.on.is_empty() || !u.off.is_empty()).collect();
        reg.sort_by(|a, b| b.1.last_t.total_cmp(&a.1.last_t));
        s += "recent registrations (on/off counts, last event):\n";
        for (u, ua) in reg.iter().take(top) {
            let tgs: Vec<String> = ua.listen.iter().chain(ua.talk.keys()).map(|t| t.to_string()).collect::<BTreeSet<_>>().into_iter().collect();
            s += &format!(
                "  radio {:>9}  on {:>3}  off {:>3}  last {:.0} s ago  tgs [{}]\n",
                u,
                ua.on.len(),
                ua.off.len(),
                self.t1 - ua.last_t,
                tgs.join(" ")
            );
        }
        s
    }
}

fn now() -> f64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs_f64()).unwrap_or(0.0)
}

fn opt(args: &[String], k: &str) -> Option<String> {
    args.iter().position(|a| a == k).and_then(|i| args.get(i + 1)).cloned()
}

fn live(args: &[String]) -> Result<()> {
    let port = opt(args, "--port").unwrap_or("COM11".into());
    let csv = opt(args, "--csv").unwrap_or("events.csv".into());
    let every: u64 = opt(args, "--every").and_then(|v| v.parse().ok()).unwrap_or(60);
    let new = !std::path::Path::new(&csv).exists();
    let mut out = OpenOptions::new().create(true).append(true).open(&csv).with_context(|| format!("opening {csv}"))?;
    if new {
        writeln!(out, "t,kind,tg,unit,ch")?;
    }
    // Optional raw log: every line from the board, tab-prefixed with its receive time.
    let mut raw = match opt(args, "--raw") {
        Some(p) => Some(OpenOptions::new().create(true).append(true).open(&p).with_context(|| format!("opening {p}"))?),
        None => None,
    };
    let mut sp = serialport::new(&port, 2_000_000).timeout(Duration::from_millis(200)).open()?;
    eprintln!("reading {port}, appending events to {csv}, report every {every} s (Ctrl-C to stop)");
    let mut agg = Agg::default();
    let (mut pending, mut buf, mut last) = (String::new(), [0u8; 4096], Instant::now());
    loop {
        if let Ok(n) = sp.read(&mut buf) {
            pending.push_str(&String::from_utf8_lossy(&buf[..n]));
            while let Some(i) = pending.find('\n') {
                let line: String = pending.drain(..=i).collect();
                let line = line.trim();
                let t = now();
                if let Some(r) = raw.as_mut() {
                    writeln!(r, "{t:.3}\t{line}")?;
                }
                if line.starts_with("ALERT") || line.starts_with("SUM t=") {
                    eprintln!("{line}");
                }
                for e in parse(line, t) {
                    writeln!(out, "{}", e.csv())?;
                    agg.add(&e);
                }
            }
        }
        if last.elapsed() >= Duration::from_secs(every) {
            out.flush()?;
            if let Some(r) = raw.as_mut() {
                r.flush()?;
            }
            println!("{}", agg.report(15));
            last = Instant::now();
        }
    }
}

fn report(args: &[String]) -> Result<()> {
    let Some(path) = args.get(1) else { bail!("usage: swdr-p25 report events.csv") };
    let mut agg = Agg::default();
    for line in BufReader::new(std::fs::File::open(path)?).lines() {
        if let Some(e) = Event::from_csv(&line?) {
            agg.add(&e);
        }
    }
    print!("{}", agg.report(25));
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("live") => live(&args),
        Some("report") => report(&args),
        _ => bail!("usage: swdr-p25 live [--port COM11] [--csv events.csv] [--every 60] | swdr-p25 report events.csv"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_aggregates() {
        let lines = [
            "nac=123 UNIT_REG_RESPONSE result=0 sys=123 src=2000001 [..] e=0 nid_e=0",
            "nac=123 GRP_AFFIL_RESPONSE result=0 tg=1001 unit=2000001 [..] e=0 nid_e=0",
            "nac=123 GRP_VCH_GRANT tg=1001 src=2000001 ch=8-419(852.312500MHz) [..] e=0 nid_e=0",
            "nac=123 GRP_VCH_GRANT tg=1001 src=2000002 ch=8-419(852.312500MHz) [..] e=0 nid_e=0",
            "nac=123 GRP_VCH_GRANT_UPD ch1=8-419 tg1=1001 ch2=8-419 tg2=1001 [..] e=0 nid_e=0",
            "nac=123 DEREG_ACK wacn=12345 sys=123 src=2000001 [..] e=0 nid_e=0",
            "SUM t=30s window=30s tsbk=798",
        ];
        let mut agg = Agg::default();
        for (i, l) in lines.iter().enumerate() {
            for e in parse(l, i as f64) {
                assert_eq!(Event::from_csv(&e.csv()).unwrap().csv(), e.csv());
                agg.add(&e);
            }
        }
        let tg = &agg.tgs[&1001];
        assert_eq!((tg.grants, tg.talkers.len(), tg.listeners.len()), (2, 2, 1));
        let u = &agg.units[&2000001];
        assert_eq!((u.on.len(), u.off.len(), Agg::primary(u)), (1, 1, Some(1001)));
        assert!(agg.report(5).contains("tg   1001: 2 radios"));
    }
}
