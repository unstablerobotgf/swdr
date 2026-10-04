#[doc = "Register `KRMR` reader"]
pub type R = crate::R<KrmrSpec>;
#[doc = "Register `KRMR` writer"]
pub type W = crate::W<KrmrSpec>;
#[doc = "Field `KRM_EN` reader - KRM_EN: Variable rate multiplier Enable Reset source only for this field: PORESETn 0: KRM is disabled (default) 1: KRM is enabled."]
pub type KrmEnR = crate::BitReader;
#[doc = "Field `KRM_EN` writer - KRM_EN: Variable rate multiplier Enable Reset source only for this field: PORESETn 0: KRM is disabled (default) 1: KRM is enabled."]
pub type KrmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KRM` reader - KRM\\[4:0\\] :SMPS clock dividing Ratio (CLK_SPMS_KRM frequency= CLK_ROOT frequency (depending on RCC_CFGR.HSESEL) divided by KRM when KRMEN=1) Reset source only for this field: PORESETn - 0x00 to 0x08: SMPS clock frequency equals CLK_ROOT/8 (8.00 MHz / 6.00 MHz) - 0x09: SMPS clock frequency equals CLK_ROOT/9 (7.11 MHz / 5.33 MHz) - 0x0A: SMPS clock frequency equals CLK_ROOT/10 (6.40 MHz / 4.80 MHz) - 0x0B: SMPS clock frequency equals CLK_ROOT/11 (5.82 MHz / 4.36 MHz) - 0x0C: SMPS clock frequency equals CLK_ROOT/12 (5.33 MHz / 4.00 MHz) - 0x0D: SMPS clock frequency equals CLK_ROOT/13 (4.92 MHz / 3.69 MHz) - 0x0E: SMPS clock frequency equals CLK_ROOT/14 (4.57 MHz / 3.43 MHz) - 0x0F: SMPS clock frequency equals CLK_ROOT/15 (4.27 MHz / 3.20 MHz) - 0x10: SMPS clock frequency equals CLK_ROOT/16 (4.00 MHz / 3.00 MHz) - 0x1x: Reserved Note: SMPS clock frequency must be selected in a range \\[4-8\\] MHz (depending on RCC_KRMR.KRM and RCC_CFGR.HSESEL)."]
pub type KrmR = crate::FieldReader;
#[doc = "Field `KRM` writer - KRM\\[4:0\\] :SMPS clock dividing Ratio (CLK_SPMS_KRM frequency= CLK_ROOT frequency (depending on RCC_CFGR.HSESEL) divided by KRM when KRMEN=1) Reset source only for this field: PORESETn - 0x00 to 0x08: SMPS clock frequency equals CLK_ROOT/8 (8.00 MHz / 6.00 MHz) - 0x09: SMPS clock frequency equals CLK_ROOT/9 (7.11 MHz / 5.33 MHz) - 0x0A: SMPS clock frequency equals CLK_ROOT/10 (6.40 MHz / 4.80 MHz) - 0x0B: SMPS clock frequency equals CLK_ROOT/11 (5.82 MHz / 4.36 MHz) - 0x0C: SMPS clock frequency equals CLK_ROOT/12 (5.33 MHz / 4.00 MHz) - 0x0D: SMPS clock frequency equals CLK_ROOT/13 (4.92 MHz / 3.69 MHz) - 0x0E: SMPS clock frequency equals CLK_ROOT/14 (4.57 MHz / 3.43 MHz) - 0x0F: SMPS clock frequency equals CLK_ROOT/15 (4.27 MHz / 3.20 MHz) - 0x10: SMPS clock frequency equals CLK_ROOT/16 (4.00 MHz / 3.00 MHz) - 0x1x: Reserved Note: SMPS clock frequency must be selected in a range \\[4-8\\] MHz (depending on RCC_KRMR.KRM and RCC_CFGR.HSESEL)."]
pub type KrmW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - KRM_EN: Variable rate multiplier Enable Reset source only for this field: PORESETn 0: KRM is disabled (default) 1: KRM is enabled."]
    #[inline(always)]
    pub fn krm_en(&self) -> KrmEnR {
        KrmEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - KRM\\[4:0\\] :SMPS clock dividing Ratio (CLK_SPMS_KRM frequency= CLK_ROOT frequency (depending on RCC_CFGR.HSESEL) divided by KRM when KRMEN=1) Reset source only for this field: PORESETn - 0x00 to 0x08: SMPS clock frequency equals CLK_ROOT/8 (8.00 MHz / 6.00 MHz) - 0x09: SMPS clock frequency equals CLK_ROOT/9 (7.11 MHz / 5.33 MHz) - 0x0A: SMPS clock frequency equals CLK_ROOT/10 (6.40 MHz / 4.80 MHz) - 0x0B: SMPS clock frequency equals CLK_ROOT/11 (5.82 MHz / 4.36 MHz) - 0x0C: SMPS clock frequency equals CLK_ROOT/12 (5.33 MHz / 4.00 MHz) - 0x0D: SMPS clock frequency equals CLK_ROOT/13 (4.92 MHz / 3.69 MHz) - 0x0E: SMPS clock frequency equals CLK_ROOT/14 (4.57 MHz / 3.43 MHz) - 0x0F: SMPS clock frequency equals CLK_ROOT/15 (4.27 MHz / 3.20 MHz) - 0x10: SMPS clock frequency equals CLK_ROOT/16 (4.00 MHz / 3.00 MHz) - 0x1x: Reserved Note: SMPS clock frequency must be selected in a range \\[4-8\\] MHz (depending on RCC_KRMR.KRM and RCC_CFGR.HSESEL)."]
    #[inline(always)]
    pub fn krm(&self) -> KrmR {
        KrmR::new(((self.bits >> 1) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - KRM_EN: Variable rate multiplier Enable Reset source only for this field: PORESETn 0: KRM is disabled (default) 1: KRM is enabled."]
    #[inline(always)]
    pub fn krm_en(&mut self) -> KrmEnW<'_, KrmrSpec> {
        KrmEnW::new(self, 0)
    }
    #[doc = "Bits 1:5 - KRM\\[4:0\\] :SMPS clock dividing Ratio (CLK_SPMS_KRM frequency= CLK_ROOT frequency (depending on RCC_CFGR.HSESEL) divided by KRM when KRMEN=1) Reset source only for this field: PORESETn - 0x00 to 0x08: SMPS clock frequency equals CLK_ROOT/8 (8.00 MHz / 6.00 MHz) - 0x09: SMPS clock frequency equals CLK_ROOT/9 (7.11 MHz / 5.33 MHz) - 0x0A: SMPS clock frequency equals CLK_ROOT/10 (6.40 MHz / 4.80 MHz) - 0x0B: SMPS clock frequency equals CLK_ROOT/11 (5.82 MHz / 4.36 MHz) - 0x0C: SMPS clock frequency equals CLK_ROOT/12 (5.33 MHz / 4.00 MHz) - 0x0D: SMPS clock frequency equals CLK_ROOT/13 (4.92 MHz / 3.69 MHz) - 0x0E: SMPS clock frequency equals CLK_ROOT/14 (4.57 MHz / 3.43 MHz) - 0x0F: SMPS clock frequency equals CLK_ROOT/15 (4.27 MHz / 3.20 MHz) - 0x10: SMPS clock frequency equals CLK_ROOT/16 (4.00 MHz / 3.00 MHz) - 0x1x: Reserved Note: SMPS clock frequency must be selected in a range \\[4-8\\] MHz (depending on RCC_KRMR.KRM and RCC_CFGR.HSESEL)."]
    #[inline(always)]
    pub fn krm(&mut self) -> KrmW<'_, KrmrSpec> {
        KrmW::new(self, 1)
    }
}
#[doc = "KRMR register\n\nYou can [`read`](crate::Reg::read) this register and get [`krmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`krmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KrmrSpec;
impl crate::RegisterSpec for KrmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`krmr::R`](R) reader structure"]
impl crate::Readable for KrmrSpec {}
#[doc = "`write(|w| ..)` method takes [`krmr::W`](W) writer structure"]
impl crate::Writable for KrmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KRMR to value 0"]
impl crate::Resettable for KrmrSpec {}
