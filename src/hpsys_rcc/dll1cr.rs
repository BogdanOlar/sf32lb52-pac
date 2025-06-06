///Register `DLL1CR` reader
pub type R = crate::R<DLL1CRrs>;
///Register `DLL1CR` writer
pub type W = crate::W<DLL1CRrs>;
///Field `EN` reader - 0: dll disabled 1: dll enabled
pub type EnR = crate::BitReader;
///Field `EN` writer - 0: dll disabled 1: dll enabled
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SW` reader -
pub type SwR = crate::BitReader;
///Field `SW` writer -
pub type SwW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STG` reader - DLL lock freqency is decided by STG. DLL output frequency is (STG+1)*24MHz e.g. STG=9,DLL output is 240M
pub type StgR = crate::FieldReader;
///Field `STG` writer - DLL lock freqency is decided by STG. DLL output frequency is (STG+1)*24MHz e.g. STG=9,DLL output is 240M
pub type StgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `XTALIN_EN` reader -
pub type XtalinEnR = crate::BitReader;
///Field `XTALIN_EN` writer -
pub type XtalinEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODE48M_EN` reader -
pub type Mode48mEnR = crate::BitReader;
///Field `MODE48M_EN` writer -
pub type Mode48mEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LDO_VREF` reader -
pub type LdoVrefR = crate::FieldReader;
///Field `LDO_VREF` writer -
pub type LdoVrefW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `IN_DIV2_EN` reader -
pub type InDiv2EnR = crate::BitReader;
///Field `IN_DIV2_EN` writer -
pub type InDiv2EnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUT_DIV2_EN` reader - 0: dll output not divided 1: dll output divided by 2
pub type OutDiv2EnR = crate::BitReader;
///Field `OUT_DIV2_EN` writer - 0: dll output not divided 1: dll output divided by 2
pub type OutDiv2EnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCU_PRCHG_EN` reader -
pub type McuPrchgEnR = crate::BitReader;
///Field `MCU_PRCHG_EN` writer -
pub type McuPrchgEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCU_PRCHG` reader -
pub type McuPrchgR = crate::BitReader;
///Field `MCU_PRCHG` writer -
pub type McuPrchgW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRCHG_EN` reader -
pub type PrchgEnR = crate::BitReader;
///Field `PRCHG_EN` writer -
pub type PrchgEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRCHG_EXT` reader -
pub type PrchgExtR = crate::BitReader;
///Field `PRCHG_EXT` writer -
pub type PrchgExtW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VST_SEL` reader -
pub type VstSelR = crate::BitReader;
///Field `VST_SEL` writer -
pub type VstSelW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BYPASS` reader -
pub type BypassR = crate::BitReader;
///Field `BYPASS` writer -
pub type BypassW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTEST_EN` reader -
pub type DtestEnR = crate::BitReader;
///Field `DTEST_EN` writer -
pub type DtestEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTEST_TR` reader -
pub type DtestTrR = crate::FieldReader;
///Field `DTEST_TR` writer -
pub type DtestTrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PU_DLY` reader -
pub type PuDlyR = crate::FieldReader;
///Field `PU_DLY` writer -
pub type PuDlyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `LOCK_DLY` reader -
pub type LockDlyR = crate::FieldReader;
///Field `LOCK_DLY` writer -
pub type LockDlyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `READY` reader - 0: dll not ready 1: dll ready
pub type ReadyR = crate::BitReader;
///Field `READY` writer - 0: dll not ready 1: dll ready
pub type ReadyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - 0: dll disabled 1: dll enabled
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn sw(&self) -> SwR {
        SwR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:5 - DLL lock freqency is decided by STG. DLL output frequency is (STG+1)*24MHz e.g. STG=9,DLL output is 240M
    #[inline(always)]
    pub fn stg(&self) -> StgR {
        StgR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    ///Bit 6
    #[inline(always)]
    pub fn xtalin_en(&self) -> XtalinEnR {
        XtalinEnR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7
    #[inline(always)]
    pub fn mode48m_en(&self) -> Mode48mEnR {
        Mode48mEnR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11
    #[inline(always)]
    pub fn ldo_vref(&self) -> LdoVrefR {
        LdoVrefR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12
    #[inline(always)]
    pub fn in_div2_en(&self) -> InDiv2EnR {
        InDiv2EnR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - 0: dll output not divided 1: dll output divided by 2
    #[inline(always)]
    pub fn out_div2_en(&self) -> OutDiv2EnR {
        OutDiv2EnR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14
    #[inline(always)]
    pub fn mcu_prchg_en(&self) -> McuPrchgEnR {
        McuPrchgEnR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15
    #[inline(always)]
    pub fn mcu_prchg(&self) -> McuPrchgR {
        McuPrchgR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16
    #[inline(always)]
    pub fn prchg_en(&self) -> PrchgEnR {
        PrchgEnR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17
    #[inline(always)]
    pub fn prchg_ext(&self) -> PrchgExtR {
        PrchgExtR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18
    #[inline(always)]
    pub fn vst_sel(&self) -> VstSelR {
        VstSelR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19
    #[inline(always)]
    pub fn bypass(&self) -> BypassR {
        BypassR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20
    #[inline(always)]
    pub fn dtest_en(&self) -> DtestEnR {
        DtestEnR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:24
    #[inline(always)]
    pub fn dtest_tr(&self) -> DtestTrR {
        DtestTrR::new(((self.bits >> 21) & 0x0f) as u8)
    }
    ///Bits 25:27
    #[inline(always)]
    pub fn pu_dly(&self) -> PuDlyR {
        PuDlyR::new(((self.bits >> 25) & 7) as u8)
    }
    ///Bits 28:30
    #[inline(always)]
    pub fn lock_dly(&self) -> LockDlyR {
        LockDlyR::new(((self.bits >> 28) & 7) as u8)
    }
    ///Bit 31 - 0: dll not ready 1: dll ready
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLL1CR")
            .field("ready", &self.ready())
            .field("lock_dly", &self.lock_dly())
            .field("pu_dly", &self.pu_dly())
            .field("dtest_tr", &self.dtest_tr())
            .field("dtest_en", &self.dtest_en())
            .field("bypass", &self.bypass())
            .field("vst_sel", &self.vst_sel())
            .field("prchg_ext", &self.prchg_ext())
            .field("prchg_en", &self.prchg_en())
            .field("mcu_prchg", &self.mcu_prchg())
            .field("mcu_prchg_en", &self.mcu_prchg_en())
            .field("out_div2_en", &self.out_div2_en())
            .field("in_div2_en", &self.in_div2_en())
            .field("ldo_vref", &self.ldo_vref())
            .field("mode48m_en", &self.mode48m_en())
            .field("xtalin_en", &self.xtalin_en())
            .field("stg", &self.stg())
            .field("sw", &self.sw())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    ///Bit 0 - 0: dll disabled 1: dll enabled
    #[inline(always)]
    pub fn en(&mut self) -> EnW<DLL1CRrs> {
        EnW::new(self, 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn sw(&mut self) -> SwW<DLL1CRrs> {
        SwW::new(self, 1)
    }
    ///Bits 2:5 - DLL lock freqency is decided by STG. DLL output frequency is (STG+1)*24MHz e.g. STG=9,DLL output is 240M
    #[inline(always)]
    pub fn stg(&mut self) -> StgW<DLL1CRrs> {
        StgW::new(self, 2)
    }
    ///Bit 6
    #[inline(always)]
    pub fn xtalin_en(&mut self) -> XtalinEnW<DLL1CRrs> {
        XtalinEnW::new(self, 6)
    }
    ///Bit 7
    #[inline(always)]
    pub fn mode48m_en(&mut self) -> Mode48mEnW<DLL1CRrs> {
        Mode48mEnW::new(self, 7)
    }
    ///Bits 8:11
    #[inline(always)]
    pub fn ldo_vref(&mut self) -> LdoVrefW<DLL1CRrs> {
        LdoVrefW::new(self, 8)
    }
    ///Bit 12
    #[inline(always)]
    pub fn in_div2_en(&mut self) -> InDiv2EnW<DLL1CRrs> {
        InDiv2EnW::new(self, 12)
    }
    ///Bit 13 - 0: dll output not divided 1: dll output divided by 2
    #[inline(always)]
    pub fn out_div2_en(&mut self) -> OutDiv2EnW<DLL1CRrs> {
        OutDiv2EnW::new(self, 13)
    }
    ///Bit 14
    #[inline(always)]
    pub fn mcu_prchg_en(&mut self) -> McuPrchgEnW<DLL1CRrs> {
        McuPrchgEnW::new(self, 14)
    }
    ///Bit 15
    #[inline(always)]
    pub fn mcu_prchg(&mut self) -> McuPrchgW<DLL1CRrs> {
        McuPrchgW::new(self, 15)
    }
    ///Bit 16
    #[inline(always)]
    pub fn prchg_en(&mut self) -> PrchgEnW<DLL1CRrs> {
        PrchgEnW::new(self, 16)
    }
    ///Bit 17
    #[inline(always)]
    pub fn prchg_ext(&mut self) -> PrchgExtW<DLL1CRrs> {
        PrchgExtW::new(self, 17)
    }
    ///Bit 18
    #[inline(always)]
    pub fn vst_sel(&mut self) -> VstSelW<DLL1CRrs> {
        VstSelW::new(self, 18)
    }
    ///Bit 19
    #[inline(always)]
    pub fn bypass(&mut self) -> BypassW<DLL1CRrs> {
        BypassW::new(self, 19)
    }
    ///Bit 20
    #[inline(always)]
    pub fn dtest_en(&mut self) -> DtestEnW<DLL1CRrs> {
        DtestEnW::new(self, 20)
    }
    ///Bits 21:24
    #[inline(always)]
    pub fn dtest_tr(&mut self) -> DtestTrW<DLL1CRrs> {
        DtestTrW::new(self, 21)
    }
    ///Bits 25:27
    #[inline(always)]
    pub fn pu_dly(&mut self) -> PuDlyW<DLL1CRrs> {
        PuDlyW::new(self, 25)
    }
    ///Bits 28:30
    #[inline(always)]
    pub fn lock_dly(&mut self) -> LockDlyW<DLL1CRrs> {
        LockDlyW::new(self, 28)
    }
    ///Bit 31 - 0: dll not ready 1: dll ready
    #[inline(always)]
    pub fn ready(&mut self) -> ReadyW<DLL1CRrs> {
        ReadyW::new(self, 31)
    }
}
///DLL1 Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`dll1cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dll1cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DLL1CRrs;
impl crate::RegisterSpec for DLL1CRrs {
    type Ux = u32;
}
///`read()` method returns [`dll1cr::R`](R) reader structure
impl crate::Readable for DLL1CRrs {}
///`write(|w| ..)` method takes [`dll1cr::W`](W) writer structure
impl crate::Writable for DLL1CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DLL1CR to value 0
impl crate::Resettable for DLL1CRrs {
    const RESET_VALUE: u32 = 0;
}
