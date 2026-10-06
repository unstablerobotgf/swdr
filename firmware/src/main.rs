#![no_std]
#![no_main]

mod activity;
mod bch;
mod p25;
mod radio;
mod rcc;
mod spi;
mod tsbk;
mod uart;

use core::fmt::Write;
use core::sync::atomic::Ordering;
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use panic_rtt_target as _;
use portable_atomic::AtomicU32;
use rtt_target::{rprintln, rtt_init, ChannelMode, DownChannel, UpChannel};
use stm32wl33_pac as pac;

use radio::{Radio, IRQ_AHB_ERR, IRQ_DB0_USED, IRQ_DB1_USED};
use uart::{CmdParser, Frame, Vcp, PAYLOAD};

/// DBM ping-pong half: 512 complex i16 samples -> one 1 KB u8 frame.
const DB_LEN: usize = 2 * PAYLOAD;
/// Frame rate_exp tag for raw demod-tap frames (P25 modes): 0xF0 | RX_MODE.
const TAG_RAW: u8 = 0xF0;
/// Default 250 kS/s (E=3): fits SWD/RTT (~630 kB/s). The 2 Mbaud VCP only keeps up at E>=5.
const DEFAULT_RATE_EXP: u8 = 3;

#[repr(C, align(4))]
struct DbBuf([u8; 2 * DB_LEN]);
static mut DB: DbBuf = DbBuf([0; 2 * DB_LEN]);
static mut FRAMES: [Frame; 2] = [Frame::new(), Frame::new()];
static mut P25: p25::Decoder = p25::Decoder::new();
static mut ACTIVITY: activity::Activity = activity::Activity::new();
/// 100 Hz ticks since boot (SysTick).
static TICKS: AtomicU32 = AtomicU32::new(0);

/// Raw-tap mode that runs the on-board P25 TSBK decoder instead of streaming frames.
const MODE_P25_DECODE: u8 = 4;

static PENDING: AtomicU32 = AtomicU32::new(0);
static WAKE: Signal<CriticalSectionRawMutex, ()> = Signal::new();

/// Host-readable counters (ELF symbol): buffers, DBM overruns, RTT drops, UART drops, AHB errors.
#[no_mangle]
static SDR_STATS: [AtomicU32; 5] = [const { AtomicU32::new(0) }; 5];

/// Last tune report (ELF symbol): Hz, FSM state, RFSEQ_STATUS_DETAIL, VCO_CALFREQ_OUT.
#[no_mangle]
static TUNE_STATUS: [AtomicU32; 4] = [const { AtomicU32::new(0) }; 4];

/// Give the synth time to calibrate and lock, then publish the result.
fn report_tune(radio: &Radio, hz: u32) {
    cortex_m::asm::delay(64_000); // ~1 ms at 64 MHz
    let (fsm, detail, cal) = radio.tune_status();
    for (i, v) in [hz, fsm as u32, detail, cal as u32].into_iter().enumerate() {
        TUNE_STATUS[i].store(v, Ordering::Relaxed);
    }
    // RX state (16) with no PLL lock/cal flags; a failed lock drops the FSM back to idle.
    let lock = if fsm == 16 && detail & 0xF00 == 0 { "locked" } else { "LOCK FAIL" };
    rprintln!("tune {} Hz: {} fsm={} detail={:#x} vco_cal={}", hz, lock, fsm, detail, cal);
}

/// ROM bootloader jumps to RAM_VR.AppBase on Deepstop wakeup; ST's SystemInit sets it to VTOR.
fn set_app_base() {
    const RAM_VR_APPBASE: *mut u32 = 0x2000_0014 as *mut u32;
    unsafe { RAM_VR_APPBASE.write_volatile(0x1004_0000) };
}

/// Keep SWD attachable while the executor idles in WFE (DBG_CR is POR-reset to 0, RM0511 30.3.1).
fn keep_debug_alive(p: &pac::Peripherals) {
    p.rcc.apb0enr().modify(|_, w| w.dbgmcuen().set_bit());
    p.dbgmcu.cr().write(|w| w.dbg_sleep().set_bit().dbg_stop().set_bit());
}

/// Strong symbol overrides device.x PROVIDE(MR_SUBG = DefaultHandler).
#[no_mangle]
extern "C" fn MR_SUBG() {
    let p = unsafe { pac::Peripherals::steal() };
    let st = p.status.rfseq_irq_status().read().bits() & (IRQ_DB0_USED | IRQ_DB1_USED | IRQ_AHB_ERR);
    p.status.rfseq_irq_status().write(|w| unsafe { w.bits(st) });
    PENDING.fetch_or(st, Ordering::Relaxed);
    WAKE.signal(());
}

/// 100 Hz heartbeat: commands must be serviced even when the radio has stopped (e.g. no PLL lock).
#[cortex_m_rt::exception]
fn SysTick() {
    TICKS.fetch_add(1, Ordering::Relaxed);
    WAKE.signal(());
}

/// i16 -> rtl_tcp u8 offset binary, keeping bits [shift+7 : shift].
fn narrow(src: &[u8], dst: &mut [u8], shift: u32) {
    for (d, s) in dst.iter_mut().zip(src.chunks_exact(2)) {
        let v = (i16::from_le_bytes([s[0], s[1]]) as i32) >> shift;
        *d = (v.clamp(-128, 127) as i8 as u8) ^ 0x80;
    }
}

fn bump(i: usize) {
    SDR_STATS[i].fetch_add(1, Ordering::Relaxed);
}

#[embassy_executor::task]
async fn stream(mut radio: Radio, mut vcp: Vcp, mut iq: UpChannel, mut cmd_rtt: DownChannel) {
    let base = unsafe { core::ptr::addr_of_mut!(DB.0) as *mut u8 };
    let (b0, b1) = (base, unsafe { base.add(DB_LEN) });
    let frames = unsafe { &mut *core::ptr::addr_of_mut!(FRAMES) };
    let (mut shift, mut rate, mut freq) = (4u32, DEFAULT_RATE_EXP, 868_000_000u32);
    // 0: I/Q. Otherwise the P25 config with an MRSUBG raw tap: 1 hard bits, 2 freq detector,
    // 3 soft symbols, 4 freq detector into the on-board TSBK decoder (text lines on the VCP).
    let mut raw_mode = if cfg!(feature = "p25") { MODE_P25_DECODE } else { 0u8 };
    let decoder = unsafe { &mut *core::ptr::addr_of_mut!(P25) };
    let mut idens = tsbk::Idens::new();
    let act = unsafe { &mut *core::ptr::addr_of_mut!(ACTIVITY) };
    let (mut next, mut cur, mut seq) = (0usize, 0usize, 0u16);
    let mut rtt_cmd = CmdParser::new();
    // RF health over one SUM window, sampled at 100 Hz: rssi min/max/sum/n, agc min/max.
    let (mut rf, mut rf_tick) = ((i16::MAX, i16::MIN, 0i32, 0i32, u8::MAX, 0u8), 0u32);
    // Software AFC: hardware AFC sits after the freq tap, so retune the LO from the sync fits instead.
    let (mut afc_on, mut afc_base, mut afc_tick) = (true, 0u32, 0u32);

    if raw_mode == MODE_P25_DECODE {
        freq = option_env!("SWDR_P25_FREQ").and_then(|v| v.parse().ok()).unwrap_or(851_975_000);
        radio.set_frequency(freq);
        radio.configure_p25();
        radio.start_raw(0b100, b0, unsafe { b0.add(PAYLOAD) }, PAYLOAD as u16);
        rprintln!("p25: decoding at {} Hz", freq);
    } else {
        radio.set_rate_exp(rate);
        radio.start_iq(b0, b1, DB_LEN as u16);
        rprintln!("iq: started, Fs={} Hz", radio.sample_rate());
    }
    report_tune(&radio, freq);

    loop {
        WAKE.wait().await;
        let pend = PENDING.swap(0, Ordering::Relaxed);
        if pend & IRQ_AHB_ERR != 0 {
            bump(4);
        }
        let both = IRQ_DB0_USED | IRQ_DB1_USED;
        if pend & both == both {
            bump(1);
        }
        for _ in 0..(pend & both).count_ones() {
            let p25 = raw_mode != 0;
            let half = if p25 { unsafe { b0.add(PAYLOAD) } } else { b1 };
            let src = unsafe { core::slice::from_raw_parts(if next == 0 { b0 } else { half }, DB_LEN) };
            next ^= 1;
            bump(0);
            seq = seq.wrapping_add(1);
            if raw_mode == MODE_P25_DECODE {
                let samples = unsafe { core::slice::from_raw_parts(src.as_ptr() as *const i8, PAYLOAD) };
                let now_s = TICKS.load(Ordering::Relaxed) / 100;
                decoder.push(samples, &mut |t| {
                    act.on_tsbk(&t.bytes, t.trellis_errs, t.nid_errs, now_s);
                    let line = tsbk::format(t.nac, &t.bytes, t.trellis_errs, t.nid_errs, &mut idens);
                    vcp.mark(core::str::from_utf8(&line.buf[..line.len]).unwrap_or("?"));
                });
                continue;
            }
            // frames[cur] is never the one the UART DMA is reading.
            if p25 {
                frames[cur].payload().copy_from_slice(&src[..PAYLOAD]);
                frames[cur].seal(seq, TAG_RAW | raw_mode, 0);
            } else {
                narrow(src, frames[cur].payload(), shift);
                frames[cur].seal(seq, rate, shift as u8);
            }
            // NoBlockSkip writes whole frames or nothing, so the RTT stream stays frame-aligned.
            if iq.write(frames[cur].bytes()) == 0 {
                bump(2);
            }
            if vcp.busy() {
                bump(3);
            } else {
                vcp.send(&frames[cur]);
                cur ^= 1;
            }
        }

        if raw_mode == MODE_P25_DECODE {
            let tick = TICKS.load(Ordering::Relaxed);
            if tick != rf_tick {
                rf_tick = tick;
                let (rssi, agc, _) = radio.rf_sample();
                rf = (rf.0.min(rssi), rf.1.max(rssi), rf.2 + rssi as i32, rf.3 + 1, rf.4.min(agc), rf.5.max(agc));
            }
            let mut health = false;
            act.poll(tick / 100, decoder.frames_seen, decoder.nid_rejects, &mut |l| {
                health |= l.buf[..l.len].starts_with(b"SUM t=");
                vcp.mark(core::str::from_utf8(&l.buf[..l.len]).unwrap_or("?"))
            });
            // Every 2 s: dc/a = -3.3 needed LO -800 Hz on this site (242 Hz per unit, measured once).
            // Half-gain steps, 60 Hz deadband, +-5 kHz range, so retunes (and lost frames) stay rare.
            if afc_on && tick.wrapping_sub(afc_tick) >= 200 {
                afc_tick = tick;
                let (dc, a, n) = core::mem::take(&mut decoder.fit_sums);
                if afc_base == 0 {
                    afc_base = freq;
                }
                if n >= 5 && a > 0 {
                    let err_hz = dc as i64 * 242 / a as i64;
                    if err_hz.abs() > 60 {
                        let off = (freq as i64 - afc_base as i64 + (err_hz / 2).clamp(-400, 400)).clamp(-5000, 5000);
                        freq = (afc_base as i64 + off) as u32;
                        radio.abort();
                        radio.set_frequency(freq);
                        PENDING.store(0, Ordering::Relaxed);
                        next = 0;
                        radio.start_raw(0b100, b0, unsafe { b0.add(PAYLOAD) }, PAYLOAD as u16);
                    }
                }
            }
            if health && rf.3 > 0 {
                let mut l = tsbk::Line { buf: [0; 192], len: 0 };
                let afc = radio.rf_sample().2;
                let lo = if afc_base == 0 { 0 } else { freq as i32 - afc_base as i32 };
                let _ = write!(l, "SUM rf rssi={}/{}/{}dBm agc={}..{} hw_afc={} lo={:+}Hz", rf.0, rf.2 / rf.3, rf.1, rf.4, rf.5, afc, lo);
                vcp.mark(core::str::from_utf8(&l.buf[..l.len]).unwrap_or("?"));
                rf = (i16::MAX, i16::MIN, 0, 0, u8::MAX, 0);
            }
        }

        let mut cmd = vcp.poll_cmd();
        let mut b = [0u8; 1];
        while cmd.is_none() && cmd_rtt.read(&mut b) == 1 {
            cmd = rtt_cmd.feed(b[0]);
        }
        // op 5 = summary interval (s, 0 = off); it does not touch the radio.
        if let Some((5, arg)) = cmd {
            act.interval_s = arg;
            cmd = None;
        }
        if let Some((8, on)) = cmd {
            afc_on = on != 0;
            cmd = None;
        }
        if let Some((7, off)) = cmd {
            let mut l = tsbk::Line { buf: [0; 192], len: 0 };
            let _ = write!(l, "REG mr_subg+{:#05x} = {:#010x}", off & 0x3FC, radio.mr_read(off));
            vcp.mark(core::str::from_utf8(&l.buf[..l.len]).unwrap_or("?"));
            cmd = None;
        }
        if let Some((op, arg)) = cmd {
            radio.abort();
            match op {
                1 => {
                    freq = arg;
                    afc_base = 0; // AFC offsets are relative to the last commanded frequency
                    radio.set_frequency(arg);
                }
                2 => {
                    rate = (arg as u8).min(9);
                    radio.set_rate_exp(rate);
                }
                3 => shift = arg.min(8),
                // arg = offset << 16 | value; RX restarts below so the change applies from a clean start.
                6 => radio.mr_write(arg >> 16, arg & 0xFFFF),
                4 => {
                    raw_mode = (arg as u8).min(MODE_P25_DECODE);
                    if raw_mode != 0 {
                        radio.configure_p25();
                    } else {
                        radio.configure_iq(rate);
                    }
                }
                _ => {}
            }
            PENDING.store(0, Ordering::Relaxed);
            next = 0;
            // Raw taps run at 1.2..15.6 kB/s, so use 1 KB buffers (one frame each) instead of 2 KB.
            if raw_mode != 0 {
                let rx_mode = [0, 0b001, 0b100, 0b101, 0b100][raw_mode as usize];
                radio.start_raw(rx_mode, b0, unsafe { b0.add(PAYLOAD) }, PAYLOAD as u16);
            } else {
                radio.start_iq(b0, b1, DB_LEN as u16);
            }
            rprintln!("cmd {} {} -> Fs={} shift={}", op, arg, radio.sample_rate(), shift);
            report_tune(&radio, freq);
        }
    }
}

static mut LINK_TX: spi::Frame = spi::Frame::new();
static mut LINK_RX: [u8; spi::FRAME] = [0; spi::FRAME];

/// SPI link bring-up: counting frames to the DK as fast as it reads them, no radio.
/// Payload byte i of frame `seq` is (seq + i) as u8.
#[embassy_executor::task]
async fn spilink(vcp: Vcp) {
    let mut link = spi::Link::new();
    let (tx, rx) = unsafe { (&mut *core::ptr::addr_of_mut!(LINK_TX), &mut *core::ptr::addr_of_mut!(LINK_RX)) };
    let (mut seq, mut cmds, mut aborts, mut ovr, mut last_op) = (0u16, 0u32, 0u32, 0u32, 0u8);
    let mut report = TICKS.load(Ordering::Relaxed);
    let mut sent_at_report = 0u32;
    let mut sent = 0u32;
    vcp.mark("spilink: ready");
    loop {
        for (i, b) in tx.payload().iter_mut().enumerate() {
            *b = (seq as u8).wrapping_add(i as u8);
        }
        tx.seal(spi::KIND_TEST, last_op, seq, cmds, aborts);
        link.arm(tx, rx);
        link.drdy(true);
        // A transfer that starts and ends short of FRAME is dropped and the same seq re-sent.
        let ok = loop {
            if link.done() {
                break true;
            }
            if link.received() > 0 && link.idle() {
                cortex_m::asm::delay(64_000); // 1 ms: NSS really released, not a glitch
                if link.idle() && !link.done() {
                    break false;
                }
            }
        };
        link.drdy(false);
        if link.overrun() {
            ovr += 1;
        }
        if ok {
            let mut parser = CmdParser::new();
            if let Some((op, _arg)) = rx[..6].iter().find_map(|&b| parser.feed(b)) {
                cmds += 1;
                last_op = op;
            }
            seq = seq.wrapping_add(1);
            sent += 1;
        } else {
            aborts += 1;
        }
        let now = TICKS.load(Ordering::Relaxed);
        if now.wrapping_sub(report) >= 500 {
            let mut l = tsbk::Line { buf: [0; 192], len: 0 };
            let fps = (sent - sent_at_report) * 100 / now.wrapping_sub(report);
            let _ = write!(l, "spilink: sent={} fps={} cmds={} aborts={} ovr={}", sent, fps, cmds, aborts, ovr);
            vcp.mark(core::str::from_utf8(&l.buf[..l.len]).unwrap_or("?"));
            (report, sent_at_report) = (now, sent);
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = unsafe { pac::Peripherals::steal() };
    keep_debug_alive(&p);
    set_app_base();
    // ROM bootloader jumps to us with PRIMASK=1; ST's SystemInit ends with __enable_irq().
    unsafe { cortex_m::interrupt::enable() };

    // The standalone P25 build streams over the VCP, so the RTT I/Q ring shrinks to free RAM.
    #[cfg(not(feature = "p25"))]
    let ch = rtt_init! {
        up: {
            0: { size: 512, mode: ChannelMode::NoBlockSkip, name: "log" }
            // 12 KB, not 16: P25 + ACTIVITY statics left a 2 KB stack, which overflowed into .bss.
            1: { size: 12288, mode: ChannelMode::NoBlockSkip, name: "iq" }
        }
        down: {
            0: { size: 64, name: "cmd" }
        }
    };
    #[cfg(feature = "p25")]
    let ch = rtt_init! {
        up: {
            0: { size: 512, mode: ChannelMode::NoBlockSkip, name: "log" }
            1: { size: 1024, mode: ChannelMode::NoBlockSkip, name: "iq" }
        }
        down: {
            0: { size: 64, name: "cmd" }
        }
    };
    rtt_target::set_print_channel(ch.up.0);

    // USART kernel clock is 16 MHz from reset, so boot markers work before rcc::init.
    let mut vcp = Vcp::new();
    // Brief pause so a host can (re)open the VCP after a cold boot; core runs 16 MHz here.
    cortex_m::asm::delay(16_000_000);
    vcp.mark("");
    vcp.mark("swdr boot");
    rcc::init(&p, |m| vcp.mark(m));

    let mut cp = unsafe { cortex_m::Peripherals::steal() };
    cp.SYST.set_clock_source(cortex_m::peripheral::syst::SystClkSource::Core);
    cp.SYST.set_reload(640_000 - 1); // 100 Hz at 64 MHz
    cp.SYST.clear_current();
    cp.SYST.enable_interrupt();
    cp.SYST.enable_counter();

    if cfg!(feature = "spilink") {
        spawner.spawn(spilink(vcp).unwrap());
        return;
    }

    let radio = Radio::new();
    vcp.mark("radio: init ok");
    unsafe { cortex_m::peripheral::NVIC::unmask(pac::Interrupt::MR_SUBG) };
    vcp.start_dma();
    spawner.spawn(stream(radio, vcp, ch.up.1, ch.down.0).unwrap());
}
