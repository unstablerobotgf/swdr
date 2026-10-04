#[doc = "Register `PRESC` reader"]
pub type R = crate::R<PrescSpec>;
#[doc = "Register `PRESC` writer"]
pub type W = crate::W<PrescSpec>;
#[doc = "Field `PRESCALER` reader - PRESCALER\\[3:0\\]: Clock prescaler The USART input clock can be divided by a prescaler: -0000: input clock not divided -0001: input clock divided by 2 -0010: input clock divided by 4 -0011: input clock divided by 6 -0100: input clock divided by 8 -0101: input clock divided by 10 -0110: input clock divided by 12 -0111: input clock divided by 16 -1000: input clock divided by 32 -1001: input clock divided by 64 -1010: input clock divided by 128 -1011: input clock divided by 256 Remaing combinations: Reserved. Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value will be '1011' i.e. input clock divided by 256"]
pub type PrescalerR = crate::FieldReader;
#[doc = "Field `PRESCALER` writer - PRESCALER\\[3:0\\]: Clock prescaler The USART input clock can be divided by a prescaler: -0000: input clock not divided -0001: input clock divided by 2 -0010: input clock divided by 4 -0011: input clock divided by 6 -0100: input clock divided by 8 -0101: input clock divided by 10 -0110: input clock divided by 12 -0111: input clock divided by 16 -1000: input clock divided by 32 -1001: input clock divided by 64 -1010: input clock divided by 128 -1011: input clock divided by 256 Remaing combinations: Reserved. Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value will be '1011' i.e. input clock divided by 256"]
pub type PrescalerW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - PRESCALER\\[3:0\\]: Clock prescaler The USART input clock can be divided by a prescaler: -0000: input clock not divided -0001: input clock divided by 2 -0010: input clock divided by 4 -0011: input clock divided by 6 -0100: input clock divided by 8 -0101: input clock divided by 10 -0110: input clock divided by 12 -0111: input clock divided by 16 -1000: input clock divided by 32 -1001: input clock divided by 64 -1010: input clock divided by 128 -1011: input clock divided by 256 Remaing combinations: Reserved. Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value will be '1011' i.e. input clock divided by 256"]
    #[inline(always)]
    pub fn prescaler(&self) -> PrescalerR {
        PrescalerR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PRESCALER\\[3:0\\]: Clock prescaler The USART input clock can be divided by a prescaler: -0000: input clock not divided -0001: input clock divided by 2 -0010: input clock divided by 4 -0011: input clock divided by 6 -0100: input clock divided by 8 -0101: input clock divided by 10 -0110: input clock divided by 12 -0111: input clock divided by 16 -1000: input clock divided by 32 -1001: input clock divided by 64 -1010: input clock divided by 128 -1011: input clock divided by 256 Remaing combinations: Reserved. Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value will be '1011' i.e. input clock divided by 256"]
    #[inline(always)]
    pub fn prescaler(&mut self) -> PrescalerW<'_, PrescSpec> {
        PrescalerW::new(self, 0)
    }
}
#[doc = "PRESC register\n\nYou can [`read`](crate::Reg::read) this register and get [`presc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`presc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrescSpec;
impl crate::RegisterSpec for PrescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`presc::R`](R) reader structure"]
impl crate::Readable for PrescSpec {}
#[doc = "`write(|w| ..)` method takes [`presc::W`](W) writer structure"]
impl crate::Writable for PrescSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRESC to value 0"]
impl crate::Resettable for PrescSpec {}
