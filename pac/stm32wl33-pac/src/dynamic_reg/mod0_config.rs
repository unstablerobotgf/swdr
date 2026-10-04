#[doc = "Register `MOD0_CONFIG` reader"]
pub type R = crate::R<Mod0ConfigSpec>;
#[doc = "Register `MOD0_CONFIG` writer"]
pub type W = crate::W<Mod0ConfigSpec>;
#[doc = "Field `DATARATE_M` reader - The mantissa of the specified data rate (default: 38."]
pub type DatarateMR = crate::FieldReader<u16>;
#[doc = "Field `DATARATE_M` writer - The mantissa of the specified data rate (default: 38."]
pub type DatarateMW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DATARATE_E` reader - The exponent of the specified data rate (default: 38."]
pub type DatarateER = crate::FieldReader;
#[doc = "Field `DATARATE_E` writer - The exponent of the specified data rate (default: 38."]
pub type DatarateEW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MOD_TYPE` reader - Select the modulation type"]
pub type ModTypeR = crate::FieldReader;
#[doc = "Field `MOD_TYPE` writer - Select the modulation type"]
pub type ModTypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CONST_MAP` reader - Also known as FOUR_GFSK_CONST_MAP"]
pub type ConstMapR = crate::FieldReader;
#[doc = "Field `CONST_MAP` writer - Also known as FOUR_GFSK_CONST_MAP"]
pub type ConstMapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BT_SEL` reader - Select BT value for GFSK"]
pub type BtSelR = crate::BitReader;
#[doc = "Field `BT_SEL` writer - Select BT value for GFSK"]
pub type BtSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA_CLKON_LOCKONTX` reader - Enable the clock on analog PA in LOCKONTX state"]
pub type PaClkonLockontxR = crate::BitReader;
#[doc = "Field `PA_CLKON_LOCKONTX` writer - Enable the clock on analog PA in LOCKONTX state"]
pub type PaClkonLockontxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - The mantissa of the specified data rate (default: 38."]
    #[inline(always)]
    pub fn datarate_m(&self) -> DatarateMR {
        DatarateMR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - The exponent of the specified data rate (default: 38."]
    #[inline(always)]
    pub fn datarate_e(&self) -> DatarateER {
        DatarateER::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - Select the modulation type"]
    #[inline(always)]
    pub fn mod_type(&self) -> ModTypeR {
        ModTypeR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:25 - Also known as FOUR_GFSK_CONST_MAP"]
    #[inline(always)]
    pub fn const_map(&self) -> ConstMapR {
        ConstMapR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Select BT value for GFSK"]
    #[inline(always)]
    pub fn bt_sel(&self) -> BtSelR {
        BtSelR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable the clock on analog PA in LOCKONTX state"]
    #[inline(always)]
    pub fn pa_clkon_lockontx(&self) -> PaClkonLockontxR {
        PaClkonLockontxR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - The mantissa of the specified data rate (default: 38."]
    #[inline(always)]
    pub fn datarate_m(&mut self) -> DatarateMW<'_, Mod0ConfigSpec> {
        DatarateMW::new(self, 0)
    }
    #[doc = "Bits 16:19 - The exponent of the specified data rate (default: 38."]
    #[inline(always)]
    pub fn datarate_e(&mut self) -> DatarateEW<'_, Mod0ConfigSpec> {
        DatarateEW::new(self, 16)
    }
    #[doc = "Bits 20:22 - Select the modulation type"]
    #[inline(always)]
    pub fn mod_type(&mut self) -> ModTypeW<'_, Mod0ConfigSpec> {
        ModTypeW::new(self, 20)
    }
    #[doc = "Bits 24:25 - Also known as FOUR_GFSK_CONST_MAP"]
    #[inline(always)]
    pub fn const_map(&mut self) -> ConstMapW<'_, Mod0ConfigSpec> {
        ConstMapW::new(self, 24)
    }
    #[doc = "Bit 26 - Select BT value for GFSK"]
    #[inline(always)]
    pub fn bt_sel(&mut self) -> BtSelW<'_, Mod0ConfigSpec> {
        BtSelW::new(self, 26)
    }
    #[doc = "Bit 31 - Enable the clock on analog PA in LOCKONTX state"]
    #[inline(always)]
    pub fn pa_clkon_lockontx(&mut self) -> PaClkonLockontxW<'_, Mod0ConfigSpec> {
        PaClkonLockontxW::new(self, 31)
    }
}
#[doc = "MOD0_CONFIG register\n\nYou can [`read`](crate::Reg::read) this register and get [`mod0_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mod0_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mod0ConfigSpec;
impl crate::RegisterSpec for Mod0ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mod0_config::R`](R) reader structure"]
impl crate::Readable for Mod0ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`mod0_config::W`](W) writer structure"]
impl crate::Writable for Mod0ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MOD0_CONFIG to value 0x0008_3a93"]
impl crate::Resettable for Mod0ConfigSpec {
    const RESET_VALUE: u32 = 0x0008_3a93;
}
