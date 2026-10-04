#[doc = "Register `PCKTLEN_CONFIG` reader"]
pub type R = crate::R<PcktlenConfigSpec>;
#[doc = "Register `PCKTLEN_CONFIG` writer"]
pub type W = crate::W<PcktlenConfigSpec>;
#[doc = "Field `PCKTLEN` reader - This bit field has different meanings/usages:"]
pub type PcktlenR = crate::FieldReader<u16>;
#[doc = "Field `PCKTLEN` writer - This bit field has different meanings/usages:"]
pub type PcktlenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This bit field has different meanings/usages:"]
    #[inline(always)]
    pub fn pcktlen(&self) -> PcktlenR {
        PcktlenR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This bit field has different meanings/usages:"]
    #[inline(always)]
    pub fn pcktlen(&mut self) -> PcktlenW<'_, PcktlenConfigSpec> {
        PcktlenW::new(self, 0)
    }
}
#[doc = "PCKTLEN_CONFIG register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcktlen_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcktlen_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcktlenConfigSpec;
impl crate::RegisterSpec for PcktlenConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcktlen_config::R`](R) reader structure"]
impl crate::Readable for PcktlenConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`pcktlen_config::W`](W) writer structure"]
impl crate::Writable for PcktlenConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCKTLEN_CONFIG to value 0x14"]
impl crate::Resettable for PcktlenConfigSpec {
    const RESET_VALUE: u32 = 0x14;
}
