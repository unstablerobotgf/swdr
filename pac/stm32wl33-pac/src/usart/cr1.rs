#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `UE` reader - UE: USART enable When this bit is cleared, the USART prescalers and outputs are stopped immediately, and current operations are discarded. The configuration of the USART is kept, but all the status flags, in the USART_ISR are reset. This bit is set and cleared by software. -0: USART prescaler and outputs disabled, low power mode -1: USART enabled"]
pub type UeR = crate::BitReader;
#[doc = "Field `UE` writer - UE: USART enable When this bit is cleared, the USART prescalers and outputs are stopped immediately, and current operations are discarded. The configuration of the USART is kept, but all the status flags, in the USART_ISR are reset. This bit is set and cleared by software. -0: USART prescaler and outputs disabled, low power mode -1: USART enabled"]
pub type UeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RE` reader - RE: Receiver enable This bit enables the receiver. It is set and cleared by software. -0: Receiver is disabled -1: Receiver is enabled and begins searching for a start bit"]
pub type ReR = crate::BitReader;
#[doc = "Field `RE` writer - RE: Receiver enable This bit enables the receiver. It is set and cleared by software. -0: Receiver is disabled -1: Receiver is enabled and begins searching for a start bit"]
pub type ReW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE` reader - TE: Transmitter enable This bit enables the transmitter. It is set and cleared by software. -0: Transmitter is disabled -1: Transmitter is enabled"]
pub type TeR = crate::BitReader;
#[doc = "Field `TE` writer - TE: Transmitter enable This bit enables the transmitter. It is set and cleared by software. -0: Transmitter is disabled -1: Transmitter is enabled"]
pub type TeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLEIE` reader - IDLEIE: IDLE interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated whenever IDLE=1 in the USART_ISR register"]
pub type IdleieR = crate::BitReader;
#[doc = "Field `IDLEIE` writer - IDLEIE: IDLE interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated whenever IDLE=1 in the USART_ISR register"]
pub type IdleieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXNEIE_RXFNEIE` reader - RXNEIE/RXFNEIE: Receive data register not empty/RXFIFO not empty interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated whenever ORE=1 or RXNE/RXFNE=1 in the USART_ISR register"]
pub type RxneieRxfneieR = crate::BitReader;
#[doc = "Field `RXNEIE_RXFNEIE` writer - RXNEIE/RXFNEIE: Receive data register not empty/RXFIFO not empty interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated whenever ORE=1 or RXNE/RXFNE=1 in the USART_ISR register"]
pub type RxneieRxfneieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIE` reader - TCIE: Transmission complete interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated whenever TC=1 in the USART_ISR register"]
pub type TcieR = crate::BitReader;
#[doc = "Field `TCIE` writer - TCIE: Transmission complete interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated whenever TC=1 in the USART_ISR register"]
pub type TcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEIE_TXFNFIE` reader - TXEIE/TXFNFIE: Transmit data regsiter empty/TXFIFO not full interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated whenever TXE/TXFNF =1 in the USART_ISR register"]
pub type TxeieTxfnfieR = crate::BitReader;
#[doc = "Field `TXEIE_TXFNFIE` writer - TXEIE/TXFNFIE: Transmit data regsiter empty/TXFIFO not full interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated whenever TXE/TXFNF =1 in the USART_ISR register"]
pub type TxeieTxfnfieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEIE` reader - PEIE: PE interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated whenever PE=1 in the USART_ISR register"]
pub type PeieR = crate::BitReader;
#[doc = "Field `PEIE` writer - PEIE: PE interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated whenever PE=1 in the USART_ISR register"]
pub type PeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS` reader - PS: Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity will be selected after the current byte. -0: Even parity -1: Odd parity This bit field can only be written when the USART is disabled (UE=0)."]
pub type PsR = crate::BitReader;
#[doc = "Field `PS` writer - PS: Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity will be selected after the current byte. -0: Even parity -1: Odd parity This bit field can only be written when the USART is disabled (UE=0)."]
pub type PsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCE` reader - PCE: Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M=1; 8th bit if M=0) and parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). -0: Parity control disabled -1: Parity control enabled This bit field can only be written when the USART is disabled (UE=0)."]
pub type PceR = crate::BitReader;
#[doc = "Field `PCE` writer - PCE: Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M=1; 8th bit if M=0) and parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). -0: Parity control disabled -1: Parity control enabled This bit field can only be written when the USART is disabled (UE=0)."]
pub type PceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKE` reader - WAKE: Receiver wakeup method This bit determines the USART wakeup method from Mute mode. It is set or cleared by software. -0: Idle line -1: Address mark This bit field can only be written when the USART is disabled (UE=0)."]
pub type WakeR = crate::BitReader;
#[doc = "Field `WAKE` writer - WAKE: Receiver wakeup method This bit determines the USART wakeup method from Mute mode. It is set or cleared by software. -0: Idle line -1: Address mark This bit field can only be written when the USART is disabled (UE=0)."]
pub type WakeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M_0` reader - M0: Word length This bit, with bit 28 (M1) determine the word length. It is set or cleared by software. See Bit -28 (M1)description. This bit can only be written when the USART is disabled (UE=0)."]
pub type M0R = crate::BitReader;
#[doc = "Field `M_0` writer - M0: Word length This bit, with bit 28 (M1) determine the word length. It is set or cleared by software. See Bit -28 (M1)description. This bit can only be written when the USART is disabled (UE=0)."]
pub type M0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MME` reader - MME: Mute mode enable This bit activates the mute mode function of the USART. When set, the USART can switch between the active and mute modes, as defined by the WAKE bit. It is set and cleared by software. -0: Receiver in active mode permanently -1: Receiver can switch between mute mode and active mode"]
pub type MmeR = crate::BitReader;
#[doc = "Field `MME` writer - MME: Mute mode enable This bit activates the mute mode function of the USART. When set, the USART can switch between the active and mute modes, as defined by the WAKE bit. It is set and cleared by software. -0: Receiver in active mode permanently -1: Receiver can switch between mute mode and active mode"]
pub type MmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMIE` reader - CMIE: Character match interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated when the CMF bit is set in the USART_ISR register."]
pub type CmieR = crate::BitReader;
#[doc = "Field `CMIE` writer - CMIE: Character match interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated when the CMF bit is set in the USART_ISR register."]
pub type CmieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVER8` reader - OVER8: Oversampling mode -0: Oversampling by 16 This bit can only be written when the USART is disabled (UE=0)."]
pub type Over8R = crate::BitReader;
#[doc = "Field `OVER8` writer - OVER8: Oversampling mode -0: Oversampling by 16 This bit can only be written when the USART is disabled (UE=0)."]
pub type Over8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEDT` reader - DEDT\\[4:0\\]: Driver Enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). If the USART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bit field can only be written when the USART is disabled (UE=0)."]
pub type DedtR = crate::FieldReader;
#[doc = "Field `DEDT` writer - DEDT\\[4:0\\]: Driver Enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). If the USART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bit field can only be written when the USART is disabled (UE=0)."]
pub type DedtW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DEAT` reader - DEAT\\[4:0\\]: Driver Enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). This bit field can only be written when the USART is disabled (UE=0)."]
pub type DeatR = crate::FieldReader;
#[doc = "Field `DEAT` writer - DEAT\\[4:0\\]: Driver Enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). This bit field can only be written when the USART is disabled (UE=0)."]
pub type DeatW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RTOIE` reader - RTOIE: Receiver timeout interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when the RTOF bit is set in the USART_ISR register"]
pub type RtoieR = crate::BitReader;
#[doc = "Field `RTOIE` writer - RTOIE: Receiver timeout interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when the RTOF bit is set in the USART_ISR register"]
pub type RtoieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "EOBIE: End of Block interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eobie {
    #[doc = "0: Interrupt is inhibited"]
    B0x0 = 0,
    #[doc = "1: A USART interrupt is generated when the EOBF flag is set in the USART_ISR register"]
    B0x1 = 1,
}
impl From<Eobie> for bool {
    #[inline(always)]
    fn from(variant: Eobie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOBIE` reader - EOBIE: End of Block interrupt enable This bit is set and cleared by software."]
pub type EobieR = crate::BitReader<Eobie>;
impl EobieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eobie {
        match self.bits {
            false => Eobie::B0x0,
            true => Eobie::B0x1,
        }
    }
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Eobie::B0x0
    }
    #[doc = "A USART interrupt is generated when the EOBF flag is set in the USART_ISR register"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Eobie::B0x1
    }
}
#[doc = "Field `EOBIE` writer - EOBIE: End of Block interrupt enable This bit is set and cleared by software."]
pub type EobieW<'a, REG> = crate::BitWriter<'a, REG, Eobie>;
impl<'a, REG> EobieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Eobie::B0x0)
    }
    #[doc = "A USART interrupt is generated when the EOBF flag is set in the USART_ISR register"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Eobie::B0x1)
    }
}
#[doc = "Field `M_1` reader - Word length This bit, with bit 12 (M0) determine the word length. It is set or cleared by software. M\\[1:0\\] = 00: 1 Start bit, 8 Data bits, n Stop bit M\\[1:0\\] = 01: 1 Start bit, 9 Data bits, n Stop bit M\\[1:0\\] = 10: 1 Start bit, 7 Data bits, n Stop bit This bit can only be written when the USART is disabled (UE=0).s"]
pub type M1R = crate::BitReader;
#[doc = "Field `M_1` writer - Word length This bit, with bit 12 (M0) determine the word length. It is set or cleared by software. M\\[1:0\\] = 00: 1 Start bit, 8 Data bits, n Stop bit M\\[1:0\\] = 01: 1 Start bit, 9 Data bits, n Stop bit M\\[1:0\\] = 10: 1 Start bit, 7 Data bits, n Stop bit This bit can only be written when the USART is disabled (UE=0).s"]
pub type M1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOEN` reader - FIFOEN :FIFO mode enable This bit is set and cleared by software. -0: FIFO mode is disabled. -1: FIFO mode is enabled."]
pub type FifoenR = crate::BitReader;
#[doc = "Field `FIFOEN` writer - FIFOEN :FIFO mode enable This bit is set and cleared by software. -0: FIFO mode is disabled. -1: FIFO mode is enabled."]
pub type FifoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFEIE` reader - TXFEIE :TXFIFO empty interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when TXFE=1 in the USART_ISR register"]
pub type TxfeieR = crate::BitReader;
#[doc = "Field `TXFEIE` writer - TXFEIE :TXFIFO empty interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when TXFE=1 in the USART_ISR register"]
pub type TxfeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFFIE` reader - RXFFIE :RXFIFO Full interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when RXFF=1 in the USART_ISR register"]
pub type RxffieR = crate::BitReader;
#[doc = "Field `RXFFIE` writer - RXFFIE :RXFIFO Full interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when RXFF=1 in the USART_ISR register"]
pub type RxffieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - UE: USART enable When this bit is cleared, the USART prescalers and outputs are stopped immediately, and current operations are discarded. The configuration of the USART is kept, but all the status flags, in the USART_ISR are reset. This bit is set and cleared by software. -0: USART prescaler and outputs disabled, low power mode -1: USART enabled"]
    #[inline(always)]
    pub fn ue(&self) -> UeR {
        UeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - RE: Receiver enable This bit enables the receiver. It is set and cleared by software. -0: Receiver is disabled -1: Receiver is enabled and begins searching for a start bit"]
    #[inline(always)]
    pub fn re(&self) -> ReR {
        ReR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TE: Transmitter enable This bit enables the transmitter. It is set and cleared by software. -0: Transmitter is disabled -1: Transmitter is enabled"]
    #[inline(always)]
    pub fn te(&self) -> TeR {
        TeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLEIE: IDLE interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated whenever IDLE=1 in the USART_ISR register"]
    #[inline(always)]
    pub fn idleie(&self) -> IdleieR {
        IdleieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXNEIE/RXFNEIE: Receive data register not empty/RXFIFO not empty interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated whenever ORE=1 or RXNE/RXFNE=1 in the USART_ISR register"]
    #[inline(always)]
    pub fn rxneie_rxfneie(&self) -> RxneieRxfneieR {
        RxneieRxfneieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TCIE: Transmission complete interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated whenever TC=1 in the USART_ISR register"]
    #[inline(always)]
    pub fn tcie(&self) -> TcieR {
        TcieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXEIE/TXFNFIE: Transmit data regsiter empty/TXFIFO not full interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated whenever TXE/TXFNF =1 in the USART_ISR register"]
    #[inline(always)]
    pub fn txeie_txfnfie(&self) -> TxeieTxfnfieR {
        TxeieTxfnfieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PEIE: PE interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated whenever PE=1 in the USART_ISR register"]
    #[inline(always)]
    pub fn peie(&self) -> PeieR {
        PeieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PS: Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity will be selected after the current byte. -0: Even parity -1: Odd parity This bit field can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PCE: Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M=1; 8th bit if M=0) and parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). -0: Parity control disabled -1: Parity control enabled This bit field can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn pce(&self) -> PceR {
        PceR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - WAKE: Receiver wakeup method This bit determines the USART wakeup method from Mute mode. It is set or cleared by software. -0: Idle line -1: Address mark This bit field can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn wake(&self) -> WakeR {
        WakeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - M0: Word length This bit, with bit 28 (M1) determine the word length. It is set or cleared by software. See Bit -28 (M1)description. This bit can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn m_0(&self) -> M0R {
        M0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MME: Mute mode enable This bit activates the mute mode function of the USART. When set, the USART can switch between the active and mute modes, as defined by the WAKE bit. It is set and cleared by software. -0: Receiver in active mode permanently -1: Receiver can switch between mute mode and active mode"]
    #[inline(always)]
    pub fn mme(&self) -> MmeR {
        MmeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CMIE: Character match interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated when the CMF bit is set in the USART_ISR register."]
    #[inline(always)]
    pub fn cmie(&self) -> CmieR {
        CmieR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - OVER8: Oversampling mode -0: Oversampling by 16 This bit can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn over8(&self) -> Over8R {
        Over8R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - DEDT\\[4:0\\]: Driver Enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). If the USART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bit field can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn dedt(&self) -> DedtR {
        DedtR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - DEAT\\[4:0\\]: Driver Enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). This bit field can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn deat(&self) -> DeatR {
        DeatR::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 26 - RTOIE: Receiver timeout interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when the RTOF bit is set in the USART_ISR register"]
    #[inline(always)]
    pub fn rtoie(&self) -> RtoieR {
        RtoieR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - EOBIE: End of Block interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn eobie(&self) -> EobieR {
        EobieR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Word length This bit, with bit 12 (M0) determine the word length. It is set or cleared by software. M\\[1:0\\] = 00: 1 Start bit, 8 Data bits, n Stop bit M\\[1:0\\] = 01: 1 Start bit, 9 Data bits, n Stop bit M\\[1:0\\] = 10: 1 Start bit, 7 Data bits, n Stop bit This bit can only be written when the USART is disabled (UE=0).s"]
    #[inline(always)]
    pub fn m_1(&self) -> M1R {
        M1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - FIFOEN :FIFO mode enable This bit is set and cleared by software. -0: FIFO mode is disabled. -1: FIFO mode is enabled."]
    #[inline(always)]
    pub fn fifoen(&self) -> FifoenR {
        FifoenR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - TXFEIE :TXFIFO empty interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when TXFE=1 in the USART_ISR register"]
    #[inline(always)]
    pub fn txfeie(&self) -> TxfeieR {
        TxfeieR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - RXFFIE :RXFIFO Full interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when RXFF=1 in the USART_ISR register"]
    #[inline(always)]
    pub fn rxffie(&self) -> RxffieR {
        RxffieR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UE: USART enable When this bit is cleared, the USART prescalers and outputs are stopped immediately, and current operations are discarded. The configuration of the USART is kept, but all the status flags, in the USART_ISR are reset. This bit is set and cleared by software. -0: USART prescaler and outputs disabled, low power mode -1: USART enabled"]
    #[inline(always)]
    pub fn ue(&mut self) -> UeW<'_, Cr1Spec> {
        UeW::new(self, 0)
    }
    #[doc = "Bit 2 - RE: Receiver enable This bit enables the receiver. It is set and cleared by software. -0: Receiver is disabled -1: Receiver is enabled and begins searching for a start bit"]
    #[inline(always)]
    pub fn re(&mut self) -> ReW<'_, Cr1Spec> {
        ReW::new(self, 2)
    }
    #[doc = "Bit 3 - TE: Transmitter enable This bit enables the transmitter. It is set and cleared by software. -0: Transmitter is disabled -1: Transmitter is enabled"]
    #[inline(always)]
    pub fn te(&mut self) -> TeW<'_, Cr1Spec> {
        TeW::new(self, 3)
    }
    #[doc = "Bit 4 - IDLEIE: IDLE interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated whenever IDLE=1 in the USART_ISR register"]
    #[inline(always)]
    pub fn idleie(&mut self) -> IdleieW<'_, Cr1Spec> {
        IdleieW::new(self, 4)
    }
    #[doc = "Bit 5 - RXNEIE/RXFNEIE: Receive data register not empty/RXFIFO not empty interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated whenever ORE=1 or RXNE/RXFNE=1 in the USART_ISR register"]
    #[inline(always)]
    pub fn rxneie_rxfneie(&mut self) -> RxneieRxfneieW<'_, Cr1Spec> {
        RxneieRxfneieW::new(self, 5)
    }
    #[doc = "Bit 6 - TCIE: Transmission complete interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated whenever TC=1 in the USART_ISR register"]
    #[inline(always)]
    pub fn tcie(&mut self) -> TcieW<'_, Cr1Spec> {
        TcieW::new(self, 6)
    }
    #[doc = "Bit 7 - TXEIE/TXFNFIE: Transmit data regsiter empty/TXFIFO not full interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated whenever TXE/TXFNF =1 in the USART_ISR register"]
    #[inline(always)]
    pub fn txeie_txfnfie(&mut self) -> TxeieTxfnfieW<'_, Cr1Spec> {
        TxeieTxfnfieW::new(self, 7)
    }
    #[doc = "Bit 8 - PEIE: PE interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated whenever PE=1 in the USART_ISR register"]
    #[inline(always)]
    pub fn peie(&mut self) -> PeieW<'_, Cr1Spec> {
        PeieW::new(self, 8)
    }
    #[doc = "Bit 9 - PS: Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity will be selected after the current byte. -0: Even parity -1: Odd parity This bit field can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn ps(&mut self) -> PsW<'_, Cr1Spec> {
        PsW::new(self, 9)
    }
    #[doc = "Bit 10 - PCE: Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M=1; 8th bit if M=0) and parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). -0: Parity control disabled -1: Parity control enabled This bit field can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn pce(&mut self) -> PceW<'_, Cr1Spec> {
        PceW::new(self, 10)
    }
    #[doc = "Bit 11 - WAKE: Receiver wakeup method This bit determines the USART wakeup method from Mute mode. It is set or cleared by software. -0: Idle line -1: Address mark This bit field can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn wake(&mut self) -> WakeW<'_, Cr1Spec> {
        WakeW::new(self, 11)
    }
    #[doc = "Bit 12 - M0: Word length This bit, with bit 28 (M1) determine the word length. It is set or cleared by software. See Bit -28 (M1)description. This bit can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn m_0(&mut self) -> M0W<'_, Cr1Spec> {
        M0W::new(self, 12)
    }
    #[doc = "Bit 13 - MME: Mute mode enable This bit activates the mute mode function of the USART. When set, the USART can switch between the active and mute modes, as defined by the WAKE bit. It is set and cleared by software. -0: Receiver in active mode permanently -1: Receiver can switch between mute mode and active mode"]
    #[inline(always)]
    pub fn mme(&mut self) -> MmeW<'_, Cr1Spec> {
        MmeW::new(self, 13)
    }
    #[doc = "Bit 14 - CMIE: Character match interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated when the CMF bit is set in the USART_ISR register."]
    #[inline(always)]
    pub fn cmie(&mut self) -> CmieW<'_, Cr1Spec> {
        CmieW::new(self, 14)
    }
    #[doc = "Bit 15 - OVER8: Oversampling mode -0: Oversampling by 16 This bit can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn over8(&mut self) -> Over8W<'_, Cr1Spec> {
        Over8W::new(self, 15)
    }
    #[doc = "Bits 16:20 - DEDT\\[4:0\\]: Driver Enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). If the USART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bit field can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn dedt(&mut self) -> DedtW<'_, Cr1Spec> {
        DedtW::new(self, 16)
    }
    #[doc = "Bits 21:25 - DEAT\\[4:0\\]: Driver Enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). This bit field can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn deat(&mut self) -> DeatW<'_, Cr1Spec> {
        DeatW::new(self, 21)
    }
    #[doc = "Bit 26 - RTOIE: Receiver timeout interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when the RTOF bit is set in the USART_ISR register"]
    #[inline(always)]
    pub fn rtoie(&mut self) -> RtoieW<'_, Cr1Spec> {
        RtoieW::new(self, 26)
    }
    #[doc = "Bit 27 - EOBIE: End of Block interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn eobie(&mut self) -> EobieW<'_, Cr1Spec> {
        EobieW::new(self, 27)
    }
    #[doc = "Bit 28 - Word length This bit, with bit 12 (M0) determine the word length. It is set or cleared by software. M\\[1:0\\] = 00: 1 Start bit, 8 Data bits, n Stop bit M\\[1:0\\] = 01: 1 Start bit, 9 Data bits, n Stop bit M\\[1:0\\] = 10: 1 Start bit, 7 Data bits, n Stop bit This bit can only be written when the USART is disabled (UE=0).s"]
    #[inline(always)]
    pub fn m_1(&mut self) -> M1W<'_, Cr1Spec> {
        M1W::new(self, 28)
    }
    #[doc = "Bit 29 - FIFOEN :FIFO mode enable This bit is set and cleared by software. -0: FIFO mode is disabled. -1: FIFO mode is enabled."]
    #[inline(always)]
    pub fn fifoen(&mut self) -> FifoenW<'_, Cr1Spec> {
        FifoenW::new(self, 29)
    }
    #[doc = "Bit 30 - TXFEIE :TXFIFO empty interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when TXFE=1 in the USART_ISR register"]
    #[inline(always)]
    pub fn txfeie(&mut self) -> TxfeieW<'_, Cr1Spec> {
        TxfeieW::new(self, 30)
    }
    #[doc = "Bit 31 - RXFFIE :RXFIFO Full interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when RXFF=1 in the USART_ISR register"]
    #[inline(always)]
    pub fn rxffie(&mut self) -> RxffieW<'_, Cr1Spec> {
        RxffieW::new(self, 31)
    }
}
#[doc = "CR1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {}
