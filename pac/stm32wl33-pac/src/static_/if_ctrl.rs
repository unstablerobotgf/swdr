#[doc = "Register `IF_CTRL` reader"]
pub type R = crate::R<IfCtrlSpec>;
#[doc = "Register `IF_CTRL` writer"]
pub type W = crate::W<IfCtrlSpec>;
#[doc = "Field `IF_OFFSET_DIG` reader - Intermediate frequency setting for the digital shift-to-baseband circuits (default: 300 kHz)"]
pub type IfOffsetDigR = crate::FieldReader<u16>;
#[doc = "Field `IF_OFFSET_DIG` writer - Intermediate frequency setting for the digital shift-to-baseband circuits (default: 300 kHz)"]
pub type IfOffsetDigW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `IF_OFFSET_ANA` reader - Intermediate frequency setting for the synthesizer configuration (default: 300 kHz)."]
pub type IfOffsetAnaR = crate::FieldReader<u16>;
#[doc = "Field `IF_OFFSET_ANA` writer - Intermediate frequency setting for the synthesizer configuration (default: 300 kHz)."]
pub type IfOffsetAnaW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `IF_MODE` reader - Select the cutoff frequency of the AAF for the analog RFSUBG IP"]
pub type IfModeR = crate::BitReader;
#[doc = "Field `IF_MODE` writer - Select the cutoff frequency of the AAF for the analog RFSUBG IP"]
pub type IfModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:12 - Intermediate frequency setting for the digital shift-to-baseband circuits (default: 300 kHz)"]
    #[inline(always)]
    pub fn if_offset_dig(&self) -> IfOffsetDigR {
        IfOffsetDigR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Intermediate frequency setting for the synthesizer configuration (default: 300 kHz)."]
    #[inline(always)]
    pub fn if_offset_ana(&self) -> IfOffsetAnaR {
        IfOffsetAnaR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
    #[doc = "Bit 31 - Select the cutoff frequency of the AAF for the analog RFSUBG IP"]
    #[inline(always)]
    pub fn if_mode(&self) -> IfModeR {
        IfModeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12 - Intermediate frequency setting for the digital shift-to-baseband circuits (default: 300 kHz)"]
    #[inline(always)]
    pub fn if_offset_dig(&mut self) -> IfOffsetDigW<'_, IfCtrlSpec> {
        IfOffsetDigW::new(self, 0)
    }
    #[doc = "Bits 16:28 - Intermediate frequency setting for the synthesizer configuration (default: 300 kHz)."]
    #[inline(always)]
    pub fn if_offset_ana(&mut self) -> IfOffsetAnaW<'_, IfCtrlSpec> {
        IfOffsetAnaW::new(self, 16)
    }
    #[doc = "Bit 31 - Select the cutoff frequency of the AAF for the analog RFSUBG IP"]
    #[inline(always)]
    pub fn if_mode(&mut self) -> IfModeW<'_, IfCtrlSpec> {
        IfModeW::new(self, 31)
    }
}
#[doc = "IF_CTRL register\n\nYou can [`read`](crate::Reg::read) this register and get [`if_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfCtrlSpec;
impl crate::RegisterSpec for IfCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_ctrl::R`](R) reader structure"]
impl crate::Readable for IfCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`if_ctrl::W`](W) writer structure"]
impl crate::Writable for IfCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IF_CTRL to value 0x04cd_04cd"]
impl crate::Resettable for IfCtrlSpec {
    const RESET_VALUE: u32 = 0x04cd_04cd;
}
