#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CfgrSpec>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CfgrSpec>;
#[doc = "Field `HSESEL` reader - Clock source selection request: 0: HSI clock source is requested (default) 1: HSE clock source is requested"]
pub type HseselR = crate::BitReader;
#[doc = "Field `HSESEL` writer - Clock source selection request: 0: HSI clock source is requested (default) 1: HSE clock source is requested"]
pub type HseselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPHSI` reader - Stop HSI clock source request 0: HSI is enabled (default) 1: disable HSI is requested"]
pub type StophsiR = crate::BitReader;
#[doc = "Field `STOPHSI` writer - Stop HSI clock source request 0: HSI is enabled (default) 1: disable HSI is requested"]
pub type StophsiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSESEL_STATUS` reader - Clock source selection Status 0: HSI clock source is selected 1: HSE clock source is selected Mirror the actual system clock source, depending on clock switching mechanism and limitations"]
pub type HseselStatusR = crate::BitReader;
#[doc = "Field `CLKSYSDIV` reader - system clock frequency selection request 000: div1 (HSI 64M / HSE 48M) 001: div2 (HSI 32M / HSE 24M) 010: div4/div3 (HSI/HSE) (16M) 011: div8/div6 (HSI/HSE) (8M) * 100: div16/div12 (HSI/HSE) (4M) * 101: div32/div24 (HSI/HSE) (2M) * 110: div64/div48 (HSI/HSE) (1M) * Note: behavior depends on depending on CFGR.HSESEL and (*) APB2ENR.MRSUBGEN or LPAWUREN register"]
pub type ClksysdivR = crate::FieldReader;
#[doc = "Field `CLKSYSDIV` writer - system clock frequency selection request 000: div1 (HSI 64M / HSE 48M) 001: div2 (HSI 32M / HSE 24M) 010: div4/div3 (HSI/HSE) (16M) 011: div8/div6 (HSI/HSE) (8M) * 100: div16/div12 (HSI/HSE) (4M) * 101: div32/div24 (HSI/HSE) (2M) * 110: div64/div48 (HSI/HSE) (1M) * Note: behavior depends on depending on CFGR.HSESEL and (*) APB2ENR.MRSUBGEN or LPAWUREN register"]
pub type ClksysdivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CLKSYSDIV_STATUS` reader - system clock frequency selection status 000: div1 (HSI 64M / HSE 48M) 001: div2 (HSI 32M / HSE 24M) 010: div4/div3 (HSI/HSE) (16M) 011: div8/div6 (HSI/HSE) (8M) 100: div16/div12 (HSI/HSE) (4M) 101: div32/div24 (HSI/HSE) (2M) 110: div64/div48 (HSI/HSE) (1M) Note: behavior depends on depending on CFGR.HSESEL and APB2ENR.MRSUBGEN register"]
pub type ClksysdivStatusR = crate::FieldReader;
#[doc = "Field `SMPSDIV` reader - SMPS clock prescaling factor to generate 4MHz or 8MHz 0: SMPS clock 8MHz (default ) 1: SMPS clock 4MHz"]
pub type SmpsdivR = crate::BitReader;
#[doc = "Field `SMPSDIV` writer - SMPS clock prescaling factor to generate 4MHz or 8MHz 0: SMPS clock 8MHz (default ) 1: SMPS clock 4MHz"]
pub type SmpsdivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPUCLKSEL` reader - LPUCLKSEL: Selection of LPUART clock 0: 16 MHz peripheral clock (default) 1: LSE clock (Mandatory in LPUART deepstop mode)"]
pub type LpuclkselR = crate::BitReader;
#[doc = "Field `LPUCLKSEL` writer - LPUCLKSEL: Selection of LPUART clock 0: 16 MHz peripheral clock (default) 1: LSE clock (Mandatory in LPUART deepstop mode)"]
pub type LpuclkselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKSLOWSEL` reader - slow clock source selection Set by software to select the clock source. This is no glitch free mechanism Reset source only for this field: PORESETn 00: '0' (default) 01: LSE oscillator clock used as slow clock 10: LSI oscillator clock used as slow clock 11:HSI_64M divided by 2048 used as slow clock"]
pub type ClkslowselR = crate::FieldReader;
#[doc = "Field `CLKSLOWSEL` writer - slow clock source selection Set by software to select the clock source. This is no glitch free mechanism Reset source only for this field: PORESETn 00: '0' (default) 01: LSE oscillator clock used as slow clock 10: LSI oscillator clock used as slow clock 11:HSI_64M divided by 2048 used as slow clock"]
pub type ClkslowselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOBOOSTEN` reader - IOBOOSTEN: IO BOOSTER enable 0: IO BOOSTER block is disabled 1: IO BOOSTER block is enabled."]
pub type IoboostenR = crate::BitReader;
#[doc = "Field `IOBOOSTEN` writer - IOBOOSTEN: IO BOOSTER enable 0: IO BOOSTER block is disabled 1: IO BOOSTER block is enabled."]
pub type IoboostenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCOEN` reader - LCOEN: LCO enable on PA10 also in deepstop. 0: LCO output on PA10 is disabled 1: LCO output on PA10 is enabled."]
pub type LcoenR = crate::BitReader;
#[doc = "Field `LCOEN` writer - LCOEN: LCO enable on PA10 also in deepstop. 0: LCO output on PA10 is disabled 1: LCO output on PA10 is enabled."]
pub type LcoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3I2SCLKSEL` reader - SPI3I2SCLKSEL: Selection of I2S clock for SPI3 IP. 00: 32 MHz peripheral clock (default) 01: 16 MHz peripheral clock 10: CLK_SYS 11: CLK_SYS Note: the I2S clock frequency must be higher or equal to the system clock (configured through RCC_CFGR.CLKSYSDIV\\[2:0\\] bit field)."]
pub type Spi3i2sclkselR = crate::FieldReader;
#[doc = "Field `SPI3I2SCLKSEL` writer - SPI3I2SCLKSEL: Selection of I2S clock for SPI3 IP. 00: 32 MHz peripheral clock (default) 01: 16 MHz peripheral clock 10: CLK_SYS 11: CLK_SYS Note: the I2S clock frequency must be higher or equal to the system clock (configured through RCC_CFGR.CLKSYSDIV\\[2:0\\] bit field)."]
pub type Spi3i2sclkselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LCOSEL` reader - Low speed Configurable Clock Output Selection. Set and reset by software. Glitches propagation possible. Reset source only for this field: PORESETn 00: LCO output disabled, no clock on LCO 01: not used 10: internal 32 KHz (LSI) oscillator clock selected 11: external 32 KHz (LSE) oscillator clock selected"]
pub type LcoselR = crate::FieldReader;
#[doc = "Field `LCOSEL` writer - Low speed Configurable Clock Output Selection. Set and reset by software. Glitches propagation possible. Reset source only for this field: PORESETn 00: LCO output disabled, no clock on LCO 01: not used 10: internal 32 KHz (LSI) oscillator clock selected 11: external 32 KHz (LSE) oscillator clock selected"]
pub type LcoselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MCOSEL` reader - Main Configurable Clock Output Selection. Set and reset by software. Glitches propagation possible. 000: MCO output disabled, no clock on MCO 001: system clock selected 010: na 011: internal RC 64 MHz (HSI) oscillator clock selected 100: external oscillator (HSE) clock selected 101: internal RC 64 MHz (HSI) oscillator divided by 2048 and used as slow clock selected 110: SMPS clock selected 111: AUX ADC ANA clock selected"]
pub type McoselR = crate::FieldReader;
#[doc = "Field `MCOSEL` writer - Main Configurable Clock Output Selection. Set and reset by software. Glitches propagation possible. 000: MCO output disabled, no clock on MCO 001: system clock selected 010: na 011: internal RC 64 MHz (HSI) oscillator clock selected 100: external oscillator (HSE) clock selected 101: internal RC 64 MHz (HSI) oscillator divided by 2048 and used as slow clock selected 110: SMPS clock selected 111: AUX ADC ANA clock selected"]
pub type McoselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CCOPRE` reader - Configurable Clock Output Prescaler. Set and reset by software. Glitches propagation if CCOPRE is modified after CCO output is enabled. 000: CCO clock is divided by 1 001: CCO clock is divided by 2 010: CCO clock is divided by 4 011: CCO clock is divided by 8 100: CCO clock is divided by 16 101: CCO clock is divided by 32 Others: not used"]
pub type CcopreR = crate::FieldReader;
#[doc = "Field `CCOPRE` writer - Configurable Clock Output Prescaler. Set and reset by software. Glitches propagation if CCOPRE is modified after CCO output is enabled. 000: CCO clock is divided by 1 001: CCO clock is divided by 2 010: CCO clock is divided by 4 011: CCO clock is divided by 8 100: CCO clock is divided by 16 101: CCO clock is divided by 32 Others: not used"]
pub type CcopreW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 1 - Clock source selection request: 0: HSI clock source is requested (default) 1: HSE clock source is requested"]
    #[inline(always)]
    pub fn hsesel(&self) -> HseselR {
        HseselR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stop HSI clock source request 0: HSI is enabled (default) 1: disable HSI is requested"]
    #[inline(always)]
    pub fn stophsi(&self) -> StophsiR {
        StophsiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clock source selection Status 0: HSI clock source is selected 1: HSE clock source is selected Mirror the actual system clock source, depending on clock switching mechanism and limitations"]
    #[inline(always)]
    pub fn hsesel_status(&self) -> HseselStatusR {
        HseselStatusR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 5:7 - system clock frequency selection request 000: div1 (HSI 64M / HSE 48M) 001: div2 (HSI 32M / HSE 24M) 010: div4/div3 (HSI/HSE) (16M) 011: div8/div6 (HSI/HSE) (8M) * 100: div16/div12 (HSI/HSE) (4M) * 101: div32/div24 (HSI/HSE) (2M) * 110: div64/div48 (HSI/HSE) (1M) * Note: behavior depends on depending on CFGR.HSESEL and (*) APB2ENR.MRSUBGEN or LPAWUREN register"]
    #[inline(always)]
    pub fn clksysdiv(&self) -> ClksysdivR {
        ClksysdivR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - system clock frequency selection status 000: div1 (HSI 64M / HSE 48M) 001: div2 (HSI 32M / HSE 24M) 010: div4/div3 (HSI/HSE) (16M) 011: div8/div6 (HSI/HSE) (8M) 100: div16/div12 (HSI/HSE) (4M) 101: div32/div24 (HSI/HSE) (2M) 110: div64/div48 (HSI/HSE) (1M) Note: behavior depends on depending on CFGR.HSESEL and APB2ENR.MRSUBGEN register"]
    #[inline(always)]
    pub fn clksysdiv_status(&self) -> ClksysdivStatusR {
        ClksysdivStatusR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - SMPS clock prescaling factor to generate 4MHz or 8MHz 0: SMPS clock 8MHz (default ) 1: SMPS clock 4MHz"]
    #[inline(always)]
    pub fn smpsdiv(&self) -> SmpsdivR {
        SmpsdivR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LPUCLKSEL: Selection of LPUART clock 0: 16 MHz peripheral clock (default) 1: LSE clock (Mandatory in LPUART deepstop mode)"]
    #[inline(always)]
    pub fn lpuclksel(&self) -> LpuclkselR {
        LpuclkselR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 15:16 - slow clock source selection Set by software to select the clock source. This is no glitch free mechanism Reset source only for this field: PORESETn 00: '0' (default) 01: LSE oscillator clock used as slow clock 10: LSI oscillator clock used as slow clock 11:HSI_64M divided by 2048 used as slow clock"]
    #[inline(always)]
    pub fn clkslowsel(&self) -> ClkslowselR {
        ClkslowselR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17 - IOBOOSTEN: IO BOOSTER enable 0: IO BOOSTER block is disabled 1: IO BOOSTER block is enabled."]
    #[inline(always)]
    pub fn ioboosten(&self) -> IoboostenR {
        IoboostenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - LCOEN: LCO enable on PA10 also in deepstop. 0: LCO output on PA10 is disabled 1: LCO output on PA10 is enabled."]
    #[inline(always)]
    pub fn lcoen(&self) -> LcoenR {
        LcoenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 22:23 - SPI3I2SCLKSEL: Selection of I2S clock for SPI3 IP. 00: 32 MHz peripheral clock (default) 01: 16 MHz peripheral clock 10: CLK_SYS 11: CLK_SYS Note: the I2S clock frequency must be higher or equal to the system clock (configured through RCC_CFGR.CLKSYSDIV\\[2:0\\] bit field)."]
    #[inline(always)]
    pub fn spi3i2sclksel(&self) -> Spi3i2sclkselR {
        Spi3i2sclkselR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Low speed Configurable Clock Output Selection. Set and reset by software. Glitches propagation possible. Reset source only for this field: PORESETn 00: LCO output disabled, no clock on LCO 01: not used 10: internal 32 KHz (LSI) oscillator clock selected 11: external 32 KHz (LSE) oscillator clock selected"]
    #[inline(always)]
    pub fn lcosel(&self) -> LcoselR {
        LcoselR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:28 - Main Configurable Clock Output Selection. Set and reset by software. Glitches propagation possible. 000: MCO output disabled, no clock on MCO 001: system clock selected 010: na 011: internal RC 64 MHz (HSI) oscillator clock selected 100: external oscillator (HSE) clock selected 101: internal RC 64 MHz (HSI) oscillator divided by 2048 and used as slow clock selected 110: SMPS clock selected 111: AUX ADC ANA clock selected"]
    #[inline(always)]
    pub fn mcosel(&self) -> McoselR {
        McoselR::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:31 - Configurable Clock Output Prescaler. Set and reset by software. Glitches propagation if CCOPRE is modified after CCO output is enabled. 000: CCO clock is divided by 1 001: CCO clock is divided by 2 010: CCO clock is divided by 4 011: CCO clock is divided by 8 100: CCO clock is divided by 16 101: CCO clock is divided by 32 Others: not used"]
    #[inline(always)]
    pub fn ccopre(&self) -> CcopreR {
        CcopreR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Clock source selection request: 0: HSI clock source is requested (default) 1: HSE clock source is requested"]
    #[inline(always)]
    pub fn hsesel(&mut self) -> HseselW<'_, CfgrSpec> {
        HseselW::new(self, 1)
    }
    #[doc = "Bit 2 - Stop HSI clock source request 0: HSI is enabled (default) 1: disable HSI is requested"]
    #[inline(always)]
    pub fn stophsi(&mut self) -> StophsiW<'_, CfgrSpec> {
        StophsiW::new(self, 2)
    }
    #[doc = "Bits 5:7 - system clock frequency selection request 000: div1 (HSI 64M / HSE 48M) 001: div2 (HSI 32M / HSE 24M) 010: div4/div3 (HSI/HSE) (16M) 011: div8/div6 (HSI/HSE) (8M) * 100: div16/div12 (HSI/HSE) (4M) * 101: div32/div24 (HSI/HSE) (2M) * 110: div64/div48 (HSI/HSE) (1M) * Note: behavior depends on depending on CFGR.HSESEL and (*) APB2ENR.MRSUBGEN or LPAWUREN register"]
    #[inline(always)]
    pub fn clksysdiv(&mut self) -> ClksysdivW<'_, CfgrSpec> {
        ClksysdivW::new(self, 5)
    }
    #[doc = "Bit 12 - SMPS clock prescaling factor to generate 4MHz or 8MHz 0: SMPS clock 8MHz (default ) 1: SMPS clock 4MHz"]
    #[inline(always)]
    pub fn smpsdiv(&mut self) -> SmpsdivW<'_, CfgrSpec> {
        SmpsdivW::new(self, 12)
    }
    #[doc = "Bit 13 - LPUCLKSEL: Selection of LPUART clock 0: 16 MHz peripheral clock (default) 1: LSE clock (Mandatory in LPUART deepstop mode)"]
    #[inline(always)]
    pub fn lpuclksel(&mut self) -> LpuclkselW<'_, CfgrSpec> {
        LpuclkselW::new(self, 13)
    }
    #[doc = "Bits 15:16 - slow clock source selection Set by software to select the clock source. This is no glitch free mechanism Reset source only for this field: PORESETn 00: '0' (default) 01: LSE oscillator clock used as slow clock 10: LSI oscillator clock used as slow clock 11:HSI_64M divided by 2048 used as slow clock"]
    #[inline(always)]
    pub fn clkslowsel(&mut self) -> ClkslowselW<'_, CfgrSpec> {
        ClkslowselW::new(self, 15)
    }
    #[doc = "Bit 17 - IOBOOSTEN: IO BOOSTER enable 0: IO BOOSTER block is disabled 1: IO BOOSTER block is enabled."]
    #[inline(always)]
    pub fn ioboosten(&mut self) -> IoboostenW<'_, CfgrSpec> {
        IoboostenW::new(self, 17)
    }
    #[doc = "Bit 19 - LCOEN: LCO enable on PA10 also in deepstop. 0: LCO output on PA10 is disabled 1: LCO output on PA10 is enabled."]
    #[inline(always)]
    pub fn lcoen(&mut self) -> LcoenW<'_, CfgrSpec> {
        LcoenW::new(self, 19)
    }
    #[doc = "Bits 22:23 - SPI3I2SCLKSEL: Selection of I2S clock for SPI3 IP. 00: 32 MHz peripheral clock (default) 01: 16 MHz peripheral clock 10: CLK_SYS 11: CLK_SYS Note: the I2S clock frequency must be higher or equal to the system clock (configured through RCC_CFGR.CLKSYSDIV\\[2:0\\] bit field)."]
    #[inline(always)]
    pub fn spi3i2sclksel(&mut self) -> Spi3i2sclkselW<'_, CfgrSpec> {
        Spi3i2sclkselW::new(self, 22)
    }
    #[doc = "Bits 24:25 - Low speed Configurable Clock Output Selection. Set and reset by software. Glitches propagation possible. Reset source only for this field: PORESETn 00: LCO output disabled, no clock on LCO 01: not used 10: internal 32 KHz (LSI) oscillator clock selected 11: external 32 KHz (LSE) oscillator clock selected"]
    #[inline(always)]
    pub fn lcosel(&mut self) -> LcoselW<'_, CfgrSpec> {
        LcoselW::new(self, 24)
    }
    #[doc = "Bits 26:28 - Main Configurable Clock Output Selection. Set and reset by software. Glitches propagation possible. 000: MCO output disabled, no clock on MCO 001: system clock selected 010: na 011: internal RC 64 MHz (HSI) oscillator clock selected 100: external oscillator (HSE) clock selected 101: internal RC 64 MHz (HSI) oscillator divided by 2048 and used as slow clock selected 110: SMPS clock selected 111: AUX ADC ANA clock selected"]
    #[inline(always)]
    pub fn mcosel(&mut self) -> McoselW<'_, CfgrSpec> {
        McoselW::new(self, 26)
    }
    #[doc = "Bits 29:31 - Configurable Clock Output Prescaler. Set and reset by software. Glitches propagation if CCOPRE is modified after CCO output is enabled. 000: CCO clock is divided by 1 001: CCO clock is divided by 2 010: CCO clock is divided by 4 011: CCO clock is divided by 8 100: CCO clock is divided by 16 101: CCO clock is divided by 32 Others: not used"]
    #[inline(always)]
    pub fn ccopre(&mut self) -> CcopreW<'_, CfgrSpec> {
        CcopreW::new(self, 29)
    }
}
#[doc = "CFGR register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgrSpec;
impl crate::RegisterSpec for CfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR to value 0x0240"]
impl crate::Resettable for CfgrSpec {
    const RESET_VALUE: u32 = 0x0240;
}
