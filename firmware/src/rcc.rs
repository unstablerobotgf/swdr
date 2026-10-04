//! Clock/power bring-up, transcribed from ST's SystemInit + HAL_RCC_* for NUCLEO-WL33CC
//! (48 MHz XO, SMPS on, SYSCLK = RC64MPLL/1 = 64 MHz, LSE as slow clock).

use stm32wl33_pac as pac;

const VALIDITY_LOCATION: *const u32 = 0x1000_1EF8 as *const u32;
const VALIDITY_TAG: u32 = 0xFCBC_ECCC;

pub fn init(p: &pac::Peripherals, mark: impl Fn(&str)) {
    let (rcc, pwr) = (&p.rcc, &p.pwrc);

    // SystemInit: RAM retention, SMPS (BOM3 = 10 uH, open in low power).
    pwr.cr2().modify(|_, w| w.ramret1().set_bit().gpioret().clear_bit());
    while pwr.sr2().read().smpsrdy().bit_is_clear() {}
    pwr.cr5().modify(|_, w| unsafe { w.smpsbomsel().bits(2) }.nosmps().clear_bit().smpslpopen().set_bit());

    // Engineering trims missing from the info page: load ST's defaults.
    if unsafe { VALIDITY_LOCATION.read_volatile() } != VALIDITY_TAG {
        rcc.csswcr().modify(|_, w| unsafe {
            w.hsitrimsw().bits(0x21).hsiswtrimen().set_bit().lsiswbw().bits(8).lsiswtrimen().set_bit()
        });
        pwr.engtrim().modify(|_, w| unsafe {
            w.trim_mr().bits(3).trimmren().set_bit().smps_trim().bits(3).smpstrimen().set_bit()
        });
    }

    mark("rcc: smps+trim ok");
    // HSE: cap tune 32 (CFG_HW_RCC_HSE_CAPACITOR_TUNE), Gm 40, startup current/threshold 0.
    rcc.rfswhsecr().modify(|_, w| unsafe {
        w.swxotune().bits(32).swxotuneen().set_bit().istartup().bits(0).amplthresh().bits(0).gmc().bits(40)
    });
    rcc.cr().modify(|_, w| w.hseon().set_bit());
    while rcc.cr().read().hserdy().bit_is_clear() {}

    mark("rcc: hse ready");
    // LSE: off, release PB12/PB13 pulls, select as slow clock, medium-low drive, on.
    rcc.cr().modify(|_, w| w.lsion().clear_bit().lseon().clear_bit());
    while rcc.cr().read().lserdy().bit_is_set() {}
    pwr.pucrb().modify(|r, w| unsafe { w.bits(r.bits() & !0x3000) });
    pwr.pdcrb().modify(|r, w| unsafe { w.bits(r.bits() & !0x3000) });
    rcc.cfgr().modify(|_, w| unsafe { w.clkslowsel().bits(0b01) });
    rcc.csswcr().modify(|_, w| unsafe { w.lsedrv().bits(0b01) });
    rcc.cr().modify(|_, w| w.lseon().set_bit());
    while rcc.cr().read().lserdy().bit_is_clear() {}

    mark("rcc: lse ready");
    // SYSCLK: 1 flash wait state, RC64MPLL locked to HSE, /1.
    p.flash_ctrl.config().modify(|_, w| unsafe { w.wait_state().bits(1) });
    while p.flash_ctrl.config().read().wait_state().bits() != 1 {}
    rcc.cr().modify(|_, w| w.hsipllon().set_bit());
    while rcc.cr().read().hsipllrdy().bit_is_clear() {}
    rcc.cfgr().modify(|_, w| unsafe { w.clksysdiv().bits(0) });
    while rcc.cfgr().read().clksysdiv_status().bits() != 0 {}

    // SMPS clock /4. HAL also writes KRMR=4 but never sets KRM_EN, so it is a no-op; skipped.
    rcc.cfgr().modify(|_, w| w.smpsdiv().set_bit());
    mark("rcc: 64 MHz pll ok");
}
