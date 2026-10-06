#!/usr/bin/env python3
"""P25 Phase 2 (TDMA) outbound MAC decoder for voice-channel I/Q captures from swdr-tap --hop.

usage: p2_mac.py IQ_FILE [IQ_FILE...] --nac HEX --sysid HEX --wacn HEX [--all]
The scrambling seed comes from the control channel (NET_STATUS_BCAST / RFSS_STATUS_BCAST).
H-DQPSK demod as in p2_sync.py; slots are numbered from the S-ISCH pairs (slots 2/3, 6/7,
10/11), trying each superframe position and keeping the one whose ACCH CRCs pass. SACCH and
FACCH are checked with CRC-12 on the systematic bits (no Reed-Solomon correction yet).
Prints MAC PDU types and Group Voice Channel User (talkgroup, source) messages.
"""
import argparse
import numpy as np
import p2_sync

BURST = 180  # dibits per timeslot, counted from the ISCH
# DUID (8,4) codewords, value in the high nibble; decoded to the nearest within 1 bit.
DUID_CW = [0x00, 0x17, 0x2E, 0x39, 0x4B, 0x5C, 0x65, 0x72, 0x8D, 0x9A, 0xA3, 0xB4, 0xC6, 0xD1, 0xE8, 0xFF]
SACCH, FACCH = {3: True, 12: False}, {9: True, 15: False}  # duid -> scrambled
MAC = {0: "SIGNAL", 1: "PTT", 2: "END_PTT", 3: "IDLE", 4: "ACTIVE", 6: "HANGTIME"}


def scrambler(nac, sysid, wacn, n=12 * BURST * 2):
    """Superframe scrambling bits: 44-bit Galois LFSR (taps 40,35,29,24,10,0) whose seed is
    WACN|SYSID|NAC spread by the fixed offsets {0,4,9,15,20,34}."""
    v, r = (wacn << 24) | (sysid << 12) | nac, 0
    for i in range(44):
        if (v >> (43 - i)) & 1:
            for o in (0, 4, 9, 15, 20, 34):
                if i + o < 44:
                    r ^= 1 << (43 - i - o)
    taps, out = (1 << 40) | (1 << 35) | (1 << 29) | (1 << 24) | (1 << 10) | 1, []
    for _ in range(n):
        b = (r >> 43) & 1
        out.append(b)
        r = ((r << 1) & ((1 << 44) - 1)) ^ (taps if b else 0)
    bits = np.array(out, np.uint8)
    return (bits[0::2] << 1) | bits[1::2]  # dibits


def duid(burst):
    cw = (burst[10] << 6) | (burst[47] << 4) | (burst[132] << 2) | burst[169]
    d = min(range(16), key=lambda k: bin(cw ^ DUID_CW[k]).count("1"))
    return d if bin(cw ^ DUID_CW[d]).count("1") <= 1 else -1


def crc12(bits):
    """x^12+x^11+x^7+x^4+x^2+x+1, complemented."""
    reg = list(bits) + [0] * 12
    poly = [1, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1]
    for i in range(len(bits)):
        if reg[i]:
            for j in range(13):
                reg[i + j] ^= poly[j]
    return int("".join(map(str, reg[len(bits):])), 2) ^ 0xFFF


def acch_bits(b, fast):
    spans = [(11, 36), (48, 31), (100, 32), (133, 36)] if fast else [(11, 36), (48, 84), (133, 36)]
    return [x for a, n in spans for d in b[a:a + n] for x in ((d >> 1) & 1, d & 1)]


def describe(pdu):
    op = pdu[0] >> 5
    s = MAC.get(op, f"op{op}")
    if op in (1, 2):
        s += f" tg={int.from_bytes(pdu[16:18], 'big')} src={int.from_bytes(pdu[13:16], 'big')}"
    elif op in (3, 4, 6) and pdu[1] == 0x01:  # Group Voice Channel User (abbreviated)
        s += f" GRP_V_CH_USR tg={int.from_bytes(pdu[3:5], 'big')} src={int.from_bytes(pdu[5:8], 'big')}"
    elif op in (3, 4, 6):
        s += f" mco=0x{pdu[1]:02x}"
    return s


def decode(path, xm):
    dib, off, err, dur = p2_sync.dibits(path, 31250.0, 6000.0)
    bits = np.unpackbits(dib[:, None], axis=1)[:, 6:].reshape(-1)
    win = np.lib.stride_tricks.sliding_window_view(bits, 40)[::2]
    hits = set(np.where((win != p2_sync.SYNC_BITS).sum(1) <= 4)[0].tolist())
    pairs = [h for h in sorted(hits) if h + BURST in hits]
    if not pairs:
        return err, None, []
    anchor = pairs[0]
    best = (-1, None, [])
    for base in (2, 6, 10):
        res, good = [], 0
        for pos in range(anchor % BURST, len(dib) - BURST, BURST):
            slot = (base + (pos - anchor) // BURST) % 12
            burst = dib[pos + 10:pos + BURST]
            d = duid(burst)
            if d not in SACCH and d not in FACCH:
                res.append((pos, slot, d, None))
                continue
            scr = SACCH.get(d, FACCH.get(d))
            b = burst ^ xm[slot * BURST:slot * BURST + 170] if scr else burst
            fast = d in FACCH
            x = acch_bits(b, fast)
            n = 144 if fast else 168
            ok = crc12(x[:n]) == int("".join(map(str, x[n:n + 12])), 2)
            good += ok
            pdu = bytes(int("".join(map(str, x[i:i + 8])), 2) for i in range(0, n, 8))
            res.append((pos, slot, d, describe(pdu) if ok else None))
        if good > best[0]:
            best = (good, base, res)
    return err, best[1], best[2]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("files", nargs="+")
    ap.add_argument("--nac", required=True)
    ap.add_argument("--sysid", required=True)
    ap.add_argument("--wacn", required=True)
    ap.add_argument("--all", action="store_true", help="print every CRC-good PDU, not just new talkgroup lines")
    a = ap.parse_args()
    xm = scrambler(int(a.nac, 16), int(a.sysid, 16), int(a.wacn, 16))
    for path in a.files:
        err, base, res = decode(path, xm)
        name = path.replace("\\", "/").split("/")[-1]
        if base is None:
            print(f"{name}: level err {err:.3f}, no S-ISCH pair (no slot lock)")
            continue
        acch = [r for r in res if r[2] in SACCH or r[2] in FACCH]
        good = [r for r in acch if r[3]]
        tgs = sorted({r[3].split("tg=")[1].split()[0] for r in good if "tg=" in r[3]})
        print(f"{name}: level err {err:.3f}, {len(res)} slots, ACCH {len(good)}/{len(acch)} CRC good, talkgroups {tgs or '-'}")
        seen = set()
        for pos, slot, d, m in res:
            if m and (a.all or ("tg=" in m and m not in seen)):
                seen.add(m)
                print(f"   t={pos / 6000:6.3f}s slot {slot:2d} {'FACCH' if d in FACCH else 'SACCH'}: {m}")


if __name__ == "__main__":
    main()
