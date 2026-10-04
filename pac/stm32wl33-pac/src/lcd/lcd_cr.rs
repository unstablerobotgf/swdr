#[doc = "Register `LCD_CR` reader"]
pub type R = crate::R<LcdCrSpec>;
#[doc = "Register `LCD_CR` writer"]
pub type W = crate::W<LcdCrSpec>;
#[doc = "Field `LCDEN` reader - LCD controller enable"]
pub type LcdenR = crate::BitReader;
#[doc = "Field `LCDEN` writer - LCD controller enable"]
pub type LcdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSEL` reader - Voltage source selection"]
pub type VselR = crate::BitReader;
#[doc = "Field `VSEL` writer - Voltage source selection"]
pub type VselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY` reader - Duty selection"]
pub type DutyR = crate::FieldReader;
#[doc = "Field `DUTY` writer - Duty selection"]
pub type DutyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIAS` reader - Bias selector"]
pub type BiasR = crate::FieldReader;
#[doc = "Field `BIAS` writer - Bias selector"]
pub type BiasW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BUFEN` reader - Voltage output buffer enable"]
pub type BufenR = crate::BitReader;
#[doc = "Field `BUFEN` writer - Voltage output buffer enable"]
pub type BufenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LCD controller enable"]
    #[inline(always)]
    pub fn lcden(&self) -> LcdenR {
        LcdenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Voltage source selection"]
    #[inline(always)]
    pub fn vsel(&self) -> VselR {
        VselR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Duty selection"]
    #[inline(always)]
    pub fn duty(&self) -> DutyR {
        DutyR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:6 - Bias selector"]
    #[inline(always)]
    pub fn bias(&self) -> BiasR {
        BiasR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 8 - Voltage output buffer enable"]
    #[inline(always)]
    pub fn bufen(&self) -> BufenR {
        BufenR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD controller enable"]
    #[inline(always)]
    pub fn lcden(&mut self) -> LcdenW<'_, LcdCrSpec> {
        LcdenW::new(self, 0)
    }
    #[doc = "Bit 1 - Voltage source selection"]
    #[inline(always)]
    pub fn vsel(&mut self) -> VselW<'_, LcdCrSpec> {
        VselW::new(self, 1)
    }
    #[doc = "Bits 2:4 - Duty selection"]
    #[inline(always)]
    pub fn duty(&mut self) -> DutyW<'_, LcdCrSpec> {
        DutyW::new(self, 2)
    }
    #[doc = "Bits 5:6 - Bias selector"]
    #[inline(always)]
    pub fn bias(&mut self) -> BiasW<'_, LcdCrSpec> {
        BiasW::new(self, 5)
    }
    #[doc = "Bit 8 - Voltage output buffer enable"]
    #[inline(always)]
    pub fn bufen(&mut self) -> BufenW<'_, LcdCrSpec> {
        BufenW::new(self, 8)
    }
}
#[doc = "LCD_CR register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdCrSpec;
impl crate::RegisterSpec for LcdCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_cr::R`](R) reader structure"]
impl crate::Readable for LcdCrSpec {}
#[doc = "`write(|w| ..)` method takes [`lcd_cr::W`](W) writer structure"]
impl crate::Writable for LcdCrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD_CR to value 0"]
impl crate::Resettable for LcdCrSpec {}
