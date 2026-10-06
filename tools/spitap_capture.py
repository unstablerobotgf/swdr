#!/usr/bin/env python3
"""Capture the WL33 freq-tap stream over SPI on the MP257F-DK (firmware built with --features spitap).

usage: spitap_capture.py OUT.i8 [seconds] [sck_hz]
Writes the raw i8 samples (15625 S/s) back to back and reports sequence gaps, firmware queue
drops (aux0) and frame timing (aux1, 10 ms ticks). DRDY is gpiochip9 line 0 (PZ0, CN5 15).
"""
import ctypes, fcntl, os, select, struct, sys, time

FRAME, HDR, PAYLOAD, KIND_TAP = 1040, 16, 1024, 1
out = sys.argv[1]
secs = float(sys.argv[2]) if len(sys.argv) > 2 else 60
hz = int(sys.argv[3]) if len(sys.argv) > 3 else 16000000

# GPIO v2 uAPI line request (592 bytes): line 0, INPUT | EDGE_RISING.
chip = os.open("/dev/gpiochip9", os.O_RDWR)
req = bytearray(592)
req[256:256 + 9] = b"swdr-drdy"
struct.pack_into("<Q", req, 288, (1 << 2) | (1 << 4))
struct.pack_into("<II", req, 560, 1, 64)
fcntl.ioctl(chip, 0xC250B407, req)  # GPIO_V2_GET_LINE_IOCTL
lfd = struct.unpack_from("<i", req, 588)[0]


def drdy():
    v = bytearray(struct.pack("<QQ", 0, 1))
    fcntl.ioctl(lfd, 0xC010B40E, v)  # GPIO_V2_LINE_GET_VALUES_IOCTL
    return struct.unpack_from("<Q", v)[0] & 1


spi = os.open("/dev/spidev0.0", os.O_RDWR)
txb, rxb = ctypes.create_string_buffer(FRAME), ctypes.create_string_buffer(FRAME)
msg = struct.pack("QQIIHBBBBBB", ctypes.addressof(txb), ctypes.addressof(rxb), FRAME, hz, 0, 8, 0, 0, 0, 0, 0)

n = bad = gaps = lost = 0
last_seq = first_drop = last_drop = first_tick = last_tick = None
with open(out, "wb") as f:
    t0 = time.time()
    while time.time() - t0 < secs:
        # Edge events only wake the poll; the level is what gates the transfer.
        while not drdy():
            if select.select([lfd], [], [], 1.0)[0]:
                os.read(lfd, 48 * 64)
        fcntl.ioctl(spi, 0x40206B00, msg)  # SPI_IOC_MESSAGE(1)
        fr = rxb.raw
        if fr[:2] != b"\xA5\x5B" or fr[2] != KIND_TAP:
            bad += 1
            continue
        seq, drop, tick = struct.unpack_from("<H", fr, 4)[0], *struct.unpack_from("<II", fr, 8)
        if last_seq is not None and seq != (last_seq + 1) & 0xFFFF:
            gaps += 1
            lost += (seq - last_seq - 1) & 0xFFFF
        if first_drop is None:
            first_drop, first_tick = drop, tick
        last_seq, last_drop, last_tick = seq, drop, tick
        f.write(fr[HDR:])
        n += 1
dt = time.time() - t0
span = (last_tick - first_tick) / 100 if n else 0
print(f"frames {n} in {dt:.1f}s ({n * PAYLOAD / dt:.0f} S/s) | firmware span {span:.1f}s -> "
      f"{n * PAYLOAD / span if span else 0:.0f} S/s | bad {bad} seq gaps {gaps} ({lost} frames) | "
      f"queue drops during capture {last_drop - first_drop if n else 0}")
