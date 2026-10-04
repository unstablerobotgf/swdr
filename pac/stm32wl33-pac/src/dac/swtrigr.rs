#[doc = "Register `SWTRIGR` reader"]
pub type R = crate::R<SwtrigrSpec>;
#[doc = "Register `SWTRIGR` writer"]
pub type W = crate::W<SwtrigrSpec>;
#[doc = "Field `SWTRIG` writer - SWTRIG: DAC channel software trigger This bit is set by software to enable/disable the software trigger. 0: Software trigger disabled 1: Software trigger enabled Note: This bit is cleared by hardware (one APB0 clock cycle later) once the DAC_DHR register value has been loaded into the DAC_DOR register."]
pub type SwtrigW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - SWTRIG: DAC channel software trigger This bit is set by software to enable/disable the software trigger. 0: Software trigger disabled 1: Software trigger enabled Note: This bit is cleared by hardware (one APB0 clock cycle later) once the DAC_DHR register value has been loaded into the DAC_DOR register."]
    #[inline(always)]
    pub fn swtrig(&mut self) -> SwtrigW<'_, SwtrigrSpec> {
        SwtrigW::new(self, 0)
    }
}
#[doc = "SWTRIGR register\n\nYou can [`read`](crate::Reg::read) this register and get [`swtrigr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrigr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwtrigrSpec;
impl crate::RegisterSpec for SwtrigrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swtrigr::R`](R) reader structure"]
impl crate::Readable for SwtrigrSpec {}
#[doc = "`write(|w| ..)` method takes [`swtrigr::W`](W) writer structure"]
impl crate::Writable for SwtrigrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWTRIGR to value 0"]
impl crate::Resettable for SwtrigrSpec {}
