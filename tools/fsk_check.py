"""Regression check on a captured paging channel (u8 I/Q, rtl_sdr format).

FLEX 4-FSK puts tones at carrier +-1.6 and +-4.8 kHz. Finding four tones with 3.2 kHz spacing
checks tuning (carrier offset), sample rate (spacing) and sample integrity (tone SNR) at once.
usage: fsk_check.py IQ_FILE [FS_HZ] [RF_HZ]
"""
import sys
import numpy as np

path = sys.argv[1]
fs = float(sys.argv[2]) if len(sys.argv) > 2 else 250e3
rf = float(sys.argv[3]) if len(sys.argv) > 3 else 929.6125e6
N = 4096

b = np.fromfile(path, dtype=np.uint8).astype(np.float32) - 127.5
z = b[0::2] + 1j * b[1::2]
segs = z[: len(z) // N * N].reshape(-1, N) * np.hanning(N)
P = np.abs(np.fft.fftshift(np.fft.fft(segs, axis=1), axes=1)) ** 2
f = np.fft.fftshift(np.fft.fftfreq(N, 1 / fs))
avg = 10 * np.log10(P.mean(0) + 1e-9)
floor = np.median(avg)

# Four strongest averaged peaks within +-15 kHz, at least 1.5 kHz apart.
near = np.where(np.abs(f) < 15e3)[0]
tones = []
for k in near[np.argsort(avg[near])[::-1]]:
    if all(abs(f[k] - t) > 1.5e3 for t, _ in tones):
        tones.append((f[k], avg[k] - floor))
    if len(tones) == 4:
        break
tones.sort()
spacing = np.diff([t for t, _ in tones])
center = np.mean([t for t, _ in tones])
snr = min(s for _, s in tones)
print(f"{len(z) / fs:.1f} s, tones (kHz): {[round(float(t) / 1e3, 2) for t, _ in tones]}")
print(f"spacing (kHz): {[round(float(s) / 1e3, 2) for s in spacing]}  center {center / 1e3:+.2f} kHz = {center / rf * 1e6:+.2f} ppm  weakest tone {snr:.1f} dB")

ok = len(tones) == 4 and all(abs(s - 3.2e3) < 0.1 * 3.2e3 for s in spacing) and abs(center) < 10e3 and snr > 10
print("PASS: 4-FSK tones at expected spacing" if ok else "FAIL (or channel idle during capture)")
sys.exit(0 if ok else 1)
