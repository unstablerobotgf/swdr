#[doc = "Register `VCO_CAL_CONFIG` reader"]
pub type R = crate::R<VcoCalConfigSpec>;
#[doc = "Register `VCO_CAL_CONFIG` writer"]
pub type W = crate::W<VcoCalConfigSpec>;
#[doc = "Field `VCO_CALAMP_EXT` reader - VCO magnitude calibration word in thermometric code"]
pub type VcoCalampExtR = crate::FieldReader<u16>;
#[doc = "Field `VCO_CALAMP_EXT` writer - VCO magnitude calibration word in thermometric code"]
pub type VcoCalampExtW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `VCO_CALAMP_EXT_SEL` reader - Select the mode to provide an external VCO amplitude calibration value through VCO_CALAMP_EXT bit field"]
pub type VcoCalampExtSelR = crate::BitReader;
#[doc = "Field `VCO_CALAMP_EXT_SEL` writer - Select the mode to provide an external VCO amplitude calibration value through VCO_CALAMP_EXT bit field"]
pub type VcoCalampExtSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VCO_CALFREQ_EXT` reader - VCO Cbank frequency calibration word."]
pub type VcoCalfreqExtR = crate::FieldReader;
#[doc = "Field `VCO_CALFREQ_EXT` writer - VCO Cbank frequency calibration word."]
pub type VcoCalfreqExtW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `VCO_CALFREQ_EXT_SEL` reader - Select the mode to provide an external VCO frequency calibration value through VCO_CALFREQ_EXT bit field"]
pub type VcoCalfreqExtSelR = crate::BitReader;
#[doc = "Field `VCO_CALFREQ_EXT_SEL` writer - Select the mode to provide an external VCO frequency calibration value through VCO_CALFREQ_EXT bit field"]
pub type VcoCalfreqExtSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VCO_CALIB_REQ` reader - Define if the Radio FSM must launch a VCO calibration request after VCO start-up"]
pub type VcoCalibReqR = crate::BitReader;
#[doc = "Field `VCO_CALIB_REQ` writer - Define if the Radio FSM must launch a VCO calibration request after VCO start-up"]
pub type VcoCalibReqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13 - VCO magnitude calibration word in thermometric code"]
    #[inline(always)]
    pub fn vco_calamp_ext(&self) -> VcoCalampExtR {
        VcoCalampExtR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 15 - Select the mode to provide an external VCO amplitude calibration value through VCO_CALAMP_EXT bit field"]
    #[inline(always)]
    pub fn vco_calamp_ext_sel(&self) -> VcoCalampExtSelR {
        VcoCalampExtSelR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - VCO Cbank frequency calibration word."]
    #[inline(always)]
    pub fn vco_calfreq_ext(&self) -> VcoCalfreqExtR {
        VcoCalfreqExtR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - Select the mode to provide an external VCO frequency calibration value through VCO_CALFREQ_EXT bit field"]
    #[inline(always)]
    pub fn vco_calfreq_ext_sel(&self) -> VcoCalfreqExtSelR {
        VcoCalfreqExtSelR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 31 - Define if the Radio FSM must launch a VCO calibration request after VCO start-up"]
    #[inline(always)]
    pub fn vco_calib_req(&self) -> VcoCalibReqR {
        VcoCalibReqR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - VCO magnitude calibration word in thermometric code"]
    #[inline(always)]
    pub fn vco_calamp_ext(&mut self) -> VcoCalampExtW<'_, VcoCalConfigSpec> {
        VcoCalampExtW::new(self, 0)
    }
    #[doc = "Bit 15 - Select the mode to provide an external VCO amplitude calibration value through VCO_CALAMP_EXT bit field"]
    #[inline(always)]
    pub fn vco_calamp_ext_sel(&mut self) -> VcoCalampExtSelW<'_, VcoCalConfigSpec> {
        VcoCalampExtSelW::new(self, 15)
    }
    #[doc = "Bits 16:22 - VCO Cbank frequency calibration word."]
    #[inline(always)]
    pub fn vco_calfreq_ext(&mut self) -> VcoCalfreqExtW<'_, VcoCalConfigSpec> {
        VcoCalfreqExtW::new(self, 16)
    }
    #[doc = "Bit 23 - Select the mode to provide an external VCO frequency calibration value through VCO_CALFREQ_EXT bit field"]
    #[inline(always)]
    pub fn vco_calfreq_ext_sel(&mut self) -> VcoCalfreqExtSelW<'_, VcoCalConfigSpec> {
        VcoCalfreqExtSelW::new(self, 23)
    }
    #[doc = "Bit 31 - Define if the Radio FSM must launch a VCO calibration request after VCO start-up"]
    #[inline(always)]
    pub fn vco_calib_req(&mut self) -> VcoCalibReqW<'_, VcoCalConfigSpec> {
        VcoCalibReqW::new(self, 31)
    }
}
#[doc = "VCO_CAL_CONFIG register\n\nYou can [`read`](crate::Reg::read) this register and get [`vco_cal_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vco_cal_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VcoCalConfigSpec;
impl crate::RegisterSpec for VcoCalConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vco_cal_config::R`](R) reader structure"]
impl crate::Readable for VcoCalConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`vco_cal_config::W`](W) writer structure"]
impl crate::Writable for VcoCalConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VCO_CAL_CONFIG to value 0x0040_0088"]
impl crate::Resettable for VcoCalConfigSpec {
    const RESET_VALUE: u32 = 0x0040_0088;
}
