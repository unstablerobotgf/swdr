#[doc = "Register `DHR` reader"]
pub type R = crate::R<DhrSpec>;
#[doc = "Register `DHR` writer"]
pub type W = crate::W<DhrSpec>;
#[doc = "Field `DACDHR` reader - DACDHR\\[5:0\\]: DAC channel 6-bit data These bits are written by software which specifies 6-bit data for DAC channel."]
pub type DacdhrR = crate::FieldReader;
#[doc = "Field `DACDHR` writer - DACDHR\\[5:0\\]: DAC channel 6-bit data These bits are written by software which specifies 6-bit data for DAC channel."]
pub type DacdhrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - DACDHR\\[5:0\\]: DAC channel 6-bit data These bits are written by software which specifies 6-bit data for DAC channel."]
    #[inline(always)]
    pub fn dacdhr(&self) -> DacdhrR {
        DacdhrR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - DACDHR\\[5:0\\]: DAC channel 6-bit data These bits are written by software which specifies 6-bit data for DAC channel."]
    #[inline(always)]
    pub fn dacdhr(&mut self) -> DacdhrW<'_, DhrSpec> {
        DacdhrW::new(self, 0)
    }
}
#[doc = "DHR register\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DhrSpec;
impl crate::RegisterSpec for DhrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dhr::R`](R) reader structure"]
impl crate::Readable for DhrSpec {}
#[doc = "`write(|w| ..)` method takes [`dhr::W`](W) writer structure"]
impl crate::Writable for DhrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DHR to value 0"]
impl crate::Resettable for DhrSpec {}
