#[doc = "Register `CR3` reader"]
pub type R = crate::R<Cr3Spec>;
#[doc = "Register `CR3` writer"]
pub type W = crate::W<Cr3Spec>;
#[doc = "Field `EIE` reader - EIE: Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FE=1 or ORE=1 or NF=1or UDR = 1 in the USART_ISR register). -0: Interrupt is inhibited -1: An interrupt is generated when FE=1 or ORE=1 or NF=1 or UDR = 1 (in SPI slave mode) in the USART_ISR register."]
pub type EieR = crate::BitReader;
#[doc = "Field `EIE` writer - EIE: Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FE=1 or ORE=1 or NF=1or UDR = 1 in the USART_ISR register). -0: Interrupt is inhibited -1: An interrupt is generated when FE=1 or ORE=1 or NF=1 or UDR = 1 (in SPI slave mode) in the USART_ISR register."]
pub type EieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDSEL` reader - HDSEL: Half-duplex selection Selection of Single-wire Half-duplex mode -0: Half duplex mode is not selected -1: Half duplex mode is selected This bit can only be written when the USART is disabled (UE=0)."]
pub type HdselR = crate::BitReader;
#[doc = "Field `HDSEL` writer - HDSEL: Half-duplex selection Selection of Single-wire Half-duplex mode -0: Half duplex mode is not selected -1: Half duplex mode is selected This bit can only be written when the USART is disabled (UE=0)."]
pub type HdselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAR` reader - DMAR: DMA enable receiver This bit is set/reset by software -1: DMA mode is enabled for reception -0: DMA mode is disabled for reception"]
pub type DmarR = crate::BitReader;
#[doc = "Field `DMAR` writer - DMAR: DMA enable receiver This bit is set/reset by software -1: DMA mode is enabled for reception -0: DMA mode is disabled for reception"]
pub type DmarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAT` reader - DMAT: DMA enable transmitter This bit is set/reset by software -1: DMA mode is enabled for transmission -0: DMA mode is disabled for transmission"]
pub type DmatR = crate::BitReader;
#[doc = "Field `DMAT` writer - DMAT: DMA enable transmitter This bit is set/reset by software -1: DMA mode is enabled for transmission -0: DMA mode is disabled for transmission"]
pub type DmatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSE` reader - RTSE: RTS enable -0: RTS hardware flow control disabled -1: RTS output enabled, data is only requested when there is space in the receive buffer. The transmission of data is expected to cease after the current character has been transmitted. The nRTS output is asserted (pulled to 0) when data can be received. This bit can only be written when the USART is disabled (UE=0)."]
pub type RtseR = crate::BitReader;
#[doc = "Field `RTSE` writer - RTSE: RTS enable -0: RTS hardware flow control disabled -1: RTS output enabled, data is only requested when there is space in the receive buffer. The transmission of data is expected to cease after the current character has been transmitted. The nRTS output is asserted (pulled to 0) when data can be received. This bit can only be written when the USART is disabled (UE=0)."]
pub type RtseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSE` reader - CTSE: CTS enable -0: CTS hardware flow control disabled -1: CTS mode enabled, data is only transmitted when the nCTS input is asserted (tied to 0). If the nCTS input is deasserted while data is being transmitted, then the transmission is completed before stopping. If data is written into the data register while nCTS is asserted, the transmission is postponed until nCTS is asserted. This bit can only be written when the USART is disabled (UE=0)"]
pub type CtseR = crate::BitReader;
#[doc = "Field `CTSE` writer - CTSE: CTS enable -0: CTS hardware flow control disabled -1: CTS mode enabled, data is only transmitted when the nCTS input is asserted (tied to 0). If the nCTS input is deasserted while data is being transmitted, then the transmission is completed before stopping. If data is written into the data register while nCTS is asserted, the transmission is postponed until nCTS is asserted. This bit can only be written when the USART is disabled (UE=0)"]
pub type CtseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSIE` reader - CTSIE: CTS interrupt enable -0: Interrupt is inhibited -1: An interrupt is generated whenever CTSIF=1 in the USART_ISR register"]
pub type CtsieR = crate::BitReader;
#[doc = "Field `CTSIE` writer - CTSIE: CTS interrupt enable -0: Interrupt is inhibited -1: An interrupt is generated whenever CTSIF=1 in the USART_ISR register"]
pub type CtsieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRDIS` reader - OVRDIS: Overrun Disable This bit is used to disable the receive overrun detection. -0: Overrun Error Flag, ORE, is set when received data is not read before receiving new data. -1: Overrun functionality is disabled. If new data is received while the RXNE flag is still set the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO will be bypassed and data will be written directly in USARTx_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UE=0)."]
pub type OvrdisR = crate::BitReader;
#[doc = "Field `OVRDIS` writer - OVRDIS: Overrun Disable This bit is used to disable the receive overrun detection. -0: Overrun Error Flag, ORE, is set when received data is not read before receiving new data. -1: Overrun functionality is disabled. If new data is received while the RXNE flag is still set the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO will be bypassed and data will be written directly in USARTx_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UE=0)."]
pub type OvrdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRE` reader - DDRE: DMA Disable on Reception Error -0: DMA is not disabled in case of reception error. The corresponding error flag is set but RXNE is kept 0 preventing from overrun. As a consequence, the DMA request is not asserted, so the erroneous data is not transferred (no DMA request), but next correct received data will be transferred. (used for Smartcard mode) -1: DMA is disabled following a reception error. The corresponding error flag is set, as well as RXNE. The DMA request is masked until the error flag is cleared. This means that the software must first disable the DMA request (DMAR = 0) or clear RXNE(RXFNE is case FIFO mode is enabled) before clearing the error flag. This bit can only be written when the USART is disabled (UE=0)."]
pub type DdreR = crate::BitReader;
#[doc = "Field `DDRE` writer - DDRE: DMA Disable on Reception Error -0: DMA is not disabled in case of reception error. The corresponding error flag is set but RXNE is kept 0 preventing from overrun. As a consequence, the DMA request is not asserted, so the erroneous data is not transferred (no DMA request), but next correct received data will be transferred. (used for Smartcard mode) -1: DMA is disabled following a reception error. The corresponding error flag is set, as well as RXNE. The DMA request is masked until the error flag is cleared. This means that the software must first disable the DMA request (DMAR = 0) or clear RXNE(RXFNE is case FIFO mode is enabled) before clearing the error flag. This bit can only be written when the USART is disabled (UE=0)."]
pub type DdreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEM` reader - DEM: Driver enable mode This bit allows the user to activate the external transceiver control, through the DE signal. -0: DE function is disabled. -1: DE function is enabled. The DE signal is output on the RTS pin. This bit can only be written when the USART is disabled (UE=0)."]
pub type DemR = crate::BitReader;
#[doc = "Field `DEM` writer - DEM: Driver enable mode This bit allows the user to activate the external transceiver control, through the DE signal. -0: DE function is disabled. -1: DE function is enabled. The DE signal is output on the RTS pin. This bit can only be written when the USART is disabled (UE=0)."]
pub type DemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEP` reader - DEP: Driver enable polarity selection -0: DE signal is active high. -1: DE signal is active low. This bit can only be written when the USART is disabled (UE=0)."]
pub type DepR = crate::BitReader;
#[doc = "Field `DEP` writer - DEP: Driver enable polarity selection -0: DE signal is active high. -1: DE signal is active low. This bit can only be written when the USART is disabled (UE=0)."]
pub type DepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUS` reader - WUS\\[1:0\\]: Wakeup from Stop mode interrupt flag selection This bit-field specify the event which activates the WUF (Wakeup from Stop mode flag). -00: WUF active on address match (as defined by ADD\\[7:0\\] and ADDM7) -01:Reserved. -10: WUF active on Start bit detection -11: WUF active on RXNE. This bit field can only be written when the LPUART is disabled (UE=0)."]
pub type WusR = crate::FieldReader;
#[doc = "Field `WUS` writer - WUS\\[1:0\\]: Wakeup from Stop mode interrupt flag selection This bit-field specify the event which activates the WUF (Wakeup from Stop mode flag). -00: WUF active on address match (as defined by ADD\\[7:0\\] and ADDM7) -01:Reserved. -10: WUF active on Start bit detection -11: WUF active on RXNE. This bit field can only be written when the LPUART is disabled (UE=0)."]
pub type WusW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WUFIE` reader - WUFIE: Wakeup from Stop mode interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An LPUART interrupt is generated whenever WUF=1 in the LPUART_ISR register"]
pub type WufieR = crate::BitReader;
#[doc = "Field `WUFIE` writer - WUFIE: Wakeup from Stop mode interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An LPUART interrupt is generated whenever WUF=1 in the LPUART_ISR register"]
pub type WufieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFTIE` reader - TXFTIE: TXFIFO threshold interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when TXFIFO reaches the threshold programmed in TXFTCFG."]
pub type TxftieR = crate::BitReader;
#[doc = "Field `TXFTIE` writer - TXFTIE: TXFIFO threshold interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when TXFIFO reaches the threshold programmed in TXFTCFG."]
pub type TxftieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFTCFG` reader - RXFTCFG: Receive FIFO threshold configuration -000:Receive FIFO reaches 1/8 of its depth. -001:Receive FIFO reaches 1/4 of its depth. -010:Receive FIFO reaches 1/2 of its depth. -011:Receive FIFO reaches 3/4 of its depth. -100:Receive FIFO reaches 7/8 of its depth. -101:Receive FIFO becomes full. Remaining combinations: Reserved."]
pub type RxftcfgR = crate::FieldReader;
#[doc = "Field `RXFTCFG` writer - RXFTCFG: Receive FIFO threshold configuration -000:Receive FIFO reaches 1/8 of its depth. -001:Receive FIFO reaches 1/4 of its depth. -010:Receive FIFO reaches 1/2 of its depth. -011:Receive FIFO reaches 3/4 of its depth. -100:Receive FIFO reaches 7/8 of its depth. -101:Receive FIFO becomes full. Remaining combinations: Reserved."]
pub type RxftcfgW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RXFTIE` reader - RXFTIE: RXFIFO threshold interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when Receive FIFO reaches the threshold programmed in RXFTCFG."]
pub type RxftieR = crate::BitReader;
#[doc = "Field `RXFTIE` writer - RXFTIE: RXFIFO threshold interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when Receive FIFO reaches the threshold programmed in RXFTCFG."]
pub type RxftieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFTCFG` reader - TXFTCFG: TXFIFO threshold configuration -000:TXFIFO reaches 1/8 of its depth. -001:TXFIFO reaches 1/4 of its depth. -010:TXFIFO reaches 1/2 of its depth. -011:TXFIFO reaches 3/4 of its depth. -100:TXFIFO reaches 7/8 of its depth. -101:TXFIFO becomes empty. Remaining combinations: Reserved."]
pub type TxftcfgR = crate::FieldReader;
#[doc = "Field `TXFTCFG` writer - TXFTCFG: TXFIFO threshold configuration -000:TXFIFO reaches 1/8 of its depth. -001:TXFIFO reaches 1/4 of its depth. -010:TXFIFO reaches 1/2 of its depth. -011:TXFIFO reaches 3/4 of its depth. -100:TXFIFO reaches 7/8 of its depth. -101:TXFIFO becomes empty. Remaining combinations: Reserved."]
pub type TxftcfgW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - EIE: Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FE=1 or ORE=1 or NF=1or UDR = 1 in the USART_ISR register). -0: Interrupt is inhibited -1: An interrupt is generated when FE=1 or ORE=1 or NF=1 or UDR = 1 (in SPI slave mode) in the USART_ISR register."]
    #[inline(always)]
    pub fn eie(&self) -> EieR {
        EieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - HDSEL: Half-duplex selection Selection of Single-wire Half-duplex mode -0: Half duplex mode is not selected -1: Half duplex mode is selected This bit can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn hdsel(&self) -> HdselR {
        HdselR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - DMAR: DMA enable receiver This bit is set/reset by software -1: DMA mode is enabled for reception -0: DMA mode is disabled for reception"]
    #[inline(always)]
    pub fn dmar(&self) -> DmarR {
        DmarR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMAT: DMA enable transmitter This bit is set/reset by software -1: DMA mode is enabled for transmission -0: DMA mode is disabled for transmission"]
    #[inline(always)]
    pub fn dmat(&self) -> DmatR {
        DmatR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RTSE: RTS enable -0: RTS hardware flow control disabled -1: RTS output enabled, data is only requested when there is space in the receive buffer. The transmission of data is expected to cease after the current character has been transmitted. The nRTS output is asserted (pulled to 0) when data can be received. This bit can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn rtse(&self) -> RtseR {
        RtseR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTSE: CTS enable -0: CTS hardware flow control disabled -1: CTS mode enabled, data is only transmitted when the nCTS input is asserted (tied to 0). If the nCTS input is deasserted while data is being transmitted, then the transmission is completed before stopping. If data is written into the data register while nCTS is asserted, the transmission is postponed until nCTS is asserted. This bit can only be written when the USART is disabled (UE=0)"]
    #[inline(always)]
    pub fn ctse(&self) -> CtseR {
        CtseR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTSIE: CTS interrupt enable -0: Interrupt is inhibited -1: An interrupt is generated whenever CTSIF=1 in the USART_ISR register"]
    #[inline(always)]
    pub fn ctsie(&self) -> CtsieR {
        CtsieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - OVRDIS: Overrun Disable This bit is used to disable the receive overrun detection. -0: Overrun Error Flag, ORE, is set when received data is not read before receiving new data. -1: Overrun functionality is disabled. If new data is received while the RXNE flag is still set the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO will be bypassed and data will be written directly in USARTx_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn ovrdis(&self) -> OvrdisR {
        OvrdisR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DDRE: DMA Disable on Reception Error -0: DMA is not disabled in case of reception error. The corresponding error flag is set but RXNE is kept 0 preventing from overrun. As a consequence, the DMA request is not asserted, so the erroneous data is not transferred (no DMA request), but next correct received data will be transferred. (used for Smartcard mode) -1: DMA is disabled following a reception error. The corresponding error flag is set, as well as RXNE. The DMA request is masked until the error flag is cleared. This means that the software must first disable the DMA request (DMAR = 0) or clear RXNE(RXFNE is case FIFO mode is enabled) before clearing the error flag. This bit can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn ddre(&self) -> DdreR {
        DdreR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DEM: Driver enable mode This bit allows the user to activate the external transceiver control, through the DE signal. -0: DE function is disabled. -1: DE function is enabled. The DE signal is output on the RTS pin. This bit can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn dem(&self) -> DemR {
        DemR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DEP: Driver enable polarity selection -0: DE signal is active high. -1: DE signal is active low. This bit can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn dep(&self) -> DepR {
        DepR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 20:21 - WUS\\[1:0\\]: Wakeup from Stop mode interrupt flag selection This bit-field specify the event which activates the WUF (Wakeup from Stop mode flag). -00: WUF active on address match (as defined by ADD\\[7:0\\] and ADDM7) -01:Reserved. -10: WUF active on Start bit detection -11: WUF active on RXNE. This bit field can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    pub fn wus(&self) -> WusR {
        WusR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - WUFIE: Wakeup from Stop mode interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An LPUART interrupt is generated whenever WUF=1 in the LPUART_ISR register"]
    #[inline(always)]
    pub fn wufie(&self) -> WufieR {
        WufieR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TXFTIE: TXFIFO threshold interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when TXFIFO reaches the threshold programmed in TXFTCFG."]
    #[inline(always)]
    pub fn txftie(&self) -> TxftieR {
        TxftieR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 25:27 - RXFTCFG: Receive FIFO threshold configuration -000:Receive FIFO reaches 1/8 of its depth. -001:Receive FIFO reaches 1/4 of its depth. -010:Receive FIFO reaches 1/2 of its depth. -011:Receive FIFO reaches 3/4 of its depth. -100:Receive FIFO reaches 7/8 of its depth. -101:Receive FIFO becomes full. Remaining combinations: Reserved."]
    #[inline(always)]
    pub fn rxftcfg(&self) -> RxftcfgR {
        RxftcfgR::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - RXFTIE: RXFIFO threshold interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when Receive FIFO reaches the threshold programmed in RXFTCFG."]
    #[inline(always)]
    pub fn rxftie(&self) -> RxftieR {
        RxftieR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - TXFTCFG: TXFIFO threshold configuration -000:TXFIFO reaches 1/8 of its depth. -001:TXFIFO reaches 1/4 of its depth. -010:TXFIFO reaches 1/2 of its depth. -011:TXFIFO reaches 3/4 of its depth. -100:TXFIFO reaches 7/8 of its depth. -101:TXFIFO becomes empty. Remaining combinations: Reserved."]
    #[inline(always)]
    pub fn txftcfg(&self) -> TxftcfgR {
        TxftcfgR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - EIE: Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FE=1 or ORE=1 or NF=1or UDR = 1 in the USART_ISR register). -0: Interrupt is inhibited -1: An interrupt is generated when FE=1 or ORE=1 or NF=1 or UDR = 1 (in SPI slave mode) in the USART_ISR register."]
    #[inline(always)]
    pub fn eie(&mut self) -> EieW<'_, Cr3Spec> {
        EieW::new(self, 0)
    }
    #[doc = "Bit 3 - HDSEL: Half-duplex selection Selection of Single-wire Half-duplex mode -0: Half duplex mode is not selected -1: Half duplex mode is selected This bit can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn hdsel(&mut self) -> HdselW<'_, Cr3Spec> {
        HdselW::new(self, 3)
    }
    #[doc = "Bit 6 - DMAR: DMA enable receiver This bit is set/reset by software -1: DMA mode is enabled for reception -0: DMA mode is disabled for reception"]
    #[inline(always)]
    pub fn dmar(&mut self) -> DmarW<'_, Cr3Spec> {
        DmarW::new(self, 6)
    }
    #[doc = "Bit 7 - DMAT: DMA enable transmitter This bit is set/reset by software -1: DMA mode is enabled for transmission -0: DMA mode is disabled for transmission"]
    #[inline(always)]
    pub fn dmat(&mut self) -> DmatW<'_, Cr3Spec> {
        DmatW::new(self, 7)
    }
    #[doc = "Bit 8 - RTSE: RTS enable -0: RTS hardware flow control disabled -1: RTS output enabled, data is only requested when there is space in the receive buffer. The transmission of data is expected to cease after the current character has been transmitted. The nRTS output is asserted (pulled to 0) when data can be received. This bit can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn rtse(&mut self) -> RtseW<'_, Cr3Spec> {
        RtseW::new(self, 8)
    }
    #[doc = "Bit 9 - CTSE: CTS enable -0: CTS hardware flow control disabled -1: CTS mode enabled, data is only transmitted when the nCTS input is asserted (tied to 0). If the nCTS input is deasserted while data is being transmitted, then the transmission is completed before stopping. If data is written into the data register while nCTS is asserted, the transmission is postponed until nCTS is asserted. This bit can only be written when the USART is disabled (UE=0)"]
    #[inline(always)]
    pub fn ctse(&mut self) -> CtseW<'_, Cr3Spec> {
        CtseW::new(self, 9)
    }
    #[doc = "Bit 10 - CTSIE: CTS interrupt enable -0: Interrupt is inhibited -1: An interrupt is generated whenever CTSIF=1 in the USART_ISR register"]
    #[inline(always)]
    pub fn ctsie(&mut self) -> CtsieW<'_, Cr3Spec> {
        CtsieW::new(self, 10)
    }
    #[doc = "Bit 12 - OVRDIS: Overrun Disable This bit is used to disable the receive overrun detection. -0: Overrun Error Flag, ORE, is set when received data is not read before receiving new data. -1: Overrun functionality is disabled. If new data is received while the RXNE flag is still set the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO will be bypassed and data will be written directly in USARTx_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn ovrdis(&mut self) -> OvrdisW<'_, Cr3Spec> {
        OvrdisW::new(self, 12)
    }
    #[doc = "Bit 13 - DDRE: DMA Disable on Reception Error -0: DMA is not disabled in case of reception error. The corresponding error flag is set but RXNE is kept 0 preventing from overrun. As a consequence, the DMA request is not asserted, so the erroneous data is not transferred (no DMA request), but next correct received data will be transferred. (used for Smartcard mode) -1: DMA is disabled following a reception error. The corresponding error flag is set, as well as RXNE. The DMA request is masked until the error flag is cleared. This means that the software must first disable the DMA request (DMAR = 0) or clear RXNE(RXFNE is case FIFO mode is enabled) before clearing the error flag. This bit can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn ddre(&mut self) -> DdreW<'_, Cr3Spec> {
        DdreW::new(self, 13)
    }
    #[doc = "Bit 14 - DEM: Driver enable mode This bit allows the user to activate the external transceiver control, through the DE signal. -0: DE function is disabled. -1: DE function is enabled. The DE signal is output on the RTS pin. This bit can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn dem(&mut self) -> DemW<'_, Cr3Spec> {
        DemW::new(self, 14)
    }
    #[doc = "Bit 15 - DEP: Driver enable polarity selection -0: DE signal is active high. -1: DE signal is active low. This bit can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn dep(&mut self) -> DepW<'_, Cr3Spec> {
        DepW::new(self, 15)
    }
    #[doc = "Bits 20:21 - WUS\\[1:0\\]: Wakeup from Stop mode interrupt flag selection This bit-field specify the event which activates the WUF (Wakeup from Stop mode flag). -00: WUF active on address match (as defined by ADD\\[7:0\\] and ADDM7) -01:Reserved. -10: WUF active on Start bit detection -11: WUF active on RXNE. This bit field can only be written when the LPUART is disabled (UE=0)."]
    #[inline(always)]
    pub fn wus(&mut self) -> WusW<'_, Cr3Spec> {
        WusW::new(self, 20)
    }
    #[doc = "Bit 22 - WUFIE: Wakeup from Stop mode interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An LPUART interrupt is generated whenever WUF=1 in the LPUART_ISR register"]
    #[inline(always)]
    pub fn wufie(&mut self) -> WufieW<'_, Cr3Spec> {
        WufieW::new(self, 22)
    }
    #[doc = "Bit 23 - TXFTIE: TXFIFO threshold interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when TXFIFO reaches the threshold programmed in TXFTCFG."]
    #[inline(always)]
    pub fn txftie(&mut self) -> TxftieW<'_, Cr3Spec> {
        TxftieW::new(self, 23)
    }
    #[doc = "Bits 25:27 - RXFTCFG: Receive FIFO threshold configuration -000:Receive FIFO reaches 1/8 of its depth. -001:Receive FIFO reaches 1/4 of its depth. -010:Receive FIFO reaches 1/2 of its depth. -011:Receive FIFO reaches 3/4 of its depth. -100:Receive FIFO reaches 7/8 of its depth. -101:Receive FIFO becomes full. Remaining combinations: Reserved."]
    #[inline(always)]
    pub fn rxftcfg(&mut self) -> RxftcfgW<'_, Cr3Spec> {
        RxftcfgW::new(self, 25)
    }
    #[doc = "Bit 28 - RXFTIE: RXFIFO threshold interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when Receive FIFO reaches the threshold programmed in RXFTCFG."]
    #[inline(always)]
    pub fn rxftie(&mut self) -> RxftieW<'_, Cr3Spec> {
        RxftieW::new(self, 28)
    }
    #[doc = "Bits 29:31 - TXFTCFG: TXFIFO threshold configuration -000:TXFIFO reaches 1/8 of its depth. -001:TXFIFO reaches 1/4 of its depth. -010:TXFIFO reaches 1/2 of its depth. -011:TXFIFO reaches 3/4 of its depth. -100:TXFIFO reaches 7/8 of its depth. -101:TXFIFO becomes empty. Remaining combinations: Reserved."]
    #[inline(always)]
    pub fn txftcfg(&mut self) -> TxftcfgW<'_, Cr3Spec> {
        TxftcfgW::new(self, 29)
    }
}
#[doc = "CR3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr3Spec;
impl crate::RegisterSpec for Cr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr3::R`](R) reader structure"]
impl crate::Readable for Cr3Spec {}
#[doc = "`write(|w| ..)` method takes [`cr3::W`](W) writer structure"]
impl crate::Writable for Cr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR3 to value 0"]
impl crate::Resettable for Cr3Spec {}
