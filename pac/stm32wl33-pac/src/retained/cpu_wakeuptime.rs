#[doc = "Register `CPU_WAKEUPTIME` reader"]
pub type R = crate::R<CpuWakeuptimeSpec>;
#[doc = "Register `CPU_WAKEUPTIME` writer"]
pub type W = crate::W<CpuWakeuptimeSpec>;
#[doc = "Field `CPU_WAKEUPTIME` reader - (Absolute) Target time to wakeup the CPU."]
pub type CpuWakeuptimeR = crate::FieldReader<u32>;
#[doc = "Field `CPU_WAKEUPTIME` writer - (Absolute) Target time to wakeup the CPU."]
pub type CpuWakeuptimeW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 1:31 - (Absolute) Target time to wakeup the CPU."]
    #[inline(always)]
    pub fn cpu_wakeuptime(&self) -> CpuWakeuptimeR {
        CpuWakeuptimeR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 1:31 - (Absolute) Target time to wakeup the CPU."]
    #[inline(always)]
    pub fn cpu_wakeuptime(&mut self) -> CpuWakeuptimeW<'_, CpuWakeuptimeSpec> {
        CpuWakeuptimeW::new(self, 1)
    }
}
#[doc = "CPU_WAKEUPTIME register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_wakeuptime::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_wakeuptime::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuWakeuptimeSpec;
impl crate::RegisterSpec for CpuWakeuptimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_wakeuptime::R`](R) reader structure"]
impl crate::Readable for CpuWakeuptimeSpec {}
#[doc = "`write(|w| ..)` method takes [`cpu_wakeuptime::W`](W) writer structure"]
impl crate::Writable for CpuWakeuptimeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_WAKEUPTIME to value 0"]
impl crate::Resettable for CpuWakeuptimeSpec {}
