#[doc = "Register `LCSC_CONFR` reader"]
pub type R = crate::R<LcscConfrSpec>;
#[doc = "Register `LCSC_CONFR` writer"]
pub type W = crate::W<LcscConfrSpec>;
#[doc = "Field `CLKWISE_THRES` reader - Number of Clock Wise revolutions target"]
pub type ClkwiseThresR = crate::FieldReader<u16>;
#[doc = "Field `CLKWISE_THRES` writer - Number of Clock Wise revolutions target"]
pub type ClkwiseThresW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ACLKWISE_THRES` reader - Number of Anti Clock Wise revolutions target"]
pub type AclkwiseThresR = crate::FieldReader<u16>;
#[doc = "Field `ACLKWISE_THRES` writer - Number of Anti Clock Wise revolutions target"]
pub type AclkwiseThresW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of Clock Wise revolutions target"]
    #[inline(always)]
    pub fn clkwise_thres(&self) -> ClkwiseThresR {
        ClkwiseThresR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Number of Anti Clock Wise revolutions target"]
    #[inline(always)]
    pub fn aclkwise_thres(&self) -> AclkwiseThresR {
        AclkwiseThresR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of Clock Wise revolutions target"]
    #[inline(always)]
    pub fn clkwise_thres(&mut self) -> ClkwiseThresW<'_, LcscConfrSpec> {
        ClkwiseThresW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Number of Anti Clock Wise revolutions target"]
    #[inline(always)]
    pub fn aclkwise_thres(&mut self) -> AclkwiseThresW<'_, LcscConfrSpec> {
        AclkwiseThresW::new(self, 16)
    }
}
#[doc = "LCSC_CONFR register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcsc_confr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcsc_confr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcscConfrSpec;
impl crate::RegisterSpec for LcscConfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcsc_confr::R`](R) reader structure"]
impl crate::Readable for LcscConfrSpec {}
#[doc = "`write(|w| ..)` method takes [`lcsc_confr::W`](W) writer structure"]
impl crate::Writable for LcscConfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCSC_CONFR to value 0"]
impl crate::Resettable for LcscConfrSpec {}
