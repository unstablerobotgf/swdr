#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `SLVEN` reader - SLVEN: Synchronous Slave mode enable When the SLVEN bit is set, the synchronous slave mode is enabled. -0: Slave mode disabled. -1: Slave mode enabled. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value"]
pub type SlvenR = crate::BitReader;
#[doc = "Field `SLVEN` writer - SLVEN: Synchronous Slave mode enable When the SLVEN bit is set, the synchronous slave mode is enabled. -0: Slave mode disabled. -1: Slave mode enabled. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value"]
pub type SlvenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS_NSS` reader - DIS_NSS When the DSI_NSS bit is set, the NSS pin input will be ignored. -0: SPI slave selection depends on NSS input pin. -1: SPI slave will be always selected and NSS input pin will be ignored. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value"]
pub type DisNssR = crate::BitReader;
#[doc = "Field `DIS_NSS` writer - DIS_NSS When the DSI_NSS bit is set, the NSS pin input will be ignored. -0: SPI slave selection depends on NSS input pin. -1: SPI slave will be always selected and NSS input pin will be ignored. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value"]
pub type DisNssW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDM7` reader - ADDM7:7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. -0: 4-bit address detection -1: 7-bit address detection (in 8-bit data mode) This bit can only be written when the USART is disabled (UE=0)"]
pub type Addm7R = crate::BitReader;
#[doc = "Field `ADDM7` writer - ADDM7:7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. -0: 4-bit address detection -1: 7-bit address detection (in 8-bit data mode) This bit can only be written when the USART is disabled (UE=0)"]
pub type Addm7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBDL` reader - LBDL: LIN break detection length This bit is for selection between 11 bit or 10 bit break detection. -0: 10-bit break detection -1: 11-bit break detection This bit can only be written when the USART is disabled (UE=0)."]
pub type LbdlR = crate::BitReader;
#[doc = "Field `LBDL` writer - LBDL: LIN break detection length This bit is for selection between 11 bit or 10 bit break detection. -0: 10-bit break detection -1: 11-bit break detection This bit can only be written when the USART is disabled (UE=0)."]
pub type LbdlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBDIE` reader - LBDIE: LIN break detection interrupt enable Break interrupt mask (break detection using break delimiter). -0: Interrupt is inhibited -1: An interrupt is generated whenever LBDF=1 in the USART_ISR register"]
pub type LbdieR = crate::BitReader;
#[doc = "Field `LBDIE` writer - LBDIE: LIN break detection interrupt enable Break interrupt mask (break detection using break delimiter). -0: Interrupt is inhibited -1: An interrupt is generated whenever LBDF=1 in the USART_ISR register"]
pub type LbdieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBCL` reader - LBCL: Last bit clock pulse This bit is used to select whether the clock pulse associated with the last data bit transmitted (MSB) has to be output on the SCLK pin in synchronous mode. -0: The clock pulse of the last data bit is not output to the SCLK pin -1: The clock pulse of the last data bit is output to the SCLK pin Caution: The last bit is the 7th or 8th or 9th data bit transmitted depending on the 7 or 8 or 9 bit format selected by the M bit in the USART_CR1 register. This bit can only be written when the USART is disabled (UE=0)."]
pub type LbclR = crate::BitReader;
#[doc = "Field `LBCL` writer - LBCL: Last bit clock pulse This bit is used to select whether the clock pulse associated with the last data bit transmitted (MSB) has to be output on the SCLK pin in synchronous mode. -0: The clock pulse of the last data bit is not output to the SCLK pin -1: The clock pulse of the last data bit is output to the SCLK pin Caution: The last bit is the 7th or 8th or 9th data bit transmitted depending on the 7 or 8 or 9 bit format selected by the M bit in the USART_CR1 register. This bit can only be written when the USART is disabled (UE=0)."]
pub type LbclW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPHA` reader - CPHA: Clock phase This bit is used to select the phase of the clock output on the SCLK pin in synchronous mode. It works in conjunction with the CPOL bit to produce the desired clock/data relationship (see Figure 137 and Figure 138) -0: The first clock transition is the first data capture edge -1: The second clock transition is the first data capture edge This bit can only be written when the USART is disabled (UE=0)."]
pub type CphaR = crate::BitReader;
#[doc = "Field `CPHA` writer - CPHA: Clock phase This bit is used to select the phase of the clock output on the SCLK pin in synchronous mode. It works in conjunction with the CPOL bit to produce the desired clock/data relationship (see Figure 137 and Figure 138) -0: The first clock transition is the first data capture edge -1: The second clock transition is the first data capture edge This bit can only be written when the USART is disabled (UE=0)."]
pub type CphaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - CPOL: Clock polarity This bit allows the user to select the polarity of the clock output on the SCLK pin in synchronous mode. It works in conjunction with the CPHA bit to produce the desired clock/data relationship -0: Steady low value on SCLK pin outside transmission window -1: Steady high value on SCLK pin outside transmission window This bit can only be written when the USART is disabled (UE=0)."]
pub type CpolR = crate::BitReader;
#[doc = "Field `CPOL` writer - CPOL: Clock polarity This bit allows the user to select the polarity of the clock output on the SCLK pin in synchronous mode. It works in conjunction with the CPHA bit to produce the desired clock/data relationship -0: Steady low value on SCLK pin outside transmission window -1: Steady high value on SCLK pin outside transmission window This bit can only be written when the USART is disabled (UE=0)."]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKEN` reader - CLKEN: Clock enable This bit allows the user to enable the SCLK pin. -0: SCLK pin disabled -1: SCLK pin enabled This bit can only be written when the USART is disabled (UE=0). Note: If neither synchronous mode nor Smartcard mode is supported, this bit is reserved and forced by hardware to 0. Please refer to Section 23.4: USART implementation on page 483. Note: In Smartcard mode, in order to provide correctly the SCLK clock to the smartcard, the steps below must be respected: - UE = 0 - SCEN = 1 - GTPR configuration - CLKEN= 1 - UE = 1"]
pub type ClkenR = crate::BitReader;
#[doc = "Field `CLKEN` writer - CLKEN: Clock enable This bit allows the user to enable the SCLK pin. -0: SCLK pin disabled -1: SCLK pin enabled This bit can only be written when the USART is disabled (UE=0). Note: If neither synchronous mode nor Smartcard mode is supported, this bit is reserved and forced by hardware to 0. Please refer to Section 23.4: USART implementation on page 483. Note: In Smartcard mode, in order to provide correctly the SCLK clock to the smartcard, the steps below must be respected: - UE = 0 - SCEN = 1 - GTPR configuration - CLKEN= 1 - UE = 1"]
pub type ClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - STOP\\[1:0\\]: STOP bits These bits are used for programming the stop bits. -00: 1 stop bit -01: 0.5 stop bit. -10: 2 stop bits -11: 1.5 stop bits This bit field can only be written when the USART is disabled (UE=0)."]
pub type StopR = crate::FieldReader;
#[doc = "Field `STOP` writer - STOP\\[1:0\\]: STOP bits These bits are used for programming the stop bits. -00: 1 stop bit -01: 0.5 stop bit. -10: 2 stop bits -11: 1.5 stop bits This bit field can only be written when the USART is disabled (UE=0)."]
pub type StopW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LINEN` reader - LINEN: LIN mode enable This bit is set and cleared by software. -0: LIN mode disabled -1: LIN mode enabled The LIN mode enables the capability to send LIN Synch Breaks (13 low bits) using the SBKRQ bit in the USART_CR1 register, and to detect LIN Sync breaks. This bit field can only be written when the USART is disabled (UE=0)."]
pub type LinenR = crate::BitReader;
#[doc = "Field `LINEN` writer - LINEN: LIN mode enable This bit is set and cleared by software. -0: LIN mode disabled -1: LIN mode enabled The LIN mode enables the capability to send LIN Synch Breaks (13 low bits) using the SBKRQ bit in the USART_CR1 register, and to detect LIN Sync breaks. This bit field can only be written when the USART is disabled (UE=0)."]
pub type LinenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWAP` reader - SWAP: Swap TX/RX pins This bit is set and cleared by software. -0: TX/RX pins are used as defined in standard pinout -1: The TX and RX pins functions are swapped. This allows to work in the case of a cross-wired connection to another UART. This bit field can only be written when the USART is disabled (UE=0)."]
pub type SwapR = crate::BitReader;
#[doc = "Field `SWAP` writer - SWAP: Swap TX/RX pins This bit is set and cleared by software. -0: TX/RX pins are used as defined in standard pinout -1: The TX and RX pins functions are swapped. This allows to work in the case of a cross-wired connection to another UART. This bit field can only be written when the USART is disabled (UE=0)."]
pub type SwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXINV` reader - RXINV: RX pin active level inversion This bit is set and cleared by software. -0: RX pin signal works using the standard logic levels (VDD =1/idle, Gnd=0/mark) -1: RX pin signal values are inverted. ((VDD =0/mark, Gnd=1/idle). This allows the use of an external inverter on the RX line. This bit field can only be written when the USART is disabled (UE=0)."]
pub type RxinvR = crate::BitReader;
#[doc = "Field `RXINV` writer - RXINV: RX pin active level inversion This bit is set and cleared by software. -0: RX pin signal works using the standard logic levels (VDD =1/idle, Gnd=0/mark) -1: RX pin signal values are inverted. ((VDD =0/mark, Gnd=1/idle). This allows the use of an external inverter on the RX line. This bit field can only be written when the USART is disabled (UE=0)."]
pub type RxinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXINV` reader - TXINV: TX pin active level inversion This bit is set and cleared by software. -0: TX pin signal works using the standard logic levels (VDD =1/idle, Gnd=0/mark) -1: TX pin signal values are inverted. ((VDD =0/mark, Gnd=1/idle). This allows the use of an external inverter on the TX line. This bit field can only be written when the USART is disabled (UE=0)."]
pub type TxinvR = crate::BitReader;
#[doc = "Field `TXINV` writer - TXINV: TX pin active level inversion This bit is set and cleared by software. -0: TX pin signal works using the standard logic levels (VDD =1/idle, Gnd=0/mark) -1: TX pin signal values are inverted. ((VDD =0/mark, Gnd=1/idle). This allows the use of an external inverter on the TX line. This bit field can only be written when the USART is disabled (UE=0)."]
pub type TxinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAINV` reader - DATAINV: Binary data inversion This bit is set and cleared by software. -0: Logical data from the data register are send/received in positive/direct logic. (1=H, 0=L) -1: Logical data from the data register are send/received in negative/inverse logic. (1=L, 0=H). The parity bit is also inverted. This bit field can only be written when the USART is disabled (UE=0)."]
pub type DatainvR = crate::BitReader;
#[doc = "Field `DATAINV` writer - DATAINV: Binary data inversion This bit is set and cleared by software. -0: Logical data from the data register are send/received in positive/direct logic. (1=H, 0=L) -1: Logical data from the data register are send/received in negative/inverse logic. (1=L, 0=H). The parity bit is also inverted. This bit field can only be written when the USART is disabled (UE=0)."]
pub type DatainvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSBFIRST` reader - MSBFIRST: Most significant bit first This bit is set and cleared by software. -0: data is transmitted/received with data bit 0 first, following the start bit. -1: data is transmitted/received with the MSB (bit 7/8) first, following the start bit. This bit field can only be written when the USART is disabled (UE=0)."]
pub type MsbfirstR = crate::BitReader;
#[doc = "Field `MSBFIRST` writer - MSBFIRST: Most significant bit first This bit is set and cleared by software. -0: data is transmitted/received with data bit 0 first, following the start bit. -1: data is transmitted/received with the MSB (bit 7/8) first, following the start bit. This bit field can only be written when the USART is disabled (UE=0)."]
pub type MsbfirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABREN` reader - ABREN: Auto baud rate enable This bit is set and cleared by software. -0: Auto baud rate detection is disabled. -1: Auto baud rate detection is enabled."]
pub type AbrenR = crate::BitReader;
#[doc = "Field `ABREN` writer - ABREN: Auto baud rate enable This bit is set and cleared by software. -0: Auto baud rate detection is disabled. -1: Auto baud rate detection is enabled."]
pub type AbrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABRMOD` reader - ABRMOD\\[1:0\\]: Auto baud rate mode These bits are set and cleared by software. -00: Measurement of the start bit is used to detect the baud rate. -01: Falling edge to falling edge measurement. (the received frame must start with a single bit = 1 -> Frame = Start10xxxxxx) -10: 0x7F frame detection. -11: 0x55 frame detection This bit field can only be written when ABREN = 0 or the USART is disabled (UE=0)."]
pub type AbrmodR = crate::FieldReader;
#[doc = "Field `ABRMOD` writer - ABRMOD\\[1:0\\]: Auto baud rate mode These bits are set and cleared by software. -00: Measurement of the start bit is used to detect the baud rate. -01: Falling edge to falling edge measurement. (the received frame must start with a single bit = 1 -> Frame = Start10xxxxxx) -10: 0x7F frame detection. -11: 0x55 frame detection This bit field can only be written when ABREN = 0 or the USART is disabled (UE=0)."]
pub type AbrmodW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RTOEN` reader - RTOEN: Receiver timeout enable This bit is set and cleared by software. -0: Receiver timeout feature disabled. -1: Receiver timeout feature enabled. When this feature is enabled, the RTOF flag in the USART_ISR register is set if the RX line is idle (no reception) for the duration programmed in the RTOR (receiver timeout register)."]
pub type RtoenR = crate::BitReader;
#[doc = "Field `RTOEN` writer - RTOEN: Receiver timeout enable This bit is set and cleared by software. -0: Receiver timeout feature disabled. -1: Receiver timeout feature enabled. When this feature is enabled, the RTOF flag in the USART_ISR register is set if the RX line is idle (no reception) for the duration programmed in the RTOR (receiver timeout register)."]
pub type RtoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADD` reader - ADD\\[7:0\\]: Address of the USART node This bit-field gives the address of the USART node or a character code to be recognized. This is used in multiprocessor communication during Mute mode or Stop mode, for wakeup with 7- bit address mark detection. The MSB of the character sent by the transmitter should be equal to 1. It may also be used for character detection during normal reception, Mute mode inactive (for example, end of block detection in ModBus protocol). In this case, the whole received character (8- bit) is compared to the ADD\\[7:0\\] value and CMF flag is set on match. This bit field can only be written when reception is disabled (RE = 0) or the USART is disabled (UE=0)"]
pub type AddR = crate::FieldReader;
#[doc = "Field `ADD` writer - ADD\\[7:0\\]: Address of the USART node This bit-field gives the address of the USART node or a character code to be recognized. This is used in multiprocessor communication during Mute mode or Stop mode, for wakeup with 7- bit address mark detection. The MSB of the character sent by the transmitter should be equal to 1. It may also be used for character detection during normal reception, Mute mode inactive (for example, end of block detection in ModBus protocol). In this case, the whole received character (8- bit) is compared to the ADD\\[7:0\\] value and CMF flag is set on match. This bit field can only be written when reception is disabled (RE = 0) or the USART is disabled (UE=0)"]
pub type AddW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - SLVEN: Synchronous Slave mode enable When the SLVEN bit is set, the synchronous slave mode is enabled. -0: Slave mode disabled. -1: Slave mode enabled. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value"]
    #[inline(always)]
    pub fn slven(&self) -> SlvenR {
        SlvenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - DIS_NSS When the DSI_NSS bit is set, the NSS pin input will be ignored. -0: SPI slave selection depends on NSS input pin. -1: SPI slave will be always selected and NSS input pin will be ignored. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value"]
    #[inline(always)]
    pub fn dis_nss(&self) -> DisNssR {
        DisNssR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADDM7:7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. -0: 4-bit address detection -1: 7-bit address detection (in 8-bit data mode) This bit can only be written when the USART is disabled (UE=0)"]
    #[inline(always)]
    pub fn addm7(&self) -> Addm7R {
        Addm7R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LBDL: LIN break detection length This bit is for selection between 11 bit or 10 bit break detection. -0: 10-bit break detection -1: 11-bit break detection This bit can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn lbdl(&self) -> LbdlR {
        LbdlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LBDIE: LIN break detection interrupt enable Break interrupt mask (break detection using break delimiter). -0: Interrupt is inhibited -1: An interrupt is generated whenever LBDF=1 in the USART_ISR register"]
    #[inline(always)]
    pub fn lbdie(&self) -> LbdieR {
        LbdieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - LBCL: Last bit clock pulse This bit is used to select whether the clock pulse associated with the last data bit transmitted (MSB) has to be output on the SCLK pin in synchronous mode. -0: The clock pulse of the last data bit is not output to the SCLK pin -1: The clock pulse of the last data bit is output to the SCLK pin Caution: The last bit is the 7th or 8th or 9th data bit transmitted depending on the 7 or 8 or 9 bit format selected by the M bit in the USART_CR1 register. This bit can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn lbcl(&self) -> LbclR {
        LbclR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPHA: Clock phase This bit is used to select the phase of the clock output on the SCLK pin in synchronous mode. It works in conjunction with the CPOL bit to produce the desired clock/data relationship (see Figure 137 and Figure 138) -0: The first clock transition is the first data capture edge -1: The second clock transition is the first data capture edge This bit can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn cpha(&self) -> CphaR {
        CphaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPOL: Clock polarity This bit allows the user to select the polarity of the clock output on the SCLK pin in synchronous mode. It works in conjunction with the CPHA bit to produce the desired clock/data relationship -0: Steady low value on SCLK pin outside transmission window -1: Steady high value on SCLK pin outside transmission window This bit can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CLKEN: Clock enable This bit allows the user to enable the SCLK pin. -0: SCLK pin disabled -1: SCLK pin enabled This bit can only be written when the USART is disabled (UE=0). Note: If neither synchronous mode nor Smartcard mode is supported, this bit is reserved and forced by hardware to 0. Please refer to Section 23.4: USART implementation on page 483. Note: In Smartcard mode, in order to provide correctly the SCLK clock to the smartcard, the steps below must be respected: - UE = 0 - SCEN = 1 - GTPR configuration - CLKEN= 1 - UE = 1"]
    #[inline(always)]
    pub fn clken(&self) -> ClkenR {
        ClkenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - STOP\\[1:0\\]: STOP bits These bits are used for programming the stop bits. -00: 1 stop bit -01: 0.5 stop bit. -10: 2 stop bits -11: 1.5 stop bits This bit field can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - LINEN: LIN mode enable This bit is set and cleared by software. -0: LIN mode disabled -1: LIN mode enabled The LIN mode enables the capability to send LIN Synch Breaks (13 low bits) using the SBKRQ bit in the USART_CR1 register, and to detect LIN Sync breaks. This bit field can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn linen(&self) -> LinenR {
        LinenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SWAP: Swap TX/RX pins This bit is set and cleared by software. -0: TX/RX pins are used as defined in standard pinout -1: The TX and RX pins functions are swapped. This allows to work in the case of a cross-wired connection to another UART. This bit field can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn swap(&self) -> SwapR {
        SwapR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RXINV: RX pin active level inversion This bit is set and cleared by software. -0: RX pin signal works using the standard logic levels (VDD =1/idle, Gnd=0/mark) -1: RX pin signal values are inverted. ((VDD =0/mark, Gnd=1/idle). This allows the use of an external inverter on the RX line. This bit field can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn rxinv(&self) -> RxinvR {
        RxinvR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TXINV: TX pin active level inversion This bit is set and cleared by software. -0: TX pin signal works using the standard logic levels (VDD =1/idle, Gnd=0/mark) -1: TX pin signal values are inverted. ((VDD =0/mark, Gnd=1/idle). This allows the use of an external inverter on the TX line. This bit field can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn txinv(&self) -> TxinvR {
        TxinvR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DATAINV: Binary data inversion This bit is set and cleared by software. -0: Logical data from the data register are send/received in positive/direct logic. (1=H, 0=L) -1: Logical data from the data register are send/received in negative/inverse logic. (1=L, 0=H). The parity bit is also inverted. This bit field can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn datainv(&self) -> DatainvR {
        DatainvR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - MSBFIRST: Most significant bit first This bit is set and cleared by software. -0: data is transmitted/received with data bit 0 first, following the start bit. -1: data is transmitted/received with the MSB (bit 7/8) first, following the start bit. This bit field can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn msbfirst(&self) -> MsbfirstR {
        MsbfirstR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ABREN: Auto baud rate enable This bit is set and cleared by software. -0: Auto baud rate detection is disabled. -1: Auto baud rate detection is enabled."]
    #[inline(always)]
    pub fn abren(&self) -> AbrenR {
        AbrenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - ABRMOD\\[1:0\\]: Auto baud rate mode These bits are set and cleared by software. -00: Measurement of the start bit is used to detect the baud rate. -01: Falling edge to falling edge measurement. (the received frame must start with a single bit = 1 -> Frame = Start10xxxxxx) -10: 0x7F frame detection. -11: 0x55 frame detection This bit field can only be written when ABREN = 0 or the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn abrmod(&self) -> AbrmodR {
        AbrmodR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - RTOEN: Receiver timeout enable This bit is set and cleared by software. -0: Receiver timeout feature disabled. -1: Receiver timeout feature enabled. When this feature is enabled, the RTOF flag in the USART_ISR register is set if the RX line is idle (no reception) for the duration programmed in the RTOR (receiver timeout register)."]
    #[inline(always)]
    pub fn rtoen(&self) -> RtoenR {
        RtoenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - ADD\\[7:0\\]: Address of the USART node This bit-field gives the address of the USART node or a character code to be recognized. This is used in multiprocessor communication during Mute mode or Stop mode, for wakeup with 7- bit address mark detection. The MSB of the character sent by the transmitter should be equal to 1. It may also be used for character detection during normal reception, Mute mode inactive (for example, end of block detection in ModBus protocol). In this case, the whole received character (8- bit) is compared to the ADD\\[7:0\\] value and CMF flag is set on match. This bit field can only be written when reception is disabled (RE = 0) or the USART is disabled (UE=0)"]
    #[inline(always)]
    pub fn add(&self) -> AddR {
        AddR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SLVEN: Synchronous Slave mode enable When the SLVEN bit is set, the synchronous slave mode is enabled. -0: Slave mode disabled. -1: Slave mode enabled. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value"]
    #[inline(always)]
    pub fn slven(&mut self) -> SlvenW<'_, Cr2Spec> {
        SlvenW::new(self, 0)
    }
    #[doc = "Bit 3 - DIS_NSS When the DSI_NSS bit is set, the NSS pin input will be ignored. -0: SPI slave selection depends on NSS input pin. -1: SPI slave will be always selected and NSS input pin will be ignored. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value"]
    #[inline(always)]
    pub fn dis_nss(&mut self) -> DisNssW<'_, Cr2Spec> {
        DisNssW::new(self, 3)
    }
    #[doc = "Bit 4 - ADDM7:7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. -0: 4-bit address detection -1: 7-bit address detection (in 8-bit data mode) This bit can only be written when the USART is disabled (UE=0)"]
    #[inline(always)]
    pub fn addm7(&mut self) -> Addm7W<'_, Cr2Spec> {
        Addm7W::new(self, 4)
    }
    #[doc = "Bit 5 - LBDL: LIN break detection length This bit is for selection between 11 bit or 10 bit break detection. -0: 10-bit break detection -1: 11-bit break detection This bit can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn lbdl(&mut self) -> LbdlW<'_, Cr2Spec> {
        LbdlW::new(self, 5)
    }
    #[doc = "Bit 6 - LBDIE: LIN break detection interrupt enable Break interrupt mask (break detection using break delimiter). -0: Interrupt is inhibited -1: An interrupt is generated whenever LBDF=1 in the USART_ISR register"]
    #[inline(always)]
    pub fn lbdie(&mut self) -> LbdieW<'_, Cr2Spec> {
        LbdieW::new(self, 6)
    }
    #[doc = "Bit 8 - LBCL: Last bit clock pulse This bit is used to select whether the clock pulse associated with the last data bit transmitted (MSB) has to be output on the SCLK pin in synchronous mode. -0: The clock pulse of the last data bit is not output to the SCLK pin -1: The clock pulse of the last data bit is output to the SCLK pin Caution: The last bit is the 7th or 8th or 9th data bit transmitted depending on the 7 or 8 or 9 bit format selected by the M bit in the USART_CR1 register. This bit can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn lbcl(&mut self) -> LbclW<'_, Cr2Spec> {
        LbclW::new(self, 8)
    }
    #[doc = "Bit 9 - CPHA: Clock phase This bit is used to select the phase of the clock output on the SCLK pin in synchronous mode. It works in conjunction with the CPOL bit to produce the desired clock/data relationship (see Figure 137 and Figure 138) -0: The first clock transition is the first data capture edge -1: The second clock transition is the first data capture edge This bit can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn cpha(&mut self) -> CphaW<'_, Cr2Spec> {
        CphaW::new(self, 9)
    }
    #[doc = "Bit 10 - CPOL: Clock polarity This bit allows the user to select the polarity of the clock output on the SCLK pin in synchronous mode. It works in conjunction with the CPHA bit to produce the desired clock/data relationship -0: Steady low value on SCLK pin outside transmission window -1: Steady high value on SCLK pin outside transmission window This bit can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn cpol(&mut self) -> CpolW<'_, Cr2Spec> {
        CpolW::new(self, 10)
    }
    #[doc = "Bit 11 - CLKEN: Clock enable This bit allows the user to enable the SCLK pin. -0: SCLK pin disabled -1: SCLK pin enabled This bit can only be written when the USART is disabled (UE=0). Note: If neither synchronous mode nor Smartcard mode is supported, this bit is reserved and forced by hardware to 0. Please refer to Section 23.4: USART implementation on page 483. Note: In Smartcard mode, in order to provide correctly the SCLK clock to the smartcard, the steps below must be respected: - UE = 0 - SCEN = 1 - GTPR configuration - CLKEN= 1 - UE = 1"]
    #[inline(always)]
    pub fn clken(&mut self) -> ClkenW<'_, Cr2Spec> {
        ClkenW::new(self, 11)
    }
    #[doc = "Bits 12:13 - STOP\\[1:0\\]: STOP bits These bits are used for programming the stop bits. -00: 1 stop bit -01: 0.5 stop bit. -10: 2 stop bits -11: 1.5 stop bits This bit field can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, Cr2Spec> {
        StopW::new(self, 12)
    }
    #[doc = "Bit 14 - LINEN: LIN mode enable This bit is set and cleared by software. -0: LIN mode disabled -1: LIN mode enabled The LIN mode enables the capability to send LIN Synch Breaks (13 low bits) using the SBKRQ bit in the USART_CR1 register, and to detect LIN Sync breaks. This bit field can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn linen(&mut self) -> LinenW<'_, Cr2Spec> {
        LinenW::new(self, 14)
    }
    #[doc = "Bit 15 - SWAP: Swap TX/RX pins This bit is set and cleared by software. -0: TX/RX pins are used as defined in standard pinout -1: The TX and RX pins functions are swapped. This allows to work in the case of a cross-wired connection to another UART. This bit field can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn swap(&mut self) -> SwapW<'_, Cr2Spec> {
        SwapW::new(self, 15)
    }
    #[doc = "Bit 16 - RXINV: RX pin active level inversion This bit is set and cleared by software. -0: RX pin signal works using the standard logic levels (VDD =1/idle, Gnd=0/mark) -1: RX pin signal values are inverted. ((VDD =0/mark, Gnd=1/idle). This allows the use of an external inverter on the RX line. This bit field can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn rxinv(&mut self) -> RxinvW<'_, Cr2Spec> {
        RxinvW::new(self, 16)
    }
    #[doc = "Bit 17 - TXINV: TX pin active level inversion This bit is set and cleared by software. -0: TX pin signal works using the standard logic levels (VDD =1/idle, Gnd=0/mark) -1: TX pin signal values are inverted. ((VDD =0/mark, Gnd=1/idle). This allows the use of an external inverter on the TX line. This bit field can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn txinv(&mut self) -> TxinvW<'_, Cr2Spec> {
        TxinvW::new(self, 17)
    }
    #[doc = "Bit 18 - DATAINV: Binary data inversion This bit is set and cleared by software. -0: Logical data from the data register are send/received in positive/direct logic. (1=H, 0=L) -1: Logical data from the data register are send/received in negative/inverse logic. (1=L, 0=H). The parity bit is also inverted. This bit field can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn datainv(&mut self) -> DatainvW<'_, Cr2Spec> {
        DatainvW::new(self, 18)
    }
    #[doc = "Bit 19 - MSBFIRST: Most significant bit first This bit is set and cleared by software. -0: data is transmitted/received with data bit 0 first, following the start bit. -1: data is transmitted/received with the MSB (bit 7/8) first, following the start bit. This bit field can only be written when the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn msbfirst(&mut self) -> MsbfirstW<'_, Cr2Spec> {
        MsbfirstW::new(self, 19)
    }
    #[doc = "Bit 20 - ABREN: Auto baud rate enable This bit is set and cleared by software. -0: Auto baud rate detection is disabled. -1: Auto baud rate detection is enabled."]
    #[inline(always)]
    pub fn abren(&mut self) -> AbrenW<'_, Cr2Spec> {
        AbrenW::new(self, 20)
    }
    #[doc = "Bits 21:22 - ABRMOD\\[1:0\\]: Auto baud rate mode These bits are set and cleared by software. -00: Measurement of the start bit is used to detect the baud rate. -01: Falling edge to falling edge measurement. (the received frame must start with a single bit = 1 -> Frame = Start10xxxxxx) -10: 0x7F frame detection. -11: 0x55 frame detection This bit field can only be written when ABREN = 0 or the USART is disabled (UE=0)."]
    #[inline(always)]
    pub fn abrmod(&mut self) -> AbrmodW<'_, Cr2Spec> {
        AbrmodW::new(self, 21)
    }
    #[doc = "Bit 23 - RTOEN: Receiver timeout enable This bit is set and cleared by software. -0: Receiver timeout feature disabled. -1: Receiver timeout feature enabled. When this feature is enabled, the RTOF flag in the USART_ISR register is set if the RX line is idle (no reception) for the duration programmed in the RTOR (receiver timeout register)."]
    #[inline(always)]
    pub fn rtoen(&mut self) -> RtoenW<'_, Cr2Spec> {
        RtoenW::new(self, 23)
    }
    #[doc = "Bits 24:31 - ADD\\[7:0\\]: Address of the USART node This bit-field gives the address of the USART node or a character code to be recognized. This is used in multiprocessor communication during Mute mode or Stop mode, for wakeup with 7- bit address mark detection. The MSB of the character sent by the transmitter should be equal to 1. It may also be used for character detection during normal reception, Mute mode inactive (for example, end of block detection in ModBus protocol). In this case, the whole received character (8- bit) is compared to the ADD\\[7:0\\] value and CMF flag is set on match. This bit field can only be written when reception is disabled (RE = 0) or the USART is disabled (UE=0)"]
    #[inline(always)]
    pub fn add(&mut self) -> AddW<'_, Cr2Spec> {
        AddW::new(self, 24)
    }
}
#[doc = "CR2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {}
