#[doc = "Register `APB0ENR` reader"]
pub type R = crate::R<Apb0enrSpec>;
#[doc = "Register `APB0ENR` writer"]
pub type W = crate::W<Apb0enrSpec>;
#[doc = "Field `TIM2EN` reader - TIM2: Advanced Timer clock enable Set and enable by software. 0: clock disable 1: clock enable"]
pub type Tim2enR = crate::BitReader;
#[doc = "Field `TIM2EN` writer - TIM2: Advanced Timer clock enable Set and enable by software. 0: clock disable 1: clock enable"]
pub type Tim2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16EN` reader - TIM16: Advanced Timer clock enable Set and enable by software. 0: clock disable 1: clock enable"]
pub type Tim16enR = crate::BitReader;
#[doc = "Field `TIM16EN` writer - TIM16: Advanced Timer clock enable Set and enable by software. 0: clock disable 1: clock enable"]
pub type Tim16enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCFGEN` reader - SYSTEM CONFIG clock enable Set and enable by software. 0: clock disable 1: clock enable"]
pub type SyscfgenR = crate::BitReader;
#[doc = "Field `SYSCFGEN` writer - SYSTEM CONFIG clock enable Set and enable by software. 0: clock disable 1: clock enable"]
pub type SyscfgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDEN` reader - LCD clock enable Set and enable by software. 0: clock disable 1: clock enable"]
pub type LcdenR = crate::BitReader;
#[doc = "Field `LCDEN` writer - LCD clock enable Set and enable by software. 0: clock disable 1: clock enable"]
pub type LcdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPEN` reader - COMP clock enable Set and enable by software. 0: clock disable 1: clock enable"]
pub type CompenR = crate::BitReader;
#[doc = "Field `COMPEN` writer - COMP clock enable Set and enable by software. 0: clock disable 1: clock enable"]
pub type CompenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DACEN` reader - DAC clock enable Set and enable by software. 0: clock disable 1: clock enable"]
pub type DacenR = crate::BitReader;
#[doc = "Field `DACEN` writer - DAC clock enable Set and enable by software. 0: clock disable 1: clock enable"]
pub type DacenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCEN` reader - RTC clock enable Set and enable by software. Reset source only for this field: PORESETn 0: clock disable 1: clock enable"]
pub type RtcenR = crate::BitReader;
#[doc = "Field `RTCEN` writer - RTC clock enable Set and enable by software. Reset source only for this field: PORESETn 0: clock disable 1: clock enable"]
pub type RtcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCSCEN` reader - LCSC clock enable. Set and enable by software. 0: clock disable 1: clock enable"]
pub type LcscenR = crate::BitReader;
#[doc = "Field `LCSCEN` writer - LCSC clock enable. Set and enable by software. 0: clock disable 1: clock enable"]
pub type LcscenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDGEN` reader - Watchdog clock enable. Set and enable by software. 0: clock disable 1: clock enable"]
pub type WdgenR = crate::BitReader;
#[doc = "Field `WDGEN` writer - Watchdog clock enable. Set and enable by software. 0: clock disable 1: clock enable"]
pub type WdgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGMCUEN` reader - DBG MCU clock enable. Set and enable by software. 0: clock disable 1: clock enable"]
pub type DbgmcuenR = crate::BitReader;
#[doc = "Field `DBGMCUEN` writer - DBG MCU clock enable. Set and enable by software. 0: clock disable 1: clock enable"]
pub type DbgmcuenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM2: Advanced Timer clock enable Set and enable by software. 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn tim2en(&self) -> Tim2enR {
        Tim2enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM16: Advanced Timer clock enable Set and enable by software. 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn tim16en(&self) -> Tim16enR {
        Tim16enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - SYSTEM CONFIG clock enable Set and enable by software. 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn syscfgen(&self) -> SyscfgenR {
        SyscfgenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LCD clock enable Set and enable by software. 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn lcden(&self) -> LcdenR {
        LcdenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - COMP clock enable Set and enable by software. 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn compen(&self) -> CompenR {
        CompenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DAC clock enable Set and enable by software. 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn dacen(&self) -> DacenR {
        DacenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RTC clock enable Set and enable by software. Reset source only for this field: PORESETn 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn rtcen(&self) -> RtcenR {
        RtcenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LCSC clock enable. Set and enable by software. 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn lcscen(&self) -> LcscenR {
        LcscenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Watchdog clock enable. Set and enable by software. 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn wdgen(&self) -> WdgenR {
        WdgenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DBG MCU clock enable. Set and enable by software. 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn dbgmcuen(&self) -> DbgmcuenR {
        DbgmcuenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2: Advanced Timer clock enable Set and enable by software. 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn tim2en(&mut self) -> Tim2enW<'_, Apb0enrSpec> {
        Tim2enW::new(self, 0)
    }
    #[doc = "Bit 1 - TIM16: Advanced Timer clock enable Set and enable by software. 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn tim16en(&mut self) -> Tim16enW<'_, Apb0enrSpec> {
        Tim16enW::new(self, 1)
    }
    #[doc = "Bit 8 - SYSTEM CONFIG clock enable Set and enable by software. 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn syscfgen(&mut self) -> SyscfgenW<'_, Apb0enrSpec> {
        SyscfgenW::new(self, 8)
    }
    #[doc = "Bit 9 - LCD clock enable Set and enable by software. 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn lcden(&mut self) -> LcdenW<'_, Apb0enrSpec> {
        LcdenW::new(self, 9)
    }
    #[doc = "Bit 10 - COMP clock enable Set and enable by software. 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn compen(&mut self) -> CompenW<'_, Apb0enrSpec> {
        CompenW::new(self, 10)
    }
    #[doc = "Bit 11 - DAC clock enable Set and enable by software. 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn dacen(&mut self) -> DacenW<'_, Apb0enrSpec> {
        DacenW::new(self, 11)
    }
    #[doc = "Bit 12 - RTC clock enable Set and enable by software. Reset source only for this field: PORESETn 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn rtcen(&mut self) -> RtcenW<'_, Apb0enrSpec> {
        RtcenW::new(self, 12)
    }
    #[doc = "Bit 13 - LCSC clock enable. Set and enable by software. 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn lcscen(&mut self) -> LcscenW<'_, Apb0enrSpec> {
        LcscenW::new(self, 13)
    }
    #[doc = "Bit 14 - Watchdog clock enable. Set and enable by software. 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn wdgen(&mut self) -> WdgenW<'_, Apb0enrSpec> {
        WdgenW::new(self, 14)
    }
    #[doc = "Bit 15 - DBG MCU clock enable. Set and enable by software. 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn dbgmcuen(&mut self) -> DbgmcuenW<'_, Apb0enrSpec> {
        DbgmcuenW::new(self, 15)
    }
}
#[doc = "APB0ENR register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb0enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb0enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb0enrSpec;
impl crate::RegisterSpec for Apb0enrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb0enr::R`](R) reader structure"]
impl crate::Readable for Apb0enrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb0enr::W`](W) writer structure"]
impl crate::Writable for Apb0enrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB0ENR to value 0"]
impl crate::Resettable for Apb0enrSpec {}
