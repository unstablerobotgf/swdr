#[doc = "Register `PCKT_CONFIG` reader"]
pub type R = crate::R<PcktConfigSpec>;
#[doc = "Register `PCKT_CONFIG` writer"]
pub type W = crate::W<PcktConfigSpec>;
#[doc = "Field `CRC_MODE` reader - CRC type (0, 8, 16, 16 802."]
pub type CrcModeR = crate::FieldReader;
#[doc = "Field `CRC_MODE` writer - CRC type (0, 8, 16, 16 802."]
pub type CrcModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SECONDARY_SYNC_SEL` reader - In TX mode: this bit selects which synchro word is sent on the frame between SYNC and SEC_SYNC"]
pub type SecondarySyncSelR = crate::BitReader;
#[doc = "Field `SECONDARY_SYNC_SEL` writer - In TX mode: this bit selects which synchro word is sent on the frame between SYNC and SEC_SYNC"]
pub type SecondarySyncSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC_LEN` reader - Length of the SYNC (and secondary) SYNC word in 1-bit granularity"]
pub type SyncLenR = crate::FieldReader;
#[doc = "Field `SYNC_LEN` writer - Length of the SYNC (and secondary) SYNC word in 1-bit granularity"]
pub type SyncLenW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SYNC_PRESENT` reader - Indicate if a SYNC word is present on the frame or not (null length)"]
pub type SyncPresentR = crate::BitReader;
#[doc = "Field `SYNC_PRESENT` writer - Indicate if a SYNC word is present on the frame or not (null length)"]
pub type SyncPresentW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN_WIDTH` reader - Indicates if the LENGTH field is defined on 1 byte or 2 bytes"]
pub type LenWidthR = crate::BitReader;
#[doc = "Field `LEN_WIDTH` writer - Indicates if the LENGTH field is defined on 1 byte or 2 bytes"]
pub type LenWidthW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIX_VAR_LEN` reader - Select the length mode"]
pub type FixVarLenR = crate::BitReader;
#[doc = "Field `FIX_VAR_LEN` writer - Select the length mode"]
pub type FixVarLenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREAMBLE_LENGTH` reader - Length of the PREAMBLE in pairs of bits (0 to 2046)"]
pub type PreambleLengthR = crate::FieldReader<u16>;
#[doc = "Field `PREAMBLE_LENGTH` writer - Length of the PREAMBLE in pairs of bits (0 to 2046)"]
pub type PreambleLengthW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PREAMBLE_SEQ` reader - Select the PREAMBLE pattern to be applied"]
pub type PreambleSeqR = crate::FieldReader;
#[doc = "Field `PREAMBLE_SEQ` writer - Select the PREAMBLE pattern to be applied"]
pub type PreambleSeqW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `POSTAMBLE_LENGTH` reader - Length of the POSTAMBLE in pair of bits (0 to 126 bits)"]
pub type PostambleLengthR = crate::FieldReader;
#[doc = "Field `POSTAMBLE_LENGTH` writer - Length of the POSTAMBLE in pair of bits (0 to 126 bits)"]
pub type PostambleLengthW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `POSTAMBLE_SEQ` reader - Packet postamble control: postamble bit sequence selection"]
pub type PostambleSeqR = crate::FieldReader;
#[doc = "Field `POSTAMBLE_SEQ` writer - Packet postamble control: postamble bit sequence selection"]
pub type PostambleSeqW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - CRC type (0, 8, 16, 16 802."]
    #[inline(always)]
    pub fn crc_mode(&self) -> CrcModeR {
        CrcModeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - In TX mode: this bit selects which synchro word is sent on the frame between SYNC and SEC_SYNC"]
    #[inline(always)]
    pub fn secondary_sync_sel(&self) -> SecondarySyncSelR {
        SecondarySyncSelR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:8 - Length of the SYNC (and secondary) SYNC word in 1-bit granularity"]
    #[inline(always)]
    pub fn sync_len(&self) -> SyncLenR {
        SyncLenR::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 9 - Indicate if a SYNC word is present on the frame or not (null length)"]
    #[inline(always)]
    pub fn sync_present(&self) -> SyncPresentR {
        SyncPresentR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Indicates if the LENGTH field is defined on 1 byte or 2 bytes"]
    #[inline(always)]
    pub fn len_width(&self) -> LenWidthR {
        LenWidthR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Select the length mode"]
    #[inline(always)]
    pub fn fix_var_len(&self) -> FixVarLenR {
        FixVarLenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:21 - Length of the PREAMBLE in pairs of bits (0 to 2046)"]
    #[inline(always)]
    pub fn preamble_length(&self) -> PreambleLengthR {
        PreambleLengthR::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bits 22:23 - Select the PREAMBLE pattern to be applied"]
    #[inline(always)]
    pub fn preamble_seq(&self) -> PreambleSeqR {
        PreambleSeqR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:29 - Length of the POSTAMBLE in pair of bits (0 to 126 bits)"]
    #[inline(always)]
    pub fn postamble_length(&self) -> PostambleLengthR {
        PostambleLengthR::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 30:31 - Packet postamble control: postamble bit sequence selection"]
    #[inline(always)]
    pub fn postamble_seq(&self) -> PostambleSeqR {
        PostambleSeqR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - CRC type (0, 8, 16, 16 802."]
    #[inline(always)]
    pub fn crc_mode(&mut self) -> CrcModeW<'_, PcktConfigSpec> {
        CrcModeW::new(self, 0)
    }
    #[doc = "Bit 3 - In TX mode: this bit selects which synchro word is sent on the frame between SYNC and SEC_SYNC"]
    #[inline(always)]
    pub fn secondary_sync_sel(&mut self) -> SecondarySyncSelW<'_, PcktConfigSpec> {
        SecondarySyncSelW::new(self, 3)
    }
    #[doc = "Bits 4:8 - Length of the SYNC (and secondary) SYNC word in 1-bit granularity"]
    #[inline(always)]
    pub fn sync_len(&mut self) -> SyncLenW<'_, PcktConfigSpec> {
        SyncLenW::new(self, 4)
    }
    #[doc = "Bit 9 - Indicate if a SYNC word is present on the frame or not (null length)"]
    #[inline(always)]
    pub fn sync_present(&mut self) -> SyncPresentW<'_, PcktConfigSpec> {
        SyncPresentW::new(self, 9)
    }
    #[doc = "Bit 10 - Indicates if the LENGTH field is defined on 1 byte or 2 bytes"]
    #[inline(always)]
    pub fn len_width(&mut self) -> LenWidthW<'_, PcktConfigSpec> {
        LenWidthW::new(self, 10)
    }
    #[doc = "Bit 11 - Select the length mode"]
    #[inline(always)]
    pub fn fix_var_len(&mut self) -> FixVarLenW<'_, PcktConfigSpec> {
        FixVarLenW::new(self, 11)
    }
    #[doc = "Bits 12:21 - Length of the PREAMBLE in pairs of bits (0 to 2046)"]
    #[inline(always)]
    pub fn preamble_length(&mut self) -> PreambleLengthW<'_, PcktConfigSpec> {
        PreambleLengthW::new(self, 12)
    }
    #[doc = "Bits 22:23 - Select the PREAMBLE pattern to be applied"]
    #[inline(always)]
    pub fn preamble_seq(&mut self) -> PreambleSeqW<'_, PcktConfigSpec> {
        PreambleSeqW::new(self, 22)
    }
    #[doc = "Bits 24:29 - Length of the POSTAMBLE in pair of bits (0 to 126 bits)"]
    #[inline(always)]
    pub fn postamble_length(&mut self) -> PostambleLengthW<'_, PcktConfigSpec> {
        PostambleLengthW::new(self, 24)
    }
    #[doc = "Bits 30:31 - Packet postamble control: postamble bit sequence selection"]
    #[inline(always)]
    pub fn postamble_seq(&mut self) -> PostambleSeqW<'_, PcktConfigSpec> {
        PostambleSeqW::new(self, 30)
    }
}
#[doc = "PCKT_CONFIG register\n\nYou can [`read`](crate::Reg::read) this register and get [`pckt_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pckt_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcktConfigSpec;
impl crate::RegisterSpec for PcktConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pckt_config::R`](R) reader structure"]
impl crate::Readable for PcktConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`pckt_config::W`](W) writer structure"]
impl crate::Writable for PcktConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCKT_CONFIG to value 0x0001_03f1"]
impl crate::Resettable for PcktConfigSpec {
    const RESET_VALUE: u32 = 0x0001_03f1;
}
