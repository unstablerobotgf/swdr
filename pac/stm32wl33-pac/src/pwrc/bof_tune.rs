#[doc = "Register `BOF_TUNE` reader"]
pub type R = crate::R<BofTuneSpec>;
#[doc = "Register `BOF_TUNE` writer"]
pub type W = crate::W<BofTuneSpec>;
#[doc = "Field `BOF_TUNE` reader - BOF_TUNE: selection of the Bypass on the Fly LDO output voltage. - 0: 1.2V - 1: 1.2V - 2: 1.2V - 3: 1.3V - 4: 1.4V (Default) - 5: 1.5V - 6: 1.6V - 7: 1.7V - 8: 1.8V - 9: 1.9V - 10: 2V - 11: 2.1V - 12: 2.2V - 13: 2.3V - 14: 2.4V - 15: 2.4V"]
pub type BofTuneR = crate::FieldReader;
#[doc = "Field `BOF_TUNE` writer - BOF_TUNE: selection of the Bypass on the Fly LDO output voltage. - 0: 1.2V - 1: 1.2V - 2: 1.2V - 3: 1.3V - 4: 1.4V (Default) - 5: 1.5V - 6: 1.6V - 7: 1.7V - 8: 1.8V - 9: 1.9V - 10: 2V - 11: 2.1V - 12: 2.2V - 13: 2.3V - 14: 2.4V - 15: 2.4V"]
pub type BofTuneW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - BOF_TUNE: selection of the Bypass on the Fly LDO output voltage. - 0: 1.2V - 1: 1.2V - 2: 1.2V - 3: 1.3V - 4: 1.4V (Default) - 5: 1.5V - 6: 1.6V - 7: 1.7V - 8: 1.8V - 9: 1.9V - 10: 2V - 11: 2.1V - 12: 2.2V - 13: 2.3V - 14: 2.4V - 15: 2.4V"]
    #[inline(always)]
    pub fn bof_tune(&self) -> BofTuneR {
        BofTuneR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - BOF_TUNE: selection of the Bypass on the Fly LDO output voltage. - 0: 1.2V - 1: 1.2V - 2: 1.2V - 3: 1.3V - 4: 1.4V (Default) - 5: 1.5V - 6: 1.6V - 7: 1.7V - 8: 1.8V - 9: 1.9V - 10: 2V - 11: 2.1V - 12: 2.2V - 13: 2.3V - 14: 2.4V - 15: 2.4V"]
    #[inline(always)]
    pub fn bof_tune(&mut self) -> BofTuneW<'_, BofTuneSpec> {
        BofTuneW::new(self, 0)
    }
}
#[doc = "BOF_TUNE register\n\nYou can [`read`](crate::Reg::read) this register and get [`bof_tune::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bof_tune::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BofTuneSpec;
impl crate::RegisterSpec for BofTuneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bof_tune::R`](R) reader structure"]
impl crate::Readable for BofTuneSpec {}
#[doc = "`write(|w| ..)` method takes [`bof_tune::W`](W) writer structure"]
impl crate::Writable for BofTuneSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BOF_TUNE to value 0x04"]
impl crate::Resettable for BofTuneSpec {
    const RESET_VALUE: u32 = 0x04;
}
