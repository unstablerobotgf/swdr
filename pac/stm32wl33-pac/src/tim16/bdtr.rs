#[doc = "Register `BDTR` reader"]
pub type R = crate::R<BdtrSpec>;
#[doc = "Register `BDTR` writer"]
pub type W = crate::W<BdtrSpec>;
#[doc = "Field `DTG` reader - DTG\\[7:0\\]: Dead-time generator setup This bit-field defines the duration of the dead-time inserted between the complementary outputs. DT correspond to this duration. DTG\\[7:5\\]=0xx => DT=DTG\\[7:0\\]x tdtg with tdtg=tDTS DTG\\[7:5\\]=10x => DT=(64+DTG\\[5:0\\])xtdtg with Tdtg=2xtDTS DTG\\[7:5\\]=110 => DT=(32+DTG\\[4:0\\])xtdtg with Tdtg=8xtDTS DTG\\[7:5\\]=111 => DT=(32+DTG\\[4:0\\])xtdtg with Tdtg=16xtDTS Example if TDTS=125ns (8MHz), dead-time possible values are: 0 to 15875 ns by 125 ns steps, 16 us to 31750 ns by 250 ns steps, 32 us to 63 us by 1 us steps, 64 us to 126 us by 2 us steps Note: This bit-field can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type DtgR = crate::FieldReader;
#[doc = "Field `DTG` writer - DTG\\[7:0\\]: Dead-time generator setup This bit-field defines the duration of the dead-time inserted between the complementary outputs. DT correspond to this duration. DTG\\[7:5\\]=0xx => DT=DTG\\[7:0\\]x tdtg with tdtg=tDTS DTG\\[7:5\\]=10x => DT=(64+DTG\\[5:0\\])xtdtg with Tdtg=2xtDTS DTG\\[7:5\\]=110 => DT=(32+DTG\\[4:0\\])xtdtg with Tdtg=8xtDTS DTG\\[7:5\\]=111 => DT=(32+DTG\\[4:0\\])xtdtg with Tdtg=16xtDTS Example if TDTS=125ns (8MHz), dead-time possible values are: 0 to 15875 ns by 125 ns steps, 16 us to 31750 ns by 250 ns steps, 32 us to 63 us by 1 us steps, 64 us to 126 us by 2 us steps Note: This bit-field can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type DtgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LOCK` reader - LOCK\\[1:0\\]: Lock configuration These bits offer a write protection against software errors. 00: LOCK OFF - No bit is write protected 01: LOCK Level 1 = DTG bits in TIMx_BDTR register, OISx and OISxN bits in TIMx_CR2 register, BKE/BKP/AOE/BKBID/BKDSRM bits in TIMx_BDTR register and all used bits in TIMx_AF1 register can no longer be written. 10: LOCK Level 2 = LOCK Level 1 + CC Polarity bits (CCxP/CCxNP bits in TIMx_CCER register, as long as the related channel is configured in output through the CCxS bits) as well as OSSR and OSSI bits can no longer be written. 11: LOCK Level 3 = LOCK Level 2 + CC Control bits (OCxM and OCxPE bits in TIMx_CCMRx registers, as long as the related channel is configured in output through the CCxS bits) can no longer be written. Note: The LOCK bits can be written only once after the reset. Once the TIMx_BDTR register has been written, their content is frozen until the next reset."]
pub type LockR = crate::FieldReader;
#[doc = "Field `LOCK` writer - LOCK\\[1:0\\]: Lock configuration These bits offer a write protection against software errors. 00: LOCK OFF - No bit is write protected 01: LOCK Level 1 = DTG bits in TIMx_BDTR register, OISx and OISxN bits in TIMx_CR2 register, BKE/BKP/AOE/BKBID/BKDSRM bits in TIMx_BDTR register and all used bits in TIMx_AF1 register can no longer be written. 10: LOCK Level 2 = LOCK Level 1 + CC Polarity bits (CCxP/CCxNP bits in TIMx_CCER register, as long as the related channel is configured in output through the CCxS bits) as well as OSSR and OSSI bits can no longer be written. 11: LOCK Level 3 = LOCK Level 2 + CC Control bits (OCxM and OCxPE bits in TIMx_CCMRx registers, as long as the related channel is configured in output through the CCxS bits) can no longer be written. Note: The LOCK bits can be written only once after the reset. Once the TIMx_BDTR register has been written, their content is frozen until the next reset."]
pub type LockW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSSI` reader - OSSI: Off-state selection for Idle mode This bit is used when MOE=0 on channels configured as outputs. See OC/OCN enable description for more details (Section 22.4.8: TIM16 capture/compare enable register (TIMx_CCER)). 0: When inactive, OC/OCN outputs are disabled (OC/OCN enable output signal=0) 1: When inactive, OC/OCN outputs are forced first with their idle level as soon as CCxE=1 or CCxNE=1. OC/OCN enable output signal=1) Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OssiR = crate::BitReader;
#[doc = "Field `OSSI` writer - OSSI: Off-state selection for Idle mode This bit is used when MOE=0 on channels configured as outputs. See OC/OCN enable description for more details (Section 22.4.8: TIM16 capture/compare enable register (TIMx_CCER)). 0: When inactive, OC/OCN outputs are disabled (OC/OCN enable output signal=0) 1: When inactive, OC/OCN outputs are forced first with their idle level as soon as CCxE=1 or CCxNE=1. OC/OCN enable output signal=1) Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OssiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSR` reader - OSSR: Off-state selection for Run mode This bit is used when MOE=1 on channels that have a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. See OC/OCN enable description for more details (Section 22.4.8: TIM16 capture/compare enable register (TIMx_CCER)). 0: When inactive, OC/OCN outputs are disabled (the timer releases the output control which is taken over by the AFIO logic, which forces a Hi-Z state) 1: When inactive, OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1 (the output is still controlled by the timer). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OssrR = crate::BitReader;
#[doc = "Field `OSSR` writer - OSSR: Off-state selection for Run mode This bit is used when MOE=1 on channels that have a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. See OC/OCN enable description for more details (Section 22.4.8: TIM16 capture/compare enable register (TIMx_CCER)). 0: When inactive, OC/OCN outputs are disabled (the timer releases the output control which is taken over by the AFIO logic, which forces a Hi-Z state) 1: When inactive, OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1 (the output is still controlled by the timer). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OssrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "BKE: Break enable 1; Break inputs (BRK) enabled Note: 1. This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: 2. Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bke {
    #[doc = "0: Break inputs (BRK) disabled"]
    B0x0 = 0,
}
impl From<Bke> for bool {
    #[inline(always)]
    fn from(variant: Bke) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKE` reader - BKE: Break enable 1; Break inputs (BRK) enabled Note: 1. This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: 2. Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BkeR = crate::BitReader<Bke>;
impl BkeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bke> {
        match self.bits {
            false => Some(Bke::B0x0),
            _ => None,
        }
    }
    #[doc = "Break inputs (BRK) disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Bke::B0x0
    }
}
#[doc = "Field `BKE` writer - BKE: Break enable 1; Break inputs (BRK) enabled Note: 1. This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: 2. Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BkeW<'a, REG> = crate::BitWriter<'a, REG, Bke>;
impl<'a, REG> BkeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break inputs (BRK) disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Bke::B0x0)
    }
}
#[doc = "Field `BKP` reader - BKP: Break polarity 0: Break input BRK is active low. 1: Break input BRK is active high Note: 1. This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: 2. Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BkpR = crate::BitReader;
#[doc = "Field `BKP` writer - BKP: Break polarity 0: Break input BRK is active low. 1: Break input BRK is active high Note: 1. This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: 2. Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BkpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "AOE: Automatic output enable not be active) Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aoe {
    #[doc = "0: MOE can be set only by software"]
    B0x0 = 0,
    #[doc = "1: MOE can be set by software or automatically at the next update event (if the break input is"]
    B0x1 = 1,
}
impl From<Aoe> for bool {
    #[inline(always)]
    fn from(variant: Aoe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AOE` reader - AOE: Automatic output enable not be active) Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type AoeR = crate::BitReader<Aoe>;
impl AoeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aoe {
        match self.bits {
            false => Aoe::B0x0,
            true => Aoe::B0x1,
        }
    }
    #[doc = "MOE can be set only by software"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == Aoe::B0x0
    }
    #[doc = "MOE can be set by software or automatically at the next update event (if the break input is"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == Aoe::B0x1
    }
}
#[doc = "Field `AOE` writer - AOE: Automatic output enable not be active) Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type AoeW<'a, REG> = crate::BitWriter<'a, REG, Aoe>;
impl<'a, REG> AoeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MOE can be set only by software"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Aoe::B0x0)
    }
    #[doc = "MOE can be set by software or automatically at the next update event (if the break input is"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Aoe::B0x1)
    }
}
#[doc = "Field `MOE` reader - MOE: Main output enable This bit is cleared asynchronously by hardware as soon as the break input is active. It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. 0: OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit. 1: OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in TIMx_CCER register) See OC/OCN enable description for more details (Section 22.4.8: TIM16 capture/compare enable register (TIMx_CCER))."]
pub type MoeR = crate::BitReader;
#[doc = "Field `MOE` writer - MOE: Main output enable This bit is cleared asynchronously by hardware as soon as the break input is active. It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. 0: OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit. 1: OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in TIMx_CCER register) See OC/OCN enable description for more details (Section 22.4.8: TIM16 capture/compare enable register (TIMx_CCER))."]
pub type MoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKDSRM` reader - BKDSRM: Break Disarm 0: Break input BRK is armed 1: Break input BRK is disarmed This bit is cleared by hardware when no break source is active. The BKDSRM bit must be set by software to release the bidirectional output control (opendrain output in Hi-Z state) and then be polled it until it is reset by hardware, indicating that the fault condition has disappeared. Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BkdsrmR = crate::BitReader;
#[doc = "Field `BKDSRM` writer - BKDSRM: Break Disarm 0: Break input BRK is armed 1: Break input BRK is disarmed This bit is cleared by hardware when no break source is active. The BKDSRM bit must be set by software to release the bidirectional output control (opendrain output in Hi-Z state) and then be polled it until it is reset by hardware, indicating that the fault condition has disappeared. Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BkdsrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKBID` reader - BKBID: Break Bidirectional 0: Break input BRK in input mode 1: Break input BRK in bidirectional mode In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input mode and in open drain output mode. Any active break event asserts a low logic level on the Break input to indicate an internal break event to external devices. Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BkbidR = crate::BitReader;
#[doc = "Field `BKBID` writer - BKBID: Break Bidirectional 0: Break input BRK in input mode 1: Break input BRK in bidirectional mode In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input mode and in open drain output mode. Any active break event asserts a low logic level on the Break input to indicate an internal break event to external devices. Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BkbidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - DTG\\[7:0\\]: Dead-time generator setup This bit-field defines the duration of the dead-time inserted between the complementary outputs. DT correspond to this duration. DTG\\[7:5\\]=0xx => DT=DTG\\[7:0\\]x tdtg with tdtg=tDTS DTG\\[7:5\\]=10x => DT=(64+DTG\\[5:0\\])xtdtg with Tdtg=2xtDTS DTG\\[7:5\\]=110 => DT=(32+DTG\\[4:0\\])xtdtg with Tdtg=8xtDTS DTG\\[7:5\\]=111 => DT=(32+DTG\\[4:0\\])xtdtg with Tdtg=16xtDTS Example if TDTS=125ns (8MHz), dead-time possible values are: 0 to 15875 ns by 125 ns steps, 16 us to 31750 ns by 250 ns steps, 32 us to 63 us by 1 us steps, 64 us to 126 us by 2 us steps Note: This bit-field can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn dtg(&self) -> DtgR {
        DtgR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - LOCK\\[1:0\\]: Lock configuration These bits offer a write protection against software errors. 00: LOCK OFF - No bit is write protected 01: LOCK Level 1 = DTG bits in TIMx_BDTR register, OISx and OISxN bits in TIMx_CR2 register, BKE/BKP/AOE/BKBID/BKDSRM bits in TIMx_BDTR register and all used bits in TIMx_AF1 register can no longer be written. 10: LOCK Level 2 = LOCK Level 1 + CC Polarity bits (CCxP/CCxNP bits in TIMx_CCER register, as long as the related channel is configured in output through the CCxS bits) as well as OSSR and OSSI bits can no longer be written. 11: LOCK Level 3 = LOCK Level 2 + CC Control bits (OCxM and OCxPE bits in TIMx_CCMRx registers, as long as the related channel is configured in output through the CCxS bits) can no longer be written. Note: The LOCK bits can be written only once after the reset. Once the TIMx_BDTR register has been written, their content is frozen until the next reset."]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - OSSI: Off-state selection for Idle mode This bit is used when MOE=0 on channels configured as outputs. See OC/OCN enable description for more details (Section 22.4.8: TIM16 capture/compare enable register (TIMx_CCER)). 0: When inactive, OC/OCN outputs are disabled (OC/OCN enable output signal=0) 1: When inactive, OC/OCN outputs are forced first with their idle level as soon as CCxE=1 or CCxNE=1. OC/OCN enable output signal=1) Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn ossi(&self) -> OssiR {
        OssiR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - OSSR: Off-state selection for Run mode This bit is used when MOE=1 on channels that have a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. See OC/OCN enable description for more details (Section 22.4.8: TIM16 capture/compare enable register (TIMx_CCER)). 0: When inactive, OC/OCN outputs are disabled (the timer releases the output control which is taken over by the AFIO logic, which forces a Hi-Z state) 1: When inactive, OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1 (the output is still controlled by the timer). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn ossr(&self) -> OssrR {
        OssrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BKE: Break enable 1; Break inputs (BRK) enabled Note: 1. This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: 2. Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn bke(&self) -> BkeR {
        BkeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - BKP: Break polarity 0: Break input BRK is active low. 1: Break input BRK is active high Note: 1. This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: 2. Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn bkp(&self) -> BkpR {
        BkpR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - AOE: Automatic output enable not be active) Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn aoe(&self) -> AoeR {
        AoeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MOE: Main output enable This bit is cleared asynchronously by hardware as soon as the break input is active. It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. 0: OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit. 1: OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in TIMx_CCER register) See OC/OCN enable description for more details (Section 22.4.8: TIM16 capture/compare enable register (TIMx_CCER))."]
    #[inline(always)]
    pub fn moe(&self) -> MoeR {
        MoeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 26 - BKDSRM: Break Disarm 0: Break input BRK is armed 1: Break input BRK is disarmed This bit is cleared by hardware when no break source is active. The BKDSRM bit must be set by software to release the bidirectional output control (opendrain output in Hi-Z state) and then be polled it until it is reset by hardware, indicating that the fault condition has disappeared. Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn bkdsrm(&self) -> BkdsrmR {
        BkdsrmR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - BKBID: Break Bidirectional 0: Break input BRK in input mode 1: Break input BRK in bidirectional mode In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input mode and in open drain output mode. Any active break event asserts a low logic level on the Break input to indicate an internal break event to external devices. Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn bkbid(&self) -> BkbidR {
        BkbidR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - DTG\\[7:0\\]: Dead-time generator setup This bit-field defines the duration of the dead-time inserted between the complementary outputs. DT correspond to this duration. DTG\\[7:5\\]=0xx => DT=DTG\\[7:0\\]x tdtg with tdtg=tDTS DTG\\[7:5\\]=10x => DT=(64+DTG\\[5:0\\])xtdtg with Tdtg=2xtDTS DTG\\[7:5\\]=110 => DT=(32+DTG\\[4:0\\])xtdtg with Tdtg=8xtDTS DTG\\[7:5\\]=111 => DT=(32+DTG\\[4:0\\])xtdtg with Tdtg=16xtDTS Example if TDTS=125ns (8MHz), dead-time possible values are: 0 to 15875 ns by 125 ns steps, 16 us to 31750 ns by 250 ns steps, 32 us to 63 us by 1 us steps, 64 us to 126 us by 2 us steps Note: This bit-field can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn dtg(&mut self) -> DtgW<'_, BdtrSpec> {
        DtgW::new(self, 0)
    }
    #[doc = "Bits 8:9 - LOCK\\[1:0\\]: Lock configuration These bits offer a write protection against software errors. 00: LOCK OFF - No bit is write protected 01: LOCK Level 1 = DTG bits in TIMx_BDTR register, OISx and OISxN bits in TIMx_CR2 register, BKE/BKP/AOE/BKBID/BKDSRM bits in TIMx_BDTR register and all used bits in TIMx_AF1 register can no longer be written. 10: LOCK Level 2 = LOCK Level 1 + CC Polarity bits (CCxP/CCxNP bits in TIMx_CCER register, as long as the related channel is configured in output through the CCxS bits) as well as OSSR and OSSI bits can no longer be written. 11: LOCK Level 3 = LOCK Level 2 + CC Control bits (OCxM and OCxPE bits in TIMx_CCMRx registers, as long as the related channel is configured in output through the CCxS bits) can no longer be written. Note: The LOCK bits can be written only once after the reset. Once the TIMx_BDTR register has been written, their content is frozen until the next reset."]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, BdtrSpec> {
        LockW::new(self, 8)
    }
    #[doc = "Bit 10 - OSSI: Off-state selection for Idle mode This bit is used when MOE=0 on channels configured as outputs. See OC/OCN enable description for more details (Section 22.4.8: TIM16 capture/compare enable register (TIMx_CCER)). 0: When inactive, OC/OCN outputs are disabled (OC/OCN enable output signal=0) 1: When inactive, OC/OCN outputs are forced first with their idle level as soon as CCxE=1 or CCxNE=1. OC/OCN enable output signal=1) Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn ossi(&mut self) -> OssiW<'_, BdtrSpec> {
        OssiW::new(self, 10)
    }
    #[doc = "Bit 11 - OSSR: Off-state selection for Run mode This bit is used when MOE=1 on channels that have a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. See OC/OCN enable description for more details (Section 22.4.8: TIM16 capture/compare enable register (TIMx_CCER)). 0: When inactive, OC/OCN outputs are disabled (the timer releases the output control which is taken over by the AFIO logic, which forces a Hi-Z state) 1: When inactive, OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1 (the output is still controlled by the timer). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn ossr(&mut self) -> OssrW<'_, BdtrSpec> {
        OssrW::new(self, 11)
    }
    #[doc = "Bit 12 - BKE: Break enable 1; Break inputs (BRK) enabled Note: 1. This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: 2. Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn bke(&mut self) -> BkeW<'_, BdtrSpec> {
        BkeW::new(self, 12)
    }
    #[doc = "Bit 13 - BKP: Break polarity 0: Break input BRK is active low. 1: Break input BRK is active high Note: 1. This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: 2. Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn bkp(&mut self) -> BkpW<'_, BdtrSpec> {
        BkpW::new(self, 13)
    }
    #[doc = "Bit 14 - AOE: Automatic output enable not be active) Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn aoe(&mut self) -> AoeW<'_, BdtrSpec> {
        AoeW::new(self, 14)
    }
    #[doc = "Bit 15 - MOE: Main output enable This bit is cleared asynchronously by hardware as soon as the break input is active. It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. 0: OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit. 1: OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in TIMx_CCER register) See OC/OCN enable description for more details (Section 22.4.8: TIM16 capture/compare enable register (TIMx_CCER))."]
    #[inline(always)]
    pub fn moe(&mut self) -> MoeW<'_, BdtrSpec> {
        MoeW::new(self, 15)
    }
    #[doc = "Bit 26 - BKDSRM: Break Disarm 0: Break input BRK is armed 1: Break input BRK is disarmed This bit is cleared by hardware when no break source is active. The BKDSRM bit must be set by software to release the bidirectional output control (opendrain output in Hi-Z state) and then be polled it until it is reset by hardware, indicating that the fault condition has disappeared. Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn bkdsrm(&mut self) -> BkdsrmW<'_, BdtrSpec> {
        BkdsrmW::new(self, 26)
    }
    #[doc = "Bit 28 - BKBID: Break Bidirectional 0: Break input BRK in input mode 1: Break input BRK in bidirectional mode In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input mode and in open drain output mode. Any active break event asserts a low logic level on the Break input to indicate an internal break event to external devices. Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn bkbid(&mut self) -> BkbidW<'_, BdtrSpec> {
        BkbidW::new(self, 28)
    }
}
#[doc = "BDTR register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BdtrSpec;
impl crate::RegisterSpec for BdtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdtr::R`](R) reader structure"]
impl crate::Readable for BdtrSpec {}
#[doc = "`write(|w| ..)` method takes [`bdtr::W`](W) writer structure"]
impl crate::Writable for BdtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BDTR to value 0"]
impl crate::Resettable for BdtrSpec {}
