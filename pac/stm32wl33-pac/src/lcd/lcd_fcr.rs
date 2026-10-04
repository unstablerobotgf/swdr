#[doc = "Register `LCD_FCR` reader"]
pub type R = crate::R<LcdFcrSpec>;
#[doc = "Register `LCD_FCR` writer"]
pub type W = crate::W<LcdFcrSpec>;
#[doc = "Field `HD` reader - High drive enable"]
pub type HdR = crate::BitReader;
#[doc = "Field `HD` writer - High drive enable"]
pub type HdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFIE` reader - Start of frame interrupt enable"]
pub type SofieR = crate::BitReader;
#[doc = "Field `SOFIE` writer - Start of frame interrupt enable"]
pub type SofieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDDIE` reader - Update display done interrupt enable"]
pub type UddieR = crate::BitReader;
#[doc = "Field `UDDIE` writer - Update display done interrupt enable"]
pub type UddieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PON` reader - Pulse ON duration"]
pub type PonR = crate::FieldReader;
#[doc = "Field `PON` writer - Pulse ON duration"]
pub type PonW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DEAD` reader - Dead time duration"]
pub type DeadR = crate::FieldReader;
#[doc = "Field `DEAD` writer - Dead time duration"]
pub type DeadW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CC` reader - Contrast control"]
pub type CcR = crate::FieldReader;
#[doc = "Field `CC` writer - Contrast control"]
pub type CcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Blink frequency selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Blinkf {
    #[doc = "0: fLCD/8"]
    B0x0 = 0,
}
impl From<Blinkf> for u8 {
    #[inline(always)]
    fn from(variant: Blinkf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Blinkf {
    type Ux = u8;
}
impl crate::IsEnum for Blinkf {}
#[doc = "Field `BLINKF` reader - Blink frequency selection"]
pub type BlinkfR = crate::FieldReader<Blinkf>;
impl BlinkfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Blinkf> {
        match self.bits {
            0 => Some(Blinkf::B0x0),
            _ => None,
        }
    }
    #[doc = "fLCD/8"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Blinkf::B0x0
    }
}
#[doc = "Field `BLINKF` writer - Blink frequency selection"]
pub type BlinkfW<'a, REG> = crate::FieldWriter<'a, REG, 3, Blinkf>;
impl<'a, REG> BlinkfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "fLCD/8"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Blinkf::B0x0)
    }
}
#[doc = "Field `BLINK` reader - Blink mode selection"]
pub type BlinkR = crate::FieldReader;
#[doc = "Field `BLINK` writer - Blink mode selection"]
pub type BlinkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DIV` reader - DIV clock divider"]
pub type DivR = crate::FieldReader;
#[doc = "Field `DIV` writer - DIV clock divider"]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PS` reader - PS 16-bit prescaler"]
pub type PsR = crate::FieldReader;
#[doc = "Field `PS` writer - PS 16-bit prescaler"]
pub type PsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - High drive enable"]
    #[inline(always)]
    pub fn hd(&self) -> HdR {
        HdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start of frame interrupt enable"]
    #[inline(always)]
    pub fn sofie(&self) -> SofieR {
        SofieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Update display done interrupt enable"]
    #[inline(always)]
    pub fn uddie(&self) -> UddieR {
        UddieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Pulse ON duration"]
    #[inline(always)]
    pub fn pon(&self) -> PonR {
        PonR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:9 - Dead time duration"]
    #[inline(always)]
    pub fn dead(&self) -> DeadR {
        DeadR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:12 - Contrast control"]
    #[inline(always)]
    pub fn cc(&self) -> CcR {
        CcR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Blink frequency selection"]
    #[inline(always)]
    pub fn blinkf(&self) -> BlinkfR {
        BlinkfR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:17 - Blink mode selection"]
    #[inline(always)]
    pub fn blink(&self) -> BlinkR {
        BlinkR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:21 - DIV clock divider"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:25 - PS 16-bit prescaler"]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new(((self.bits >> 22) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - High drive enable"]
    #[inline(always)]
    pub fn hd(&mut self) -> HdW<'_, LcdFcrSpec> {
        HdW::new(self, 0)
    }
    #[doc = "Bit 1 - Start of frame interrupt enable"]
    #[inline(always)]
    pub fn sofie(&mut self) -> SofieW<'_, LcdFcrSpec> {
        SofieW::new(self, 1)
    }
    #[doc = "Bit 3 - Update display done interrupt enable"]
    #[inline(always)]
    pub fn uddie(&mut self) -> UddieW<'_, LcdFcrSpec> {
        UddieW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Pulse ON duration"]
    #[inline(always)]
    pub fn pon(&mut self) -> PonW<'_, LcdFcrSpec> {
        PonW::new(self, 4)
    }
    #[doc = "Bits 7:9 - Dead time duration"]
    #[inline(always)]
    pub fn dead(&mut self) -> DeadW<'_, LcdFcrSpec> {
        DeadW::new(self, 7)
    }
    #[doc = "Bits 10:12 - Contrast control"]
    #[inline(always)]
    pub fn cc(&mut self) -> CcW<'_, LcdFcrSpec> {
        CcW::new(self, 10)
    }
    #[doc = "Bits 13:15 - Blink frequency selection"]
    #[inline(always)]
    pub fn blinkf(&mut self) -> BlinkfW<'_, LcdFcrSpec> {
        BlinkfW::new(self, 13)
    }
    #[doc = "Bits 16:17 - Blink mode selection"]
    #[inline(always)]
    pub fn blink(&mut self) -> BlinkW<'_, LcdFcrSpec> {
        BlinkW::new(self, 16)
    }
    #[doc = "Bits 18:21 - DIV clock divider"]
    #[inline(always)]
    pub fn div(&mut self) -> DivW<'_, LcdFcrSpec> {
        DivW::new(self, 18)
    }
    #[doc = "Bits 22:25 - PS 16-bit prescaler"]
    #[inline(always)]
    pub fn ps(&mut self) -> PsW<'_, LcdFcrSpec> {
        PsW::new(self, 22)
    }
}
#[doc = "LCD_FCR register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_fcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_fcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdFcrSpec;
impl crate::RegisterSpec for LcdFcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_fcr::R`](R) reader structure"]
impl crate::Readable for LcdFcrSpec {}
#[doc = "`write(|w| ..)` method takes [`lcd_fcr::W`](W) writer structure"]
impl crate::Writable for LcdFcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD_FCR to value 0"]
impl crate::Resettable for LcdFcrSpec {}
