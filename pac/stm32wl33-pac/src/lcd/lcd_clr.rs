#[doc = "Register `LCD_CLR` reader"]
pub type R = crate::R<LcdClrSpec>;
#[doc = "Register `LCD_CLR` writer"]
pub type W = crate::W<LcdClrSpec>;
#[doc = "Field `SOFC` writer - Start of frame flag clear"]
pub type SofcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDDC` writer - Update display done clear"]
pub type UddcW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 1 - Start of frame flag clear"]
    #[inline(always)]
    pub fn sofc(&mut self) -> SofcW<'_, LcdClrSpec> {
        SofcW::new(self, 1)
    }
    #[doc = "Bit 3 - Update display done clear"]
    #[inline(always)]
    pub fn uddc(&mut self) -> UddcW<'_, LcdClrSpec> {
        UddcW::new(self, 3)
    }
}
#[doc = "LCD_CLR register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdClrSpec;
impl crate::RegisterSpec for LcdClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_clr::R`](R) reader structure"]
impl crate::Readable for LcdClrSpec {}
#[doc = "`write(|w| ..)` method takes [`lcd_clr::W`](W) writer structure"]
impl crate::Writable for LcdClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD_CLR to value 0"]
impl crate::Resettable for LcdClrSpec {}
