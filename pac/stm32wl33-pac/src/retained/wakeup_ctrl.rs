#[doc = "Register `WAKEUP_CTRL` reader"]
pub type R = crate::R<WakeupCtrlSpec>;
#[doc = "Register `WAKEUP_CTRL` writer"]
pub type W = crate::W<WakeupCtrlSpec>;
#[doc = "Field `SOC_WAKEUP_OFFSET` reader - Delay to be considered by the Wakeup block to anticipate the wakeup request to the PWRC of the SoC versus the target to wakeup the RFIP (or the CPU)."]
pub type SocWakeupOffsetR = crate::FieldReader;
#[doc = "Field `SOC_WAKEUP_OFFSET` writer - Delay to be considered by the Wakeup block to anticipate the wakeup request to the PWRC of the SoC versus the target to wakeup the RFIP (or the CPU)."]
pub type SocWakeupOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CPU_WAKEUP_EN` reader - Indicates if the wakeup timer has to wakeup the SoC (match on CPU_WAKEUPTIME\\[31:4\\] bit field only) + set the CPU_WAKEUP_F in the WAKEUP_IRQ_STATUS Misc register when match on CPU_WAKEUPTIME\\[31:0\\] occurs."]
pub type CpuWakeupEnR = crate::BitReader;
#[doc = "Field `CPU_WAKEUP_EN` writer - Indicates if the wakeup timer has to wakeup the SoC (match on CPU_WAKEUPTIME\\[31:4\\] bit field only) + set the CPU_WAKEUP_F in the WAKEUP_IRQ_STATUS Misc register when match on CPU_WAKEUPTIME\\[31:0\\] occurs."]
pub type CpuWakeupEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFIP_WAKEUP_EN` reader - Indicates if the wakeup timer has to wakeup the SoC (match on RFIP_WAKEUPTIME\\[31:4\\] bit field only) + trigger an event on the Sequencer and set the RFIP_WAKEUP_F in the WAKEUP_IRQ_STATUS Misc register when match on RFIP_WAKEUPTIME\\[31:0\\] occurs."]
pub type RfipWakeupEnR = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - Delay to be considered by the Wakeup block to anticipate the wakeup request to the PWRC of the SoC versus the target to wakeup the RFIP (or the CPU)."]
    #[inline(always)]
    pub fn soc_wakeup_offset(&self) -> SocWakeupOffsetR {
        SocWakeupOffsetR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 30 - Indicates if the wakeup timer has to wakeup the SoC (match on CPU_WAKEUPTIME\\[31:4\\] bit field only) + set the CPU_WAKEUP_F in the WAKEUP_IRQ_STATUS Misc register when match on CPU_WAKEUPTIME\\[31:0\\] occurs."]
    #[inline(always)]
    pub fn cpu_wakeup_en(&self) -> CpuWakeupEnR {
        CpuWakeupEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Indicates if the wakeup timer has to wakeup the SoC (match on RFIP_WAKEUPTIME\\[31:4\\] bit field only) + trigger an event on the Sequencer and set the RFIP_WAKEUP_F in the WAKEUP_IRQ_STATUS Misc register when match on RFIP_WAKEUPTIME\\[31:0\\] occurs."]
    #[inline(always)]
    pub fn rfip_wakeup_en(&self) -> RfipWakeupEnR {
        RfipWakeupEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Delay to be considered by the Wakeup block to anticipate the wakeup request to the PWRC of the SoC versus the target to wakeup the RFIP (or the CPU)."]
    #[inline(always)]
    pub fn soc_wakeup_offset(&mut self) -> SocWakeupOffsetW<'_, WakeupCtrlSpec> {
        SocWakeupOffsetW::new(self, 0)
    }
    #[doc = "Bit 30 - Indicates if the wakeup timer has to wakeup the SoC (match on CPU_WAKEUPTIME\\[31:4\\] bit field only) + set the CPU_WAKEUP_F in the WAKEUP_IRQ_STATUS Misc register when match on CPU_WAKEUPTIME\\[31:0\\] occurs."]
    #[inline(always)]
    pub fn cpu_wakeup_en(&mut self) -> CpuWakeupEnW<'_, WakeupCtrlSpec> {
        CpuWakeupEnW::new(self, 30)
    }
}
#[doc = "WAKEUP_CTRL register\n\nYou can [`read`](crate::Reg::read) this register and get [`wakeup_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeup_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WakeupCtrlSpec;
impl crate::RegisterSpec for WakeupCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wakeup_ctrl::R`](R) reader structure"]
impl crate::Readable for WakeupCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`wakeup_ctrl::W`](W) writer structure"]
impl crate::Writable for WakeupCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WAKEUP_CTRL to value 0"]
impl crate::Resettable for WakeupCtrlSpec {}
