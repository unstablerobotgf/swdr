#[doc = "Register `LCD_RAM_COM5` reader"]
pub type R = crate::R<LcdRamCom5Spec>;
#[doc = "Register `LCD_RAM_COM5` writer"]
pub type W = crate::W<LcdRamCom5Spec>;
#[doc = "Field `SEGMENT_DATA` reader - Each bit corresponds to one pixel of the LCD display."]
pub type SegmentDataR = crate::FieldReader<u16>;
#[doc = "Field `SEGMENT_DATA` writer - Each bit corresponds to one pixel of the LCD display."]
pub type SegmentDataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Each bit corresponds to one pixel of the LCD display."]
    #[inline(always)]
    pub fn segment_data(&self) -> SegmentDataR {
        SegmentDataR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Each bit corresponds to one pixel of the LCD display."]
    #[inline(always)]
    pub fn segment_data(&mut self) -> SegmentDataW<'_, LcdRamCom5Spec> {
        SegmentDataW::new(self, 0)
    }
}
#[doc = "LCD_RAM_COMx register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_ram_com5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_ram_com5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdRamCom5Spec;
impl crate::RegisterSpec for LcdRamCom5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_ram_com5::R`](R) reader structure"]
impl crate::Readable for LcdRamCom5Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_ram_com5::W`](W) writer structure"]
impl crate::Writable for LcdRamCom5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD_RAM_COM5 to value 0"]
impl crate::Resettable for LcdRamCom5Spec {}
