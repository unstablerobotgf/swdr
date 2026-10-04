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
#[doc = "Field `COMIE` reader - COMIE: COM interrupt enable 0: COM interrupt disabled 1: COM interrupt enabled"]
pub type ComieR = crate::BitReader;
#[doc = "Field `COMIE` writer - COMIE: COM interrupt enable 0: COM interrupt disabled 1: COM interrupt enabled"]
pub type ComieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE` reader - TIE: Trigger interrupt enable 0: Trigger interrupt disabled 1: Trigger interrupt enabled"]
pub type TieR = crate::BitReader;
#[doc = "Field `TIE` writer - TIE: Trigger interrupt enable 0: Trigger interrupt disabled 1: Trigger interrupt enabled"]
pub type TieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIE` reader - BIE: Break interrupt enable 0: Break interrupt disabled 1: Break interrupt enabled"]
pub type BieR = crate::BitReader;
#[doc = "Field `BIE` writer - BIE: Break interrupt enable 0: Break interrupt disabled 1: Break interrupt enabled"]
pub type BieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDE` reader - UDE: Update DMA request enable 0: Update DMA request disabled 1: Update DMA request enabled"]
pub type UdeR = crate::BitReader;
#[doc = "Field `UDE` writer - UDE: Update DMA request enable 0: Update DMA request disabled 1: Update DMA request enabled"]
pub type UdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1DE` reader - CC1DE: Capture/Compare 1 DMA request enable 0: CC1 DMA request disabled 1: CC1 DMA request enabled"]
pub type Cc1deR = crate::BitReader;
#[doc = "Field `CC1DE` writer - CC1DE: Capture/Compare 1 DMA request enable 0: CC1 DMA request disabled 1: CC1 DMA request enabled"]
pub type Cc1deW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCUDE` reader - CCUDE: CC-Update DMA request Enable. Not used in Blue51. Not available in IUM 0: CC-Update DMA request disabled. 1: CC-Update DMA request enabled."]
pub type CcudeR = crate::BitReader;
#[doc = "Field `CCUDE` writer - CCUDE: CC-Update DMA request Enable. Not used in Blue51. Not available in IUM 0: CC-Update DMA request disabled. 1: CC-Update DMA request enabled."]
pub type CcudeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDE` reader - TDE: Trigger DMA request enable 0: Trigger DMA request disabled 1: Trigger DMA request enabled"]
pub type TdeR = crate::BitReader;
#[doc = "Field `TDE` writer - TDE: Trigger DMA request enable 0: Trigger DMA request disabled 1: Trigger DMA request enabled"]
pub type TdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BDE` reader - BDE: Break DMA request Enable. Not used in Blue51. Not available in IUM 0: Break DMA request disabled. 1: Break DMA request enabled."]
pub type BdeR = crate::BitReader;
#[doc = "Field `BDE` writer - BDE: Break DMA request Enable. Not used in Blue51. Not available in IUM 0: Break DMA request disabled. 1: Break DMA request enabled."]
pub type BdeW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 5 - COMIE: COM interrupt enable 0: COM interrupt disabled 1: COM interrupt enabled"]
    #[inline(always)]
    pub fn comie(&self) -> ComieR {
        ComieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TIE: Trigger interrupt enable 0: Trigger interrupt disabled 1: Trigger interrupt enabled"]
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BIE: Break interrupt enable 0: Break interrupt disabled 1: Break interrupt enabled"]
    #[inline(always)]
    pub fn bie(&self) -> BieR {
        BieR::new(((self.bits >> 7) & 1) != 0)
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
    #[doc = "Bit 13 - CCUDE: CC-Update DMA request Enable. Not used in Blue51. Not available in IUM 0: CC-Update DMA request disabled. 1: CC-Update DMA request enabled."]
    #[inline(always)]
    pub fn ccude(&self) -> CcudeR {
        CcudeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TDE: Trigger DMA request enable 0: Trigger DMA request disabled 1: Trigger DMA request enabled"]
    #[inline(always)]
    pub fn tde(&self) -> TdeR {
        TdeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - BDE: Break DMA request Enable. Not used in Blue51. Not available in IUM 0: Break DMA request disabled. 1: Break DMA request enabled."]
    #[inline(always)]
    pub fn bde(&self) -> BdeR {
        BdeR::new(((self.bits >> 15) & 1) != 0)
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
    #[doc = "Bit 5 - COMIE: COM interrupt enable 0: COM interrupt disabled 1: COM interrupt enabled"]
    #[inline(always)]
    pub fn comie(&mut self) -> ComieW<'_, DierSpec> {
        ComieW::new(self, 5)
    }
    #[doc = "Bit 6 - TIE: Trigger interrupt enable 0: Trigger interrupt disabled 1: Trigger interrupt enabled"]
    #[inline(always)]
    pub fn tie(&mut self) -> TieW<'_, DierSpec> {
        TieW::new(self, 6)
    }
    #[doc = "Bit 7 - BIE: Break interrupt enable 0: Break interrupt disabled 1: Break interrupt enabled"]
    #[inline(always)]
    pub fn bie(&mut self) -> BieW<'_, DierSpec> {
        BieW::new(self, 7)
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
    #[doc = "Bit 13 - CCUDE: CC-Update DMA request Enable. Not used in Blue51. Not available in IUM 0: CC-Update DMA request disabled. 1: CC-Update DMA request enabled."]
    #[inline(always)]
    pub fn ccude(&mut self) -> CcudeW<'_, DierSpec> {
        CcudeW::new(self, 13)
    }
    #[doc = "Bit 14 - TDE: Trigger DMA request enable 0: Trigger DMA request disabled 1: Trigger DMA request enabled"]
    #[inline(always)]
    pub fn tde(&mut self) -> TdeW<'_, DierSpec> {
        TdeW::new(self, 14)
    }
    #[doc = "Bit 15 - BDE: Break DMA request Enable. Not used in Blue51. Not available in IUM 0: Break DMA request disabled. 1: Break DMA request enabled."]
    #[inline(always)]
    pub fn bde(&mut self) -> BdeW<'_, DierSpec> {
        BdeW::new(self, 15)
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
