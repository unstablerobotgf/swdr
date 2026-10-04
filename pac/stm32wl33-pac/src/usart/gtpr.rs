#[doc = "Register `GTPR` reader"]
pub type R = crate::R<GtprSpec>;
#[doc = "Register `GTPR` writer"]
pub type W = crate::W<GtprSpec>;
#[doc = "Field `PSC` reader - PSC\\[7:0\\]: Prescaler value In IrDA Low-power and normal IrDA mode: PSC\\[7:0\\] = IrDA Normal and Low-Power Baud Rate Used for programming the prescaler for dividing the USART source clock to achieve the lowpower frequency: The source clock is divided by the value given in the register (8 significant bits): -00000000: Reserved - do not program this value -00000001: divides the source clock by 1 -00000010: divides the source clock by 2 ... In Smartcard mode: PSC\\[4:0\\]: Prescaler value Used for programming the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: -00000: Reserved - do not program this value -00001: divides the source clock by 2 -00010: divides the source clock by 4 -00011: divides the source clock by 6 ... This bit field can only be written when the USART is disabled (UE=0)."]
pub type PscR = crate::FieldReader;
#[doc = "Field `PSC` writer - PSC\\[7:0\\]: Prescaler value In IrDA Low-power and normal IrDA mode: PSC\\[7:0\\] = IrDA Normal and Low-Power Baud Rate Used for programming the prescaler for dividing the USART source clock to achieve the lowpower frequency: The source clock is divided by the value given in the register (8 significant bits): -00000000: Reserved - do not program this value -00000001: divides the source clock by 1 -00000010: divides the source clock by 2 ... In Smartcard mode: PSC\\[4:0\\]: Prescaler value Used for programming the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: -00000: Reserved - do not program this value -00001: divides the source clock by 2 -00010: divides the source clock by 4 -00011: divides the source clock by 6 ... This bit field can only be written when the USART is disabled (UE=0)."]
pub type PscW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GT` reader - GT\\[7:0\\]: Guard time value This bit-field is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bit field can only be written when the USART is disabled (UE=0)."]
pub type GtR = crate::FieldReader;
#[doc = "Field `GT` writer - GT\\[7:0\\]: Guard time value This bit-field is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bit field can only be written when the USART is disabled (UE=0)."]
pub type GtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - PSC\\[7:0\\]: Prescaler value In IrDA Low-power and normal IrDA mode: PSC\\[7:0\\] = IrDA Normal and Low-Power Baud Rate Used for programming the prescaler for dividing the USART source clock to achieve the lowpower frequency: The source clock is divided by the value given in the register (8 significant bits): -00000000: Reserved - do not program this value -00000001: divides the source clock by 1 -00000010: divides the source clock by 2 ... In Smartcard mode: PSC\\[4:0\\]: Prescaler value Used for programming the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: -00000: Reserved - do not program this value -00001: divides the source clock by 2 -00010: divides the source clock by 4 -00011: divides the source clock by 6 ... This bit field can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn psc(&self) -> PscR {
        PscR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GT\\[7:0\\]: Guard time value This bit-field is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bit field can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn gt(&self) -> GtR {
        GtR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PSC\\[7:0\\]: Prescaler value In IrDA Low-power and normal IrDA mode: PSC\\[7:0\\] = IrDA Normal and Low-Power Baud Rate Used for programming the prescaler for dividing the USART source clock to achieve the lowpower frequency: The source clock is divided by the value given in the register (8 significant bits): -00000000: Reserved - do not program this value -00000001: divides the source clock by 1 -00000010: divides the source clock by 2 ... In Smartcard mode: PSC\\[4:0\\]: Prescaler value Used for programming the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: -00000: Reserved - do not program this value -00001: divides the source clock by 2 -00010: divides the source clock by 4 -00011: divides the source clock by 6 ... This bit field can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn psc(&mut self) -> PscW<'_, GtprSpec> {
        PscW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GT\\[7:0\\]: Guard time value This bit-field is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bit field can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn gt(&mut self) -> GtW<'_, GtprSpec> {
        GtW::new(self, 8)
    }
}
#[doc = "GTPR register\n\nYou can [`read`](crate::Reg::read) this register and get [`gtpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtprSpec;
impl crate::RegisterSpec for GtprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtpr::R`](R) reader structure"]
impl crate::Readable for GtprSpec {}
#[doc = "`write(|w| ..)` method takes [`gtpr::W`](W) writer structure"]
impl crate::Writable for GtprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GTPR to value 0"]
impl crate::Resettable for GtprSpec {}
