#[doc = "Register `OR1` reader"]
pub type R = crate::R<Or1Spec>;
#[doc = "Register `OR1` writer"]
pub type W = crate::W<Or1Spec>;
#[doc = "Field `OR1_0` reader - Not used in Blue51. Not available in IUM"]
pub type Or1_0R = crate::BitReader;
#[doc = "Field `OR1_0` writer - Not used in Blue51. Not available in IUM"]
pub type Or1_0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TI1_RMP` reader - TI1_RMP\\[1:0\\]: Timer 16 input 1 connection This bit is set and cleared by software. 00: TIM16 TI1 is connected to GPIO 01: TIM16 TI1 is connected to LCO 10: TIM16 TI1 is connected to COMP_OUT 11: TIM16 TI1 is connected to MCO"]
pub type Ti1RmpR = crate::FieldReader;
#[doc = "Field `TI1_RMP` writer - TI1_RMP\\[1:0\\]: Timer 16 input 1 connection This bit is set and cleared by software. 00: TIM16 TI1 is connected to GPIO 01: TIM16 TI1 is connected to LCO 10: TIM16 TI1 is connected to COMP_OUT 11: TIM16 TI1 is connected to MCO"]
pub type Ti1RmpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Not used in Blue51. Not available in IUM"]
    #[inline(always)]
    pub fn or1_0(&self) -> Or1_0R {
        Or1_0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - TI1_RMP\\[1:0\\]: Timer 16 input 1 connection This bit is set and cleared by software. 00: TIM16 TI1 is connected to GPIO 01: TIM16 TI1 is connected to LCO 10: TIM16 TI1 is connected to COMP_OUT 11: TIM16 TI1 is connected to MCO"]
    #[inline(always)]
    pub fn ti1_rmp(&self) -> Ti1RmpR {
        Ti1RmpR::new(((self.bits >> 1) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Not used in Blue51. Not available in IUM"]
    #[inline(always)]
    pub fn or1_0(&mut self) -> Or1_0W<'_, Or1Spec> {
        Or1_0W::new(self, 0)
    }
    #[doc = "Bits 1:2 - TI1_RMP\\[1:0\\]: Timer 16 input 1 connection This bit is set and cleared by software. 00: TIM16 TI1 is connected to GPIO 01: TIM16 TI1 is connected to LCO 10: TIM16 TI1 is connected to COMP_OUT 11: TIM16 TI1 is connected to MCO"]
    #[inline(always)]
    pub fn ti1_rmp(&mut self) -> Ti1RmpW<'_, Or1Spec> {
        Ti1RmpW::new(self, 1)
    }
}
#[doc = "OR1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`or1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Or1Spec;
impl crate::RegisterSpec for Or1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or1::R`](R) reader structure"]
impl crate::Readable for Or1Spec {}
#[doc = "`write(|w| ..)` method takes [`or1::W`](W) writer structure"]
impl crate::Writable for Or1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OR1 to value 0"]
impl crate::Resettable for Or1Spec {}
