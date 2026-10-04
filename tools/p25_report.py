"""Self-contained HTML report from swdr-p25 output (events CSV + raw log). Local use only:
the output contains radio and talkgroup IDs from the monitored system.
usage: p25_report.py EVENTS_CSV RAW_LOG OUT_HTML [BIN_MINUTES]
"""
import html
import re
import sys
import time
from collections import Counter, defaultdict

ev_path, raw_path, out_path = sys.argv[1:4]
BIN = int(sys.argv[4]) * 60 if len(sys.argv) > 4 else 300

events = []
for line in open(ev_path, encoding="utf-8"):
    p = line.rstrip("\n").split(",")
    if len(p) < 5 or p[0] == "t":
        continue
    events.append((float(p[0]), p[1], int(p[2]) if p[2] else None, int(p[3]) if p[3] else None, p[4]))
health = []
for line in open(raw_path, encoding="utf-8", errors="replace"):
    t, _, body = line.rstrip("\n").partition("\t")
    if body.startswith("SUM t="):
        kv = dict(re.findall(r"(\w+)=([\w.]+)", body))
        win = float(kv["window"].rstrip("s"))
        health.append((float(t), int(kv["tsbk"]) / win, int(kv["nid_fixed"]), int(kv["trellis_e"]), int(kv["nid_rej"])))

t0 = min(e[0] for e in events)
t1 = max(e[0] for e in events)
nb = int((t1 - t0) // BIN) + 1
grants = Counter()
talkers, listeners = defaultdict(set), defaultdict(set)
bins = defaultdict(lambda: [0] * nb)
unit_tg, onoff = defaultdict(Counter), []
for t, kind, tg, unit, _ in events:
    if kind == "talk" and tg is not None:
        grants[tg] += 1
        talkers[tg].add(unit)
        bins[tg][int((t - t0) // BIN)] += 1
        unit_tg[unit][tg] += 1
    elif kind == "listen" and tg is not None:
        listeners[tg].add(unit)
        unit_tg[unit][tg] += 0
    elif kind in ("on", "off"):
        onoff.append((t, kind, unit))

esc = html.escape
clock = lambda t: time.strftime("%H:%M", time.localtime(t))

# --- heatmap: talkgroups (rows) x time bins, grant counts, sequential blue ---
SEQ = ["#cde2fb", "#9ec5f4", "#6da7ec", "#3987e5", "#256abf", "#184f95", "#0d366b"]
top = [tg for tg, _ in grants.most_common(20)]
peak = max((max(bins[tg]) for tg in top), default=1) or 1
# Fixed 830-unit coordinate width so text keeps its size however many bins there are.
lw, ch = 70, 18
cw = (830 - lw - 10) / max(nb, 1)
hm = [f'<svg viewBox="0 0 830 {len(top) * ch + 30}" role="img" aria-label="Grants per talkgroup over time">']
for r, tg in enumerate(top):
    y = r * ch
    hm.append(f'<text x="{lw - 6}" y="{y + 13}" text-anchor="end" class="lab">tg {tg}</text>')
    for c, n in enumerate(bins[tg]):
        if n == 0:
            fill = "var(--empty)"
        else:
            fill = SEQ[min(len(SEQ) - 1, (n * len(SEQ) - 1) // peak)]
        hm.append(
            f'<rect x="{lw + c * cw:.1f}" y="{y}" width="{cw - 2:.1f}" height="{ch - 2}" rx="2" fill="{fill}">'
            f"<title>tg {tg}, {clock(t0 + c * BIN)}: {n} grants</title></rect>"
        )
for c in range(0, nb, max(1, nb // 8)):
    hm.append(f'<text x="{lw + c * cw:.1f}" y="{len(top) * ch + 14}" class="lab">{clock(t0 + c * BIN)}</text>')
hm.append("</svg>")
legend = "".join(f'<span class="sw" style="background:{s}"></span>' for s in SEQ)

# --- bars: grants for the busiest talkgroups (one series) ---
bw = 680  # 830-unit coordinate width, matching the other charts
bars = [f'<svg viewBox="0 0 830 {len(top) * 22 + 10}" role="img" aria-label="Grants per talkgroup">']
gmax = max((grants[tg] for tg in top), default=1)
for r, tg in enumerate(top):
    w = max(2, bw * grants[tg] / gmax)
    y = r * 22
    bars.append(f'<text x="64" y="{y + 15}" text-anchor="end" class="lab">tg {tg}</text>')
    bars.append(
        f'<rect x="70" y="{y + 3}" width="{w:.1f}" height="14" rx="4" fill="var(--series-1)">'
        f"<title>tg {tg}: {grants[tg]} grants, {len(talkers[tg])} talkers, {len(listeners[tg])} listeners</title></rect>"
    )
    bars.append(f'<text x="{76 + w:.1f}" y="{y + 15}" class="val">{grants[tg]}</text>')
bars.append("</svg>")

# --- site health: two single-series charts sharing the time axis ---
def line_chart(vals, label, unit):
    if not health:
        return "<p class='muted'>no SUM lines in the raw log</p>"
    W, H, P = 720, 120, 36
    xs = [P + (W - P - 10) * (h[0] - health[0][0]) / max(1, health[-1][0] - health[0][0]) for h in health]
    vmax = max(vals) or 1
    ys = [H - 18 - (H - 30) * v / vmax for v in vals]
    pts = " ".join(f"{x:.1f},{y:.1f}" for x, y in zip(xs, ys))
    dots = "".join(
        f'<circle cx="{x:.1f}" cy="{y:.1f}" r="4" fill="var(--series-1)" stroke="var(--surface)" stroke-width="2">'
        f"<title>{clock(h[0])}: {v:g} {unit}</title></circle>"
        for x, y, v, h in zip(xs, ys, vals, health)
    )
    return (
        f'<svg viewBox="0 0 {W} {H}" role="img" aria-label="{esc(label)}">'
        f'<line x1="{P}" y1="{H - 18}" x2="{W - 10}" y2="{H - 18}" class="axis"/>'
        f'<text x="{P - 4}" y="16" text-anchor="end" class="lab">{vmax:g}</text>'
        f'<text x="{P - 4}" y="{H - 18}" text-anchor="end" class="lab">0</text>'
        f'<polyline points="{pts}" fill="none" stroke="var(--series-1)" stroke-width="2"/>{dots}'
        f'<text x="{P}" y="{H - 2}" class="lab">{clock(health[0][0])}</text>'
        f'<text x="{W - 10}" y="{H - 2}" text-anchor="end" class="lab">{clock(health[-1][0])}</text></svg>'
    )

rate_svg = line_chart([round(h[1], 1) for h in health], "TSBK rate per window", "TSBK/s")
corr_svg = line_chart([h[2] for h in health], "NID corrections per window", "NIDs corrected")

# --- concurrency: distinct voice carriers (frequencies) in use per 10 s, for receiver sizing ---
CONC_BIN = 10
carriers = defaultdict(set)
for t, kind, tg, unit, ch in events:
    m = re.search(r"\(([\d.]+)MHz\)", ch)
    if kind in ("talk", "active") and m:
        carriers[int((t - t0) // CONC_BIN)].add(m.group(1))
nconc = int((t1 - t0) // CONC_BIN) + 1
conc = [len(carriers[i]) for i in range(nconc)]
peak_conc = max(conc) if conc else 0
cmax = max(peak_conc, 1)
cw2 = 720 / max(nconc, 1)
cs = [f'<svg viewBox="0 0 740 130" role="img" aria-label="Voice carriers in use per 10 seconds">',
      f'<line x1="30" y1="110" x2="740" y2="110" class="axis"/>',
      f'<text x="26" y="16" text-anchor="end" class="lab">{cmax}</text><text x="26" y="110" text-anchor="end" class="lab">0</text>']
for i, n in enumerate(conc):
    if n:
        h = 94 * n / cmax
        cs.append(f'<rect x="{30 + i * cw2:.1f}" y="{110 - h:.1f}" width="{max(cw2 - 1, 1):.1f}" height="{h:.1f}" fill="var(--series-1)">'
                  f"<title>{clock(t0 + i * CONC_BIN)}: {n} carrier(s) in use</title></rect>")
cs.append(f'<text x="30" y="126" class="lab">{clock(t0)}</text><text x="740" y="126" text-anchor="end" class="lab">{clock(t1)}</text></svg>')
dist = Counter(conc)
conc_note = ", ".join(f"{k} carrier(s): {100 * v / nconc:.0f}% of the time" for k, v in sorted(dist.items()))

# --- radio on/off timeline (two categories) ---
W = 720
oo = [f'<svg viewBox="0 0 {W} 70" role="img" aria-label="Radio registrations over time">']
for t, kind, unit in onoff:
    x = 20 + (W - 30) * (t - t0) / max(1, t1 - t0)
    y, col = (22, "var(--series-1)") if kind == "on" else (48, "var(--series-2)")
    oo.append(
        f'<circle cx="{x:.1f}" cy="{y}" r="4" fill="{col}" stroke="var(--surface)" stroke-width="2">'
        f"<title>radio {unit} {kind} at {clock(t)}</title></circle>"
    )
oo.append(f'<text x="20" y="68" class="lab">{clock(t0)}</text><text x="{W - 10}" y="68" text-anchor="end" class="lab">{clock(t1)}</text></svg>')
n_on = sum(1 for o in onoff if o[1] == "on")

# --- groupings table: radios by primary talkgroup ---
groups = defaultdict(list)
for unit, c in unit_tg.items():
    if unit is None or not c:
        continue
    prim = max(c.items(), key=lambda kv: (kv[1], -kv[0]))[0]
    groups[prim].append(unit)
grp_rows = "".join(
    f"<tr><td>{tg}</td><td class='num'>{len(us)}</td><td>{esc(' '.join(str(u) for u in sorted(us)[:24]))}"
    f"{' ...' if len(us) > 24 else ''}</td></tr>"
    for tg, us in sorted(groups.items(), key=lambda kv: -len(kv[1]))[:25]
)
tg_rows = "".join(
    f"<tr><td>{tg}</td><td class='num'>{grants[tg]}</td><td class='num'>{len(talkers[tg])}</td><td class='num'>{len(listeners[tg])}</td></tr>"
    for tg in sorted(set(grants) | set(listeners), key=lambda t: (-grants[t], -len(listeners[t])))[:40]
)

page = f"""<!doctype html><html><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>P25 Activity Report</title><style>
:root {{ color-scheme: light; --surface:#fcfcfb; --text:#0b0b0b; --text2:#52514e; --muted:#8a8984; --grid:#e4e3df;
  --empty:#f0efec; --series-1:#2a78d6; --series-2:#eb6834; }}
@media (prefers-color-scheme: dark) {{ :root:not([data-theme="light"]) {{ color-scheme: dark; --surface:#1a1a19; --text:#fff;
  --text2:#c3c2b7; --muted:#8f8e86; --grid:#383835; --empty:#262624; --series-1:#3987e5; --series-2:#d95926; }} }}
:root[data-theme="dark"] {{ color-scheme: dark; --surface:#1a1a19; --text:#fff; --text2:#c3c2b7; --muted:#8f8e86;
  --grid:#383835; --empty:#262624; --series-1:#3987e5; --series-2:#d95926; }}
body {{ background:var(--surface); color:var(--text); font:14px/1.45 system-ui,sans-serif; margin:0; padding:16px; }}
main {{ max-width:860px; margin:0 auto; }} h1 {{ font-size:20px; }} h2 {{ font-size:16px; margin-top:28px; }}
svg {{ width:100%; height:auto; }} .lab {{ font-size:11px; fill:var(--text2); }} .val {{ font-size:11px; fill:var(--text); }}
.axis {{ stroke:var(--grid); }} .muted {{ color:var(--muted); }} table {{ border-collapse:collapse; width:100%; font-size:13px; }}
td,th {{ text-align:left; padding:3px 8px; border-bottom:1px solid var(--grid); }} .num {{ text-align:right; font-variant-numeric:tabular-nums; }}
.sw {{ display:inline-block; width:16px; height:10px; border-radius:2px; margin-right:2px; }}
.key {{ display:inline-flex; align-items:center; gap:6px; margin-right:14px; color:var(--text2); }}
.dot {{ width:10px; height:10px; border-radius:50%; display:inline-block; }}
.stats {{ display:flex; gap:24px; flex-wrap:wrap; }} .stat b {{ display:block; font-size:22px; }}
</style></head><body><main>
<h1>P25 control channel activity</h1>
<p class="muted">{clock(t0)} to {clock(t1)} ({(t1 - t0) / 60:.0f} min). Local report: contains IDs from the monitored system.</p>
<div class="stats"><div class="stat"><b>{sum(grants.values())}</b>voice grants</div><div class="stat"><b>{len(set(grants) | set(listeners))}</b>talkgroups</div>
<div class="stat"><b>{len([u for u in unit_tg if u is not None])}</b>radios</div><div class="stat"><b>{n_on}</b>radio registrations</div><div class="stat"><b>{peak_conc}</b>peak voice carriers in use</div></div>
<h2>Grants per talkgroup over time ({BIN // 60}-minute bins)</h2>{''.join(hm)}
<p class="lab">fewer {legend} more grants per bin; empty cells had none</p>
<h2>Voice carriers in use (receiver sizing)</h2><p class="muted">Distinct voice frequencies seen in grants or grant updates per 10 s; both TDMA slots of a carrier count once. {conc_note}.</p>{''.join(cs)}
<h2>Busiest talkgroups</h2>{''.join(bars)}
<h2>Site health</h2><p class="muted">TSBK/s per 30 s window</p>{rate_svg}<p class="muted">NIDs corrected by BCH per window</p>{corr_svg}
<h2>Radio registrations</h2><p><span class="key"><span class="dot" style="background:var(--series-1)"></span>on (registered)</span>
<span class="key"><span class="dot" style="background:var(--series-2)"></span>off (deregistered)</span></p>{''.join(oo)}
<h2>Radios grouped by primary talkgroup</h2><table><tr><th>talkgroup</th><th class="num">radios</th><th>radio IDs</th></tr>{grp_rows}</table>
<h2>Talkgroups (table view)</h2><table><tr><th>talkgroup</th><th class="num">grants</th><th class="num">talkers</th><th class="num">listeners</th></tr>{tg_rows}</table>
</main></body></html>"""
open(out_path, "w", encoding="utf-8").write(page)
print(f"wrote {out_path}: {len(events)} events, {len(grants)} talkgroups with grants, {len(health)} health windows")
