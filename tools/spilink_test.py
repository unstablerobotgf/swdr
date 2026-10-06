#!/usr/bin/env python3
# Usage on the MP257F-DK: spilink_test.py [sck_hz] [seconds]. Needs the firmware built with --features spilink.
# swdr SPI link test: DRDY on gpiochip9 line 0 (PZ0, CN5 15), frames on /dev/spidev0.0.
import os, fcntl, struct, ctypes, select, sys, time
FRAME, HDR = 1040, 16
hz = int(sys.argv[1]) if len(sys.argv) > 1 else 16000000
secs = float(sys.argv[2]) if len(sys.argv) > 2 else 5

# GPIO v2 uAPI: gpio_v2_line_request is 592 bytes; INPUT | EDGE_RISING.
chip = os.open("/dev/gpiochip9", os.O_RDWR)
req = bytearray(592)
struct.pack_into("<I", req, 0, 0)                      # offsets[0] = line 0
req[256:256 + 8] = b"swdr-drdy"
struct.pack_into("<Q", req, 288, (1 << 2) | (1 << 4))  # config.flags
struct.pack_into("<II", req, 560, 1, 64)               # num_lines, event_buffer_size
fcntl.ioctl(chip, 0xC250B407, req)                     # GPIO_V2_GET_LINE_IOCTL
lfd = struct.unpack_from("<i", req, 588)[0]

def drdy():
    v = bytearray(struct.pack("<QQ", 0, 1))
    fcntl.ioctl(lfd, 0xC010B40E, v)                    # GPIO_V2_LINE_GET_VALUES_IOCTL
    return struct.unpack("<Q", bytes(v[:8]))[0] & 1

def drain():
    while select.select([lfd], [], [], 0)[0]:
        os.read(lfd, 48 * 16)

spi = os.open("/dev/spidev0.0", os.O_RDWR)
txb = ctypes.create_string_buffer(FRAME); rxb = ctypes.create_string_buffer(FRAME)
def xfer(tx):
    ctypes.memmove(txb, tx, len(tx))
    m = struct.pack("QQIIHBBBBBB", ctypes.addressof(txb), ctypes.addressof(rxb), FRAME, hz, 0, 8, 0, 0, 0, 0, 0)
    fcntl.ioctl(spi, 0x40206b00, m)
    return rxb.raw

print(f"drdy at start = {drdy()}  SCK req {hz/1e6:.0f} MHz")
n = bad_hdr = bad_pay = gaps = waits = 0; last = None; cmd_sent = 0; acks = 0
t0 = time.time()
while time.time() - t0 < secs:
    if not drdy():
        waits += 1
        drain()
        if not drdy() and not select.select([lfd], [], [], 0.5)[0]:
            print("timeout waiting for DRDY"); break
        drain()
    # Every 100th frame carries a command (op 9, arg = frame count) in the first MOSI bytes.
    tx = bytes(FRAME)
    if n % 100 == 0:
        tx = bytes([0xC5, 9]) + n.to_bytes(4, "little") + bytes(FRAME - 6); cmd_sent += 1
    f = xfer(tx); n += 1
    if f[:2] != b"\xA5\x5B" or f[6:8] != (1024).to_bytes(2, "little"):
        bad_hdr += 1
        if bad_hdr <= 3: print("bad header", f[:16].hex())
        continue
    seq = int.from_bytes(f[4:6], "little")
    exp = bytes(((seq + i) & 0xFF) for i in range(1024))
    if f[16:] != exp:
        bad_pay += 1
        if bad_pay <= 3:
            i = next(k for k in range(1024) if f[16 + k] != exp[k]); print(f"payload err seq {seq} at {i}: {f[16+i]:02x} != {exp[i]:02x}")
    if last is not None and seq != (last + 1) & 0xFFFF:
        gaps += 1
    last = seq; acks = int.from_bytes(f[8:12], "little")
dt = time.time() - t0
print(f"frames {n} in {dt:.1f}s = {n/dt:.0f} fps, {n*FRAME*8/dt/1e6:.2f} Mbit/s | bad hdr {bad_hdr} bad payload {bad_pay} seq gaps {gaps} | drdy waits {waits} | cmds sent {cmd_sent} seen {acks} last op {f[3]}")
