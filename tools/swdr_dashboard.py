#!/usr/bin/env python3
"""Dashboard for swdr-tap on the MP257F-DK: control channel health, followed calls, call audio.

usage: swdr_dashboard.py [DATA_DIR] [--port 8025] [--bind 0.0.0.0]
DATA_DIR (default /var/lib/swdr) holds events.log (swdr-tap stdout: TSBK, HOP and P2 lines) and
calls/ (WAVs from --calls). An optional DATA_DIR/talkgroups.tsv (id, alpha tag, description, ...)
names talkgroups. Standard library only. It shows talkgroup and radio IDs and plays recorded
calls, so bind it to a trusted network.
"""
import argparse
import json
import os
import re
import threading
import time
from collections import Counter, deque
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

ap = argparse.ArgumentParser()
ap.add_argument("data", nargs="?", default="/var/lib/swdr")
ap.add_argument("--port", type=int, default=8025)
ap.add_argument("--bind", default="0.0.0.0")
A = ap.parse_args()
LOG = os.path.join(A.data, "events.log")
CALLS = os.path.join(A.data, "calls")
WAV_RE = re.compile(r"^(\d+)_tg(\d+)_(\d+)_(clear|alg-unknown)\.wav$")
HOP_RE = re.compile(r"^HOP (\d+) (start|end) (.*)$")

NAMES = {}
_tsv = os.path.join(A.data, "talkgroups.tsv")
if os.path.exists(_tsv):
    for i, line in enumerate(open(_tsv, encoding="utf-8")):
        p = line.rstrip("\n").split("\t")
        if i and len(p) >= 2 and p[0].isdigit():
            NAMES[int(p[0])] = p[1]


class State:
    def __init__(self):
        self.lock = threading.Lock()
        self.off = 0
        self.tsbk = deque()  # timestamps, last 10 min
        self.tsbk_total = 0
        self.last_tsbk = None
        self.grants = deque()  # (t, tg), last hour
        self.hops = deque(maxlen=200)  # end records
        self.p2 = 0

    def poll(self):
        try:
            size = os.path.getsize(LOG)
        except OSError:
            return
        if size < self.off:  # logrotate copytruncate
            self.off = 0
        with open(LOG, "rb") as f:
            f.seek(self.off)
            data = f.read()
        cut = data.rfind(b"\n") + 1
        self.off += cut
        now = time.time()
        with self.lock:
            for raw in data[:cut].decode("utf-8", "replace").splitlines():
                t, _, rest = raw.partition("\t")
                try:
                    t = float(t)
                except ValueError:
                    continue
                if rest.startswith("nac="):
                    self.tsbk.append(t)
                    self.tsbk_total += 1
                    self.last_tsbk = t
                    m = re.search(r"GRP_VCH_GRANT tg=(\d+)", rest)
                    if m:
                        self.grants.append((t, int(m.group(1))))
                elif rest.startswith("HOP "):
                    m = HOP_RE.match(rest)
                    if m and m.group(2) == "end":
                        kv = dict(x.split("=", 1) for x in m.group(3).split() if "=" in x)
                        kv["t"], kv["n"] = t, int(m.group(1))
                        self.hops.append(kv)
                elif "P2 slot=" in rest:
                    self.p2 += 1
            while self.tsbk and self.tsbk[0] < now - 600:
                self.tsbk.popleft()
            while self.grants and self.grants[0][0] < now - 3600:
                self.grants.popleft()

    def snapshot(self):
        now = time.time()
        with self.lock:
            rate60 = sum(1 for t in self.tsbk if t > now - 60) / 60
            per_min = Counter(int((now - t) // 60) for t in self.tsbk)
            hops = list(self.hops)[::-1][:60]
            grants = Counter(tg for _, tg in self.grants).most_common(12)
            out = {
                "now": now,
                "cc": {"rate60": rate60, "total": self.tsbk_total, "age": None if self.last_tsbk is None else now - self.last_tsbk,
                       "per_min": [per_min.get(i, 0) / 60 for i in range(9, -1, -1)]},
                "p2": self.p2,
                "hops": hops,
                "hour": {"hops": sum(1 for h in self.hops if h["t"] > now - 3600),
                         "clear": sum(1 for h in self.hops if h["t"] > now - 3600 and h.get("alg") == "0x80"),
                         "enc": sum(1 for h in self.hops if h["t"] > now - 3600 and h.get("alg") not in ("0x80", "-", None)),
                         "unknown": sum(1 for h in self.hops if h["t"] > now - 3600 and h.get("alg") in ("-", None))},
                "grants": [{"tg": tg, "n": n, "name": NAMES.get(tg, "")} for tg, n in grants],
                "names": {str(h.get("tg")): NAMES.get(int(h["tg"]), "") for h in hops if str(h.get("tg", "")).isdigit()},
            }
        calls = []
        try:
            for name in sorted(os.listdir(CALLS), reverse=True)[:80]:
                m = WAV_RE.match(name)
                if m:
                    calls.append({"file": name, "t": int(m.group(1)), "tg": int(m.group(2)), "hz": int(m.group(3)), "tag": m.group(4),
                                  "s": max(0, os.path.getsize(os.path.join(CALLS, name)) - 44) / 16000, "name": NAMES.get(int(m.group(2)), "")})
        except OSError:
            pass
        out["calls"] = calls
        return out


S = State()


def poller():
    while True:
        S.poll()
        time.sleep(1)


PAGE = r"""<!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>swdr on mp2</title><style>
:root{--bg:#f7f7f5;--card:#fff;--ink:#1d1d1f;--muted:#6b6b70;--line:#e3e3e0;--accent:#2f6fde;--good:#1e8e5a;--warn:#b26b00;--crit:#c0392b}
@media (prefers-color-scheme:dark){:root{--bg:#141416;--card:#1d1d20;--ink:#ececef;--muted:#9a9aa2;--line:#2c2c31;--accent:#6c9cf0;--good:#4cc38a;--warn:#e0a040;--crit:#ef6b5c}}
*{box-sizing:border-box}body{margin:0;background:var(--bg);color:var(--ink);font:14px/1.45 system-ui,-apple-system,Segoe UI,sans-serif}
main{max-width:1200px;margin:0 auto;padding:16px}h1{font-size:18px;margin:0 0 4px}h2{font-size:14px;margin:0 0 10px;color:var(--muted);font-weight:600}
.sub{color:var(--muted);margin-bottom:14px}.grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(150px,1fr));gap:10px;margin-bottom:14px}
.card{background:var(--card);border:1px solid var(--line);border-radius:10px;padding:12px 14px}.kpi .v{font-size:24px;font-weight:650;font-variant-numeric:tabular-nums}.kpi .l{color:var(--muted);font-size:12px}
.two{display:grid;grid-template-columns:2fr 1fr;gap:10px}@media (max-width:820px){.two{grid-template-columns:1fr}}
table{width:100%;border-collapse:collapse;font-variant-numeric:tabular-nums}th,td{text-align:left;padding:6px 6px;border-bottom:1px solid var(--line);white-space:nowrap}th{color:var(--muted);font-weight:600;font-size:12px}
td.n{text-align:right}.wrap{overflow-x:auto}.pill{display:inline-block;padding:1px 7px;border-radius:999px;font-size:12px;border:1px solid var(--line)}
.clear{color:var(--good)}.enc{color:var(--crit)}.unk{color:var(--warn)}.bar{height:8px;background:var(--accent);border-radius:4px}
.spark{display:flex;align-items:flex-end;gap:3px;height:36px;margin-top:6px}.spark div{flex:1;background:var(--accent);border-radius:2px 2px 0 0;min-height:1px;opacity:.85}
audio{height:28px;max-width:220px}.dot{display:inline-block;width:8px;height:8px;border-radius:50%;margin-right:6px;vertical-align:middle}
</style></head><body><main>
<h1>swdr on mp2</h1><div class="sub" id="status">connecting...</div>
<div class="grid">
 <div class="card kpi"><div class="v" id="k_rate">-</div><div class="l">CC TSBK/s (60 s)</div><div class="spark" id="spark" title="TSBK/s per minute, last 10 min"></div></div>
 <div class="card kpi"><div class="v" id="k_hops">-</div><div class="l">calls followed (1 h)</div></div>
 <div class="card kpi"><div class="v clear" id="k_clear">-</div><div class="l">clear (algid 0x80)</div></div>
 <div class="card kpi"><div class="v enc" id="k_enc">-</div><div class="l">encrypted, not recorded</div></div>
 <div class="card kpi"><div class="v unk" id="k_unk">-</div><div class="l">algid not seen</div></div>
</div>
<div class="two">
 <div class="card"><h2>Followed calls</h2><div class="wrap"><table><thead><tr><th>time</th><th>talkgroup</th><th>MHz</th><th>slot</th><th class="n">s</th><th>ended</th><th>crypto</th><th class="n">ACCH</th><th>audio</th></tr></thead><tbody id="hops"></tbody></table></div></div>
 <div class="card"><h2>Grants, last hour</h2><table><tbody id="grants"></tbody></table></div>
</div>
<div class="card" style="margin-top:10px"><h2>Recorded calls</h2><div class="wrap"><table><thead><tr><th>time</th><th>talkgroup</th><th>MHz</th><th class="n">s</th><th>crypto</th><th>play</th></tr></thead><tbody id="calls"></tbody></table></div></div>
</main><script>
const $=id=>document.getElementById(id);
const esc=s=>String(s).replace(/[&<>"]/g,c=>({"&":"&amp;","<":"&lt;",">":"&gt;",'"':"&quot;"}[c]));
const tm=t=>new Date(t*1000).toLocaleTimeString([], {hour:"2-digit",minute:"2-digit",second:"2-digit"});
const tgl=(tg,name)=>esc(tg)+(name?' <span style="color:var(--muted)">'+esc(name)+'</span>':'');
const crypto=a=>a==="0x80"?'<span class="pill clear">clear</span>':(a&&a!=="-"?'<span class="pill enc">'+esc(a)+'</span>':'<span class="pill unk">unknown</span>');
const player=f=>'<audio controls preload="none" src="/calls/'+encodeURIComponent(f)+'"></audio>';
let playing=false;document.addEventListener("play",()=>playing=true,true);document.addEventListener("pause",()=>playing=false,true);
async function tick(){
 let s;try{s=await (await fetch("/api/state")).json();}catch(e){$("status").innerHTML='<span class="dot" style="background:var(--crit)"></span>dashboard unreachable';return;}
 const age=s.cc.age, ok=age!==null&&age<10;
 $("status").innerHTML='<span class="dot" style="background:'+(ok?"var(--good)":"var(--crit)")+'"></span>'+(ok?"control channel decoding":"no TSBK for "+(age===null?"ever":Math.round(age)+" s"))+" &middot; "+s.cc.total.toLocaleString()+" TSBKs &middot; "+s.p2.toLocaleString()+" Phase 2 MAC PDUs";
 $("k_rate").textContent=s.cc.rate60.toFixed(1);$("k_hops").textContent=s.hour.hops;$("k_clear").textContent=s.hour.clear;$("k_enc").textContent=s.hour.enc;$("k_unk").textContent=s.hour.unknown;
 const mx=Math.max(1,...s.cc.per_min);$("spark").innerHTML=s.cc.per_min.map(v=>'<div title="'+v.toFixed(1)+'/s" style="height:'+(100*v/mx)+'%"></div>').join("");
 $("hops").innerHTML=s.hops.map(h=>'<tr><td>'+tm(h.t)+'</td><td>'+tgl(h.tg,s.names[h.tg])+'</td><td>'+(h.hz/1e6).toFixed(4)+'</td><td>'+esc(h.slot)+'</td><td class="n">'+esc(h.dwell)+'</td><td>'+esc((h.why||"").replace(/_/g," "))+'</td><td>'+crypto(h.alg)+'</td><td class="n">'+esc(h.acch)+'</td><td>'+(h.audio&&h.audio!=="-"?player(h.audio):"")+'</td></tr>').join("")||'<tr><td colspan="9" style="color:var(--muted)">no calls followed yet</td></tr>';
 const gm=Math.max(1,...s.grants.map(g=>g.n));
 $("grants").innerHTML=s.grants.map(g=>'<tr><td>'+tgl(g.tg,g.name)+'</td><td style="width:45%"><div class="bar" style="width:'+(100*g.n/gm)+'%"></div></td><td class="n">'+g.n+'</td></tr>').join("")||'<tr><td style="color:var(--muted)">none yet</td></tr>';
 if(!playing)$("calls").innerHTML=s.calls.map(c=>'<tr><td>'+tm(c.t)+'</td><td>'+tgl(c.tg,c.name)+'</td><td>'+(c.hz/1e6).toFixed(4)+'</td><td class="n">'+c.s.toFixed(1)+'</td><td>'+crypto(c.tag==="clear"?"0x80":"-")+'</td><td>'+player(c.file)+'</td></tr>').join("")||'<tr><td colspan="6" style="color:var(--muted)">no recordings</td></tr>';
}
tick();setInterval(tick,2000);
</script></body></html>"""


class Handler(BaseHTTPRequestHandler):
    def log_message(self, *a):
        pass

    def send(self, code, ctype, body):
        self.send_response(code)
        self.send_header("Content-Type", ctype)
        self.send_header("Content-Length", str(len(body)))
        self.send_header("Cache-Control", "no-store")
        self.end_headers()
        self.wfile.write(body)

    def do_GET(self):
        if self.path == "/":
            self.send(200, "text/html; charset=utf-8", PAGE.encode())
        elif self.path == "/api/state":
            self.send(200, "application/json", json.dumps(S.snapshot()).encode())
        elif self.path.startswith("/calls/"):
            name = self.path[len("/calls/"):].split("?")[0]
            if not WAV_RE.match(name):  # strict name check: no traversal, only our recordings
                return self.send(404, "text/plain", b"not found")
            try:
                body = open(os.path.join(CALLS, name), "rb").read()
            except OSError:
                return self.send(404, "text/plain", b"not found")
            self.send(200, "audio/wav", body)
        else:
            self.send(404, "text/plain", b"not found")


if __name__ == "__main__":
    try:  # start near the end: events.log grows ~200 MB/day between rotations
        S.off = max(0, os.path.getsize(LOG) - 20_000_000)
    except OSError:
        pass
    S.poll()
    threading.Thread(target=poller, daemon=True).start()
    print(f"swdr_dashboard: http://{A.bind}:{A.port}/ ({A.data})", flush=True)
    ThreadingHTTPServer((A.bind, A.port), Handler).serve_forever()
