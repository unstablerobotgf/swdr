#[doc = "Register `ENGTRIM2` reader"]
pub type R = crate::R<Engtrim2Spec>;
#[doc = "Register `ENGTRIM2` writer"]
pub type W = crate::W<Engtrim2Spec>;
#[doc = "Field `BOFTRIMEN` reader - BOFTRIMEN: trimming BOF enabled - 1: trimming bit applied from ENGTRIM2 register - 0: trimming bit applied from OBL (can be read on TRIMR register)"]
pub type BoftrimenR = crate::BitReader;
#[doc = "Field `BOFTRIMEN` writer - BOFTRIMEN: trimming BOF enabled - 1: trimming bit applied from ENGTRIM2 register - 0: trimming bit applied from OBL (can be read on TRIMR register)"]
pub type BoftrimenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOF_TRIM` reader - SMPS_TRIM: SMPS Output Voltage Trimming By default, this value is not applied, but taken from the engi bytes; if ENGTRIM.BOFTRIMEN=1, the SMPS output voltage can be controlled by this register."]
pub type BofTrimR = crate::FieldReader;
#[doc = "Field `BOF_TRIM` writer - SMPS_TRIM: SMPS Output Voltage Trimming By default, this value is not applied, but taken from the engi bytes; if ENGTRIM.BOFTRIMEN=1, the SMPS output voltage can be controlled by this register."]
pub type BofTrimW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - BOFTRIMEN: trimming BOF enabled - 1: trimming bit applied from ENGTRIM2 register - 0: trimming bit applied from OBL (can be read on TRIMR register)"]
    #[inline(always)]
    pub fn boftrimen(&self) -> BoftrimenR {
        BoftrimenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - SMPS_TRIM: SMPS Output Voltage Trimming By default, this value is not applied, but taken from the engi bytes; if ENGTRIM.BOFTRIMEN=1, the SMPS output voltage can be controlled by this register."]
    #[inline(always)]
    pub fn bof_trim(&self) -> BofTrimR {
        BofTrimR::new(((self.bits >> 1) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - BOFTRIMEN: trimming BOF enabled - 1: trimming bit applied from ENGTRIM2 register - 0: trimming bit applied from OBL (can be read on TRIMR register)"]
    #[inline(always)]
    pub fn boftrimen(&mut self) -> BoftrimenW<'_, Engtrim2Spec> {
        BoftrimenW::new(self, 0)
    }
    #[doc = "Bits 1:3 - SMPS_TRIM: SMPS Output Voltage Trimming By default, this value is not applied, but taken from the engi bytes; if ENGTRIM.BOFTRIMEN=1, the SMPS output voltage can be controlled by this register."]
    #[inline(always)]
    pub fn bof_trim(&mut self) -> BofTrimW<'_, Engtrim2Spec> {
        BofTrimW::new(self, 1)
    }
}
#[doc = "ENGTRIM2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`engtrim2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`engtrim2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Engtrim2Spec;
impl crate::RegisterSpec for Engtrim2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`engtrim2::R`](R) reader structure"]
impl crate::Readable for Engtrim2Spec {}
#[doc = "`write(|w| ..)` method takes [`engtrim2::W`](W) writer structure"]
impl crate::Writable for Engtrim2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENGTRIM2 to value 0"]
impl crate::Resettable for Engtrim2Spec {}
