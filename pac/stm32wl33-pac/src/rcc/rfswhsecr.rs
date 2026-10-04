#[doc = "Register `RFSWHSECR` reader"]
pub type R = crate::R<RfswhsecrSpec>;
#[doc = "Register `RFSWHSECR` writer"]
pub type W = crate::W<RfswhsecrSpec>;
#[doc = "Field `GMC` reader - GMC\\[6:5\\]: High speed external XO current control reference 00: 10 uA 01: 20 uA 1x: 40 uA GMC\\[4:0\\]: High speed external XO current control multiplying factor IcoreHSE= GMC\\[4:0\\] * GMC\\[6:5\\] Example: GMC\\[6:0\\]=0x1111001 -> IcoreHSE=25*40uA / Default 3F: IcoreHSE= 10uA x 31 = 310uA Note: this value is set only by software."]
pub type GmcR = crate::FieldReader;
#[doc = "Field `GMC` writer - GMC\\[6:5\\]: High speed external XO current control reference 00: 10 uA 01: 20 uA 1x: 40 uA GMC\\[4:0\\]: High speed external XO current control multiplying factor IcoreHSE= GMC\\[4:0\\] * GMC\\[6:5\\] Example: GMC\\[6:0\\]=0x1111001 -> IcoreHSE=25*40uA / Default 3F: IcoreHSE= 10uA x 31 = 310uA Note: this value is set only by software."]
pub type GmcW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SWXOTUNEEN` reader - RF-HSE capacitor bank tuning by SW enable Set by software"]
pub type SwxotuneenR = crate::BitReader;
#[doc = "Field `SWXOTUNEEN` writer - RF-HSE capacitor bank tuning by SW enable Set by software"]
pub type SwxotuneenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWXOTUNE` reader - RF-HSE capacitor bank tuning value by SW Set by software"]
pub type SwxotuneR = crate::FieldReader;
#[doc = "Field `SWXOTUNE` writer - RF-HSE capacitor bank tuning value by SW Set by software"]
pub type SwxotuneW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ISTARTUP` reader - RF-HSE Startup current Set by software Default value 2"]
pub type IstartupR = crate::FieldReader;
#[doc = "Field `ISTARTUP` writer - RF-HSE Startup current Set by software Default value 2"]
pub type IstartupW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AMPLTHRESH` reader - RF-HSE Amplitude Control threshold Set by software Default value 0"]
pub type AmplthreshR = crate::FieldReader;
#[doc = "Field `AMPLTHRESH` writer - RF-HSE Amplitude Control threshold Set by software Default value 0"]
pub type AmplthreshW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:6 - GMC\\[6:5\\]: High speed external XO current control reference 00: 10 uA 01: 20 uA 1x: 40 uA GMC\\[4:0\\]: High speed external XO current control multiplying factor IcoreHSE= GMC\\[4:0\\] * GMC\\[6:5\\] Example: GMC\\[6:0\\]=0x1111001 -> IcoreHSE=25*40uA / Default 3F: IcoreHSE= 10uA x 31 = 310uA Note: this value is set only by software."]
    #[inline(always)]
    pub fn gmc(&self) -> GmcR {
        GmcR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - RF-HSE capacitor bank tuning by SW enable Set by software"]
    #[inline(always)]
    pub fn swxotuneen(&self) -> SwxotuneenR {
        SwxotuneenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - RF-HSE capacitor bank tuning value by SW Set by software"]
    #[inline(always)]
    pub fn swxotune(&self) -> SwxotuneR {
        SwxotuneR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - RF-HSE Startup current Set by software Default value 2"]
    #[inline(always)]
    pub fn istartup(&self) -> IstartupR {
        IstartupR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18 - RF-HSE Amplitude Control threshold Set by software Default value 0"]
    #[inline(always)]
    pub fn amplthresh(&self) -> AmplthreshR {
        AmplthreshR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - GMC\\[6:5\\]: High speed external XO current control reference 00: 10 uA 01: 20 uA 1x: 40 uA GMC\\[4:0\\]: High speed external XO current control multiplying factor IcoreHSE= GMC\\[4:0\\] * GMC\\[6:5\\] Example: GMC\\[6:0\\]=0x1111001 -> IcoreHSE=25*40uA / Default 3F: IcoreHSE= 10uA x 31 = 310uA Note: this value is set only by software."]
    #[inline(always)]
    pub fn gmc(&mut self) -> GmcW<'_, RfswhsecrSpec> {
        GmcW::new(self, 0)
    }
    #[doc = "Bit 7 - RF-HSE capacitor bank tuning by SW enable Set by software"]
    #[inline(always)]
    pub fn swxotuneen(&mut self) -> SwxotuneenW<'_, RfswhsecrSpec> {
        SwxotuneenW::new(self, 7)
    }
    #[doc = "Bits 8:13 - RF-HSE capacitor bank tuning value by SW Set by software"]
    #[inline(always)]
    pub fn swxotune(&mut self) -> SwxotuneW<'_, RfswhsecrSpec> {
        SwxotuneW::new(self, 8)
    }
    #[doc = "Bits 14:15 - RF-HSE Startup current Set by software Default value 2"]
    #[inline(always)]
    pub fn istartup(&mut self) -> IstartupW<'_, RfswhsecrSpec> {
        IstartupW::new(self, 14)
    }
    #[doc = "Bits 16:18 - RF-HSE Amplitude Control threshold Set by software Default value 0"]
    #[inline(always)]
    pub fn amplthresh(&mut self) -> AmplthreshW<'_, RfswhsecrSpec> {
        AmplthreshW::new(self, 16)
    }
}
#[doc = "RFSWHSECR register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfswhsecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfswhsecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfswhsecrSpec;
impl crate::RegisterSpec for RfswhsecrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfswhsecr::R`](R) reader structure"]
impl crate::Readable for RfswhsecrSpec {}
#[doc = "`write(|w| ..)` method takes [`rfswhsecr::W`](W) writer structure"]
impl crate::Writable for RfswhsecrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RFSWHSECR to value 0x803f"]
impl crate::Resettable for RfswhsecrSpec {
    const RESET_VALUE: u32 = 0x803f;
}
