#[doc = "Register `I2C_CR1` reader"]
pub type R = crate::R<I2cCr1Spec>;
#[doc = "Register `I2C_CR1` writer"]
pub type W = crate::W<I2cCr1Spec>;
#[doc = "Field `PE` reader - Peripheral enable - 0: Peripheral disable - 1: Peripheral enable"]
pub type PeR = crate::BitReader;
#[doc = "Field `PE` writer - Peripheral enable - 0: Peripheral disable - 1: Peripheral enable"]
pub type PeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIE` reader - TX Interrupt enable - 0: Transmit (TXIS) interrupt disabled - 1: Transmit (TXIS) interrupt enabled"]
pub type TxieR = crate::BitReader;
#[doc = "Field `TXIE` writer - TX Interrupt enable - 0: Transmit (TXIS) interrupt disabled - 1: Transmit (TXIS) interrupt enabled"]
pub type TxieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIE` reader - RX Interrupt enable - 0: Receive (RXNE) interrupt disabled - 1: Receive (RXNE) interrupt enabled"]
pub type RxieR = crate::BitReader;
#[doc = "Field `RXIE` writer - RX Interrupt enable - 0: Receive (RXNE) interrupt disabled - 1: Receive (RXNE) interrupt enabled"]
pub type RxieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRIE` reader - Address match Interrupt enable (slave only) - 0: Address match (ADDR) interrupts disabled - 1: Address match (ADDR) interrupts enabled"]
pub type AddrieR = crate::BitReader;
#[doc = "Field `ADDRIE` writer - Address match Interrupt enable (slave only) - 0: Address match (ADDR) interrupts disabled - 1: Address match (ADDR) interrupts enabled"]
pub type AddrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACKIE` reader - Not acknowledge received Interrupt enable - 0: Not acknowledge (NACKF) received interrupts disabled - 1: Not acknowledge (NACKF) received interrupts enabled"]
pub type NackieR = crate::BitReader;
#[doc = "Field `NACKIE` writer - Not acknowledge received Interrupt enable - 0: Not acknowledge (NACKF) received interrupts disabled - 1: Not acknowledge (NACKF) received interrupts enabled"]
pub type NackieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPIE` reader - STOP detection Interrupt enable - 0: Stop detection (STOPF) interrupt disabled - 1: Stop detection (STOPF) interrupt enabled"]
pub type StopieR = crate::BitReader;
#[doc = "Field `STOPIE` writer - STOP detection Interrupt enable - 0: Stop detection (STOPF) interrupt disabled - 1: Stop detection (STOPF) interrupt enabled"]
pub type StopieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIE` reader - Transfer Complete interrupt enable - 0: Transfer Complete interrupt disabled - 1: Transfer Complete interrupt enabled"]
pub type TcieR = crate::BitReader;
#[doc = "Field `TCIE` writer - Transfer Complete interrupt enable - 0: Transfer Complete interrupt disabled - 1: Transfer Complete interrupt enabled"]
pub type TcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - Error interrupts enable - 0: Error detection interrupts disabled - 1: Error detection interrupts enabled Note: Any of these errors generate an interrupt: Arbitration Loss (ARLO) Bus Error detection (BERR) Overrun/Underrun (OVR) Timeout detection (TIMEOUT) PEC error detection (PECERR) Alert pin event detection (ALERT)"]
pub type ErrieR = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupts enable - 0: Error detection interrupts disabled - 1: Error detection interrupts enabled Note: Any of these errors generate an interrupt: Arbitration Loss (ARLO) Bus Error detection (BERR) Overrun/Underrun (OVR) Timeout detection (TIMEOUT) PEC error detection (PECERR) Alert pin event detection (ALERT)"]
pub type ErrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DNF` reader - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter will filter spikes with a length of up to DNF\\[3:0\\] * tI2CCLK - 0000: Digital filter disabled - 0001: Digital filter enabled and filtering capability up to 1 tI2CCLK - 1111: digital filter enabled and filtering capability up to15 tI2CCLK"]
pub type DnfR = crate::FieldReader;
#[doc = "Field `DNF` writer - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter will filter spikes with a length of up to DNF\\[3:0\\] * tI2CCLK - 0000: Digital filter disabled - 0001: Digital filter enabled and filtering capability up to 1 tI2CCLK - 1111: digital filter enabled and filtering capability up to15 tI2CCLK"]
pub type DnfW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ANFOFF` reader - Analog noise filter OFF - 0: Analog noise filter enabled - 1: Analog noise filter disabled"]
pub type AnfoffR = crate::BitReader;
#[doc = "Field `ANFOFF` writer - Analog noise filter OFF - 0: Analog noise filter enabled - 1: Analog noise filter disabled"]
pub type AnfoffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMAEN` reader - DMA transmission requests enable - 0: DMA mode disabled for transmission - 1: DMA mode enabled for transmission"]
pub type TxdmaenR = crate::BitReader;
#[doc = "Field `TXDMAEN` writer - DMA transmission requests enable - 0: DMA mode disabled for transmission - 1: DMA mode enabled for transmission"]
pub type TxdmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDMAEN` reader - DMA reception requests enable - 0: DMA mode disabled for reception - 1: DMA mode enabled for reception"]
pub type RxdmaenR = crate::BitReader;
#[doc = "Field `SBC` reader - Slave byte control This bit is used to enable hardware byte control in slave mode. - 0: Slave byte control disabled - 1: Slave byte control enabled"]
pub type SbcR = crate::BitReader;
#[doc = "Field `SBC` writer - Slave byte control This bit is used to enable hardware byte control in slave mode. - 0: Slave byte control disabled - 1: Slave byte control enabled"]
pub type SbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOSTRETCH` reader - Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. - 0: Clock stretching enabled - 1: Clock stretching disabled Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
pub type NostretchR = crate::BitReader;
#[doc = "Field `NOSTRETCH` writer - Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. - 0: Clock stretching enabled - 1: Clock stretching disabled Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
pub type NostretchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GCEN` reader - General call enable - 0: General call disabled. Address 0b00000000 is NACKed. - 1: General call enabled. Address 0b00000000 is ACKed."]
pub type GcenR = crate::BitReader;
#[doc = "Field `GCEN` writer - General call enable - 0: General call disabled. Address 0b00000000 is NACKed. - 1: General call enabled. Address 0b00000000 is ACKed."]
pub type GcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBHEN` reader - SMBus Host address enable - 0: Host address disabled. Address 0b0001000x is NACKed. - 1: Host address enabled. Address 0b0001000x is ACKed."]
pub type SmbhenR = crate::BitReader;
#[doc = "Field `SMBHEN` writer - SMBus Host address enable - 0: Host address disabled. Address 0b0001000x is NACKed. - 1: Host address enabled. Address 0b0001000x is ACKed."]
pub type SmbhenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBDEN` reader - SMBus Device Default address enable - 0: Device default address disabled. Address 0b1100001x is NACKed. - 1: Device default address enabled. Address 0b1100001x is ACKed."]
pub type SmbdenR = crate::BitReader;
#[doc = "Field `SMBDEN` writer - SMBus Device Default address enable - 0: Device default address disabled. Address 0b1100001x is NACKed. - 1: Device default address enabled. Address 0b1100001x is ACKed."]
pub type SmbdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALERTEN` reader - SMBus alert enable Device mode (SMBHEN=0): - 0: Releases SMBA pin high and Alert Response Address Header disabled: 0001100x followed by NACK. - 1: Drives SMBA pin low and Alert Response Address Header enables: 0001100x followed by ACK. Host mode (SMBHEN=1): - 0: SMBus Alert pin (SMBA) not supported. - 1: SMBus Alert pin (SMBA) supported."]
pub type AlertenR = crate::BitReader;
#[doc = "Field `ALERTEN` writer - SMBus alert enable Device mode (SMBHEN=0): - 0: Releases SMBA pin high and Alert Response Address Header disabled: 0001100x followed by NACK. - 1: Drives SMBA pin low and Alert Response Address Header enables: 0001100x followed by ACK. Host mode (SMBHEN=1): - 0: SMBus Alert pin (SMBA) not supported. - 1: SMBus Alert pin (SMBA) supported."]
pub type AlertenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECEN` reader - PEC enable - 0: PEC calculation disabled - 1: PEC calculation enabled"]
pub type PecenR = crate::BitReader;
#[doc = "Field `PECEN` writer - PEC enable - 0: PEC calculation disabled - 1: PEC calculation enabled"]
pub type PecenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Peripheral enable - 0: Peripheral disable - 1: Peripheral enable"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX Interrupt enable - 0: Transmit (TXIS) interrupt disabled - 1: Transmit (TXIS) interrupt enabled"]
    #[inline(always)]
    pub fn txie(&self) -> TxieR {
        TxieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Interrupt enable - 0: Receive (RXNE) interrupt disabled - 1: Receive (RXNE) interrupt enabled"]
    #[inline(always)]
    pub fn rxie(&self) -> RxieR {
        RxieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Address match Interrupt enable (slave only) - 0: Address match (ADDR) interrupts disabled - 1: Address match (ADDR) interrupts enabled"]
    #[inline(always)]
    pub fn addrie(&self) -> AddrieR {
        AddrieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Not acknowledge received Interrupt enable - 0: Not acknowledge (NACKF) received interrupts disabled - 1: Not acknowledge (NACKF) received interrupts enabled"]
    #[inline(always)]
    pub fn nackie(&self) -> NackieR {
        NackieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - STOP detection Interrupt enable - 0: Stop detection (STOPF) interrupt disabled - 1: Stop detection (STOPF) interrupt enabled"]
    #[inline(always)]
    pub fn stopie(&self) -> StopieR {
        StopieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transfer Complete interrupt enable - 0: Transfer Complete interrupt disabled - 1: Transfer Complete interrupt enabled"]
    #[inline(always)]
    pub fn tcie(&self) -> TcieR {
        TcieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Error interrupts enable - 0: Error detection interrupts disabled - 1: Error detection interrupts enabled Note: Any of these errors generate an interrupt: Arbitration Loss (ARLO) Bus Error detection (BERR) Overrun/Underrun (OVR) Timeout detection (TIMEOUT) PEC error detection (PECERR) Alert pin event detection (ALERT)"]
    #[inline(always)]
    pub fn errie(&self) -> ErrieR {
        ErrieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter will filter spikes with a length of up to DNF\\[3:0\\] * tI2CCLK - 0000: Digital filter disabled - 0001: Digital filter enabled and filtering capability up to 1 tI2CCLK - 1111: digital filter enabled and filtering capability up to15 tI2CCLK"]
    #[inline(always)]
    pub fn dnf(&self) -> DnfR {
        DnfR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Analog noise filter OFF - 0: Analog noise filter enabled - 1: Analog noise filter disabled"]
    #[inline(always)]
    pub fn anfoff(&self) -> AnfoffR {
        AnfoffR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - DMA transmission requests enable - 0: DMA mode disabled for transmission - 1: DMA mode enabled for transmission"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TxdmaenR {
        TxdmaenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DMA reception requests enable - 0: DMA mode disabled for reception - 1: DMA mode enabled for reception"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RxdmaenR {
        RxdmaenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave byte control This bit is used to enable hardware byte control in slave mode. - 0: Slave byte control disabled - 1: Slave byte control enabled"]
    #[inline(always)]
    pub fn sbc(&self) -> SbcR {
        SbcR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. - 0: Clock stretching enabled - 1: Clock stretching disabled Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
    #[inline(always)]
    pub fn nostretch(&self) -> NostretchR {
        NostretchR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - General call enable - 0: General call disabled. Address 0b00000000 is NACKed. - 1: General call enabled. Address 0b00000000 is ACKed."]
    #[inline(always)]
    pub fn gcen(&self) -> GcenR {
        GcenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SMBus Host address enable - 0: Host address disabled. Address 0b0001000x is NACKed. - 1: Host address enabled. Address 0b0001000x is ACKed."]
    #[inline(always)]
    pub fn smbhen(&self) -> SmbhenR {
        SmbhenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SMBus Device Default address enable - 0: Device default address disabled. Address 0b1100001x is NACKed. - 1: Device default address enabled. Address 0b1100001x is ACKed."]
    #[inline(always)]
    pub fn smbden(&self) -> SmbdenR {
        SmbdenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SMBus alert enable Device mode (SMBHEN=0): - 0: Releases SMBA pin high and Alert Response Address Header disabled: 0001100x followed by NACK. - 1: Drives SMBA pin low and Alert Response Address Header enables: 0001100x followed by ACK. Host mode (SMBHEN=1): - 0: SMBus Alert pin (SMBA) not supported. - 1: SMBus Alert pin (SMBA) supported."]
    #[inline(always)]
    pub fn alerten(&self) -> AlertenR {
        AlertenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PEC enable - 0: PEC calculation disabled - 1: PEC calculation enabled"]
    #[inline(always)]
    pub fn pecen(&self) -> PecenR {
        PecenR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral enable - 0: Peripheral disable - 1: Peripheral enable"]
    #[inline(always)]
    pub fn pe(&mut self) -> PeW<'_, I2cCr1Spec> {
        PeW::new(self, 0)
    }
    #[doc = "Bit 1 - TX Interrupt enable - 0: Transmit (TXIS) interrupt disabled - 1: Transmit (TXIS) interrupt enabled"]
    #[inline(always)]
    pub fn txie(&mut self) -> TxieW<'_, I2cCr1Spec> {
        TxieW::new(self, 1)
    }
    #[doc = "Bit 2 - RX Interrupt enable - 0: Receive (RXNE) interrupt disabled - 1: Receive (RXNE) interrupt enabled"]
    #[inline(always)]
    pub fn rxie(&mut self) -> RxieW<'_, I2cCr1Spec> {
        RxieW::new(self, 2)
    }
    #[doc = "Bit 3 - Address match Interrupt enable (slave only) - 0: Address match (ADDR) interrupts disabled - 1: Address match (ADDR) interrupts enabled"]
    #[inline(always)]
    pub fn addrie(&mut self) -> AddrieW<'_, I2cCr1Spec> {
        AddrieW::new(self, 3)
    }
    #[doc = "Bit 4 - Not acknowledge received Interrupt enable - 0: Not acknowledge (NACKF) received interrupts disabled - 1: Not acknowledge (NACKF) received interrupts enabled"]
    #[inline(always)]
    pub fn nackie(&mut self) -> NackieW<'_, I2cCr1Spec> {
        NackieW::new(self, 4)
    }
    #[doc = "Bit 5 - STOP detection Interrupt enable - 0: Stop detection (STOPF) interrupt disabled - 1: Stop detection (STOPF) interrupt enabled"]
    #[inline(always)]
    pub fn stopie(&mut self) -> StopieW<'_, I2cCr1Spec> {
        StopieW::new(self, 5)
    }
    #[doc = "Bit 6 - Transfer Complete interrupt enable - 0: Transfer Complete interrupt disabled - 1: Transfer Complete interrupt enabled"]
    #[inline(always)]
    pub fn tcie(&mut self) -> TcieW<'_, I2cCr1Spec> {
        TcieW::new(self, 6)
    }
    #[doc = "Bit 7 - Error interrupts enable - 0: Error detection interrupts disabled - 1: Error detection interrupts enabled Note: Any of these errors generate an interrupt: Arbitration Loss (ARLO) Bus Error detection (BERR) Overrun/Underrun (OVR) Timeout detection (TIMEOUT) PEC error detection (PECERR) Alert pin event detection (ALERT)"]
    #[inline(always)]
    pub fn errie(&mut self) -> ErrieW<'_, I2cCr1Spec> {
        ErrieW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter will filter spikes with a length of up to DNF\\[3:0\\] * tI2CCLK - 0000: Digital filter disabled - 0001: Digital filter enabled and filtering capability up to 1 tI2CCLK - 1111: digital filter enabled and filtering capability up to15 tI2CCLK"]
    #[inline(always)]
    pub fn dnf(&mut self) -> DnfW<'_, I2cCr1Spec> {
        DnfW::new(self, 8)
    }
    #[doc = "Bit 12 - Analog noise filter OFF - 0: Analog noise filter enabled - 1: Analog noise filter disabled"]
    #[inline(always)]
    pub fn anfoff(&mut self) -> AnfoffW<'_, I2cCr1Spec> {
        AnfoffW::new(self, 12)
    }
    #[doc = "Bit 14 - DMA transmission requests enable - 0: DMA mode disabled for transmission - 1: DMA mode enabled for transmission"]
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TxdmaenW<'_, I2cCr1Spec> {
        TxdmaenW::new(self, 14)
    }
    #[doc = "Bit 16 - Slave byte control This bit is used to enable hardware byte control in slave mode. - 0: Slave byte control disabled - 1: Slave byte control enabled"]
    #[inline(always)]
    pub fn sbc(&mut self) -> SbcW<'_, I2cCr1Spec> {
        SbcW::new(self, 16)
    }
    #[doc = "Bit 17 - Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. - 0: Clock stretching enabled - 1: Clock stretching disabled Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
    #[inline(always)]
    pub fn nostretch(&mut self) -> NostretchW<'_, I2cCr1Spec> {
        NostretchW::new(self, 17)
    }
    #[doc = "Bit 19 - General call enable - 0: General call disabled. Address 0b00000000 is NACKed. - 1: General call enabled. Address 0b00000000 is ACKed."]
    #[inline(always)]
    pub fn gcen(&mut self) -> GcenW<'_, I2cCr1Spec> {
        GcenW::new(self, 19)
    }
    #[doc = "Bit 20 - SMBus Host address enable - 0: Host address disabled. Address 0b0001000x is NACKed. - 1: Host address enabled. Address 0b0001000x is ACKed."]
    #[inline(always)]
    pub fn smbhen(&mut self) -> SmbhenW<'_, I2cCr1Spec> {
        SmbhenW::new(self, 20)
    }
    #[doc = "Bit 21 - SMBus Device Default address enable - 0: Device default address disabled. Address 0b1100001x is NACKed. - 1: Device default address enabled. Address 0b1100001x is ACKed."]
    #[inline(always)]
    pub fn smbden(&mut self) -> SmbdenW<'_, I2cCr1Spec> {
        SmbdenW::new(self, 21)
    }
    #[doc = "Bit 22 - SMBus alert enable Device mode (SMBHEN=0): - 0: Releases SMBA pin high and Alert Response Address Header disabled: 0001100x followed by NACK. - 1: Drives SMBA pin low and Alert Response Address Header enables: 0001100x followed by ACK. Host mode (SMBHEN=1): - 0: SMBus Alert pin (SMBA) not supported. - 1: SMBus Alert pin (SMBA) supported."]
    #[inline(always)]
    pub fn alerten(&mut self) -> AlertenW<'_, I2cCr1Spec> {
        AlertenW::new(self, 22)
    }
    #[doc = "Bit 23 - PEC enable - 0: PEC calculation disabled - 1: PEC calculation enabled"]
    #[inline(always)]
    pub fn pecen(&mut self) -> PecenW<'_, I2cCr1Spec> {
        PecenW::new(self, 23)
    }
}
#[doc = "I2C_CR1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cCr1Spec;
impl crate::RegisterSpec for I2cCr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_cr1::R`](R) reader structure"]
impl crate::Readable for I2cCr1Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c_cr1::W`](W) writer structure"]
impl crate::Writable for I2cCr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_CR1 to value 0"]
impl crate::Resettable for I2cCr1Spec {}
