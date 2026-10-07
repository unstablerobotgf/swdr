#!/usr/bin/env python3
"""P25 Phase 2 voice frames: extract the AMBE+2 3600x2450 codewords from 4V/2V bursts and check FEC.

usage: p2_voice.py IQ_FILE --nac HEX --sysid HEX --wacn HEX [--lch 0|1]
Uses p2_mac.py for demod, slot grid, DUID and descrambling. Each 72-bit vocoder frame is
deinterleaved into c0 (Golay 24,12), c1 (Golay 23,12, masked by a PN sequence seeded from u0),
c2 (11 bits) and c3 (14 bits). Reports Golay-corrected bit errors per frame; with --out the
deinterleaved frames are written (9 bytes each, c0..c3 MSB first) for an optional vocoder.
No vocoder is linked here: AMBE+2 is a DVSI codec under patent.
"""
import argparse
import collections
import numpy as np
import p2_mac
import rs63

# Voice frame bit k (0..71, from the 36 dibits, MSB of each dibit first) -> (codeword, bit),
# bit 0 being the LSB of that codeword (TIA-102.BBAC voice codeword interleave).
VCW = [
    (0, 23), (0,  5), (1, 10), (2,  3), (0, 22), (0,  4), (1,  9), (2,  2),
    (0, 21), (0,  3), (1,  8), (2,  1), (0, 20), (0,  2), (1,  7), (2,  0),
    (0, 19), (0,  1), (1,  6), (3, 13), (0, 18), (0,  0), (1,  5), (3, 12),
    (0, 17), (1, 22), (1,  4), (3, 11), (0, 16), (1, 21), (1,  3), (3, 10),
    (0, 15), (1, 20), (1,  2), (3,  9), (0, 14), (1, 19), (1,  1), (3,  8),
    (0, 13), (1, 18), (1,  0), (3,  7), (0, 12), (1, 17), (2, 10), (3,  6),
    (0, 11), (1, 16), (2,  9), (3,  5), (0, 10), (1, 15), (2,  8), (3,  4),
    (0,  9), (1, 14), (2,  7), (3,  3), (0,  8), (1, 13), (2,  6), (3,  2),
    (0,  7), (1, 12), (2,  5), (3,  1), (0,  6), (1, 11), (2,  4), (3,  0),
]
LEN = (24, 23, 11, 14)
G23 = 0xC75  # Golay(23,12) generator x^11+x^10+x^6+x^5+x^4+x^2+1
VOICE = {0: (11, 48, 96, 133), 6: (11, 48)}  # 4V, 2V: frame starts in the descrambled burst


def _rem(v):
    for i in range(22, 10, -1):
        if (v >> i) & 1:
            v ^= G23 << (i - 11)
    return v


def _golay_table():
    t = {}
    for w in range(1, 4):
        for bits in __import__("itertools").combinations(range(23), w):
            e = sum(1 << b for b in bits)
            t.setdefault(_rem(e), e)
    return t


SYND = _golay_table()


def golay23(cw):
    """(data, corrected bits) or (None, None) if the syndrome is not correctable."""
    s = _rem(cw)
    if s == 0:
        return cw >> 11, 0
    e = SYND.get(s)
    return (None, None) if e is None else ((cw ^ e) >> 11, bin(e).count("1"))


def golay24(cw):
    d, n = golay23(cw >> 1)  # overall parity in the LSB
    return d, n


def pn_mask(u0):
    p, m = 16 * u0, 0
    for _ in range(23):
        p = (173 * p + 13849) % 65536
        m = (m << 1) | (p >> 15)
    return m


def frame(dibits):
    c = [0, 0, 0, 0]
    bits = [x for d in map(int, dibits) for x in ((d >> 1) & 1, d & 1)]
    for k, (w, b) in enumerate(VCW):
        c[w] |= bits[k] << b
    u0, e0 = golay24(c[0])
    if u0 is None:
        return None, c
    u1, e1 = golay23(c[1] ^ pn_mask(u0))
    if u1 is None:
        return None, c
    return (e0 + e1, (u0, u1, c[2], c[3])), c


def ess(slots):
    """Encryption sync per voice superframe on one logical channel. A channel's superframe is six
    bursts, 4V 4V SACCH 4V 4V 2V; the 2V (offset 84, skipping the DUID dibit at 132) carries 28
    hexbits of ESS-A and the four 4V bursts 5, 4, 2 and 1 bursts before it carry 4 hexbits of
    ESS-B each (offset 84), as RS(63,35) shortened to [19 zero | 16 ESS-B | 28 ESS-A]. A missing
    4V becomes four erasures. Yields (algid, keyid, mi, corrected) or None per superframe."""
    hexbit = lambda d: (int(d[0]) << 4) | (int(d[1]) << 2) | int(d[2])
    for t, (d, b) in enumerate(slots):
        if d != 6 or t < 5:
            continue
        cw, era = [0] * 19, []
        for k, back in enumerate((5, 4, 2, 1)):
            d4, b4 = slots[t - back]
            if d4 == 0:
                cw += [hexbit(b4[84 + 3 * i:87 + 3 * i]) for i in range(4)]
            else:
                era += range(19 + 4 * k, 23 + 4 * k)
                cw += [0] * 4
        cw += [hexbit(b[j:j + 3]) for j in (84 + 3 * i + (1 if i > 15 else 0) for i in range(28))]
        out, n = rs63.decode(cw, era)
        if out is None:
            yield None
            continue
        e = out[19:35]
        alg = (e[0] << 2) | (e[1] >> 4)
        key = ((e[1] & 15) << 12) | (e[2] << 6) | e[3]
        mi = bytearray()
        for j in range(4, 16, 4):
            mi += bytes([(e[j] << 2) | (e[j + 1] >> 4), ((e[j + 1] & 15) << 4) | (e[j + 2] >> 2), ((e[j + 2] & 3) << 6) | e[j + 3]])
        yield alg, key, bytes(mi), n


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("file")
    ap.add_argument("--nac", required=True)
    ap.add_argument("--sysid", required=True)
    ap.add_argument("--wacn", required=True)
    ap.add_argument("--lch", type=int, help="logical channel to keep (default both)")
    ap.add_argument("--out", help="write deinterleaved 72-bit frames, 9 bytes each (c0..c3 concatenated, MSB first)")
    a = ap.parse_args()
    xm = p2_mac.scrambler(int(a.nac, 16), int(a.sysid, 16), int(a.wacn, 16))
    dib, *_ = p2_mac.p2_sync.dibits(a.file, 31250.0, 6000.0)
    err, base, res = p2_mac.decode(a.file, xm)
    if base is None:
        print("no slot lock")
        return
    stats = collections.defaultdict(collections.Counter)
    per_lch = collections.defaultdict(list)
    out = open(a.out, "wb") if a.out else None
    for pos, slot, d, _ in res:
        lch = p2_mac.LCH[slot]
        if a.lch is not None and lch != a.lch:
            continue
        b = dib[pos + 10:pos + 180] ^ xm[slot * 180:slot * 180 + 170]
        per_lch[lch].append((d, b))
        if d not in VOICE:
            continue
        for s in VOICE[d]:
            r, c = frame(b[s:s + 36])
            stats[lch]["frames"] += 1
            if r is None:
                stats[lch]["uncorrectable"] += 1
                continue
            stats[lch]["errs=%d" % min(r[0], 3)] += 1
            if out:
                v = (c[0] << 48) | (c[1] << 25) | (c[2] << 14) | c[3]
                out.write(v.to_bytes(9, "big"))
    for lch in sorted(stats):
        print(f"lch {lch}: {dict(sorted(stats[lch].items()))}")
        for r in ess(per_lch[lch]):
            print(f"   ESS: " + ("uncorrectable" if r is None else f"algid 0x{r[0]:02x} keyid 0x{r[1]:04x} mi {r[2].hex()} ({r[3]} corrected)"))


if __name__ == "__main__":
    main()
