"""Streaming P25 TSBK decoder over MRSUBG frequency-detector samples (int8, RX_MODE=100).

Prototype of the firmware algorithm: no look-ahead statistics, fixed-point friendly.
- A bank of PHASES timing phases, each integrating over exactly one symbol (fractional, via a running sum).
- Frame sync on symbol signs only (FS is all +-3), so it is immune to DC offset and scale.
- Per frame: least-squares fit v = a*e + dc on the 24 sync symbols sets the slicer (dc, dc +- 2a).
usage: p25_stream.py FREQ_TAP_FILE [CHFLT_E]
"""
import sys
import numpy as np
from p25_ref import FS_DIBITS, crc16_gsm, decode_block

PHASES = 16
SYNC_SIGNS = [1 if d == 1 else -1 for d in FS_DIBITS]  # +3 -> +1, -3 -> -1
SIGN_MASK = sum(1 << (23 - i) for i, s in enumerate(SYNC_SIGNS) if s > 0)
FRAME_SYMS = 180 + 2 * 108  # up to 3 TSBK blocks (360 dibits)


def decode_stream(samples, fs):
    sps = fs / 4800.0
    cs = np.concatenate([[0.0], np.cumsum(samples, dtype=np.float64)])
    x = np.arange(len(cs))
    results = {}
    for p in range(PHASES):
        t0 = p * sps / PHASES + np.arange(int((len(samples) - 2 * sps) / sps)) * sps
        sym = (np.interp(t0 + sps, x, cs) - np.interp(t0, x, cs)) / sps
        sign_reg, i, n = 0, 0, len(sym)
        while i < n:
            sign_reg = ((sign_reg << 1) | (sym[i] > 0)) & 0xFFFFFF
            i += 1
            if i < 24 or bin(sign_reg ^ SIGN_MASK).count("1") > 2:
                continue
            start = i - 24
            fr = sym[start:start + 360]
            if len(fr) < 180:
                break
            e = np.array([3.0 if s > 0 else -3.0 for s in SYNC_SIGNS])
            v = fr[:24]
            a = np.dot(v - v.mean(), e - e.mean()) / np.dot(e - e.mean(), e - e.mean())
            dc = v.mean() - a * e.mean()
            if a <= 0:
                continue
            u = (fr - dc) / a  # now in symbol units
            dib = np.where(u >= 2, 1, np.where(u >= 0, 0, np.where(u >= -2, 2, 3)))
            data = [int(d) for j, d in enumerate(dib) if (j + 1) % 36 != 0]
            nid = 0
            for d in data[24:56]:
                nid = (nid << 2) | d
            if (nid >> 48) & 0xF != 0x7:
                continue
            for k in range(3):
                blk = data[56 + 98 * k: 56 + 98 * (k + 1)]
                if len(blk) < 98:
                    break
                tsbk, errs = decode_block(blk)
                if crc16_gsm(tsbk[:10]) == int.from_bytes(tsbk[10:], "big"):
                    t = round((start * sps + p * sps / PHASES) / fs, 2)
                    results[(t, tsbk.hex())] = ((nid >> 52) & 0xFFF, errs, p)
                if tsbk[0] & 0x80:
                    break
            i = start + 180  # skip past this frame's minimum length
    return results


def main():
    path = sys.argv[1]
    e = int(sys.argv[2]) if len(sys.argv) > 2 else 7
    fs = 16e6 / (8 * 2 ** e)
    samples = np.fromfile(path, dtype=np.int8).astype(np.float64)
    res = decode_stream(samples, fs)
    uniq = {}
    for (t, hx), v in res.items():
        uniq.setdefault((round(t, 1), hx), (t, v))
    print(f"{len(uniq)} distinct CRC-valid TSBKs in {len(samples) / fs:.1f} s")
    for (_, hx), (t, (nac, errs, p)) in sorted(uniq.items(), key=lambda kv: kv[1][0])[:8]:
        print(f"  t={t:7.2f}s NAC={nac:03x} op={int(hx[:2], 16) & 0x3f:02x} {hx} trellis_errs={errs} phase={p}")


if __name__ == "__main__":
    main()
