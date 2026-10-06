#!/usr/bin/env python3
"""P25 Phase 2 (TDMA) outbound check on a voice-channel I/Q capture from swdr-tap --hop.

usage: p2_sync.py IQ_FILE [FS_HZ]
H-DQPSK at 6000 sym/s through the cqpsk_demod.py chain (RRC 0.2, Gardner, differential
detection), then a search for the 40-bit Phase 2 frame sync (0x575D57F7FF, as in OP25's
p25p2_framer) in both polarities. Reports hits, their spacing in symbols, and the level error.
"""
import sys
import numpy as np
import cqpsk_demod as c

SYNC = 0x575D57F7FF
SYNC_BITS = np.array([(SYNC >> (39 - i)) & 1 for i in range(40)], np.uint8)


def dibits(path, fs, rate):
    b = np.fromfile(path, np.uint8).astype(np.float32) - 127.5
    x = (b[0::2] + 1j * b[1::2]).astype(np.complex64)
    sps = fs / rate
    off = np.mean(np.angle(x[1:] * np.conj(x[:-1]))) * fs / (2 * np.pi)
    x = x * np.exp(-2j * np.pi * off * np.arange(len(x)) / fs).astype(np.complex64)
    y = np.convolve(x, c.rrc(sps), mode="same")
    y /= np.sqrt(np.mean(np.abs(y) ** 2))
    s, _ = c.gardner(y, sps)
    d = s[1:] * np.conj(s[:-1])
    rot = np.angle(-np.mean((d / (np.abs(d) + 1e-9)) ** 4)) / 4
    lv = np.angle(d * np.exp(-1j * rot)) / (np.pi / 4)
    err = np.abs(lv - np.clip(np.round((lv - 1) / 2) * 2 + 1, -3, 3)).mean()
    dib = np.where(lv >= 2, 1, np.where(lv >= 0, 0, np.where(lv >= -2, 2, 3))).astype(np.uint8)
    return dib, off, err, len(x) / fs


def main():
    path = sys.argv[1]
    fs = float(sys.argv[2]) if len(sys.argv) > 2 else 31250.0
    for rate in (6000.0, 4800.0):
        dib, off, err, dur = dibits(path, fs, rate)
        bits = np.unpackbits(dib[:, None], axis=1)[:, 6:].reshape(-1)
        win = np.lib.stride_tricks.sliding_window_view(bits, 40)[::2]
        dist = (win != SYNC_BITS).sum(1)
        inv = (win != (SYNC_BITS ^ np.tile([1, 0], 20).astype(np.uint8))).sum(1)
        hits = np.where(np.minimum(dist, inv) <= 4)[0]
        sp = np.diff(hits)
        common = np.unique(sp, return_counts=True)
        top = sorted(zip(common[1], common[0]), reverse=True)[:4]
        print(f"{rate:.0f} sym/s: {dur:.1f}s, carrier {off:+.0f} Hz, level err {err:.3f}, sync hits {len(hits)} "
              f"(inverted {int((inv[hits] < dist[hits]).sum())}), spacing (count x symbols) {[(int(n), int(s)) for n, s in top]}")


if __name__ == "__main__":
    main()
