#[doc = "Register `AFC3_CONFIG` reader"]
pub type R = crate::R<Afc3ConfigSpec>;
#[doc = "Register `AFC3_CONFIG` writer"]
pub type W = crate::W<Afc3ConfigSpec>;
#[doc = "Field `AFC_INIT_MODE` reader - Control the initialization phase of the AFC and clock recovery algorithms:"]
pub type AfcInitModeR = crate::BitReader;
#[doc = "Field `AFC_INIT_MODE` writer - Control the initialization phase of the AFC and clock recovery algorithms:"]
pub type AfcInitModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFC_SIGN_PERM_CHECK` reader - Enable the check of sign permanence of AFC corrected signal."]
pub type AfcSignPermCheckR = crate::BitReader;
#[doc = "Field `AFC_SIGN_PERM_CHECK` writer - Enable the check of sign permanence of AFC corrected signal."]
pub type AfcSignPermCheckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFC_TH_SIGN_PERM` reader - Threshold of chech sign permanence mechanism."]
pub type AfcThSignPermR = crate::FieldReader;
#[doc = "Field `AFC_TH_SIGN_PERM` writer - Threshold of chech sign permanence mechanism."]
pub type AfcThSignPermW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFC_REINIT_OPTION` reader - Select the AFC reinitialization option:"]
pub type AfcReinitOptionR = crate::FieldReader;
#[doc = "Field `AFC_REINIT_OPTION` writer - Select the AFC reinitialization option:"]
pub type AfcReinitOptionW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Control the initialization phase of the AFC and clock recovery algorithms:"]
    #[inline(always)]
    pub fn afc_init_mode(&self) -> AfcInitModeR {
        AfcInitModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable the check of sign permanence of AFC corrected signal."]
    #[inline(always)]
    pub fn afc_sign_perm_check(&self) -> AfcSignPermCheckR {
        AfcSignPermCheckR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - Threshold of chech sign permanence mechanism."]
    #[inline(always)]
    pub fn afc_th_sign_perm(&self) -> AfcThSignPermR {
        AfcThSignPermR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - Select the AFC reinitialization option:"]
    #[inline(always)]
    pub fn afc_reinit_option(&self) -> AfcReinitOptionR {
        AfcReinitOptionR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Control the initialization phase of the AFC and clock recovery algorithms:"]
    #[inline(always)]
    pub fn afc_init_mode(&mut self) -> AfcInitModeW<'_, Afc3ConfigSpec> {
        AfcInitModeW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable the check of sign permanence of AFC corrected signal."]
    #[inline(always)]
    pub fn afc_sign_perm_check(&mut self) -> AfcSignPermCheckW<'_, Afc3ConfigSpec> {
        AfcSignPermCheckW::new(self, 1)
    }
    #[doc = "Bits 2:5 - Threshold of chech sign permanence mechanism."]
    #[inline(always)]
    pub fn afc_th_sign_perm(&mut self) -> AfcThSignPermW<'_, Afc3ConfigSpec> {
        AfcThSignPermW::new(self, 2)
    }
    #[doc = "Bits 6:7 - Select the AFC reinitialization option:"]
    #[inline(always)]
    pub fn afc_reinit_option(&mut self) -> AfcReinitOptionW<'_, Afc3ConfigSpec> {
        AfcReinitOptionW::new(self, 6)
    }
}
#[doc = "AFC3_CONFIG register\n\nYou can [`read`](crate::Reg::read) this register and get [`afc3_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afc3_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Afc3ConfigSpec;
impl crate::RegisterSpec for Afc3ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afc3_config::R`](R) reader structure"]
impl crate::Readable for Afc3ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`afc3_config::W`](W) writer structure"]
impl crate::Writable for Afc3ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AFC3_CONFIG to value 0xe8"]
impl crate::Resettable for Afc3ConfigSpec {
    const RESET_VALUE: u32 = 0xe8;
}
