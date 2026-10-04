# wl3x-sdr

An rtl_tcp-compatible SDR built on the STM32WL33's sub-GHz radio (MRSUBG), streaming I/Q to a PC
through the on-board STLINK-V3EC of a NUCLEO-WL33CC1. Inspired by [esp-sdr](https://github.com/ESPARGOS/esp-sdr).
Firmware is Rust on embassy-executor. SDR++, GQRX, gr-osmosdr and other rtl_tcp clients can connect.

| | |
|:--|:--|
| Sample source | MRSUBG I/Q sampling mode (`RX_MODE=011`), zero-IF after the channel filter |
| Sample rate | 250 kS/s over SWD/RTT (62.5 kS/s over the VCP fallback), u8 I/Q |
| Tuning | **390–510 MHz** and **780–1025 MHz** (measured, see below) |
| Front end | NUCLEO-WL33CC1 IPD is matched for 826–958 MHz; outside that, expect lower sensitivity |

## Hardware setup

- **JP2 (IDD jumper) must be fitted.** Without it the MCU is fed through its I/O protection
  diodes and browns out as soon as the radio starts: reboot loops, flaky SWD, zero-filled debug
  reads. This one jumper caused most of the debugging pain in this project's history.
- JP1 on 5V_STLK, SW1 in the default 3V3 position, JP4 not fitted (embedded STLINK).
- The STM32WL3x has no USB peripheral; everything goes through the STLINK-V3EC (USB high-speed).

## Layout

| Path | What |
|:--|:--|
| `firmware/` | embassy firmware: clocks, MRSUBG I/Q capture, RTT + VCP transports |
| `host/` | `wl3x-sdr-host` rtl_tcp bridge, plus test tools in `examples/` |
| `pac/stm32wl33-pac/` | svd2rust PAC from ST's SVD (`regen.sh` regenerates; patches `nvicPrioBits` 4 to 2) |
| `probe/` | probe-rs target YAML (see `probe/README.md` for the flash-algorithm relocation) |
| `patches/` | probe-rs 0.32.0 patch adding an STM32WL3x debug sequence |
| `tools/` | `setup-probe-rs.sh`, `fsk_check.py` regression check, ELF/SWD helper scripts |

## Setup

```sh
rustup target add thumbv6m-none-eabi
tools/setup-probe-rs.sh          # patched probe-rs into .tools/bin (host also builds against it)
```

Stock probe-rs cannot reset-and-halt this part: the AIRCR reset write is never ACKed and the
ST-Link driver replays it on WAIT, re-triggering the reset. The patch tolerates that, and also sets
`DBG_CR` from the debugger side.

## Build, flash, run

```sh
cd firmware && cargo run --release        # flash + RTT log via the patched probe-rs
cd host && cargo build --release
./target/release/wl3x-sdr-host --swd      # rtl_tcp on 127.0.0.1:1234, I/Q over SWD/RTT
./target/release/wl3x-sdr-host --port COM11   # fallback over the VCP (62.5 kS/s max)
```

Point an rtl_tcp client at `127.0.0.1:1234`, sample rate 250000. Gain steps select which 8 bits of
the radio's 16-bit samples are kept (6 dB per step). Stop the bridge with Ctrl-C; it shuts the
probe session down cleanly (killing it mid-transfer wedges the ST-Link until a replug).

Run the bridge from `host/` (it finds `../probe/STM32WL3_Series.yaml`), and close it before flashing.

## Tuning range

The synthesizer is `f_rf = f_vco / B` with `B = 8` (`BS=1`) or `B = 4` (`BS=0`). ST specifies
413–479 and 826–958 MHz. Measured on this board (`host/examples/sweep.rs`, lock = Radio FSM in RX
with no PLL lock/calibration flags):

| B | locks | fails | VCO |
|:--|:--|:--|:--|
| 8 | 390–510 MHz | 385 (VCO code 127), 515 (CALFREQ error) | 3.12–4.08 GHz |
| 4 | 780–1025 MHz | 770 (code 127), 1030 (CALFREQ error) | 3.12–4.10 GHz |

Both bands map onto the same VCO window. Edges sit at VCO code 127/1 and may move a few MHz with
temperature. 510–780 MHz is unreachable with these dividers. The firmware logs a lock report on RTT
after every retune.

## Regression check

A local 929.6125 MHz paging channel carries FLEX 4-FSK (tones at carrier ±1.6/±4.8 kHz):

```sh
cd host && ./target/release/examples/capture 929612500 40 6
python ../tools/fsk_check.py capture_iq.u8
```

PASS needs four tones at 3.2 kHz spacing (checks tuning, sample rate and sample integrity together).
Last result: spacing 3.17/3.23/3.17 kHz, carrier offset −1.87 ppm, weakest tone 27 dB over floor.

## Wire protocol

- Frames (RTT up-channel 1 and VCP): `A5 5A seq:u16 len:u16 rate_exp:u8 shift:u8` then 1024 B of
  u8 I/Q (offset binary, rtl_sdr format). Fs = 2 MHz >> rate_exp.
- Commands (RTT down-channel 0 or VCP RX): `C5 op arg:u32` little-endian. op 1 = frequency in Hz,
  2 = rate exponent, 3 = shift.

## STM32WL3x findings worth knowing

- The ROM bootloader starts the application with **PRIMASK=1** and **VTOR=0**. ST's `SystemInit`
  fixes both; cortex-m-rt does not, so the firmware enables IRQs and uses the `set-vtor` feature.
- ST's DFP flash algorithm loads at 0x20000008, over the ROM's RAM_VR block, so a reset after flashing
  can HardFault in ROM. `probe/STM32WL3_Series.yaml` moves it to 0x20001008.
- The PA10 boot pin is latched only at **power-on** (`PWR_SR2.IOBOOTVAL`); B4 does not re-latch it.
  Per UM3418's pin table the BOOT0 jumper is CN3 pins 3–5 (pin 7 is SWDIO).
- Under-reset SWD connection never works on this part (debug domain unpowered while NRST is held).
- `RFSEQ_STATUS_DETAIL` flags are sticky (rc_w1). `SABORT` never reports done if the FSM already
  left RX, so the abort wait is bounded.

## License

Apache-2.0 (see `LICENSE` and `NOTICE` for ST-derived parts). ST's reference manuals (RM0511,
UM3418, the NUCLEO-WL33CC1 data brief) are not included; download them from st.com.
