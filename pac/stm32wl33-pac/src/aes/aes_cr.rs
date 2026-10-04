#[doc = "Register `AES_CR` reader"]
pub type R = crate::R<AesCrSpec>;
#[doc = "Register `AES_CR` writer"]
pub type W = crate::W<AesCrSpec>;
#[doc = "Field `EN` reader - EN: AES IP enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - EN: AES IP enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATATYPE` reader - DATATYPE\\[1:0\\]: Data type selection"]
pub type DatatypeR = crate::FieldReader;
#[doc = "Field `DATATYPE` writer - DATATYPE\\[1:0\\]: Data type selection"]
pub type DatatypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE` reader - MODE\\[1:0\\]: AES operating mode"]
pub type ModeR = crate::FieldReader;
#[doc = "Field `MODE` writer - MODE\\[1:0\\]: AES operating mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CHMOD_1_0` reader - CHMOD\\[1:0\\]: AES Chaining Mode selection"]
pub type Chmod1_0R = crate::FieldReader;
#[doc = "Field `CHMOD_1_0` writer - CHMOD\\[1:0\\]: AES Chaining Mode selection"]
pub type Chmod1_0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CCFC` reader - CCFC: Computation Complete Flag Clear"]
pub type CcfcR = crate::BitReader;
#[doc = "Field `CCFC` writer - CCFC: Computation Complete Flag Clear"]
pub type CcfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRC` reader - ERRC: Error clear"]
pub type ErrcR = crate::BitReader;
#[doc = "Field `ERRC` writer - ERRC: Error clear"]
pub type ErrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCFIE` reader - CCFIE: CCF Flag Interrupt Enable"]
pub type CcfieR = crate::BitReader;
#[doc = "Field `CCFIE` writer - CCFIE: CCF Flag Interrupt Enable"]
pub type CcfieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - ERRIE: Error Interrupt Enable"]
pub type ErrieR = crate::BitReader;
#[doc = "Field `ERRIE` writer - ERRIE: Error Interrupt Enable"]
pub type ErrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAINEN` reader - DMAINEN: DMA Input Enable"]
pub type DmainenR = crate::BitReader;
#[doc = "Field `DMAINEN` writer - DMAINEN: DMA Input Enable"]
pub type DmainenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAOUTEN` reader - DMAOUTEN: DMA Output Enable"]
pub type DmaoutenR = crate::BitReader;
#[doc = "Field `DMAOUTEN` writer - DMAOUTEN: DMA Output Enable"]
pub type DmaoutenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GCMPH` reader - GCMPH\\[1:0\\]: GCM or CCM Phase selection"]
pub type GcmphR = crate::FieldReader;
#[doc = "Field `GCMPH` writer - GCMPH\\[1:0\\]: GCM or CCM Phase selection"]
pub type GcmphW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CHMOD_2` reader - CHMOD\\[2\\]: Chaining mode selection, bit \\[2\\]"]
pub type Chmod2R = crate::BitReader;
#[doc = "Field `CHMOD_2` writer - CHMOD\\[2\\]: Chaining mode selection, bit \\[2\\]"]
pub type Chmod2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYSIZE` reader - KEYSIZE: Key Size selection."]
pub type KeysizeR = crate::BitReader;
#[doc = "Field `KEYSIZE` writer - KEYSIZE: Key Size selection."]
pub type KeysizeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NPBLB` reader - NPBLB: Number of Padding Bytes in Last Block of payload."]
pub type NpblbR = crate::FieldReader;
#[doc = "Field `NPBLB` writer - NPBLB: Number of Padding Bytes in Last Block of payload."]
pub type NpblbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - EN: AES IP enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - DATATYPE\\[1:0\\]: Data type selection"]
    #[inline(always)]
    pub fn datatype(&self) -> DatatypeR {
        DatatypeR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - MODE\\[1:0\\]: AES operating mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - CHMOD\\[1:0\\]: AES Chaining Mode selection"]
    #[inline(always)]
    pub fn chmod_1_0(&self) -> Chmod1_0R {
        Chmod1_0R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - CCFC: Computation Complete Flag Clear"]
    #[inline(always)]
    pub fn ccfc(&self) -> CcfcR {
        CcfcR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ERRC: Error clear"]
    #[inline(always)]
    pub fn errc(&self) -> ErrcR {
        ErrcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CCFIE: CCF Flag Interrupt Enable"]
    #[inline(always)]
    pub fn ccfie(&self) -> CcfieR {
        CcfieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ERRIE: Error Interrupt Enable"]
    #[inline(always)]
    pub fn errie(&self) -> ErrieR {
        ErrieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMAINEN: DMA Input Enable"]
    #[inline(always)]
    pub fn dmainen(&self) -> DmainenR {
        DmainenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DMAOUTEN: DMA Output Enable"]
    #[inline(always)]
    pub fn dmaouten(&self) -> DmaoutenR {
        DmaoutenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - GCMPH\\[1:0\\]: GCM or CCM Phase selection"]
    #[inline(always)]
    pub fn gcmph(&self) -> GcmphR {
        GcmphR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 16 - CHMOD\\[2\\]: Chaining mode selection, bit \\[2\\]"]
    #[inline(always)]
    pub fn chmod_2(&self) -> Chmod2R {
        Chmod2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - KEYSIZE: Key Size selection."]
    #[inline(always)]
    pub fn keysize(&self) -> KeysizeR {
        KeysizeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:23 - NPBLB: Number of Padding Bytes in Last Block of payload."]
    #[inline(always)]
    pub fn npblb(&self) -> NpblbR {
        NpblbR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - EN: AES IP enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, AesCrSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - DATATYPE\\[1:0\\]: Data type selection"]
    #[inline(always)]
    pub fn datatype(&mut self) -> DatatypeW<'_, AesCrSpec> {
        DatatypeW::new(self, 1)
    }
    #[doc = "Bits 3:4 - MODE\\[1:0\\]: AES operating mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, AesCrSpec> {
        ModeW::new(self, 3)
    }
    #[doc = "Bits 5:6 - CHMOD\\[1:0\\]: AES Chaining Mode selection"]
    #[inline(always)]
    pub fn chmod_1_0(&mut self) -> Chmod1_0W<'_, AesCrSpec> {
        Chmod1_0W::new(self, 5)
    }
    #[doc = "Bit 7 - CCFC: Computation Complete Flag Clear"]
    #[inline(always)]
    pub fn ccfc(&mut self) -> CcfcW<'_, AesCrSpec> {
        CcfcW::new(self, 7)
    }
    #[doc = "Bit 8 - ERRC: Error clear"]
    #[inline(always)]
    pub fn errc(&mut self) -> ErrcW<'_, AesCrSpec> {
        ErrcW::new(self, 8)
    }
    #[doc = "Bit 9 - CCFIE: CCF Flag Interrupt Enable"]
    #[inline(always)]
    pub fn ccfie(&mut self) -> CcfieW<'_, AesCrSpec> {
        CcfieW::new(self, 9)
    }
    #[doc = "Bit 10 - ERRIE: Error Interrupt Enable"]
    #[inline(always)]
    pub fn errie(&mut self) -> ErrieW<'_, AesCrSpec> {
        ErrieW::new(self, 10)
    }
    #[doc = "Bit 11 - DMAINEN: DMA Input Enable"]
    #[inline(always)]
    pub fn dmainen(&mut self) -> DmainenW<'_, AesCrSpec> {
        DmainenW::new(self, 11)
    }
    #[doc = "Bit 12 - DMAOUTEN: DMA Output Enable"]
    #[inline(always)]
    pub fn dmaouten(&mut self) -> DmaoutenW<'_, AesCrSpec> {
        DmaoutenW::new(self, 12)
    }
    #[doc = "Bits 13:14 - GCMPH\\[1:0\\]: GCM or CCM Phase selection"]
    #[inline(always)]
    pub fn gcmph(&mut self) -> GcmphW<'_, AesCrSpec> {
        GcmphW::new(self, 13)
    }
    #[doc = "Bit 16 - CHMOD\\[2\\]: Chaining mode selection, bit \\[2\\]"]
    #[inline(always)]
    pub fn chmod_2(&mut self) -> Chmod2W<'_, AesCrSpec> {
        Chmod2W::new(self, 16)
    }
    #[doc = "Bit 18 - KEYSIZE: Key Size selection."]
    #[inline(always)]
    pub fn keysize(&mut self) -> KeysizeW<'_, AesCrSpec> {
        KeysizeW::new(self, 18)
    }
    #[doc = "Bits 20:23 - NPBLB: Number of Padding Bytes in Last Block of payload."]
    #[inline(always)]
    pub fn npblb(&mut self) -> NpblbW<'_, AesCrSpec> {
        NpblbW::new(self, 20)
    }
}
#[doc = "AES_CR register\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesCrSpec;
impl crate::RegisterSpec for AesCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_cr::R`](R) reader structure"]
impl crate::Readable for AesCrSpec {}
#[doc = "`write(|w| ..)` method takes [`aes_cr::W`](W) writer structure"]
impl crate::Writable for AesCrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AES_CR to value 0"]
impl crate::Resettable for AesCrSpec {}
