#[doc = "Register `DBGSMPS` reader"]
pub type R = crate::R<DbgsmpsSpec>;
#[doc = "Register `DBGSMPS` writer"]
pub type W = crate::W<DbgsmpsSpec>;
#[doc = "Field `TESTDIG` reader - TESTDIG: SMPS TEST_DIG_3V3\\[3:0\\] SMPS control signal"]
pub type TestdigR = crate::FieldReader;
#[doc = "Field `TESTDIG` writer - TESTDIG: SMPS TEST_DIG_3V3\\[3:0\\] SMPS control signal"]
pub type TestdigW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TESTKEL` reader - TESTKEL: SMPS TEST_KEL_3V3\\[1:0\\] SMPS control signal"]
pub type TestkelR = crate::FieldReader;
#[doc = "Field `TESTKEL` writer - TESTKEL: SMPS TEST_KEL_3V3\\[1:0\\] SMPS control signal"]
pub type TestkelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HOT_STUP` reader - HOT_STUP_3V3 SMPS control signal"]
pub type HotStupR = crate::BitReader;
#[doc = "Field `HOT_STUP` writer - HOT_STUP_3V3 SMPS control signal"]
pub type HotStupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_STUP` reader - NO_STUP_3V3 SMPS control signal"]
pub type NoStupR = crate::BitReader;
#[doc = "Field `NO_STUP` writer - NO_STUP_3V3 SMPS control signal"]
pub type NoStupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TESTILIM` reader - TESTILIM: SMPS TEST_ILIM_3V3 SMPS control signal"]
pub type TestilimR = crate::BitReader;
#[doc = "Field `TESTILIM` writer - TESTILIM: SMPS TEST_ILIM_3V3 SMPS control signal"]
pub type TestilimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTLRES_RAMP` reader - CTLRES_RAM_3V3 SMPS control signal"]
pub type CtlresRampR = crate::BitReader;
#[doc = "Field `CTLRES_RAMP` writer - CTLRES_RAM_3V3 SMPS control signal"]
pub type CtlresRampW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS_BIG_MOS` reader - DIS_BIG_MOS_3V3 SMPS control signal"]
pub type DisBigMosR = crate::BitReader;
#[doc = "Field `DIS_BIG_MOS` writer - DIS_BIG_MOS_3V3 SMPS control signal"]
pub type DisBigMosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEST_OL` reader - TEST_OL_3V3 SMPS control signal"]
pub type TestOlR = crate::BitReader;
#[doc = "Field `TEST_OL` writer - TEST_OL_3V3 SMPS control signal"]
pub type TestOlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS_ILIM` reader - DIS_ILIM_3V3 SMPS control signal"]
pub type DisIlimR = crate::BitReader;
#[doc = "Field `DIS_ILIM` writer - DIS_ILIM_3V3 SMPS control signal"]
pub type DisIlimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ILIM_BOOST` reader - ILIM_BOOST_3V3 SMPS current limitation Boost - 0: Max current = 110mA (Default) - 1: Max current = 130mA"]
pub type IlimBoostR = crate::BitReader;
#[doc = "Field `ILIM_BOOST` writer - ILIM_BOOST_3V3 SMPS current limitation Boost - 0: Max current = 110mA (Default) - 1: Max current = 130mA"]
pub type IlimBoostW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOF_CUR_SEL` reader - BOF_CUR_SEL Bypass On the Fly current limitation - 00 : 20mA - 01 : 40mA - 10 : 60mA (default) - 11 : no limit"]
pub type BofCurSelR = crate::FieldReader;
#[doc = "Field `BOF_CUR_SEL` writer - BOF_CUR_SEL Bypass On the Fly current limitation - 00 : 20mA - 01 : 40mA - 10 : 60mA (default) - 11 : no limit"]
pub type BofCurSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - TESTDIG: SMPS TEST_DIG_3V3\\[3:0\\] SMPS control signal"]
    #[inline(always)]
    pub fn testdig(&self) -> TestdigR {
        TestdigR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - TESTKEL: SMPS TEST_KEL_3V3\\[1:0\\] SMPS control signal"]
    #[inline(always)]
    pub fn testkel(&self) -> TestkelR {
        TestkelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - HOT_STUP_3V3 SMPS control signal"]
    #[inline(always)]
    pub fn hot_stup(&self) -> HotStupR {
        HotStupR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NO_STUP_3V3 SMPS control signal"]
    #[inline(always)]
    pub fn no_stup(&self) -> NoStupR {
        NoStupR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TESTILIM: SMPS TEST_ILIM_3V3 SMPS control signal"]
    #[inline(always)]
    pub fn testilim(&self) -> TestilimR {
        TestilimR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTLRES_RAM_3V3 SMPS control signal"]
    #[inline(always)]
    pub fn ctlres_ramp(&self) -> CtlresRampR {
        CtlresRampR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DIS_BIG_MOS_3V3 SMPS control signal"]
    #[inline(always)]
    pub fn dis_big_mos(&self) -> DisBigMosR {
        DisBigMosR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TEST_OL_3V3 SMPS control signal"]
    #[inline(always)]
    pub fn test_ol(&self) -> TestOlR {
        TestOlR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DIS_ILIM_3V3 SMPS control signal"]
    #[inline(always)]
    pub fn dis_ilim(&self) -> DisIlimR {
        DisIlimR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ILIM_BOOST_3V3 SMPS current limitation Boost - 0: Max current = 110mA (Default) - 1: Max current = 130mA"]
    #[inline(always)]
    pub fn ilim_boost(&self) -> IlimBoostR {
        IlimBoostR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - BOF_CUR_SEL Bypass On the Fly current limitation - 00 : 20mA - 01 : 40mA - 10 : 60mA (default) - 11 : no limit"]
    #[inline(always)]
    pub fn bof_cur_sel(&self) -> BofCurSelR {
        BofCurSelR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TESTDIG: SMPS TEST_DIG_3V3\\[3:0\\] SMPS control signal"]
    #[inline(always)]
    pub fn testdig(&mut self) -> TestdigW<'_, DbgsmpsSpec> {
        TestdigW::new(self, 0)
    }
    #[doc = "Bits 4:5 - TESTKEL: SMPS TEST_KEL_3V3\\[1:0\\] SMPS control signal"]
    #[inline(always)]
    pub fn testkel(&mut self) -> TestkelW<'_, DbgsmpsSpec> {
        TestkelW::new(self, 4)
    }
    #[doc = "Bit 6 - HOT_STUP_3V3 SMPS control signal"]
    #[inline(always)]
    pub fn hot_stup(&mut self) -> HotStupW<'_, DbgsmpsSpec> {
        HotStupW::new(self, 6)
    }
    #[doc = "Bit 7 - NO_STUP_3V3 SMPS control signal"]
    #[inline(always)]
    pub fn no_stup(&mut self) -> NoStupW<'_, DbgsmpsSpec> {
        NoStupW::new(self, 7)
    }
    #[doc = "Bit 8 - TESTILIM: SMPS TEST_ILIM_3V3 SMPS control signal"]
    #[inline(always)]
    pub fn testilim(&mut self) -> TestilimW<'_, DbgsmpsSpec> {
        TestilimW::new(self, 8)
    }
    #[doc = "Bit 9 - CTLRES_RAM_3V3 SMPS control signal"]
    #[inline(always)]
    pub fn ctlres_ramp(&mut self) -> CtlresRampW<'_, DbgsmpsSpec> {
        CtlresRampW::new(self, 9)
    }
    #[doc = "Bit 10 - DIS_BIG_MOS_3V3 SMPS control signal"]
    #[inline(always)]
    pub fn dis_big_mos(&mut self) -> DisBigMosW<'_, DbgsmpsSpec> {
        DisBigMosW::new(self, 10)
    }
    #[doc = "Bit 11 - TEST_OL_3V3 SMPS control signal"]
    #[inline(always)]
    pub fn test_ol(&mut self) -> TestOlW<'_, DbgsmpsSpec> {
        TestOlW::new(self, 11)
    }
    #[doc = "Bit 12 - DIS_ILIM_3V3 SMPS control signal"]
    #[inline(always)]
    pub fn dis_ilim(&mut self) -> DisIlimW<'_, DbgsmpsSpec> {
        DisIlimW::new(self, 12)
    }
    #[doc = "Bit 13 - ILIM_BOOST_3V3 SMPS current limitation Boost - 0: Max current = 110mA (Default) - 1: Max current = 130mA"]
    #[inline(always)]
    pub fn ilim_boost(&mut self) -> IlimBoostW<'_, DbgsmpsSpec> {
        IlimBoostW::new(self, 13)
    }
    #[doc = "Bits 14:15 - BOF_CUR_SEL Bypass On the Fly current limitation - 00 : 20mA - 01 : 40mA - 10 : 60mA (default) - 11 : no limit"]
    #[inline(always)]
    pub fn bof_cur_sel(&mut self) -> BofCurSelW<'_, DbgsmpsSpec> {
        BofCurSelW::new(self, 14)
    }
}
#[doc = "DBGSMPS register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgsmps::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgsmps::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgsmpsSpec;
impl crate::RegisterSpec for DbgsmpsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgsmps::R`](R) reader structure"]
impl crate::Readable for DbgsmpsSpec {}
#[doc = "`write(|w| ..)` method takes [`dbgsmps::W`](W) writer structure"]
impl crate::Writable for DbgsmpsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBGSMPS to value 0x8000"]
impl crate::Resettable for DbgsmpsSpec {
    const RESET_VALUE: u32 = 0x8000;
}
