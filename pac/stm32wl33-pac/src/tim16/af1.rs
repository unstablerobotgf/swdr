#[doc = "Register `AF1` reader"]
pub type R = crate::R<Af1Spec>;
#[doc = "Register `AF1` writer"]
pub type W = crate::W<Af1Spec>;
#[doc = "Field `BKINE` reader - BKINE: BRK BKIN enable. This bit enables the BKIN alternate function input for the timer's BRK input. BKIN input is ORed with the other enabled BRK sources. 0: BKIN input disabled. 1: BKIN input enabled. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)"]
pub type BkineR = crate::BitReader;
#[doc = "Field `BKINE` writer - BKINE: BRK BKIN enable. This bit enables the BKIN alternate function input for the timer's BRK input. BKIN input is ORed with the other enabled BRK sources. 0: BKIN input disabled. 1: BKIN input enabled. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)"]
pub type BkineW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP1E` reader - BKCMP1E: BRK COMP1 enable. This bit enables the COMP1 for the timer's BRK input. COMP1 output is ORed with the other enabled BRK sources. 0: COMP1 input disabled. 1: COMP1 input enabled. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)"]
pub type Bkcmp1eR = crate::BitReader;
#[doc = "Field `BKCMP1E` writer - BKCMP1E: BRK COMP1 enable. This bit enables the COMP1 for the timer's BRK input. COMP1 output is ORed with the other enabled BRK sources. 0: COMP1 input disabled. 1: COMP1 input enabled. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)"]
pub type Bkcmp1eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKINP` reader - BKINP: BRK BKIN input polarity. This bit selects the BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. 0: BKIN input is active low. 1: BKIN input is active high. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)"]
pub type BkinpR = crate::BitReader;
#[doc = "Field `BKINP` writer - BKINP: BRK BKIN input polarity. This bit selects the BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. 0: BKIN input is active low. 1: BKIN input is active high. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)"]
pub type BkinpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP1P` reader - BKCMP1P: BRK COMP1 input polarity. This bit selects the COMP1 input sensitivity. It must be programmed together with the BKP polarity bit. 0: COMP1 input is active low. 1: COMP1 input is active high. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)"]
pub type Bkcmp1pR = crate::BitReader;
#[doc = "Field `BKCMP1P` writer - BKCMP1P: BRK COMP1 input polarity. This bit selects the COMP1 input sensitivity. It must be programmed together with the BKP polarity bit. 0: COMP1 input is active low. 1: COMP1 input is active high. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)"]
pub type Bkcmp1pW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - BKINE: BRK BKIN enable. This bit enables the BKIN alternate function input for the timer's BRK input. BKIN input is ORed with the other enabled BRK sources. 0: BKIN input disabled. 1: BKIN input enabled. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)"]
    #[inline(always)]
    pub fn bkine(&self) -> BkineR {
        BkineR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BKCMP1E: BRK COMP1 enable. This bit enables the COMP1 for the timer's BRK input. COMP1 output is ORed with the other enabled BRK sources. 0: COMP1 input disabled. 1: COMP1 input enabled. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)"]
    #[inline(always)]
    pub fn bkcmp1e(&self) -> Bkcmp1eR {
        Bkcmp1eR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 9 - BKINP: BRK BKIN input polarity. This bit selects the BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. 0: BKIN input is active low. 1: BKIN input is active high. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)"]
    #[inline(always)]
    pub fn bkinp(&self) -> BkinpR {
        BkinpR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BKCMP1P: BRK COMP1 input polarity. This bit selects the COMP1 input sensitivity. It must be programmed together with the BKP polarity bit. 0: COMP1 input is active low. 1: COMP1 input is active high. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)"]
    #[inline(always)]
    pub fn bkcmp1p(&self) -> Bkcmp1pR {
        Bkcmp1pR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BKINE: BRK BKIN enable. This bit enables the BKIN alternate function input for the timer's BRK input. BKIN input is ORed with the other enabled BRK sources. 0: BKIN input disabled. 1: BKIN input enabled. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)"]
    #[inline(always)]
    pub fn bkine(&mut self) -> BkineW<'_, Af1Spec> {
        BkineW::new(self, 0)
    }
    #[doc = "Bit 1 - BKCMP1E: BRK COMP1 enable. This bit enables the COMP1 for the timer's BRK input. COMP1 output is ORed with the other enabled BRK sources. 0: COMP1 input disabled. 1: COMP1 input enabled. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)"]
    #[inline(always)]
    pub fn bkcmp1e(&mut self) -> Bkcmp1eW<'_, Af1Spec> {
        Bkcmp1eW::new(self, 1)
    }
    #[doc = "Bit 9 - BKINP: BRK BKIN input polarity. This bit selects the BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. 0: BKIN input is active low. 1: BKIN input is active high. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)"]
    #[inline(always)]
    pub fn bkinp(&mut self) -> BkinpW<'_, Af1Spec> {
        BkinpW::new(self, 9)
    }
    #[doc = "Bit 10 - BKCMP1P: BRK COMP1 input polarity. This bit selects the COMP1 input sensitivity. It must be programmed together with the BKP polarity bit. 0: COMP1 input is active low. 1: COMP1 input is active high. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)"]
    #[inline(always)]
    pub fn bkcmp1p(&mut self) -> Bkcmp1pW<'_, Af1Spec> {
        Bkcmp1pW::new(self, 10)
    }
}
#[doc = "AF1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`af1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Af1Spec;
impl crate::RegisterSpec for Af1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af1::R`](R) reader structure"]
impl crate::Readable for Af1Spec {}
#[doc = "`write(|w| ..)` method takes [`af1::W`](W) writer structure"]
impl crate::Writable for Af1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AF1 to value 0x01"]
impl crate::Resettable for Af1Spec {
    const RESET_VALUE: u32 = 0x01;
}
