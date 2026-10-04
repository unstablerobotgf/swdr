"""P25 Phase 1 control channel (TSBK) reference decoder for captured u8 I/Q.

Ground truth for the firmware decoder. Tables are from OP25 (gr-op25_repeater/lib/p25p1_fdma.cc)
and SDRTrunk (module/decode/p25/phase1, edac/trellis), cross-checked against each other.
usage: p25_ref.py IQ_FILE [FS_HZ] [CARRIER_OFFSET_HZ]
"""
import sys
import numpy as np

FS_SYNC = 0x5575F5FF77FF
FS_DIBITS = [(FS_SYNC >> (46 - 2 * i)) & 3 for i in range(24)]
DUID = {0x0: "HDU", 0x3: "TDU", 0x5: "LDU1", 0x7: "TSBK", 0xA: "LDU2", 0xC: "PDU", 0xF: "TDULC"}

# Dibit-domain deinterleave: deint[i] = rx[TB_DIBIT[i]] (derived from OP25/SDRTrunk 196-bit table).
TB_DIBIT = [0, 1, 26, 27, 50, 51, 74, 75, 2, 3, 28, 29, 52, 53, 76, 77, 4, 5, 30, 31, 54, 55, 78, 79,
            6, 7, 32, 33, 56, 57, 80, 81, 8, 9, 34, 35, 58, 59, 82, 83, 10, 11, 36, 37, 60, 61, 84, 85,
            12, 13, 38, 39, 62, 63, 86, 87, 14, 15, 40, 41, 64, 65, 88, 89, 16, 17, 42, 43, 66, 67, 90, 91,
            18, 19, 44, 45, 68, 69, 92, 93, 20, 21, 46, 47, 70, 71, 94, 95, 22, 23, 48, 49, 72, 73, 96, 97,
            24, 25]
# Rate-1/2 trellis: NEXT_WORDS[state][input] = 4-bit output (two dibits); next state = input.
NEXT_WORDS = [[0x2, 0xC, 0x1, 0xF], [0xE, 0x0, 0xD, 0x3], [0x9, 0x7, 0xA, 0x4], [0x5, 0xB, 0x6, 0x8]]
POPC = [bin(i).count("1") for i in range(16)]


def crc16_gsm(data):
    """CRC-16/GSM over bytes 0..9 (poly 0x1021, init 0, xorout 0xFFFF); compare with bytes 10..11."""
    crc = 0
    for byte in data:
        crc ^= byte << 8
        for _ in range(8):
            crc = ((crc << 1) ^ 0x1021) & 0xFFFF if crc & 0x8000 else (crc << 1) & 0xFFFF
    return crc ^ 0xFFFF


def viterbi(nibbles):
    """Hard-decision Viterbi over 49 received nibbles; returns 48 data dibits (flush dibit dropped)."""
    INF = 1 << 30
    metric = [0, INF, INF, INF]
    paths = [[], [], [], []]
    for nib in nibbles:
        nm, npaths = [INF] * 4, [None] * 4
        for s in range(4):
            if metric[s] >= INF:
                continue
            for d in range(4):
                m = metric[s] + POPC[NEXT_WORDS[s][d] ^ nib]
                if m < nm[d]:
                    nm[d], npaths[d] = m, paths[s] + [d]
        metric, paths = nm, npaths
    return paths[0][:48], metric[0]  # flush input is 0, so the survivor ends in state 0


def decode_block(dibits98):
    deint = [dibits98[TB_DIBIT[i]] for i in range(98)]
    nibbles = [(deint[2 * i] << 2) | deint[2 * i + 1] for i in range(49)]
    data, errs = viterbi(nibbles)
    out = bytearray(12)
    for i, d in enumerate(data):
        out[i // 4] |= d << (6 - 2 * (i % 4))
    return bytes(out), errs


def demod(path, fs, carrier):
    b = np.fromfile(path, dtype=np.uint8).astype(np.float32) - 127.5
    z = (b[0::2] + 1j * b[1::2]) * np.exp(-2j * np.pi * carrier * np.arange(len(b) // 2) / fs)
    Z = np.fft.fft(z)
    Z[np.abs(np.fft.fftfreq(len(z), 1 / fs)) > 5e3] = 0
    z = np.fft.ifft(Z)
    fm = np.angle(z[1:] * np.conj(z[:-1])) * fs / (2 * np.pi) / 600.0  # symbol units: +-1, +-3
    sps = fs / 4800
    return np.convolve(fm, np.ones(int(round(sps))) / round(sps), mode="same"), sps


def slice_sym(v, outer):
    t = 2 * outer / 3  # boundary between +-1 and +-3, scaled to the measured outer level
    return 1 if v >= t else (0 if v >= 0 else (2 if v >= -t else 3))


def decode_dibits(dib, tol=4):
    """Frame sync, NID, status strip, trellis and CRC over a dibit array. Returns {(sym_index, blk): (nac, hex, errs)}."""
    dib = np.asarray(dib, dtype=np.uint8)
    bits = np.unpackbits(dib[:, None], axis=1)[:, 6:].reshape(-1)
    ref = np.array([(FS_SYNC >> (47 - i)) & 1 for i in range(48)], dtype=np.uint8)
    win = np.lib.stride_tricks.sliding_window_view(bits, 48)[::2]
    dist = (win != ref).sum(1)
    dist_inv = (win != (ref ^ np.tile([1, 0], 24).astype(np.uint8))).sum(1)
    out, nids = {}, set()
    for start in np.where(np.minimum(dist, dist_inv) <= tol)[0]:
        fr = dib[start:start + 360].copy()
        if len(fr) < 180:
            continue
        if dist_inv[start] < dist[start]:
            fr ^= 2
        data = [int(d) for i, d in enumerate(fr) if (i + 1) % 36 != 0]
        nid = 0
        for d in data[24:56]:
            nid = (nid << 2) | d
        nac, duid = (nid >> 52) & 0xFFF, (nid >> 48) & 0xF
        nids.add((nac, DUID.get(duid, hex(duid))))
        if duid != 0x7:
            continue
        for k in range(3):
            blk = data[56 + 98 * k: 56 + 98 * (k + 1)]
            if len(blk) < 98:
                break
            tsbk, errs = decode_block(blk)
            if crc16_gsm(tsbk[:10]) == int.from_bytes(tsbk[10:12], "big"):
                out[(int(start), k)] = (nac, tsbk.hex(), errs)
            if tsbk[0] & 0x80:
                break
    return out, nids


def bits_file_variants(path):
    """Demodulated-bit capture -> dibit arrays under each plausible packing (order undocumented)."""
    raw = np.fromfile(path, dtype=np.uint8)
    msb = np.unpackbits(raw)                      # bit 7 first
    lsb = np.unpackbits(raw, bitorder="little")   # bit 0 first
    for name, b in (("msb-first", msb), ("lsb-first", lsb)):
        b = b[: len(b) // 2 * 2]
        yield name, (b[0::2] << 1) | b[1::2]
        yield name + ", dibit-swapped", (b[1::2] << 1) | b[0::2]


def main():
    if sys.argv[1] == "--bits":
        for name, dib in bits_file_variants(sys.argv[2]):
            out, nids = decode_dibits(dib)
            print(f"{name:24s}: {len(out)} CRC-valid TSBKs, NIDs {sorted(nids)[:4]}")
            for (st, k), (nac, hx, errs) in sorted(out.items())[:5]:
                print(f"    t={st / 4800:8.3f}s blk{k} NAC={nac:03x} {hx} trellis_errs={errs}")
        return
    path = sys.argv[1]
    fs = float(sys.argv[2]) if len(sys.argv) > 2 else 250e3
    carrier = float(sys.argv[3]) if len(sys.argv) > 3 else -1557.0
    fmi, sps = demod(path, fs, carrier)
    found = {}
    for ph in np.linspace(0, sps, 16, endpoint=False):
        sym = fmi[(ph + np.arange(int((len(fmi) - sps) / sps)) * sps).astype(int)]
        outer = np.percentile(np.abs(sym), 90)
        dib = np.array([slice_sym(v, outer) for v in sym], dtype=np.uint8)
        # Frame sync: Hamming distance on 48 bits, both polarities (inverted flips dibit MSB).
        bits = np.unpackbits(dib[:, None], axis=1)[:, 6:].reshape(-1)
        ref = np.array([(FS_SYNC >> (47 - i)) & 1 for i in range(48)], dtype=np.uint8)
        win = np.lib.stride_tricks.sliding_window_view(bits[: len(bits) - len(bits) % 2], 48)[::2]
        dist = (win != ref).sum(1)
        dist_inv = (win != (ref ^ np.tile([1, 0], 24).astype(np.uint8))).sum(1)
        for start in np.where(np.minimum(dist, dist_inv) <= 4)[0]:
            fr = dib[start:start + 360].copy()
            if len(fr) < 360:
                continue
            if dist_inv[start] < dist[start]:
                fr ^= 2
            data = [int(d) for i, d in enumerate(fr) if (i + 1) % 36 != 0]  # strip status dibits
            nid = 0
            for d in data[24:56]:
                nid = (nid << 2) | d
            nac, duid = (nid >> 52) & 0xFFF, (nid >> 48) & 0xF
            if duid != 0x7:
                found.setdefault(("nid", int(start), nac, duid), None)
                continue
            for k in range(3):
                blk = data[56 + 98 * k: 56 + 98 * (k + 1)]
                if len(blk) < 98:
                    break
                tsbk, errs = decode_block(blk)
                if crc16_gsm(tsbk[:10]) == int.from_bytes(tsbk[10:12], "big"):
                    t = round(start / 4800, 3)
                    found[("tsbk", t, k)] = (nac, tsbk.hex(), errs)
                if tsbk[0] & 0x80:
                    break
    tsbks = sorted((k[1], k[2], v) for k, v in found.items() if k[0] == "tsbk")
    others = sorted({(k[2], DUID.get(k[3], hex(k[3]))) for k in found if k[0] == "nid"})
    print(f"{len(tsbks)} CRC-valid TSBKs; other NIDs seen (nac, duid): {others[:6]}")
    for t, k, (nac, hx, errs) in tsbks[:40]:
        print(f"t={t:8.3f}s blk{k} NAC={nac:03x} op={int(hx[:2], 16) & 0x3f:02x} mfid={hx[2:4]} {hx}  trellis_errs={errs}")


if __name__ == "__main__":
    main()
