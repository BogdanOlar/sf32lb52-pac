///Register `BUCK_CR2` reader
pub type R = crate::R<BUCK_CR2rs>;
///Register `BUCK_CR2` writer
pub type W = crate::W<BUCK_CR2rs>;
///Field `H2M_EN` reader -
pub type H2mEnR = crate::BitReader;
///Field `H2M_EN` writer -
pub type H2mEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `H2L_EN` reader -
pub type H2lEnR = crate::BitReader;
///Field `H2L_EN` writer -
pub type H2lEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `M2L_EN` reader -
pub type M2lEnR = crate::BitReader;
///Field `M2L_EN` writer -
pub type M2lEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L2M_EN` reader -
pub type L2mEnR = crate::BitReader;
///Field `L2M_EN` writer -
pub type L2mEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `M2H_CNT` reader -
pub type M2hCntR = crate::FieldReader;
///Field `M2H_CNT` writer -
pub type M2hCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `L2H_CNT` reader -
pub type L2hCntR = crate::FieldReader;
///Field `L2H_CNT` writer -
pub type L2hCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `L2M_CNT` reader -
pub type L2mCntR = crate::FieldReader;
///Field `L2M_CNT` writer -
pub type L2mCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `BYPASS_PG` reader -
pub type BypassPgR = crate::BitReader;
///Field `BYPASS_PG` writer -
pub type BypassPgW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BYPASS_OCP` reader -
pub type BypassOcpR = crate::BitReader;
///Field `BYPASS_OCP` writer -
pub type BypassOcpW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BYPASS_UVLO` reader -
pub type BypassUvloR = crate::BitReader;
///Field `BYPASS_UVLO` writer -
pub type BypassUvloW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FORCE_RDY` reader -
pub type ForceRdyR = crate::BitReader;
///Field `FORCE_RDY` writer -
pub type ForceRdyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SET_VOUT_M` reader - 1.1V
pub type SetVoutMR = crate::FieldReader;
///Field `SET_VOUT_M` writer - 1.1V
pub type SetVoutMW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SET_VOUT_L` reader - 0.75V
pub type SetVoutLR = crate::FieldReader;
///Field `SET_VOUT_L` writer - 0.75V
pub type SetVoutLW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TDIS` reader - Discharge for TDIS*4 LP clock cycles during reboot
pub type TdisR = crate::FieldReader;
///Field `TDIS` writer - Discharge for TDIS*4 LP clock cycles during reboot
pub type TdisW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn h2m_en(&self) -> H2mEnR {
        H2mEnR::new((self.bits & 1) != 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn h2l_en(&self) -> H2lEnR {
        H2lEnR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2
    #[inline(always)]
    pub fn m2l_en(&self) -> M2lEnR {
        M2lEnR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3
    #[inline(always)]
    pub fn l2m_en(&self) -> L2mEnR {
        L2mEnR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:7
    #[inline(always)]
    pub fn m2h_cnt(&self) -> M2hCntR {
        M2hCntR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11
    #[inline(always)]
    pub fn l2h_cnt(&self) -> L2hCntR {
        L2hCntR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15
    #[inline(always)]
    pub fn l2m_cnt(&self) -> L2mCntR {
        L2mCntR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bit 16
    #[inline(always)]
    pub fn bypass_pg(&self) -> BypassPgR {
        BypassPgR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17
    #[inline(always)]
    pub fn bypass_ocp(&self) -> BypassOcpR {
        BypassOcpR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18
    #[inline(always)]
    pub fn bypass_uvlo(&self) -> BypassUvloR {
        BypassUvloR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19
    #[inline(always)]
    pub fn force_rdy(&self) -> ForceRdyR {
        ForceRdyR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:23 - 1.1V
    #[inline(always)]
    pub fn set_vout_m(&self) -> SetVoutMR {
        SetVoutMR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - 0.75V
    #[inline(always)]
    pub fn set_vout_l(&self) -> SetVoutLR {
        SetVoutLR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Discharge for TDIS*4 LP clock cycles during reboot
    #[inline(always)]
    pub fn tdis(&self) -> TdisR {
        TdisR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUCK_CR2")
            .field("tdis", &self.tdis())
            .field("set_vout_l", &self.set_vout_l())
            .field("set_vout_m", &self.set_vout_m())
            .field("force_rdy", &self.force_rdy())
            .field("bypass_uvlo", &self.bypass_uvlo())
            .field("bypass_ocp", &self.bypass_ocp())
            .field("bypass_pg", &self.bypass_pg())
            .field("l2m_cnt", &self.l2m_cnt())
            .field("l2h_cnt", &self.l2h_cnt())
            .field("m2h_cnt", &self.m2h_cnt())
            .field("l2m_en", &self.l2m_en())
            .field("m2l_en", &self.m2l_en())
            .field("h2l_en", &self.h2l_en())
            .field("h2m_en", &self.h2m_en())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    pub fn h2m_en(&mut self) -> H2mEnW<BUCK_CR2rs> {
        H2mEnW::new(self, 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn h2l_en(&mut self) -> H2lEnW<BUCK_CR2rs> {
        H2lEnW::new(self, 1)
    }
    ///Bit 2
    #[inline(always)]
    pub fn m2l_en(&mut self) -> M2lEnW<BUCK_CR2rs> {
        M2lEnW::new(self, 2)
    }
    ///Bit 3
    #[inline(always)]
    pub fn l2m_en(&mut self) -> L2mEnW<BUCK_CR2rs> {
        L2mEnW::new(self, 3)
    }
    ///Bits 4:7
    #[inline(always)]
    pub fn m2h_cnt(&mut self) -> M2hCntW<BUCK_CR2rs> {
        M2hCntW::new(self, 4)
    }
    ///Bits 8:11
    #[inline(always)]
    pub fn l2h_cnt(&mut self) -> L2hCntW<BUCK_CR2rs> {
        L2hCntW::new(self, 8)
    }
    ///Bits 12:15
    #[inline(always)]
    pub fn l2m_cnt(&mut self) -> L2mCntW<BUCK_CR2rs> {
        L2mCntW::new(self, 12)
    }
    ///Bit 16
    #[inline(always)]
    pub fn bypass_pg(&mut self) -> BypassPgW<BUCK_CR2rs> {
        BypassPgW::new(self, 16)
    }
    ///Bit 17
    #[inline(always)]
    pub fn bypass_ocp(&mut self) -> BypassOcpW<BUCK_CR2rs> {
        BypassOcpW::new(self, 17)
    }
    ///Bit 18
    #[inline(always)]
    pub fn bypass_uvlo(&mut self) -> BypassUvloW<BUCK_CR2rs> {
        BypassUvloW::new(self, 18)
    }
    ///Bit 19
    #[inline(always)]
    pub fn force_rdy(&mut self) -> ForceRdyW<BUCK_CR2rs> {
        ForceRdyW::new(self, 19)
    }
    ///Bits 20:23 - 1.1V
    #[inline(always)]
    pub fn set_vout_m(&mut self) -> SetVoutMW<BUCK_CR2rs> {
        SetVoutMW::new(self, 20)
    }
    ///Bits 24:27 - 0.75V
    #[inline(always)]
    pub fn set_vout_l(&mut self) -> SetVoutLW<BUCK_CR2rs> {
        SetVoutLW::new(self, 24)
    }
    ///Bits 28:31 - Discharge for TDIS*4 LP clock cycles during reboot
    #[inline(always)]
    pub fn tdis(&mut self) -> TdisW<BUCK_CR2rs> {
        TdisW::new(self, 28)
    }
}
///BUCK Control Register 2
///
///You can [`read`](crate::Reg::read) this register and get [`buck_cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buck_cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct BUCK_CR2rs;
impl crate::RegisterSpec for BUCK_CR2rs {
    type Ux = u32;
}
///`read()` method returns [`buck_cr2::R`](R) reader structure
impl crate::Readable for BUCK_CR2rs {}
///`write(|w| ..)` method takes [`buck_cr2::W`](W) writer structure
impl crate::Writable for BUCK_CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BUCK_CR2 to value 0
impl crate::Resettable for BUCK_CR2rs {
    const RESET_VALUE: u32 = 0;
}
