#[doc = "Register `WAKEUP_IRQ_STATUS` reader"]
pub type R = crate::R<WakeupIrqStatusSpec>;
#[doc = "Register `WAKEUP_IRQ_STATUS` writer"]
pub type W = crate::W<WakeupIrqStatusSpec>;
#[doc = "Field `CPU_WAKEUP_F` reader - Set when the interpolated absolute time matches the CPU_WAKEUPTIME while WAKEUP_CTRL."]
pub type CpuWakeupFR = crate::BitReader;
#[doc = "Field `CPU_WAKEUP_F` writer - Set when the interpolated absolute time matches the CPU_WAKEUPTIME while WAKEUP_CTRL."]
pub type CpuWakeupFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFIP_WAKEUP_F` reader - Set when the interpolated absolute time matches the RFIP_WAKEUPTIME while WAKEUP_CTRL."]
pub type RfipWakeupFR = crate::BitReader;
#[doc = "Field `RFIP_WAKEUP_F` writer - Set when the interpolated absolute time matches the RFIP_WAKEUPTIME while WAKEUP_CTRL."]
pub type RfipWakeupFW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set when the interpolated absolute time matches the CPU_WAKEUPTIME while WAKEUP_CTRL."]
    #[inline(always)]
    pub fn cpu_wakeup_f(&self) -> CpuWakeupFR {
        CpuWakeupFR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set when the interpolated absolute time matches the RFIP_WAKEUPTIME while WAKEUP_CTRL."]
    #[inline(always)]
    pub fn rfip_wakeup_f(&self) -> RfipWakeupFR {
        RfipWakeupFR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set when the interpolated absolute time matches the CPU_WAKEUPTIME while WAKEUP_CTRL."]
    #[inline(always)]
    pub fn cpu_wakeup_f(&mut self) -> CpuWakeupFW<'_, WakeupIrqStatusSpec> {
        CpuWakeupFW::new(self, 0)
    }
    #[doc = "Bit 1 - Set when the interpolated absolute time matches the RFIP_WAKEUPTIME while WAKEUP_CTRL."]
    #[inline(always)]
    pub fn rfip_wakeup_f(&mut self) -> RfipWakeupFW<'_, WakeupIrqStatusSpec> {
        RfipWakeupFW::new(self, 1)
    }
}
#[doc = "WAKEUP_IRQ_STATUS register\n\nYou can [`read`](crate::Reg::read) this register and get [`wakeup_irq_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeup_irq_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WakeupIrqStatusSpec;
impl crate::RegisterSpec for WakeupIrqStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wakeup_irq_status::R`](R) reader structure"]
impl crate::Readable for WakeupIrqStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`wakeup_irq_status::W`](W) writer structure"]
impl crate::Writable for WakeupIrqStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WAKEUP_IRQ_STATUS to value 0"]
impl crate::Resettable for WakeupIrqStatusSpec {}
