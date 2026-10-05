"""Live local dashboard for a running `swdr-p25 live --csv ... --raw ...` collection.

Tails the events CSV and raw log incrementally and serves a self-refreshing page on 127.0.0.1.
Local only: it shows radio and talkgroup IDs from the monitored system.
usage: p25_dashboard.py DATA_DIR [PORT]   (expects events.csv, raw.log, optional talkgroups.tsv)
"""
import json
import os
import re
import sys
import threading
import time
from collections import Counter, defaultdict, deque
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

DATA = sys.argv[1]
PORT = int(sys.argv[2]) if len(sys.argv) > 2 else 8025
BIN = 300  # timeline bin, s
WINDOW_H = 12  # timeline span, hours

names = {}
tg_path = os.path.join(DATA, "talkgroups.tsv")
if os.path.exists(tg_path):
    for i, line in enumerate(open(tg_path, encoding="utf-8")):
        p = line.rstrip("\n").split("\t")
        if i and len(p) >= 6:
            names[int(p[0])] = {"alpha": p[1], "desc": p[2], "tag": p[3], "agency": p[4], "enc": p[5] == "1"}


class State:
    def __init__(self):
        self.lock = threading.Lock()
        self.offsets = {"events.csv": 0, "raw.log": 0}
        self.t_first = self.t_last = None
        self.grants = Counter()
        self.talkers = defaultdict(set)
        self.listeners = defaultdict(set)
        self.last_t = {}
        self.timeline = Counter()  # bin index -> grants
        self.recent = deque(maxlen=30)
        self.carriers_at = deque()  # (t, freq) in the last 10 s
        self.peak_carriers = 0
        self.radios = set()
        self.regs = 0
        self.health = deque(maxlen=240)
        self.alerts = deque(maxlen=20)
        self.last_reg = {}
        self.last_call = {}  # (tg, src, ch) -> t: the control channel repeats each grant; merge within 5 s
        self.covered = set()  # timeline bins with collector coverage (a SUM line seen)
        self.tsbk_t = deque()  # decoded-line timestamps, last 180 s, for the live rate
        self.rf = deque(maxlen=360)  # SUM rf lines: RSSI min/mean/max, AGC range, AFC

    def _read_new(self, name):
        path = os.path.join(DATA, name)
        if not os.path.exists(path):
            return []
        with open(path, "rb") as f:
            f.seek(self.offsets[name])
            data = f.read()
        # Keep a partial last line for next time.
        cut = data.rfind(b"\n") + 1
        self.offsets[name] += cut
        return data[:cut].decode("utf-8", errors="replace").splitlines()

    def poll(self):
        with self.lock:
            for line in self._read_new("events.csv"):
                p = line.split(",")
                if len(p) < 5 or p[0] == "t":
                    continue
                t, kind = float(p[0]), p[1]
                tg = int(p[2]) if p[2] else None
                unit = int(p[3]) if p[3] else None
                self.t_first = self.t_first or t
                self.t_last = t
                if unit is not None:
                    self.radios.add(unit)
                if kind == "talk" and tg is not None:
                    key = (tg, unit, p[4])
                    repeat = t - self.last_call.get(key, -1e9) < 5
                    self.last_call[key] = t
                    self.last_t[tg] = t
                    if repeat:
                        continue
                    self.grants[tg] += 1
                    self.talkers[tg].add(unit)
                    self.last_t[tg] = t
                    self.timeline[int(t // BIN)] += 1
                    m = re.search(r"\(([\d.]+)MHz\)", p[4])
                    self.recent.appendleft({"t": t, "tg": tg, "src": unit, "mhz": m.group(1) if m else ""})
                elif kind == "active" and tg is not None:
                    self.last_t[tg] = t
                elif kind == "listen" and tg is not None:
                    self.listeners[tg].add(unit)
                    self.last_t.setdefault(tg, t)
                elif kind == "on":
                    # Registration responses repeat several times; merge per radio within 10 s
                    if t - self.last_reg.get(unit, -1e9) >= 10:
                        self.regs += 1
                    self.last_reg[unit] = t
                m = re.search(r"\(([\d.]+)MHz\)", p[4])
                if kind in ("talk", "active") and m:
                    self.carriers_at.append((t, m.group(1)))
                    while self.carriers_at and self.carriers_at[0][0] < t - 10:
                        self.carriers_at.popleft()
                    self.peak_carriers = max(self.peak_carriers, len({f for _, f in self.carriers_at}))
            for line in self._read_new("raw.log"):
                ts, _, body = line.partition("\t")
                if body.startswith("nac="):
                    self.tsbk_t.append(float(ts))
                elif body.startswith("SUM rf"):
                    m = re.search(r"rssi=(-?\d+)/(-?\d+)/(-?\d+)dBm agc=(\d+)\.\.(\d+) (?:hw_)?afc=(-?\d+)(?: lo=([+-]\d+)Hz)?", body)
                    if m:
                        g = m.groups()
                        self.rf.append({"t": float(ts), "min": int(g[0]), "mean": int(g[1]), "max": int(g[2]),
                                        "agc_lo": int(g[3]), "agc_hi": int(g[4]), "afc": int(g[5]),
                                        "lo": int(g[6]) if g[6] else None})
                elif body.startswith("SUM t="):
                    kv = dict(re.findall(r"(\w+)=([\w.]+)", body))
                    win = float(kv.get("window", "30s").rstrip("s")) or 30
                    for b in range(int((float(ts) - win) // BIN), int(float(ts) // BIN) + 1):
                        self.covered.add(b)
                    self.health.append({"t": float(ts), "rate": round(int(kv["tsbk"]) / win, 1),
                                        "nid": int(kv["nid_fixed"]), "trellis": int(kv["trellis_e"])})
                elif body.startswith("ALERT"):
                    self.alerts.appendleft({"t": float(ts), "text": body})

    def live(self):
        with self.lock:
            now = time.time()
            while self.tsbk_t and self.tsbk_t[0] < now - 180:
                self.tsbk_t.popleft()
            # Per-second decode counts for the last 120 s; the newest second is still filling, so skip it.
            end = int(now) - 1
            per_s = Counter(int(t) for t in self.tsbk_t)
            secs = [{"t": t, "n": per_s.get(t, 0)} for t in range(end - 119, end + 1)]
            return {"now": now, "secs": secs, "rate5": sum(x["n"] for x in secs[-5:]) / 5,
                    "rate30": sum(x["n"] for x in secs[-30:]) / 30, "rf": list(self.rf)[-120:]}

    def snapshot(self):
        with self.lock:
            now = time.time()
            live = sorted(((tg, t) for tg, t in self.last_t.items() if now - t < 20), key=lambda x: -x[1])
            lo = int((now - WINDOW_H * 3600) // BIN)
            hi = int(now // BIN)
            first_bin = max(lo, int((self.t_first or now) // BIN))
            timeline = [{"t": b * BIN, "n": self.timeline.get(b, 0), "gap": b not in self.covered and not self.timeline.get(b)}
                        for b in range(first_bin, hi + 1)]
            agency = Counter()
            for tg, n in self.grants.items():
                agency[names.get(tg, {}).get("agency", "Unlisted")] += n
            top = sorted(self.grants, key=lambda tg: -self.grants[tg])[:15]
            raw_mtime = os.path.getmtime(os.path.join(DATA, "raw.log")) if os.path.exists(os.path.join(DATA, "raw.log")) else 0
            carriers_now = len({f for t, f in self.carriers_at if now - t < 10})
            return {
                "now": now,
                "t_first": self.t_first,
                "collector_age": now - raw_mtime if raw_mtime else None,
                "totals": {"calls": sum(self.grants.values()), "talkgroups": len(set(self.grants) | set(self.listeners)),
                           "radios": len(self.radios), "regs": self.regs, "carriers_now": carriers_now,
                           "peak_carriers": self.peak_carriers},
                "rate": self.health[-1]["rate"] if self.health else None,
                "live": [{"tg": tg, "ago": round(now - t), **names.get(tg, {})} for tg, t in live],
                "timeline": timeline,
                "agency": agency.most_common(),
                "top": [{"tg": tg, "grants": self.grants[tg], "talkers": len(self.talkers[tg]),
                         "listeners": len(self.listeners[tg]), "ago": round(now - self.last_t.get(tg, now)),
                         **names.get(tg, {})} for tg in top],
                "recent": [{**r, **names.get(r["tg"], {})} for r in list(self.recent)[:15]],
                "health": list(self.health)[-120:],
                "alerts": list(self.alerts)[:8],
            }


STATE = State()

PAGE = r"""<!doctype html><html><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>Control Channel Live</title><style>
:root{color-scheme:light;--bg:#f6f5f2;--card:#fcfcfb;--text:#0b0b0b;--text2:#52514e;--muted:#8a8984;--grid:#e4e3df;
--s1:#2a78d6;--empty:#ecebe7;--good:#0ca30c;--crit:#d03b3b;--chip:#e8f0fb}
@media (prefers-color-scheme:dark){:root:not([data-theme="light"]){color-scheme:dark;--bg:#121211;--card:#1a1a19;--text:#fff;
--text2:#c3c2b7;--muted:#8f8e86;--grid:#383835;--s1:#3987e5;--empty:#262624;--chip:#1d2a3b}}
:root[data-theme="dark"]{color-scheme:dark;--bg:#121211;--card:#1a1a19;--text:#fff;--text2:#c3c2b7;--muted:#8f8e86;--grid:#383835;
--s1:#3987e5;--empty:#262624;--chip:#1d2a3b}
*{box-sizing:border-box}body{margin:0;background:var(--bg);color:var(--text);font:14px/1.45 system-ui,sans-serif;padding:16px}
main{max-width:1100px;margin:0 auto;display:grid;gap:14px;grid-template-columns:repeat(auto-fit,minmax(320px,1fr))}
header{max-width:1100px;margin:0 auto 14px;display:flex;align-items:baseline;gap:12px;flex-wrap:wrap}
h1{font-size:20px;margin:0}h2{font-size:14px;margin:0 0 8px;color:var(--text2);font-weight:600}
.card{background:var(--card);border:1px solid var(--grid);border-radius:14px;padding:14px;min-width:0}
.wide{grid-column:1/-1}.muted{color:var(--muted)}.tiles{display:flex;gap:22px;flex-wrap:wrap}
.tile b{display:block;font-size:24px;font-variant-numeric:tabular-nums}.tile span{color:var(--text2);font-size:12px}
.chips{display:flex;gap:8px;flex-wrap:wrap}.chip{background:var(--chip);border-radius:999px;padding:4px 10px;font-size:13px}
.chip small{color:var(--text2)}.status{display:inline-flex;align-items:center;gap:6px;font-size:13px;color:var(--text2)}
.dot{width:9px;height:9px;border-radius:50%;display:inline-block}
svg{width:100%;height:auto;display:block}.lab{font-size:11px;fill:var(--text2)}.axis{stroke:var(--grid)}
table{width:100%;border-collapse:collapse;font-size:13px}td,th{padding:3px 6px;border-bottom:1px solid var(--grid);text-align:left}
th{color:var(--text2);font-weight:600}.num{text-align:right;font-variant-numeric:tabular-nums;white-space:nowrap}
.lock{font-size:11px;color:var(--text2);border:1px solid var(--grid);border-radius:6px;padding:0 4px;margin-left:4px}
</style></head><body>
<header><h1>Control channel, live</h1><span class="status" id="status"></span><span class="muted" id="since"></span></header>
<main>
<section class="card wide"><div class="tiles" id="tiles"></div></section>
<section class="card wide"><h2>Live RF (antenna positioning)</h2><div class="tiles" id="rftiles"></div>
<p class="muted" style="margin:8px 0 2px">Decoded TSBKs per second, last 2 minutes (updates every second)</p><div id="persec"></div>
<p class="muted" style="margin:8px 0 2px">RSSI per summary window: mean line, min to max band</p><div id="rssi"></div></section>
<section class="card wide"><h2>On the air now</h2><div class="chips" id="live"></div></section>
<section class="card wide"><h2>Calls per 5 minutes</h2><p class="muted" style="margin:0 0 6px">Repeated grant messages for the same call are merged. Hatched bins had no collector running.</p><div id="timeline"></div></section>
<section class="card"><h2>Calls by agency</h2><div id="agency"></div></section>
<section class="card"><h2>Busiest talkgroups</h2><table id="top"></table></section>
<section class="card"><h2>Recent calls</h2><table id="recent"></table></section>
<section class="card"><h2>Site health</h2><p class="muted" style="margin:0">TSBK/s per summary window</p><div id="rate"></div>
<p class="muted" style="margin:6px 0 0">NIDs corrected by BCH per window</p><div id="nid"></div></section>
<section class="card wide"><h2>Board alerts</h2><div id="alerts" class="muted"></div></section>
</main>
<script>
const $=id=>document.getElementById(id);
const esc=s=>String(s??"").replace(/[&<>"]/g,c=>({"&":"&amp;","<":"&lt;",">":"&gt;",'"':"&quot;"}[c]));
const hhmm=t=>new Date(t*1000).toLocaleTimeString([], {hour:"2-digit",minute:"2-digit"});
const ago=s=>s<60?`${s}s ago`:s<3600?`${Math.round(s/60)}m ago`:`${(s/3600).toFixed(1)}h ago`;
const name=o=>o.alpha?`${esc(o.alpha)}${o.enc?'<span class="lock">enc</span>':""}`:`tg ${o.tg}`;
function bars(rows,label,val){ // horizontal, one series
  const max=Math.max(1,...rows.map(val)),h=22;
  return `<svg viewBox="0 0 340 ${rows.length*h+4}" role="img" aria-label="${label}">`+rows.map((r,i)=>{
    const w=Math.max(2,140*val(r)/max);
    return `<text x="150" y="${i*h+15}" text-anchor="end" class="lab">${esc(r.label)}</text>
    <rect x="156" y="${i*h+4}" width="${w.toFixed(1)}" height="14" rx="4" fill="var(--s1)"><title>${esc(r.label)}: ${val(r)} calls</title></rect>
    <text x="${(162+w).toFixed(1)}" y="${i*h+15}" class="lab">${val(r)}</text>`}).join("")+"</svg>";}
function line(pts,key,unit){
  if(!pts.length)return '<p class="muted">waiting for the first summary</p>';
  const W=340,H=90,max=Math.max(1,...pts.map(p=>p[key])),t0=pts[0].t,t1=Math.max(pts.at(-1).t,t0+1);
  const xy=pts.map(p=>[30+(W-40)*(p.t-t0)/(t1-t0),H-16-(H-26)*p[key]/max]);
  return `<svg viewBox="0 0 ${W} ${H}"><line x1="30" y1="${H-16}" x2="${W-10}" y2="${H-16}" class="axis"/>
  <text x="26" y="12" text-anchor="end" class="lab">${max}</text><text x="26" y="${H-16}" text-anchor="end" class="lab">0</text>
  <polyline points="${xy.map(p=>p.map(v=>v.toFixed(1)).join(",")).join(" ")}" fill="none" stroke="var(--s1)" stroke-width="2"/>
  ${xy.map((p,i)=>`<circle cx="${p[0].toFixed(1)}" cy="${p[1].toFixed(1)}" r="6" fill="transparent"><title>${hhmm(pts[i].t)}: ${pts[i][key]} ${unit}</title></circle>`).join("")}
  <text x="30" y="${H-2}" class="lab">${hhmm(t0)}</text><text x="${W-10}" y="${H-2}" text-anchor="end" class="lab">${hhmm(t1)}</text></svg>`;}
async function tick(){
  let s; try{s=await (await fetch("/api/state")).json();}catch(e){$("status").innerHTML='<span class="dot" style="background:var(--crit)"></span>dashboard server unreachable';return;}
  const ok=s.collector_age!==null&&s.collector_age<90;
  $("status").innerHTML=`<span class="dot" style="background:var(${ok?"--good":"--crit"})"></span>${ok?"collector receiving":"collector silent "+(s.collector_age?ago(Math.round(s.collector_age)):"")}`;
  $("since").textContent=s.t_first?`since ${new Date(s.t_first*1000).toLocaleString()}`:"";
  const T=s.totals;
  $("tiles").innerHTML=[["calls",T.calls],["talkgroups",T.talkgroups],["radios seen",T.radios],["radio registrations",T.regs],
    ["voice carriers now",T.carriers_now],["peak carriers",T.peak_carriers],["TSBK/s",s.rate??"-"]].map(([k,v])=>`<div class="tile"><b>${v}</b><span>${k}</span></div>`).join("");
  $("live").innerHTML=s.live.length?s.live.map(o=>`<span class="chip">${name(o)} <small>${esc(o.agency||"")} ${o.ago}s</small></span>`).join(""):'<span class="muted">quiet right now</span>';
  const tl=s.timeline,W=1000,H=120,max=Math.max(1,...tl.map(b=>b.n)),bw=(W-40)/Math.max(1,tl.length);
  $("timeline").innerHTML=`<svg viewBox="0 0 ${W} ${H}" role="img" aria-label="Calls per 5 minutes"><line x1="30" y1="${H-18}" x2="${W}" y2="${H-18}" class="axis"/>
   <text x="26" y="12" text-anchor="end" class="lab">${max}</text><text x="26" y="${H-18}" text-anchor="end" class="lab">0</text>`+
   `<defs><pattern id="gap" width="6" height="6" patternUnits="userSpaceOnUse" patternTransform="rotate(45)"><line x1="0" y1="0" x2="0" y2="6" stroke="var(--grid)" stroke-width="2"/></pattern></defs>`+
   tl.map((b,i)=>b.gap?`<rect x="${(32+i*bw).toFixed(1)}" y="10" width="${Math.max(1,bw-2).toFixed(1)}" height="${H-28}" fill="url(#gap)"><title>${hhmm(b.t)}: no data (collector not running)</title></rect>`:b.n?`<rect x="${(32+i*bw).toFixed(1)}" y="${(H-18-(H-28)*b.n/max).toFixed(1)}" width="${Math.max(1,bw-2).toFixed(1)}" height="${((H-28)*b.n/max).toFixed(1)}" rx="2" fill="var(--s1)"><title>${hhmm(b.t)}: ${b.n} calls</title></rect>`:"").join("")+
   (tl.length?`<text x="32" y="${H-3}" class="lab">${hhmm(tl[0].t)}</text><text x="${W}" y="${H-3}" text-anchor="end" class="lab">${hhmm(tl.at(-1).t)}</text>`:"")+"</svg>";
  $("agency").innerHTML=bars(s.agency.map(([a,n])=>({label:a,n})),"Grants by agency",r=>r.n);
  $("top").innerHTML="<tr><th>talkgroup</th><th class='num'>calls</th><th class='num'>talkers</th><th class='num'>listeners</th><th class='num'>last</th></tr>"+
    s.top.map(o=>`<tr><td title="${esc(o.desc||"")} ${esc(o.agency||"")}">${name(o)}</td><td class="num">${o.grants}</td><td class="num">${o.talkers}</td><td class="num">${o.listeners}</td><td class="num">${ago(o.ago)}</td></tr>`).join("");
  $("recent").innerHTML="<tr><th>time</th><th>talkgroup</th><th>radio</th><th class='num'>MHz</th></tr>"+
    s.recent.map(r=>`<tr><td>${hhmm(r.t)}</td><td>${name(r)}</td><td>${r.src??""}</td><td class="num">${r.mhz}</td></tr>`).join("");
  $("rate").innerHTML=line(s.health,"rate","TSBK/s");$("nid").innerHTML=line(s.health,"nid","NIDs corrected");
  $("alerts").innerHTML=s.alerts.length?s.alerts.map(a=>`<div>${hhmm(a.t)} ${esc(a.text)}</div>`).join("")+'<p class="muted">The 30 s alarm is known to be oversensitive; a longer-window rule is planned.</p>':"none";
}
async function live(){
  let s; try{s=await (await fetch("/api/live")).json();}catch(e){return;}
  const rf=s.rf.at(-1),age=rf?Math.round(s.now-rf.t):null;
  $("rftiles").innerHTML=[["TSBK/s, last 5 s",s.rate5.toFixed(1)],["TSBK/s, last 30 s",s.rate30.toFixed(1)],
    ["RSSI mean",rf?rf.mean+" dBm":"-"],["RSSI min to max",rf?`${rf.min} to ${rf.max}`:"-"],["AGC attenuation step",rf?(rf.agc_lo===rf.agc_hi?rf.agc_lo:`${rf.agc_lo} to ${rf.agc_hi}`):"-"],
    ["hardware AFC",rf?rf.afc:"-"],["software AFC LO",rf&&rf.lo!==null?`${rf.lo>0?"+":""}${rf.lo} Hz`:"not in firmware"],["RF line age",age===null?"-":age+" s"]]
    .map(([k,v])=>`<div class="tile"><b>${v}</b><span>${k}</span></div>`).join("");
  const W=1000,H=110,sec=s.secs,max=Math.max(5,...sec.map(x=>x.n)),bw=(W-40)/sec.length;
  $("persec").innerHTML=`<svg viewBox="0 0 ${W} ${H}" role="img" aria-label="Decoded TSBKs per second"><line x1="30" y1="${H-18}" x2="${W}" y2="${H-18}" class="axis"/>
    <text x="26" y="12" text-anchor="end" class="lab">${max}</text><text x="26" y="${H-18}" text-anchor="end" class="lab">0</text>`+
    sec.map((x,i)=>`<rect x="${(32+i*bw).toFixed(1)}" y="${(H-18-(H-28)*x.n/max).toFixed(1)}" width="${Math.max(1,bw-1.5).toFixed(1)}" height="${Math.max(0,(H-28)*x.n/max).toFixed(1)}" fill="var(--s1)"><title>${new Date(x.t*1000).toLocaleTimeString()}: ${x.n} TSBKs</title></rect>`).join("")+
    `<text x="32" y="${H-3}" class="lab">-2 min</text><text x="${W}" y="${H-3}" text-anchor="end" class="lab">now</text></svg>`;
  const r=s.rf;
  if(!r.length){$("rssi").innerHTML='<p class="muted">waiting for SUM rf lines (firmware with the RF health line, summary interval 5 s)</p>';return;}
  const lo=Math.min(...r.map(x=>x.min))-2,hi=Math.max(...r.map(x=>x.max))+2,t0=r[0].t,t1=Math.max(r.at(-1).t,t0+1),H2=130;
  const X=t=>32+(W-42)*(t-t0)/(t1-t0),Y=v=>H2-18-(H2-28)*(v-lo)/(hi-lo);
  const band=r.map(x=>`${X(x.t).toFixed(1)},${Y(x.max).toFixed(1)}`).join(" ")+" "+r.slice().reverse().map(x=>`${X(x.t).toFixed(1)},${Y(x.min).toFixed(1)}`).join(" ");
  $("rssi").innerHTML=`<svg viewBox="0 0 ${W} ${H2}" role="img" aria-label="RSSI per window"><line x1="30" y1="${H2-18}" x2="${W}" y2="${H2-18}" class="axis"/>
    <text x="26" y="12" text-anchor="end" class="lab">${hi}</text><text x="26" y="${H2-18}" text-anchor="end" class="lab">${lo}</text>
    <polygon points="${band}" fill="var(--s1)" opacity="0.18"/>
    <polyline points="${r.map(x=>`${X(x.t).toFixed(1)},${Y(x.mean).toFixed(1)}`).join(" ")}" fill="none" stroke="var(--s1)" stroke-width="2"/>
    ${r.map(x=>`<circle cx="${X(x.t).toFixed(1)}" cy="${Y(x.mean).toFixed(1)}" r="6" fill="transparent"><title>${new Date(x.t*1000).toLocaleTimeString()}: mean ${x.mean} dBm, ${x.min} to ${x.max}, AGC ${x.agc_lo}..${x.agc_hi}, AFC ${x.afc}</title></circle>`).join("")}
    <text x="32" y="${H2-3}" class="lab">${hhmm(t0)}</text><text x="${W}" y="${H2-3}" text-anchor="end" class="lab">${hhmm(t1)}</text></svg>`;
}
tick();setInterval(tick,5000);live();setInterval(live,1000);
</script></body></html>"""


class Handler(BaseHTTPRequestHandler):
    def do_GET(self):
        if self.path in ("/api/state", "/api/live"):
            STATE.poll()
            body = json.dumps(STATE.snapshot() if self.path == "/api/state" else STATE.live()).encode()
            ctype = "application/json"
        elif self.path in ("/", "/index.html"):
            body, ctype = PAGE.encode(), "text/html; charset=utf-8"
        else:
            self.send_error(404)
            return
        self.send_response(200)
        self.send_header("Content-Type", ctype)
        self.send_header("Cache-Control", "no-store")
        self.end_headers()
        self.wfile.write(body)

    def log_message(self, *a):
        pass


if __name__ == "__main__":
    STATE.poll()
    print(f"dashboard on http://127.0.0.1:{PORT}/ ({len(names)} talkgroup names loaded)")
    ThreadingHTTPServer(("127.0.0.1", PORT), Handler).serve_forever()
