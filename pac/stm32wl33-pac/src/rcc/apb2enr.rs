#[doc = "Register `APB2ENR` reader"]
pub type R = crate::R<Apb2enrSpec>;
#[doc = "Register `APB2ENR` writer"]
pub type W = crate::W<Apb2enrSpec>;
#[doc = "Field `MRSUBGEN` reader - MRSUBG clock enable. Note: when this bit is '1', it must prevent clk_sys different from 16, 32, 64. If the configured clock is lower than 16MHz (1, 2, 4 or 8 MHz) or equal to 24MHz, clk_sys must be 16MHz 0: clock disable 1: clock enable"]
pub type MrsubgenR = crate::BitReader;
#[doc = "Field `MRSUBGEN` writer - MRSUBG clock enable. Note: when this bit is '1', it must prevent clk_sys different from 16, 32, 64. If the configured clock is lower than 16MHz (1, 2, 4 or 8 MHz) or equal to 24MHz, clk_sys must be 16MHz 0: clock disable 1: clock enable"]
pub type MrsubgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPAWUREN` reader - Bubble clock enable Set and enable by software. 0: clock disable 1: clock enable"]
pub type LpawurenR = crate::BitReader;
#[doc = "Field `LPAWUREN` writer - Bubble clock enable Set and enable by software. 0: clock disable 1: clock enable"]
pub type LpawurenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MRSUBG clock enable. Note: when this bit is '1', it must prevent clk_sys different from 16, 32, 64. If the configured clock is lower than 16MHz (1, 2, 4 or 8 MHz) or equal to 24MHz, clk_sys must be 16MHz 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn mrsubgen(&self) -> MrsubgenR {
        MrsubgenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Bubble clock enable Set and enable by software. 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn lpawuren(&self) -> LpawurenR {
        LpawurenR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MRSUBG clock enable. Note: when this bit is '1', it must prevent clk_sys different from 16, 32, 64. If the configured clock is lower than 16MHz (1, 2, 4 or 8 MHz) or equal to 24MHz, clk_sys must be 16MHz 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn mrsubgen(&mut self) -> MrsubgenW<'_, Apb2enrSpec> {
        MrsubgenW::new(self, 0)
    }
    #[doc = "Bit 3 - Bubble clock enable Set and enable by software. 0: clock disable 1: clock enable"]
    #[inline(always)]
    pub fn lpawuren(&mut self) -> LpawurenW<'_, Apb2enrSpec> {
        LpawurenW::new(self, 3)
    }
}
#[doc = "APB2ENR register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2enrSpec;
impl crate::RegisterSpec for Apb2enrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2enr::R`](R) reader structure"]
impl crate::Readable for Apb2enrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb2enr::W`](W) writer structure"]
impl crate::Writable for Apb2enrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB2ENR to value 0"]
impl crate::Resettable for Apb2enrSpec {}
