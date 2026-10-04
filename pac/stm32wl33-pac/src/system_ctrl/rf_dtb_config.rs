#[doc = "Register `RF_DTB_CONFIG` reader"]
pub type R = crate::R<RfDtbConfigSpec>;
#[doc = "Register `RF_DTB_CONFIG` writer"]
pub type W = crate::W<RfDtbConfigSpec>;
#[doc = "Field `RF_DTB_CONFIG` reader - Controlling AF7 extended mode: - 00 : MR_SUBG DTB default configuration - 01 : MR_SUBG DTB shuffled configuration - 10 : BUBBLE_DTB configuration - 11 : MR_SUBG DTB default configuration (as per 00)"]
pub type RfDtbConfigR = crate::FieldReader;
#[doc = "Field `RF_DTB_CONFIG` writer - Controlling AF7 extended mode: - 00 : MR_SUBG DTB default configuration - 01 : MR_SUBG DTB shuffled configuration - 10 : BUBBLE_DTB configuration - 11 : MR_SUBG DTB default configuration (as per 00)"]
pub type RfDtbConfigW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Controlling AF7 extended mode: - 00 : MR_SUBG DTB default configuration - 01 : MR_SUBG DTB shuffled configuration - 10 : BUBBLE_DTB configuration - 11 : MR_SUBG DTB default configuration (as per 00)"]
    #[inline(always)]
    pub fn rf_dtb_config(&self) -> RfDtbConfigR {
        RfDtbConfigR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Controlling AF7 extended mode: - 00 : MR_SUBG DTB default configuration - 01 : MR_SUBG DTB shuffled configuration - 10 : BUBBLE_DTB configuration - 11 : MR_SUBG DTB default configuration (as per 00)"]
    #[inline(always)]
    pub fn rf_dtb_config(&mut self) -> RfDtbConfigW<'_, RfDtbConfigSpec> {
        RfDtbConfigW::new(self, 0)
    }
}
#[doc = "RF_DTB_CONFIG register\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_dtb_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_dtb_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfDtbConfigSpec;
impl crate::RegisterSpec for RfDtbConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_dtb_config::R`](R) reader structure"]
impl crate::Readable for RfDtbConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`rf_dtb_config::W`](W) writer structure"]
impl crate::Writable for RfDtbConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RF_DTB_CONFIG to value 0"]
impl crate::Resettable for RfDtbConfigSpec {}
