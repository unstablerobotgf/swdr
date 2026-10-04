#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `EN` reader - EN: DAC channel enable This bit is set and cleared by software to enable/disable DAC channel. 0: DAC channel disabled 1: DAC channel enabled"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - EN: DAC channel enable This bit is set and cleared by software to enable/disable DAC channel. 0: DAC channel disabled 1: DAC channel enabled"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BON` reader - BON: DAC channel output buffer enable. This bit is set and cleared by software to enable/disable DAC channel output buffer. 0: DAC channel output buffer disabled 1: DAC channel output buffer enabled"]
pub type BonR = crate::BitReader;
#[doc = "Field `BON` writer - BON: DAC channel output buffer enable. This bit is set and cleared by software to enable/disable DAC channel output buffer. 0: DAC channel output buffer disabled 1: DAC channel output buffer enabled"]
pub type BonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEN` reader - TEN: DAC channel trigger enable This bit is set and cleared by software to enable/disable DAC channel trigger. 0: DAC channel trigger disabled and data written into the DAC_DHR register are transferred one APB0 clock cycle later to the DAC_DOR register 1: DAC channel trigger enabled and data from the DAC_DHR register are transferred three APB0 clock cycles later to the DAC_DOR register Note: When software trigger is selected, the transfer from the DAC_DHR register to the DAC_DOR register takes only one APB0 clock cycle."]
pub type TenR = crate::BitReader;
#[doc = "Field `TEN` writer - TEN: DAC channel trigger enable This bit is set and cleared by software to enable/disable DAC channel trigger. 0: DAC channel trigger disabled and data written into the DAC_DHR register are transferred one APB0 clock cycle later to the DAC_DOR register 1: DAC channel trigger enabled and data from the DAC_DHR register are transferred three APB0 clock cycles later to the DAC_DOR register Note: When software trigger is selected, the transfer from the DAC_DHR register to the DAC_DOR register takes only one APB0 clock cycle."]
pub type TenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSEL` reader - TSEL\\[2:0\\]: DAC channel trigger selection These bits select the external event used to trigger DAC channel. 000: Timer 16 TRGO event 001: PA8 pin event from SYSCFG 010 to 011: Reserved 111: Software trigger Only used if bit TEN = 1 (DAC channel trigger enabled)."]
pub type TselR = crate::FieldReader;
#[doc = "Field `TSEL` writer - TSEL\\[2:0\\]: DAC channel trigger selection These bits select the external event used to trigger DAC channel. 000: Timer 16 TRGO event 001: PA8 pin event from SYSCFG 010 to 011: Reserved 111: Software trigger Only used if bit TEN = 1 (DAC channel trigger enabled)."]
pub type TselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WAVE` reader - WAVE\\[1:0\\]: DAC channel noise/triangle wave generation enable These bits are set and cleared by software. 00: wave generation disabled 01: Noise wave generation enabled 1x: Triangle wave generation enabled Note: Only used if bit TEN = 1 (DAC channel trigger enabled)."]
pub type WaveR = crate::FieldReader;
#[doc = "Field `WAVE` writer - WAVE\\[1:0\\]: DAC channel noise/triangle wave generation enable These bits are set and cleared by software. 00: wave generation disabled 01: Noise wave generation enabled 1x: Triangle wave generation enabled Note: Only used if bit TEN = 1 (DAC channel trigger enabled)."]
pub type WaveW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MAMP` reader - MAMP\\[3:0\\]: DAC channel mask amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. 0000: Unmask bit0 of LFSR triangle amplitude equal to 1 0001: Unmask bits\\[1:0\\] of LFSR triangle amplitude equal to 3 0010: Unmask bits\\[2:0\\] of LFSR triangle amplitude equal to 7 0011: Unmask bits\\[3:0\\] of LFSR triangle amplitude equal to 15 0100: Unmask bits\\[4:0\\] of LFSR triangle amplitude equal to 31 greater than or equal to 0101: Unmask bits\\[5:0\\] of LFSR triangle amplitude equal to 63"]
pub type MampR = crate::FieldReader;
#[doc = "Field `MAMP` writer - MAMP\\[3:0\\]: DAC channel mask amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. 0000: Unmask bit0 of LFSR triangle amplitude equal to 1 0001: Unmask bits\\[1:0\\] of LFSR triangle amplitude equal to 3 0010: Unmask bits\\[2:0\\] of LFSR triangle amplitude equal to 7 0011: Unmask bits\\[3:0\\] of LFSR triangle amplitude equal to 15 0100: Unmask bits\\[4:0\\] of LFSR triangle amplitude equal to 31 greater than or equal to 0101: Unmask bits\\[5:0\\] of LFSR triangle amplitude equal to 63"]
pub type MampW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DMAEN` reader - DMAEN: DAC channel DMA enable This bit is set and cleared by software. 0: DAC channel DMA mode disabled 1: DAC channel DMA mode enabled"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMAEN: DAC channel DMA enable This bit is set and cleared by software. 0: DAC channel DMA mode disabled 1: DAC channel DMA mode enabled"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAUDRIE` reader - DMAUDRIE: DAC channel DMA Underrun Interrupt enable This bit is set and cleared by software. 0: DAC channel DMA Underrun Interrupt disabled 1: DAC channel DMA Underrun Interrupt enabled"]
pub type DmaudrieR = crate::BitReader;
#[doc = "Field `DMAUDRIE` writer - DMAUDRIE: DAC channel DMA Underrun Interrupt enable This bit is set and cleared by software. 0: DAC channel DMA Underrun Interrupt disabled 1: DAC channel DMA Underrun Interrupt enabled"]
pub type DmaudrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPEN` reader - CMPEN: DAC channel output to COMP INMINUS enable. This bit is set and cleared by software. 0: DAC channel output to COMP INMINUS disabled 1: DAC channel output to COMP INMINUS enabled"]
pub type CmpenR = crate::BitReader;
#[doc = "Field `CMPEN` writer - CMPEN: DAC channel output to COMP INMINUS enable. This bit is set and cleared by software. 0: DAC channel output to COMP INMINUS disabled 1: DAC channel output to COMP INMINUS enabled"]
pub type CmpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VCMEN` reader - VCMEN: DAC channel output to VCM BUFFER enable. This bit is set and cleared by software. 0: DAC channel output to VCM BUFFER disabled 1: DAC channel output to VCM BUFFER enabled"]
pub type VcmenR = crate::BitReader;
#[doc = "Field `VCMEN` writer - VCMEN: DAC channel output to VCM BUFFER enable. This bit is set and cleared by software. 0: DAC channel output to VCM BUFFER disabled 1: DAC channel output to VCM BUFFER enabled"]
pub type VcmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VCMON` reader - VCMON: VCMBUFF power-up. This bit is set and cleared by software. 0: VCM BUFFER OFF 1: VCM BUFFER ON"]
pub type VcmonR = crate::BitReader;
#[doc = "Field `VCMON` writer - VCMON: VCMBUFF power-up. This bit is set and cleared by software. 0: VCM BUFFER OFF 1: VCM BUFFER ON"]
pub type VcmonW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EN: DAC channel enable This bit is set and cleared by software to enable/disable DAC channel. 0: DAC channel disabled 1: DAC channel enabled"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BON: DAC channel output buffer enable. This bit is set and cleared by software to enable/disable DAC channel output buffer. 0: DAC channel output buffer disabled 1: DAC channel output buffer enabled"]
    #[inline(always)]
    pub fn bon(&self) -> BonR {
        BonR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TEN: DAC channel trigger enable This bit is set and cleared by software to enable/disable DAC channel trigger. 0: DAC channel trigger disabled and data written into the DAC_DHR register are transferred one APB0 clock cycle later to the DAC_DOR register 1: DAC channel trigger enabled and data from the DAC_DHR register are transferred three APB0 clock cycles later to the DAC_DOR register Note: When software trigger is selected, the transfer from the DAC_DHR register to the DAC_DOR register takes only one APB0 clock cycle."]
    #[inline(always)]
    pub fn ten(&self) -> TenR {
        TenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - TSEL\\[2:0\\]: DAC channel trigger selection These bits select the external event used to trigger DAC channel. 000: Timer 16 TRGO event 001: PA8 pin event from SYSCFG 010 to 011: Reserved 111: Software trigger Only used if bit TEN = 1 (DAC channel trigger enabled)."]
    #[inline(always)]
    pub fn tsel(&self) -> TselR {
        TselR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - WAVE\\[1:0\\]: DAC channel noise/triangle wave generation enable These bits are set and cleared by software. 00: wave generation disabled 01: Noise wave generation enabled 1x: Triangle wave generation enabled Note: Only used if bit TEN = 1 (DAC channel trigger enabled)."]
    #[inline(always)]
    pub fn wave(&self) -> WaveR {
        WaveR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - MAMP\\[3:0\\]: DAC channel mask amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. 0000: Unmask bit0 of LFSR triangle amplitude equal to 1 0001: Unmask bits\\[1:0\\] of LFSR triangle amplitude equal to 3 0010: Unmask bits\\[2:0\\] of LFSR triangle amplitude equal to 7 0011: Unmask bits\\[3:0\\] of LFSR triangle amplitude equal to 15 0100: Unmask bits\\[4:0\\] of LFSR triangle amplitude equal to 31 greater than or equal to 0101: Unmask bits\\[5:0\\] of LFSR triangle amplitude equal to 63"]
    #[inline(always)]
    pub fn mamp(&self) -> MampR {
        MampR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - DMAEN: DAC channel DMA enable This bit is set and cleared by software. 0: DAC channel DMA mode disabled 1: DAC channel DMA mode enabled"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DMAUDRIE: DAC channel DMA Underrun Interrupt enable This bit is set and cleared by software. 0: DAC channel DMA Underrun Interrupt disabled 1: DAC channel DMA Underrun Interrupt enabled"]
    #[inline(always)]
    pub fn dmaudrie(&self) -> DmaudrieR {
        DmaudrieR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CMPEN: DAC channel output to COMP INMINUS enable. This bit is set and cleared by software. 0: DAC channel output to COMP INMINUS disabled 1: DAC channel output to COMP INMINUS enabled"]
    #[inline(always)]
    pub fn cmpen(&self) -> CmpenR {
        CmpenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - VCMEN: DAC channel output to VCM BUFFER enable. This bit is set and cleared by software. 0: DAC channel output to VCM BUFFER disabled 1: DAC channel output to VCM BUFFER enabled"]
    #[inline(always)]
    pub fn vcmen(&self) -> VcmenR {
        VcmenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - VCMON: VCMBUFF power-up. This bit is set and cleared by software. 0: VCM BUFFER OFF 1: VCM BUFFER ON"]
    #[inline(always)]
    pub fn vcmon(&self) -> VcmonR {
        VcmonR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EN: DAC channel enable This bit is set and cleared by software to enable/disable DAC channel. 0: DAC channel disabled 1: DAC channel enabled"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, CrSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - BON: DAC channel output buffer enable. This bit is set and cleared by software to enable/disable DAC channel output buffer. 0: DAC channel output buffer disabled 1: DAC channel output buffer enabled"]
    #[inline(always)]
    pub fn bon(&mut self) -> BonW<'_, CrSpec> {
        BonW::new(self, 1)
    }
    #[doc = "Bit 2 - TEN: DAC channel trigger enable This bit is set and cleared by software to enable/disable DAC channel trigger. 0: DAC channel trigger disabled and data written into the DAC_DHR register are transferred one APB0 clock cycle later to the DAC_DOR register 1: DAC channel trigger enabled and data from the DAC_DHR register are transferred three APB0 clock cycles later to the DAC_DOR register Note: When software trigger is selected, the transfer from the DAC_DHR register to the DAC_DOR register takes only one APB0 clock cycle."]
    #[inline(always)]
    pub fn ten(&mut self) -> TenW<'_, CrSpec> {
        TenW::new(self, 2)
    }
    #[doc = "Bits 3:5 - TSEL\\[2:0\\]: DAC channel trigger selection These bits select the external event used to trigger DAC channel. 000: Timer 16 TRGO event 001: PA8 pin event from SYSCFG 010 to 011: Reserved 111: Software trigger Only used if bit TEN = 1 (DAC channel trigger enabled)."]
    #[inline(always)]
    pub fn tsel(&mut self) -> TselW<'_, CrSpec> {
        TselW::new(self, 3)
    }
    #[doc = "Bits 6:7 - WAVE\\[1:0\\]: DAC channel noise/triangle wave generation enable These bits are set and cleared by software. 00: wave generation disabled 01: Noise wave generation enabled 1x: Triangle wave generation enabled Note: Only used if bit TEN = 1 (DAC channel trigger enabled)."]
    #[inline(always)]
    pub fn wave(&mut self) -> WaveW<'_, CrSpec> {
        WaveW::new(self, 6)
    }
    #[doc = "Bits 8:11 - MAMP\\[3:0\\]: DAC channel mask amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. 0000: Unmask bit0 of LFSR triangle amplitude equal to 1 0001: Unmask bits\\[1:0\\] of LFSR triangle amplitude equal to 3 0010: Unmask bits\\[2:0\\] of LFSR triangle amplitude equal to 7 0011: Unmask bits\\[3:0\\] of LFSR triangle amplitude equal to 15 0100: Unmask bits\\[4:0\\] of LFSR triangle amplitude equal to 31 greater than or equal to 0101: Unmask bits\\[5:0\\] of LFSR triangle amplitude equal to 63"]
    #[inline(always)]
    pub fn mamp(&mut self) -> MampW<'_, CrSpec> {
        MampW::new(self, 8)
    }
    #[doc = "Bit 12 - DMAEN: DAC channel DMA enable This bit is set and cleared by software. 0: DAC channel DMA mode disabled 1: DAC channel DMA mode enabled"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<'_, CrSpec> {
        DmaenW::new(self, 12)
    }
    #[doc = "Bit 13 - DMAUDRIE: DAC channel DMA Underrun Interrupt enable This bit is set and cleared by software. 0: DAC channel DMA Underrun Interrupt disabled 1: DAC channel DMA Underrun Interrupt enabled"]
    #[inline(always)]
    pub fn dmaudrie(&mut self) -> DmaudrieW<'_, CrSpec> {
        DmaudrieW::new(self, 13)
    }
    #[doc = "Bit 14 - CMPEN: DAC channel output to COMP INMINUS enable. This bit is set and cleared by software. 0: DAC channel output to COMP INMINUS disabled 1: DAC channel output to COMP INMINUS enabled"]
    #[inline(always)]
    pub fn cmpen(&mut self) -> CmpenW<'_, CrSpec> {
        CmpenW::new(self, 14)
    }
    #[doc = "Bit 15 - VCMEN: DAC channel output to VCM BUFFER enable. This bit is set and cleared by software. 0: DAC channel output to VCM BUFFER disabled 1: DAC channel output to VCM BUFFER enabled"]
    #[inline(always)]
    pub fn vcmen(&mut self) -> VcmenW<'_, CrSpec> {
        VcmenW::new(self, 15)
    }
    #[doc = "Bit 16 - VCMON: VCMBUFF power-up. This bit is set and cleared by software. 0: VCM BUFFER OFF 1: VCM BUFFER ON"]
    #[inline(always)]
    pub fn vcmon(&mut self) -> VcmonW<'_, CrSpec> {
        VcmonW::new(self, 16)
    }
}
#[doc = "CR register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
