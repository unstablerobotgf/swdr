#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `LSION` reader - Internal Low Speed oscillator enable Set and reset by software. Reset source only for this field: PORESETn 0: LSI RC oscillator OFF 1: LSI RC oscillator ON"]
pub type LsionR = crate::BitReader;
#[doc = "Field `LSION` writer - Internal Low Speed oscillator enable Set and reset by software. Reset source only for this field: PORESETn 0: LSI RC oscillator OFF 1: LSI RC oscillator ON"]
pub type LsionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSIRDY` reader - Internal Low Speed oscillator Ready Set and reset by hardware to indicate when the Low Speed Internal RC oscillator is stable. Reset source only for this field: PORESETn 0: LSI RC oscillator not ready 1: LSI RC oscillator ready"]
pub type LsirdyR = crate::BitReader;
#[doc = "Field `LSEON` reader - External Low Speed Clock enable. Set and reset by software. Reset source only for this field: PORESETn 0: LSE oscillator OFF 1: LSE oscillator ON Note that enablng this bit, the configuration of PB12 and PB13 will be bypassed (whatever DFTMUX or AF selection)"]
pub type LseonR = crate::BitReader;
#[doc = "Field `LSEON` writer - External Low Speed Clock enable. Set and reset by software. Reset source only for this field: PORESETn 0: LSE oscillator OFF 1: LSE oscillator ON Note that enablng this bit, the configuration of PB12 and PB13 will be bypassed (whatever DFTMUX or AF selection)"]
pub type LseonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDY` reader - External Low Speed Clock ready flag. Set by hardware to indicate that LSE oscillator is stable. 0: LSE oscillator not ready 1: LSE oscillator ready"]
pub type LserdyR = crate::BitReader;
#[doc = "Field `LSEBYP` reader - External Low Speed Clock bypass. Set and reset by software. Reset source only for this field: PORESETn 0: LSE oscillator bypass OFF 1: LSE oscillator bypass ON Note that enablng this bit, the configuration of PB13 will be bypassed (whatever DFTMUX or AF selection)"]
pub type LsebypR = crate::BitReader;
#[doc = "Field `LSEBYP` writer - External Low Speed Clock bypass. Set and reset by software. Reset source only for this field: PORESETn 0: LSE oscillator bypass OFF 1: LSE oscillator bypass ON Note that enablng this bit, the configuration of PB13 will be bypassed (whatever DFTMUX or AF selection)"]
pub type LsebypW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKDET_NSTOP` reader - Lock detector Nstop value When start_stop signal is high; a counter is incremented every 16 MHz clock cycle. When the counter reaches (NSTOP+1) x 64 value, the lock_det signal is set high indicating that the PLL is locked. As soon as the start_stop signal is low the counter is reset to 0."]
pub type LockdetNstopR = crate::FieldReader;
#[doc = "Field `LOCKDET_NSTOP` writer - Lock detector Nstop value When start_stop signal is high; a counter is incremented every 16 MHz clock cycle. When the counter reaches (NSTOP+1) x 64 value, the lock_det signal is set high indicating that the PLL is locked. As soon as the start_stop signal is low the counter is reset to 0."]
pub type LockdetNstopW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HSIRDY` reader - Internal High Speed clock ready flag. Set by hardware to indicate that internal RC 64MHz oscillator is stable. This bit is activated only if the RC is enabled by HSION (it is not activated if the RC is enabled by an IP request). 0: internal RC 64 MHz oscillator not ready 1: internal RC 64 MHz oscillator ready"]
pub type HsirdyR = crate::BitReader;
#[doc = "Field `HSEPLLBUFON` reader - External High Speed Clock Buffer for PLL RF enable. Set and reset by software. 0: HSE PLL Buffer OFF 1: HSE PLL Buffer ON (default)"]
pub type HsepllbufonR = crate::BitReader;
#[doc = "Field `HSEPLLBUFON` writer - External High Speed Clock Buffer for PLL RF enable. Set and reset by software. 0: HSE PLL Buffer OFF 1: HSE PLL Buffer ON (default)"]
pub type HsepllbufonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIPLLON` reader - Internal High Speed Clock PLL enable 0: PLL is OFF 1: PLL is ON"]
pub type HsipllonR = crate::BitReader;
#[doc = "Field `HSIPLLON` writer - Internal High Speed Clock PLL enable 0: PLL is OFF 1: PLL is ON"]
pub type HsipllonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIPLLRDY` reader - Internal High Speed Clock PLL ready flag. 0: PLL is unlocked 1: PLL is locked"]
pub type HsipllrdyR = crate::BitReader;
#[doc = "Field `FMRAT` reader - Force MRSUBG accurate clock ready status (for debug purpose) 0: no effect 1: active_transmission is force to '1' whatever the HSIPLLRDY/HSE status"]
pub type FmratR = crate::BitReader;
#[doc = "Field `FMRAT` writer - Force MRSUBG accurate clock ready status (for debug purpose) 0: no effect 1: active_transmission is force to '1' whatever the HSIPLLRDY/HSE status"]
pub type FmratW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEON` reader - External High Speed Clock enable. Set and reset by software. in low power mode, HSE is turned off. HSE is turned ON only when RFSUBG LDO is Ready 0: HSE oscillator OFF 1: HSE oscillator ON"]
pub type HseonR = crate::BitReader;
#[doc = "Field `HSEON` writer - External High Speed Clock enable. Set and reset by software. in low power mode, HSE is turned off. HSE is turned ON only when RFSUBG LDO is Ready 0: HSE oscillator OFF 1: HSE oscillator ON"]
pub type HseonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDY` reader - External High Speed Clock ready flag. Set by hardware to indicate that HSE oscillator is stable. 0: HSE oscillator not ready 1: HSE oscillator ready"]
pub type HserdyR = crate::BitReader;
impl R {
    #[doc = "Bit 2 - Internal Low Speed oscillator enable Set and reset by software. Reset source only for this field: PORESETn 0: LSI RC oscillator OFF 1: LSI RC oscillator ON"]
    #[inline(always)]
    pub fn lsion(&self) -> LsionR {
        LsionR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Internal Low Speed oscillator Ready Set and reset by hardware to indicate when the Low Speed Internal RC oscillator is stable. Reset source only for this field: PORESETn 0: LSI RC oscillator not ready 1: LSI RC oscillator ready"]
    #[inline(always)]
    pub fn lsirdy(&self) -> LsirdyR {
        LsirdyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External Low Speed Clock enable. Set and reset by software. Reset source only for this field: PORESETn 0: LSE oscillator OFF 1: LSE oscillator ON Note that enablng this bit, the configuration of PB12 and PB13 will be bypassed (whatever DFTMUX or AF selection)"]
    #[inline(always)]
    pub fn lseon(&self) -> LseonR {
        LseonR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - External Low Speed Clock ready flag. Set by hardware to indicate that LSE oscillator is stable. 0: LSE oscillator not ready 1: LSE oscillator ready"]
    #[inline(always)]
    pub fn lserdy(&self) -> LserdyR {
        LserdyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - External Low Speed Clock bypass. Set and reset by software. Reset source only for this field: PORESETn 0: LSE oscillator bypass OFF 1: LSE oscillator bypass ON Note that enablng this bit, the configuration of PB13 will be bypassed (whatever DFTMUX or AF selection)"]
    #[inline(always)]
    pub fn lsebyp(&self) -> LsebypR {
        LsebypR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:9 - Lock detector Nstop value When start_stop signal is high; a counter is incremented every 16 MHz clock cycle. When the counter reaches (NSTOP+1) x 64 value, the lock_det signal is set high indicating that the PLL is locked. As soon as the start_stop signal is low the counter is reset to 0."]
    #[inline(always)]
    pub fn lockdet_nstop(&self) -> LockdetNstopR {
        LockdetNstopR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10 - Internal High Speed clock ready flag. Set by hardware to indicate that internal RC 64MHz oscillator is stable. This bit is activated only if the RC is enabled by HSION (it is not activated if the RC is enabled by an IP request). 0: internal RC 64 MHz oscillator not ready 1: internal RC 64 MHz oscillator ready"]
    #[inline(always)]
    pub fn hsirdy(&self) -> HsirdyR {
        HsirdyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - External High Speed Clock Buffer for PLL RF enable. Set and reset by software. 0: HSE PLL Buffer OFF 1: HSE PLL Buffer ON (default)"]
    #[inline(always)]
    pub fn hsepllbufon(&self) -> HsepllbufonR {
        HsepllbufonR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Internal High Speed Clock PLL enable 0: PLL is OFF 1: PLL is ON"]
    #[inline(always)]
    pub fn hsipllon(&self) -> HsipllonR {
        HsipllonR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Internal High Speed Clock PLL ready flag. 0: PLL is unlocked 1: PLL is locked"]
    #[inline(always)]
    pub fn hsipllrdy(&self) -> HsipllrdyR {
        HsipllrdyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Force MRSUBG accurate clock ready status (for debug purpose) 0: no effect 1: active_transmission is force to '1' whatever the HSIPLLRDY/HSE status"]
    #[inline(always)]
    pub fn fmrat(&self) -> FmratR {
        FmratR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - External High Speed Clock enable. Set and reset by software. in low power mode, HSE is turned off. HSE is turned ON only when RFSUBG LDO is Ready 0: HSE oscillator OFF 1: HSE oscillator ON"]
    #[inline(always)]
    pub fn hseon(&self) -> HseonR {
        HseonR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - External High Speed Clock ready flag. Set by hardware to indicate that HSE oscillator is stable. 0: HSE oscillator not ready 1: HSE oscillator ready"]
    #[inline(always)]
    pub fn hserdy(&self) -> HserdyR {
        HserdyR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Internal Low Speed oscillator enable Set and reset by software. Reset source only for this field: PORESETn 0: LSI RC oscillator OFF 1: LSI RC oscillator ON"]
    #[inline(always)]
    pub fn lsion(&mut self) -> LsionW<'_, CrSpec> {
        LsionW::new(self, 2)
    }
    #[doc = "Bit 4 - External Low Speed Clock enable. Set and reset by software. Reset source only for this field: PORESETn 0: LSE oscillator OFF 1: LSE oscillator ON Note that enablng this bit, the configuration of PB12 and PB13 will be bypassed (whatever DFTMUX or AF selection)"]
    #[inline(always)]
    pub fn lseon(&mut self) -> LseonW<'_, CrSpec> {
        LseonW::new(self, 4)
    }
    #[doc = "Bit 6 - External Low Speed Clock bypass. Set and reset by software. Reset source only for this field: PORESETn 0: LSE oscillator bypass OFF 1: LSE oscillator bypass ON Note that enablng this bit, the configuration of PB13 will be bypassed (whatever DFTMUX or AF selection)"]
    #[inline(always)]
    pub fn lsebyp(&mut self) -> LsebypW<'_, CrSpec> {
        LsebypW::new(self, 6)
    }
    #[doc = "Bits 7:9 - Lock detector Nstop value When start_stop signal is high; a counter is incremented every 16 MHz clock cycle. When the counter reaches (NSTOP+1) x 64 value, the lock_det signal is set high indicating that the PLL is locked. As soon as the start_stop signal is low the counter is reset to 0."]
    #[inline(always)]
    pub fn lockdet_nstop(&mut self) -> LockdetNstopW<'_, CrSpec> {
        LockdetNstopW::new(self, 7)
    }
    #[doc = "Bit 12 - External High Speed Clock Buffer for PLL RF enable. Set and reset by software. 0: HSE PLL Buffer OFF 1: HSE PLL Buffer ON (default)"]
    #[inline(always)]
    pub fn hsepllbufon(&mut self) -> HsepllbufonW<'_, CrSpec> {
        HsepllbufonW::new(self, 12)
    }
    #[doc = "Bit 13 - Internal High Speed Clock PLL enable 0: PLL is OFF 1: PLL is ON"]
    #[inline(always)]
    pub fn hsipllon(&mut self) -> HsipllonW<'_, CrSpec> {
        HsipllonW::new(self, 13)
    }
    #[doc = "Bit 15 - Force MRSUBG accurate clock ready status (for debug purpose) 0: no effect 1: active_transmission is force to '1' whatever the HSIPLLRDY/HSE status"]
    #[inline(always)]
    pub fn fmrat(&mut self) -> FmratW<'_, CrSpec> {
        FmratW::new(self, 15)
    }
    #[doc = "Bit 16 - External High Speed Clock enable. Set and reset by software. in low power mode, HSE is turned off. HSE is turned ON only when RFSUBG LDO is Ready 0: HSE oscillator OFF 1: HSE oscillator ON"]
    #[inline(always)]
    pub fn hseon(&mut self) -> HseonW<'_, CrSpec> {
        HseonW::new(self, 16)
    }
}
#[doc = "CR register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0x1400"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0x1400;
}
