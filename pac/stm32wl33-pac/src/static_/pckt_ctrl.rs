#[doc = "Register `PCKT_CTRL` reader"]
pub type R = crate::R<PcktCtrlSpec>;
#[doc = "Register `PCKT_CTRL` writer"]
pub type W = crate::W<PcktCtrlSpec>;
#[doc = "Field `PCKT_FORMAT` reader - Packet format"]
pub type PcktFormatR = crate::BitReader;
#[doc = "Field `PCKT_FORMAT` writer - Packet format"]
pub type PcktFormatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYTE_SWAP` reader - Invert MSB-LSB transmission order (bitendianess)"]
pub type ByteSwapR = crate::BitReader;
#[doc = "Field `BYTE_SWAP` writer - Invert MSB-LSB transmission order (bitendianess)"]
pub type ByteSwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOUR_FSK_SYM_SWAP` reader - Invert bit to symbol mapping for 4-(G)FSK"]
pub type FourFskSymSwapR = crate::BitReader;
#[doc = "Field `FOUR_FSK_SYM_SWAP` writer - Invert bit to symbol mapping for 4-(G)FSK"]
pub type FourFskSymSwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_MODE` reader - RX mode"]
pub type RxModeR = crate::FieldReader;
#[doc = "Field `RX_MODE` writer - RX mode"]
pub type RxModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TX_MODE` reader - TX mode"]
pub type TxModeR = crate::FieldReader;
#[doc = "Field `TX_MODE` writer - TX mode"]
pub type TxModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WHIT_BF_FEC` reader - Whitening before FEC feature"]
pub type WhitBfFecR = crate::BitReader;
#[doc = "Field `WHIT_BF_FEC` writer - Whitening before FEC feature"]
pub type WhitBfFecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WHIT_EN` reader - Whitening enable"]
pub type WhitEnR = crate::BitReader;
#[doc = "Field `WHIT_EN` writer - Whitening enable"]
pub type WhitEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WHIT_INIT` reader - Whitening initialization value."]
pub type WhitInitR = crate::FieldReader<u16>;
#[doc = "Field `WHIT_INIT` writer - Whitening initialization value."]
pub type WhitInitW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `CODING_SEL` reader - Coding / decoding selection"]
pub type CodingSelR = crate::FieldReader;
#[doc = "Field `CODING_SEL` writer - Coding / decoding selection"]
pub type CodingSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MANCHESTER_TYPE` reader - Select the Manchester encoding polarity"]
pub type ManchesterTypeR = crate::BitReader;
#[doc = "Field `MANCHESTER_TYPE` writer - Select the Manchester encoding polarity"]
pub type ManchesterTypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_EN_4G` reader - This field is used as Interleaving enable for 802."]
pub type IntEn4gR = crate::BitReader;
#[doc = "Field `INT_EN_4G` writer - This field is used as Interleaving enable for 802."]
pub type IntEn4gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEC_TYPE_4G` reader - FEC type for 802."]
pub type FecType4gR = crate::BitReader;
#[doc = "Field `FEC_TYPE_4G` writer - FEC type for 802."]
pub type FecType4gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCS_TYPE_4G` reader - FCS type value in header field for 802."]
pub type FcsType4gR = crate::BitReader;
#[doc = "Field `FCS_TYPE_4G` writer - FCS type value in header field for 802."]
pub type FcsType4gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOD_INTERP_EN` reader - Enable frequency interpolator (for 2-GFSK and 4-GFSK)"]
pub type ModInterpEnR = crate::BitReader;
#[doc = "Field `MOD_INTERP_EN` writer - Enable frequency interpolator (for 2-GFSK and 4-GFSK)"]
pub type ModInterpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PN_SEL` reader - Select the Pseudo Random Binary Sequence (PRBS) polynomial to apply when the selected transmission mode is PN mode (TX_MODE = '11')"]
pub type PnSelR = crate::BitReader;
#[doc = "Field `PN_SEL` writer - Select the Pseudo Random Binary Sequence (PRBS) polynomial to apply when the selected transmission mode is PN mode (TX_MODE = '11')"]
pub type PnSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_2FSK_SYNC_MODE` reader - Force SYNC word to be formatted as a 2-(G)FSK bit steam instead of 4-(G)FSK"]
pub type Force2fskSyncModeR = crate::BitReader;
#[doc = "Field `FORCE_2FSK_SYNC_MODE` writer - Force SYNC word to be formatted as a 2-(G)FSK bit steam instead of 4-(G)FSK"]
pub type Force2fskSyncModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Packet format"]
    #[inline(always)]
    pub fn pckt_format(&self) -> PcktFormatR {
        PcktFormatR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Invert MSB-LSB transmission order (bitendianess)"]
    #[inline(always)]
    pub fn byte_swap(&self) -> ByteSwapR {
        ByteSwapR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Invert bit to symbol mapping for 4-(G)FSK"]
    #[inline(always)]
    pub fn four_fsk_sym_swap(&self) -> FourFskSymSwapR {
        FourFskSymSwapR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - RX mode"]
    #[inline(always)]
    pub fn rx_mode(&self) -> RxModeR {
        RxModeR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:8 - TX mode"]
    #[inline(always)]
    pub fn tx_mode(&self) -> TxModeR {
        TxModeR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 10 - Whitening before FEC feature"]
    #[inline(always)]
    pub fn whit_bf_fec(&self) -> WhitBfFecR {
        WhitBfFecR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Whitening enable"]
    #[inline(always)]
    pub fn whit_en(&self) -> WhitEnR {
        WhitEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:20 - Whitening initialization value."]
    #[inline(always)]
    pub fn whit_init(&self) -> WhitInitR {
        WhitInitR::new(((self.bits >> 12) & 0x01ff) as u16)
    }
    #[doc = "Bits 21:22 - Coding / decoding selection"]
    #[inline(always)]
    pub fn coding_sel(&self) -> CodingSelR {
        CodingSelR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 24 - Select the Manchester encoding polarity"]
    #[inline(always)]
    pub fn manchester_type(&self) -> ManchesterTypeR {
        ManchesterTypeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - This field is used as Interleaving enable for 802."]
    #[inline(always)]
    pub fn int_en_4g(&self) -> IntEn4gR {
        IntEn4gR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - FEC type for 802."]
    #[inline(always)]
    pub fn fec_type_4g(&self) -> FecType4gR {
        FecType4gR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - FCS type value in header field for 802."]
    #[inline(always)]
    pub fn fcs_type_4g(&self) -> FcsType4gR {
        FcsType4gR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable frequency interpolator (for 2-GFSK and 4-GFSK)"]
    #[inline(always)]
    pub fn mod_interp_en(&self) -> ModInterpEnR {
        ModInterpEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Select the Pseudo Random Binary Sequence (PRBS) polynomial to apply when the selected transmission mode is PN mode (TX_MODE = '11')"]
    #[inline(always)]
    pub fn pn_sel(&self) -> PnSelR {
        PnSelR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Force SYNC word to be formatted as a 2-(G)FSK bit steam instead of 4-(G)FSK"]
    #[inline(always)]
    pub fn force_2fsk_sync_mode(&self) -> Force2fskSyncModeR {
        Force2fskSyncModeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Packet format"]
    #[inline(always)]
    pub fn pckt_format(&mut self) -> PcktFormatW<'_, PcktCtrlSpec> {
        PcktFormatW::new(self, 0)
    }
    #[doc = "Bit 2 - Invert MSB-LSB transmission order (bitendianess)"]
    #[inline(always)]
    pub fn byte_swap(&mut self) -> ByteSwapW<'_, PcktCtrlSpec> {
        ByteSwapW::new(self, 2)
    }
    #[doc = "Bit 3 - Invert bit to symbol mapping for 4-(G)FSK"]
    #[inline(always)]
    pub fn four_fsk_sym_swap(&mut self) -> FourFskSymSwapW<'_, PcktCtrlSpec> {
        FourFskSymSwapW::new(self, 3)
    }
    #[doc = "Bits 4:6 - RX mode"]
    #[inline(always)]
    pub fn rx_mode(&mut self) -> RxModeW<'_, PcktCtrlSpec> {
        RxModeW::new(self, 4)
    }
    #[doc = "Bits 7:8 - TX mode"]
    #[inline(always)]
    pub fn tx_mode(&mut self) -> TxModeW<'_, PcktCtrlSpec> {
        TxModeW::new(self, 7)
    }
    #[doc = "Bit 10 - Whitening before FEC feature"]
    #[inline(always)]
    pub fn whit_bf_fec(&mut self) -> WhitBfFecW<'_, PcktCtrlSpec> {
        WhitBfFecW::new(self, 10)
    }
    #[doc = "Bit 11 - Whitening enable"]
    #[inline(always)]
    pub fn whit_en(&mut self) -> WhitEnW<'_, PcktCtrlSpec> {
        WhitEnW::new(self, 11)
    }
    #[doc = "Bits 12:20 - Whitening initialization value."]
    #[inline(always)]
    pub fn whit_init(&mut self) -> WhitInitW<'_, PcktCtrlSpec> {
        WhitInitW::new(self, 12)
    }
    #[doc = "Bits 21:22 - Coding / decoding selection"]
    #[inline(always)]
    pub fn coding_sel(&mut self) -> CodingSelW<'_, PcktCtrlSpec> {
        CodingSelW::new(self, 21)
    }
    #[doc = "Bit 24 - Select the Manchester encoding polarity"]
    #[inline(always)]
    pub fn manchester_type(&mut self) -> ManchesterTypeW<'_, PcktCtrlSpec> {
        ManchesterTypeW::new(self, 24)
    }
    #[doc = "Bit 25 - This field is used as Interleaving enable for 802."]
    #[inline(always)]
    pub fn int_en_4g(&mut self) -> IntEn4gW<'_, PcktCtrlSpec> {
        IntEn4gW::new(self, 25)
    }
    #[doc = "Bit 26 - FEC type for 802."]
    #[inline(always)]
    pub fn fec_type_4g(&mut self) -> FecType4gW<'_, PcktCtrlSpec> {
        FecType4gW::new(self, 26)
    }
    #[doc = "Bit 27 - FCS type value in header field for 802."]
    #[inline(always)]
    pub fn fcs_type_4g(&mut self) -> FcsType4gW<'_, PcktCtrlSpec> {
        FcsType4gW::new(self, 27)
    }
    #[doc = "Bit 28 - Enable frequency interpolator (for 2-GFSK and 4-GFSK)"]
    #[inline(always)]
    pub fn mod_interp_en(&mut self) -> ModInterpEnW<'_, PcktCtrlSpec> {
        ModInterpEnW::new(self, 28)
    }
    #[doc = "Bit 29 - Select the Pseudo Random Binary Sequence (PRBS) polynomial to apply when the selected transmission mode is PN mode (TX_MODE = '11')"]
    #[inline(always)]
    pub fn pn_sel(&mut self) -> PnSelW<'_, PcktCtrlSpec> {
        PnSelW::new(self, 29)
    }
    #[doc = "Bit 31 - Force SYNC word to be formatted as a 2-(G)FSK bit steam instead of 4-(G)FSK"]
    #[inline(always)]
    pub fn force_2fsk_sync_mode(&mut self) -> Force2fskSyncModeW<'_, PcktCtrlSpec> {
        Force2fskSyncModeW::new(self, 31)
    }
}
#[doc = "PCKT_CTRL register\n\nYou can [`read`](crate::Reg::read) this register and get [`pckt_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pckt_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcktCtrlSpec;
impl crate::RegisterSpec for PcktCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pckt_ctrl::R`](R) reader structure"]
impl crate::Readable for PcktCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pckt_ctrl::W`](W) writer structure"]
impl crate::Writable for PcktCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCKT_CTRL to value 0"]
impl crate::Resettable for PcktCtrlSpec {}
