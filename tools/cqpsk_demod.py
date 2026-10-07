#!/usr/bin/env python3
"""P25 Phase 1 CQPSK (LSM) demodulator prototype for swdr-tap I/Q captures.

usage: cqpsk_demod.py IQ_FILE [FS_HZ]   (interleaved u8 offset-binary I/Q, default 31250 S/s)
Chain: carrier offset from the mean discriminator, RRC matched filter (alpha 0.2), Gardner
timing recovery on the complex baseband, differential detection (dphi = +-pi/4, +-3pi/4 ->
+-1, +-3), then tools/p25_ref.py's frame sync / NID / trellis / CRC.
"""
import sys
import numpy as np
import p25_ref

SYM = 4800.0


def rrc(sps, alpha=0.2, span=8):
    t = (np.arange(-span * sps / 2, span * sps / 2 + 1)) / sps
    h = np.empty_like(t)
    for i, x in enumerate(t):
        if abs(x) < 1e-9:
            h[i] = 1 - alpha + 4 * alpha / np.pi
        elif abs(abs(x) - 1 / (4 * alpha)) < 1e-9:
            h[i] = alpha / np.sqrt(2) * ((1 + 2 / np.pi) * np.sin(np.pi / (4 * alpha)) + (1 - 2 / np.pi) * np.cos(np.pi / (4 * alpha)))
        else:
            h[i] = (np.sin(np.pi * x * (1 - alpha)) + 4 * alpha * x * np.cos(np.pi * x * (1 + alpha))) / (np.pi * x * (1 - (4 * alpha * x) ** 2))
    return h / np.sqrt((h ** 2).sum())


def gardner(y, sps, bw=0.01):
    """Symbol-rate samples from y with a Gardner TED and linear interpolation; returns (symbols, mu trace)."""
    out, trace = [], []
    k, mu, w = 2.0 * sps, 0.0, sps  # position, fractional correction, current period
    g1, g2 = 4 * bw, 4 * bw * bw  # PI loop gains (normalised to symbol units)
    prev = None
    interp = lambda p: y[int(p)] + (y[int(p) + 1] - y[int(p)]) * (p - int(p))
    while k + w + 2 < len(y):
        cur, mid = interp(k), interp(k - w / 2)
        if prev is not None:
            e = np.real((prev - cur) * np.conj(mid)) / (abs(mid) ** 2 + abs(cur) ** 2 + 1e-9)
            mu = min(2e-3, max(-2e-3, mu + g2 * e))  # clamp: unbounded it can false-lock while slipping
            k += w + sps * (g1 * e + mu)
        else:
            k += w
        out.append(cur)
        trace.append(mu)
        prev = cur
    return np.array(out), np.array(trace)


def main():
    path = sys.argv[1]
    fs = float(sys.argv[2]) if len(sys.argv) > 2 else 31250.0
    b = np.fromfile(path, np.uint8).astype(np.float32) - 127.5
    x = (b[0::2] + 1j * b[1::2]).astype(np.complex64)
    sps = fs / SYM

    disc = np.angle(x[1:] * np.conj(x[:-1]))
    off = np.mean(disc) * fs / (2 * np.pi)
    x = x * np.exp(-2j * np.pi * off * np.arange(len(x)) / fs).astype(np.complex64)
    y = np.convolve(x, rrc(sps), mode="same")
    y /= np.sqrt(np.mean(np.abs(y) ** 2))  # AGC once; Gardner error is amplitude-normalised anyway

    s, trace = gardner(y, sps)
    d = s[1:] * np.conj(s[:-1])
    # Residual offset shows as a constant rotation of dphi; 4th power folds the 4 states onto -1.
    rot = np.angle(-np.mean((d / (np.abs(d) + 1e-9)) ** 4)) / 4
    lv = np.angle(d * np.exp(-1j * rot)) / (np.pi / 4)  # +-1, +-3
    dib = np.where(lv >= 2, 1, np.where(lv >= 0, 0, np.where(lv >= -2, 2, 3))).astype(np.uint8)
    err = np.abs(lv - np.clip(np.round((lv - 1) / 2) * 2 + 1, -3, 3))
    out, nids = p25_ref.decode_dibits(dib)
    print(f"{len(x) / fs:.1f}s, carrier {off:+.0f} Hz, residual rot {np.degrees(rot):+.1f} deg/sym, "
          f"{len(s)} symbols ({len(s) / (len(x) / fs):.1f}/s), mean |level err| {err.mean():.3f}")
    print(f"CQPSK: {len(out)} CRC-valid TSBKs; NIDs {sorted(nids)[:5]}")
    for (st, k), (nac, hx, e) in sorted(out.items())[:3]:
        print(f"  t={st / SYM:7.3f}s blk{k} nac={nac:03x} {hx} e={e}")


if __name__ == "__main__":
    main()
