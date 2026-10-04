#[doc = "Register `DIER` reader"]
pub type R = crate::R<DierSpec>;
#[doc = "Register `DIER` writer"]
pub type W = crate::W<DierSpec>;
#[doc = "Field `UIE` reader - UIE: Update interrupt enable 0: Update interrupt disabled 1: Update interrupt enabled"]
pub type UieR = crate::BitReader;
#[doc = "Field `UIE` writer - UIE: Update interrupt enable 0: Update interrupt disabled 1: Update interrupt enabled"]
pub type UieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1IE` reader - CC1IE: Capture/Compare 1 interrupt enable 0: CC1 interrupt disabled. 1: CC1 interrupt enabled"]
pub type Cc1ieR = crate::BitReader;
#[doc = "Field `CC1IE` writer - CC1IE: Capture/Compare 1 interrupt enable 0: CC1 interrupt disabled. 1: CC1 interrupt enabled"]
pub type Cc1ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2IE` reader - CC2IE: Capture/Compare 2 interrupt enable 0: CC2 interrupt disabled 1: CC2 interrupt enabled"]
pub type Cc2ieR = crate::BitReader;
#[doc = "Field `CC2IE` writer - CC2IE: Capture/Compare 2 interrupt enable 0: CC2 interrupt disabled 1: CC2 interrupt enabled"]
pub type Cc2ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3IE` reader - CC3IE: Capture/Compare 3 interrupt enable 0: CC3 interrupt disabled 1: CC3 interrupt enabled"]
pub type Cc3ieR = crate::BitReader;
#[doc = "Field `CC3IE` writer - CC3IE: Capture/Compare 3 interrupt enable 0: CC3 interrupt disabled 1: CC3 interrupt enabled"]
pub type Cc3ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4IE` reader - CC4IE: Capture/Compare 4 interrupt enable 0: CC4 interrupt disabled 1: CC4 interrupt enabled"]
pub type Cc4ieR = crate::BitReader;
#[doc = "Field `CC4IE` writer - CC4IE: Capture/Compare 4 interrupt enable 0: CC4 interrupt disabled 1: CC4 interrupt enabled"]
pub type Cc4ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE` reader - TIE: Trigger interrupt enable 0: Trigger interrupt disabled 1: Trigger interrupt enabled"]
pub type TieR = crate::BitReader;
#[doc = "Field `TIE` writer - TIE: Trigger interrupt enable 0: Trigger interrupt disabled 1: Trigger interrupt enabled"]
pub type TieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDE` reader - UDE: Update DMA request enable 0: Update DMA request disabled 1: Update DMA request enabled"]
pub type UdeR = crate::BitReader;
#[doc = "Field `UDE` writer - UDE: Update DMA request enable 0: Update DMA request disabled 1: Update DMA request enabled"]
pub type UdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1DE` reader - CC1DE: Capture/Compare 1 DMA request enable 0: CC1 DMA request disabled 1: CC1 DMA request enabled"]
pub type Cc1deR = crate::BitReader;
#[doc = "Field `CC1DE` writer - CC1DE: Capture/Compare 1 DMA request enable 0: CC1 DMA request disabled 1: CC1 DMA request enabled"]
pub type Cc1deW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2DE` reader - CC2DE: Capture/Compare 2 DMA request enable 0: CC2 DMA request disabled 1: CC2 DMA request enabled"]
pub type Cc2deR = crate::BitReader;
#[doc = "Field `CC2DE` writer - CC2DE: Capture/Compare 2 DMA request enable 0: CC2 DMA request disabled 1: CC2 DMA request enabled"]
pub type Cc2deW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3DE` reader - CC3DE: Capture/Compare 3 DMA request enable 0: CC3 DMA request disabled 1: CC3 DMA request enabled"]
pub type Cc3deR = crate::BitReader;
#[doc = "Field `CC3DE` writer - CC3DE: Capture/Compare 3 DMA request enable 0: CC3 DMA request disabled 1: CC3 DMA request enabled"]
pub type Cc3deW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4DE` reader - CC4DE: Capture/Compare 4 DMA request enable 0: CC4 DMA request disabled 1: CC4 DMA request enabled"]
pub type Cc4deR = crate::BitReader;
#[doc = "Field `CC4DE` writer - CC4DE: Capture/Compare 4 DMA request enable 0: CC4 DMA request disabled 1: CC4 DMA request enabled"]
pub type Cc4deW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDE` reader - TDE: Trigger DMA request enable 0: Trigger DMA request disabled 1: Trigger DMA request enabled"]
pub type TdeR = crate::BitReader;
#[doc = "Field `TDE` writer - TDE: Trigger DMA request enable 0: Trigger DMA request disabled 1: Trigger DMA request enabled"]
pub type TdeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - UIE: Update interrupt enable 0: Update interrupt disabled 1: Update interrupt enabled"]
    #[inline(always)]
    pub fn uie(&self) -> UieR {
        UieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CC1IE: Capture/Compare 1 interrupt enable 0: CC1 interrupt disabled. 1: CC1 interrupt enabled"]
    #[inline(always)]
    pub fn cc1ie(&self) -> Cc1ieR {
        Cc1ieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CC2IE: Capture/Compare 2 interrupt enable 0: CC2 interrupt disabled 1: CC2 interrupt enabled"]
    #[inline(always)]
    pub fn cc2ie(&self) -> Cc2ieR {
        Cc2ieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CC3IE: Capture/Compare 3 interrupt enable 0: CC3 interrupt disabled 1: CC3 interrupt enabled"]
    #[inline(always)]
    pub fn cc3ie(&self) -> Cc3ieR {
        Cc3ieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CC4IE: Capture/Compare 4 interrupt enable 0: CC4 interrupt disabled 1: CC4 interrupt enabled"]
    #[inline(always)]
    pub fn cc4ie(&self) -> Cc4ieR {
        Cc4ieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - TIE: Trigger interrupt enable 0: Trigger interrupt disabled 1: Trigger interrupt enabled"]
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - UDE: Update DMA request enable 0: Update DMA request disabled 1: Update DMA request enabled"]
    #[inline(always)]
    pub fn ude(&self) -> UdeR {
        UdeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CC1DE: Capture/Compare 1 DMA request enable 0: CC1 DMA request disabled 1: CC1 DMA request enabled"]
    #[inline(always)]
    pub fn cc1de(&self) -> Cc1deR {
        Cc1deR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CC2DE: Capture/Compare 2 DMA request enable 0: CC2 DMA request disabled 1: CC2 DMA request enabled"]
    #[inline(always)]
    pub fn cc2de(&self) -> Cc2deR {
        Cc2deR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CC3DE: Capture/Compare 3 DMA request enable 0: CC3 DMA request disabled 1: CC3 DMA request enabled"]
    #[inline(always)]
    pub fn cc3de(&self) -> Cc3deR {
        Cc3deR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CC4DE: Capture/Compare 4 DMA request enable 0: CC4 DMA request disabled 1: CC4 DMA request enabled"]
    #[inline(always)]
    pub fn cc4de(&self) -> Cc4deR {
        Cc4deR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - TDE: Trigger DMA request enable 0: Trigger DMA request disabled 1: Trigger DMA request enabled"]
    #[inline(always)]
    pub fn tde(&self) -> TdeR {
        TdeR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UIE: Update interrupt enable 0: Update interrupt disabled 1: Update interrupt enabled"]
    #[inline(always)]
    pub fn uie(&mut self) -> UieW<'_, DierSpec> {
        UieW::new(self, 0)
    }
    #[doc = "Bit 1 - CC1IE: Capture/Compare 1 interrupt enable 0: CC1 interrupt disabled. 1: CC1 interrupt enabled"]
    #[inline(always)]
    pub fn cc1ie(&mut self) -> Cc1ieW<'_, DierSpec> {
        Cc1ieW::new(self, 1)
    }
    #[doc = "Bit 2 - CC2IE: Capture/Compare 2 interrupt enable 0: CC2 interrupt disabled 1: CC2 interrupt enabled"]
    #[inline(always)]
    pub fn cc2ie(&mut self) -> Cc2ieW<'_, DierSpec> {
        Cc2ieW::new(self, 2)
    }
    #[doc = "Bit 3 - CC3IE: Capture/Compare 3 interrupt enable 0: CC3 interrupt disabled 1: CC3 interrupt enabled"]
    #[inline(always)]
    pub fn cc3ie(&mut self) -> Cc3ieW<'_, DierSpec> {
        Cc3ieW::new(self, 3)
    }
    #[doc = "Bit 4 - CC4IE: Capture/Compare 4 interrupt enable 0: CC4 interrupt disabled 1: CC4 interrupt enabled"]
    #[inline(always)]
    pub fn cc4ie(&mut self) -> Cc4ieW<'_, DierSpec> {
        Cc4ieW::new(self, 4)
    }
    #[doc = "Bit 6 - TIE: Trigger interrupt enable 0: Trigger interrupt disabled 1: Trigger interrupt enabled"]
    #[inline(always)]
    pub fn tie(&mut self) -> TieW<'_, DierSpec> {
        TieW::new(self, 6)
    }
    #[doc = "Bit 8 - UDE: Update DMA request enable 0: Update DMA request disabled 1: Update DMA request enabled"]
    #[inline(always)]
    pub fn ude(&mut self) -> UdeW<'_, DierSpec> {
        UdeW::new(self, 8)
    }
    #[doc = "Bit 9 - CC1DE: Capture/Compare 1 DMA request enable 0: CC1 DMA request disabled 1: CC1 DMA request enabled"]
    #[inline(always)]
    pub fn cc1de(&mut self) -> Cc1deW<'_, DierSpec> {
        Cc1deW::new(self, 9)
    }
    #[doc = "Bit 10 - CC2DE: Capture/Compare 2 DMA request enable 0: CC2 DMA request disabled 1: CC2 DMA request enabled"]
    #[inline(always)]
    pub fn cc2de(&mut self) -> Cc2deW<'_, DierSpec> {
        Cc2deW::new(self, 10)
    }
    #[doc = "Bit 11 - CC3DE: Capture/Compare 3 DMA request enable 0: CC3 DMA request disabled 1: CC3 DMA request enabled"]
    #[inline(always)]
    pub fn cc3de(&mut self) -> Cc3deW<'_, DierSpec> {
        Cc3deW::new(self, 11)
    }
    #[doc = "Bit 12 - CC4DE: Capture/Compare 4 DMA request enable 0: CC4 DMA request disabled 1: CC4 DMA request enabled"]
    #[inline(always)]
    pub fn cc4de(&mut self) -> Cc4deW<'_, DierSpec> {
        Cc4deW::new(self, 12)
    }
    #[doc = "Bit 14 - TDE: Trigger DMA request enable 0: Trigger DMA request disabled 1: Trigger DMA request enabled"]
    #[inline(always)]
    pub fn tde(&mut self) -> TdeW<'_, DierSpec> {
        TdeW::new(self, 14)
    }
}
#[doc = "DIER register\n\nYou can [`read`](crate::Reg::read) this register and get [`dier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DierSpec;
impl crate::RegisterSpec for DierSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dier::R`](R) reader structure"]
impl crate::Readable for DierSpec {}
#[doc = "`write(|w| ..)` method takes [`dier::W`](W) writer structure"]
impl crate::Writable for DierSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIER to value 0"]
impl crate::Resettable for DierSpec {}
