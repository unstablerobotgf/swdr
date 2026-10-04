#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `PVDE` reader - PVDE Programmable Voltage Detector Enable When this bit is set the Power Voltage Detector is enabled"]
pub type PvdeR = crate::BitReader;
#[doc = "Field `PVDE` writer - PVDE Programmable Voltage Detector Enable When this bit is set the Power Voltage Detector is enabled"]
pub type PvdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVDLS` reader - PVDLS\\[2:0\\] Programmable Voltage Detector Level selection - 000: 2.05 V - Lowest level - 001: 2.20 V - 010: 2.36 V - 011: 2.52 V - 100: 2.64 V - 101: 2.81 V - 110: 2.91 V - Highest level - 111: External input analog voltage (compare internally to VBGP; When external input VBGP then PVDO=1)"]
pub type PvdlsR = crate::FieldReader;
#[doc = "Field `PVDLS` writer - PVDLS\\[2:0\\] Programmable Voltage Detector Level selection - 000: 2.05 V - Lowest level - 001: 2.20 V - 010: 2.36 V - 011: 2.52 V - 100: 2.64 V - 101: 2.81 V - 110: 2.91 V - Highest level - 111: External input analog voltage (compare internally to VBGP; When external input VBGP then PVDO=1)"]
pub type PvdlsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DBGRET` reader - DBGRET: PA2 and PA3 retention enable after DEEPSTOP - 0: PA2, PA3 don't retain their status exiting from DEEPSTOP (default). - 1: PA2, PA3 retain their status exiting from DEEPSTOP."]
pub type DbgretR = crate::BitReader;
#[doc = "Field `DBGRET` writer - DBGRET: PA2 and PA3 retention enable after DEEPSTOP - 0: PA2, PA3 don't retain their status exiting from DEEPSTOP (default). - 1: PA2, PA3 retain their status exiting from DEEPSTOP."]
pub type DbgretW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMRET1` reader - RAMRET1: RAM1 retention during low power mode - 1: RAM1 bank is powered during low power mode - 0: RAM1 bank is disabled during low power mode (by default)"]
pub type Ramret1R = crate::BitReader;
#[doc = "Field `RAMRET1` writer - RAMRET1: RAM1 retention during low power mode - 1: RAM1 bank is powered during low power mode - 0: RAM1 bank is disabled during low power mode (by default)"]
pub type Ramret1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPREG_FORCE_VH` reader - force LPREG=1.2V during DEEPSTOP - 1: Force LPREG=1.2V during DEEPSTOP - 0: No Force (Default) Note LPREG= 1.2v can still apply when LCDEN or COMP.SCALEREN request it"]
pub type LpregForceVhR = crate::BitReader;
#[doc = "Field `LPREG_FORCE_VH` writer - force LPREG=1.2V during DEEPSTOP - 1: Force LPREG=1.2V during DEEPSTOP - 0: No Force (Default) Note LPREG= 1.2v can still apply when LCDEN or COMP.SCALEREN request it"]
pub type LpregForceVhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPREG_VH_STATUS` reader - status LPREG VH (1.2v) during DEEPSTOP - 1: LPREG=1.2V during DEEPSTOP - 0: LPREG=1V during DEEPSTOP"]
pub type LpregVhStatusR = crate::BitReader;
#[doc = "Field `GPIORET` reader - GPIORET: GPIO retention enable. - 0: Release GPIO retention after deepstop (Should be reset after restore Context) - 1: Enable GPIO Retention during deepstop (Must be set before deepstop)"]
pub type GpioretR = crate::BitReader;
#[doc = "Field `GPIORET` writer - GPIORET: GPIO retention enable. - 0: Release GPIO retention after deepstop (Should be reset after restore Context) - 1: Enable GPIO Retention during deepstop (Must be set before deepstop)"]
pub type GpioretW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENTS` reader - ENTS: Enable Temperature Sensor - 1: Temperature sensor is enabled - 0: Temperature sensor is disabled"]
pub type EntsR = crate::BitReader;
#[doc = "Field `ENTS` writer - ENTS: Enable Temperature Sensor - 1: Temperature sensor is enabled - 0: Temperature sensor is disabled"]
pub type EntsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFREGEN` reader - RFREGEN: RF Regulator Enable - 1: Enable RF Regulator - 0: Disable RF Regulator (Note: RF Regulator can still be enabled by the RFSUGB or RCC_CR.HSEON)"]
pub type RfregenR = crate::BitReader;
#[doc = "Field `RFREGEN` writer - RFREGEN: RF Regulator Enable - 1: Enable RF Regulator - 0: Disable RF Regulator (Note: RF Regulator can still be enabled by the RFSUGB or RCC_CR.HSEON)"]
pub type RfregenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFREGCEXT` reader - RFREGCEXT: RF Regulator External Supply Bypass - 1: External supply bypass capability - 0: Internal supply only"]
pub type RfregcextR = crate::BitReader;
#[doc = "Field `RFREGCEXT` writer - RFREGCEXT: RF Regulator External Supply Bypass - 1: External supply bypass capability - 0: Internal supply only"]
pub type RfregcextW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFREGBYP` reader - RFREGBYP: RF Regulator Bypass Enable - 1: LDO output connected to VSMPS. - 0: internally generated 1.2V"]
pub type RfregbypR = crate::BitReader;
#[doc = "Field `RFREGBYP` writer - RFREGBYP: RF Regulator Bypass Enable - 1: LDO output connected to VSMPS. - 0: internally generated 1.2V"]
pub type RfregbypW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFREGRDY` reader - RFDREGRDY: RF Regulator Ready flag - 1: RF Regulator is ready - 0: RF Regulator is not ready"]
pub type RfregrdyR = crate::BitReader;
#[doc = "Field `RFREGON_STATUS` reader - RFREGON_STATUS: RF Regulator On Status - 1: RF Regulator is enabled - 0: RF Regulator is disabled"]
pub type RfregonStatusR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - PVDE Programmable Voltage Detector Enable When this bit is set the Power Voltage Detector is enabled"]
    #[inline(always)]
    pub fn pvde(&self) -> PvdeR {
        PvdeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - PVDLS\\[2:0\\] Programmable Voltage Detector Level selection - 000: 2.05 V - Lowest level - 001: 2.20 V - 010: 2.36 V - 011: 2.52 V - 100: 2.64 V - 101: 2.81 V - 110: 2.91 V - Highest level - 111: External input analog voltage (compare internally to VBGP; When external input VBGP then PVDO=1)"]
    #[inline(always)]
    pub fn pvdls(&self) -> PvdlsR {
        PvdlsR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - DBGRET: PA2 and PA3 retention enable after DEEPSTOP - 0: PA2, PA3 don't retain their status exiting from DEEPSTOP (default). - 1: PA2, PA3 retain their status exiting from DEEPSTOP."]
    #[inline(always)]
    pub fn dbgret(&self) -> DbgretR {
        DbgretR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RAMRET1: RAM1 retention during low power mode - 1: RAM1 bank is powered during low power mode - 0: RAM1 bank is disabled during low power mode (by default)"]
    #[inline(always)]
    pub fn ramret1(&self) -> Ramret1R {
        Ramret1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - force LPREG=1.2V during DEEPSTOP - 1: Force LPREG=1.2V during DEEPSTOP - 0: No Force (Default) Note LPREG= 1.2v can still apply when LCDEN or COMP.SCALEREN request it"]
    #[inline(always)]
    pub fn lpreg_force_vh(&self) -> LpregForceVhR {
        LpregForceVhR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - status LPREG VH (1.2v) during DEEPSTOP - 1: LPREG=1.2V during DEEPSTOP - 0: LPREG=1V during DEEPSTOP"]
    #[inline(always)]
    pub fn lpreg_vh_status(&self) -> LpregVhStatusR {
        LpregVhStatusR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIORET: GPIO retention enable. - 0: Release GPIO retention after deepstop (Should be reset after restore Context) - 1: Enable GPIO Retention during deepstop (Must be set before deepstop)"]
    #[inline(always)]
    pub fn gpioret(&self) -> GpioretR {
        GpioretR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ENTS: Enable Temperature Sensor - 1: Temperature sensor is enabled - 0: Temperature sensor is disabled"]
    #[inline(always)]
    pub fn ents(&self) -> EntsR {
        EntsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RFREGEN: RF Regulator Enable - 1: Enable RF Regulator - 0: Disable RF Regulator (Note: RF Regulator can still be enabled by the RFSUGB or RCC_CR.HSEON)"]
    #[inline(always)]
    pub fn rfregen(&self) -> RfregenR {
        RfregenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RFREGCEXT: RF Regulator External Supply Bypass - 1: External supply bypass capability - 0: Internal supply only"]
    #[inline(always)]
    pub fn rfregcext(&self) -> RfregcextR {
        RfregcextR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RFREGBYP: RF Regulator Bypass Enable - 1: LDO output connected to VSMPS. - 0: internally generated 1.2V"]
    #[inline(always)]
    pub fn rfregbyp(&self) -> RfregbypR {
        RfregbypR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RFDREGRDY: RF Regulator Ready flag - 1: RF Regulator is ready - 0: RF Regulator is not ready"]
    #[inline(always)]
    pub fn rfregrdy(&self) -> RfregrdyR {
        RfregrdyR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RFREGON_STATUS: RF Regulator On Status - 1: RF Regulator is enabled - 0: RF Regulator is disabled"]
    #[inline(always)]
    pub fn rfregon_status(&self) -> RfregonStatusR {
        RfregonStatusR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PVDE Programmable Voltage Detector Enable When this bit is set the Power Voltage Detector is enabled"]
    #[inline(always)]
    pub fn pvde(&mut self) -> PvdeW<'_, Cr2Spec> {
        PvdeW::new(self, 0)
    }
    #[doc = "Bits 1:3 - PVDLS\\[2:0\\] Programmable Voltage Detector Level selection - 000: 2.05 V - Lowest level - 001: 2.20 V - 010: 2.36 V - 011: 2.52 V - 100: 2.64 V - 101: 2.81 V - 110: 2.91 V - Highest level - 111: External input analog voltage (compare internally to VBGP; When external input VBGP then PVDO=1)"]
    #[inline(always)]
    pub fn pvdls(&mut self) -> PvdlsW<'_, Cr2Spec> {
        PvdlsW::new(self, 1)
    }
    #[doc = "Bit 4 - DBGRET: PA2 and PA3 retention enable after DEEPSTOP - 0: PA2, PA3 don't retain their status exiting from DEEPSTOP (default). - 1: PA2, PA3 retain their status exiting from DEEPSTOP."]
    #[inline(always)]
    pub fn dbgret(&mut self) -> DbgretW<'_, Cr2Spec> {
        DbgretW::new(self, 4)
    }
    #[doc = "Bit 5 - RAMRET1: RAM1 retention during low power mode - 1: RAM1 bank is powered during low power mode - 0: RAM1 bank is disabled during low power mode (by default)"]
    #[inline(always)]
    pub fn ramret1(&mut self) -> Ramret1W<'_, Cr2Spec> {
        Ramret1W::new(self, 5)
    }
    #[doc = "Bit 6 - force LPREG=1.2V during DEEPSTOP - 1: Force LPREG=1.2V during DEEPSTOP - 0: No Force (Default) Note LPREG= 1.2v can still apply when LCDEN or COMP.SCALEREN request it"]
    #[inline(always)]
    pub fn lpreg_force_vh(&mut self) -> LpregForceVhW<'_, Cr2Spec> {
        LpregForceVhW::new(self, 6)
    }
    #[doc = "Bit 8 - GPIORET: GPIO retention enable. - 0: Release GPIO retention after deepstop (Should be reset after restore Context) - 1: Enable GPIO Retention during deepstop (Must be set before deepstop)"]
    #[inline(always)]
    pub fn gpioret(&mut self) -> GpioretW<'_, Cr2Spec> {
        GpioretW::new(self, 8)
    }
    #[doc = "Bit 9 - ENTS: Enable Temperature Sensor - 1: Temperature sensor is enabled - 0: Temperature sensor is disabled"]
    #[inline(always)]
    pub fn ents(&mut self) -> EntsW<'_, Cr2Spec> {
        EntsW::new(self, 9)
    }
    #[doc = "Bit 10 - RFREGEN: RF Regulator Enable - 1: Enable RF Regulator - 0: Disable RF Regulator (Note: RF Regulator can still be enabled by the RFSUGB or RCC_CR.HSEON)"]
    #[inline(always)]
    pub fn rfregen(&mut self) -> RfregenW<'_, Cr2Spec> {
        RfregenW::new(self, 10)
    }
    #[doc = "Bit 11 - RFREGCEXT: RF Regulator External Supply Bypass - 1: External supply bypass capability - 0: Internal supply only"]
    #[inline(always)]
    pub fn rfregcext(&mut self) -> RfregcextW<'_, Cr2Spec> {
        RfregcextW::new(self, 11)
    }
    #[doc = "Bit 12 - RFREGBYP: RF Regulator Bypass Enable - 1: LDO output connected to VSMPS. - 0: internally generated 1.2V"]
    #[inline(always)]
    pub fn rfregbyp(&mut self) -> RfregbypW<'_, Cr2Spec> {
        RfregbypW::new(self, 12)
    }
}
#[doc = "CR2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {}
