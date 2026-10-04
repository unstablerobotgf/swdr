"""Check the board's per-window `updates` counts against GRP_VCH_GRANT_UPD lines in a raw log.

The board counts a talkgroup once per update message even when both (channel, tg) slots repeat it.
usage: verify_updates.py RAW_LOG   (lines: "<unix time>\\t<board line>", from swdr-p25 live --raw)
"""
import re
import sys
from collections import Counter

windows, cur = [], Counter()
pending_sum = None
for raw in open(sys.argv[1], encoding="utf-8", errors="replace"):
    line = raw.rstrip("\n").split("\t", 1)[-1]
    if line.startswith("SUM t="):
        if pending_sum is not None:
            windows.append(pending_sum)
        pending_sum = (cur, {})
        cur = Counter()
    elif line.startswith("SUM tg=") and pending_sum is not None:
        m = re.search(r"tg=(\d+) grants=(\d+) updates=(\d+)", line)
        pending_sum[1][int(m.group(1))] = int(m.group(3))
    elif " GRP_VCH_GRANT_UPD " in line:
        tg1 = int(re.search(r"tg1=(\d+)", line).group(1))
        tg2 = int(re.search(r"tg2=(\d+)", line).group(1))
        for tg in {tg1, tg2}:
            cur[tg] += 1
if pending_sum is not None:
    windows.append(pending_sum)

# The first window may have started before the log did, so skip it.
checked = mismatch = 0
for counted, board in windows[1:]:
    for tg, n in board.items():
        checked += 1
        if counted[tg] != n:
            mismatch += 1
            print(f"mismatch tg={tg}: board {n}, log {counted[tg]}")
print(f"{len(windows) - 1} complete windows, {checked} talkgroup rows checked, {mismatch} mismatches")
