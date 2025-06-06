///Register `DBGCLKR` reader
pub type R = crate::R<DBGCLKRrs>;
///Register `DBGCLKR` writer
pub type W = crate::W<DBGCLKRrs>;
///Field `CLK_SEL` reader - for debug only
pub type ClkSelR = crate::FieldReader;
///Field `CLK_SEL` writer - for debug only
pub type ClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CLK_EN` reader - for debug only
pub type ClkEnR = crate::BitReader;
///Field `CLK_EN` writer - for debug only
pub type ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DLL1_DBG` reader - for debug only
pub type Dll1DbgR = crate::BitReader;
///Field `DLL1_DBG` writer - for debug only
pub type Dll1DbgW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DLL1_LDO_EN` reader - for debug only
pub type Dll1LdoEnR = crate::BitReader;
///Field `DLL1_LDO_EN` writer - for debug only
pub type Dll1LdoEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DLL1_OUT_EN` reader - for debug only
pub type Dll1OutEnR = crate::BitReader;
///Field `DLL1_OUT_EN` writer - for debug only
pub type Dll1OutEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DLL1_LOOP_EN` reader - for debug only
pub type Dll1LoopEnR = crate::BitReader;
///Field `DLL1_LOOP_EN` writer - for debug only
pub type Dll1LoopEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DLL1_OUT_RSTB` reader - for debug only
pub type Dll1OutRstbR = crate::BitReader;
///Field `DLL1_OUT_RSTB` writer - for debug only
pub type Dll1OutRstbW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DLL1_CG_EN` reader - for debug only
pub type Dll1CgEnR = crate::BitReader;
///Field `DLL1_CG_EN` writer - for debug only
pub type Dll1CgEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DLL1_OUT_STR` reader - for debug only
pub type Dll1OutStrR = crate::FieldReader;
///Field `DLL1_OUT_STR` writer - for debug only
pub type Dll1OutStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DLL2_DBG` reader - for debug only
pub type Dll2DbgR = crate::BitReader;
///Field `DLL2_DBG` writer - for debug only
pub type Dll2DbgW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DLL2_LDO_EN` reader - for debug only
pub type Dll2LdoEnR = crate::BitReader;
///Field `DLL2_LDO_EN` writer - for debug only
pub type Dll2LdoEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DLL2_OUT_EN` reader - for debug only
pub type Dll2OutEnR = crate::BitReader;
///Field `DLL2_OUT_EN` writer - for debug only
pub type Dll2OutEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DLL2_LOOP_EN` reader - for debug only
pub type Dll2LoopEnR = crate::BitReader;
///Field `DLL2_LOOP_EN` writer - for debug only
pub type Dll2LoopEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DLL2_OUT_RSTB` reader - for debug only
pub type Dll2OutRstbR = crate::BitReader;
///Field `DLL2_OUT_RSTB` writer - for debug only
pub type Dll2OutRstbW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DLL2_CG_EN` reader - for debug only
pub type Dll2CgEnR = crate::BitReader;
///Field `DLL2_CG_EN` writer - for debug only
pub type Dll2CgEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DLL2_OUT_STR` reader - for debug only
pub type Dll2OutStrR = crate::FieldReader;
///Field `DLL2_OUT_STR` writer - for debug only
pub type Dll2OutStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - for debug only
    #[inline(always)]
    pub fn clk_sel(&self) -> ClkSelR {
        ClkSelR::new((self.bits & 3) as u8)
    }
    ///Bit 2 - for debug only
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - for debug only
    #[inline(always)]
    pub fn dll1_dbg(&self) -> Dll1DbgR {
        Dll1DbgR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - for debug only
    #[inline(always)]
    pub fn dll1_ldo_en(&self) -> Dll1LdoEnR {
        Dll1LdoEnR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - for debug only
    #[inline(always)]
    pub fn dll1_out_en(&self) -> Dll1OutEnR {
        Dll1OutEnR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - for debug only
    #[inline(always)]
    pub fn dll1_loop_en(&self) -> Dll1LoopEnR {
        Dll1LoopEnR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - for debug only
    #[inline(always)]
    pub fn dll1_out_rstb(&self) -> Dll1OutRstbR {
        Dll1OutRstbR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - for debug only
    #[inline(always)]
    pub fn dll1_cg_en(&self) -> Dll1CgEnR {
        Dll1CgEnR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:11 - for debug only
    #[inline(always)]
    pub fn dll1_out_str(&self) -> Dll1OutStrR {
        Dll1OutStrR::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 12 - for debug only
    #[inline(always)]
    pub fn dll2_dbg(&self) -> Dll2DbgR {
        Dll2DbgR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - for debug only
    #[inline(always)]
    pub fn dll2_ldo_en(&self) -> Dll2LdoEnR {
        Dll2LdoEnR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - for debug only
    #[inline(always)]
    pub fn dll2_out_en(&self) -> Dll2OutEnR {
        Dll2OutEnR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - for debug only
    #[inline(always)]
    pub fn dll2_loop_en(&self) -> Dll2LoopEnR {
        Dll2LoopEnR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - for debug only
    #[inline(always)]
    pub fn dll2_out_rstb(&self) -> Dll2OutRstbR {
        Dll2OutRstbR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - for debug only
    #[inline(always)]
    pub fn dll2_cg_en(&self) -> Dll2CgEnR {
        Dll2CgEnR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:19 - for debug only
    #[inline(always)]
    pub fn dll2_out_str(&self) -> Dll2OutStrR {
        Dll2OutStrR::new(((self.bits >> 18) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGCLKR")
            .field("dll2_out_str", &self.dll2_out_str())
            .field("dll2_cg_en", &self.dll2_cg_en())
            .field("dll2_out_rstb", &self.dll2_out_rstb())
            .field("dll2_loop_en", &self.dll2_loop_en())
            .field("dll2_out_en", &self.dll2_out_en())
            .field("dll2_ldo_en", &self.dll2_ldo_en())
            .field("dll2_dbg", &self.dll2_dbg())
            .field("dll1_out_str", &self.dll1_out_str())
            .field("dll1_cg_en", &self.dll1_cg_en())
            .field("dll1_out_rstb", &self.dll1_out_rstb())
            .field("dll1_loop_en", &self.dll1_loop_en())
            .field("dll1_out_en", &self.dll1_out_en())
            .field("dll1_ldo_en", &self.dll1_ldo_en())
            .field("dll1_dbg", &self.dll1_dbg())
            .field("clk_en", &self.clk_en())
            .field("clk_sel", &self.clk_sel())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - for debug only
    #[inline(always)]
    pub fn clk_sel(&mut self) -> ClkSelW<DBGCLKRrs> {
        ClkSelW::new(self, 0)
    }
    ///Bit 2 - for debug only
    #[inline(always)]
    pub fn clk_en(&mut self) -> ClkEnW<DBGCLKRrs> {
        ClkEnW::new(self, 2)
    }
    ///Bit 4 - for debug only
    #[inline(always)]
    pub fn dll1_dbg(&mut self) -> Dll1DbgW<DBGCLKRrs> {
        Dll1DbgW::new(self, 4)
    }
    ///Bit 5 - for debug only
    #[inline(always)]
    pub fn dll1_ldo_en(&mut self) -> Dll1LdoEnW<DBGCLKRrs> {
        Dll1LdoEnW::new(self, 5)
    }
    ///Bit 6 - for debug only
    #[inline(always)]
    pub fn dll1_out_en(&mut self) -> Dll1OutEnW<DBGCLKRrs> {
        Dll1OutEnW::new(self, 6)
    }
    ///Bit 7 - for debug only
    #[inline(always)]
    pub fn dll1_loop_en(&mut self) -> Dll1LoopEnW<DBGCLKRrs> {
        Dll1LoopEnW::new(self, 7)
    }
    ///Bit 8 - for debug only
    #[inline(always)]
    pub fn dll1_out_rstb(&mut self) -> Dll1OutRstbW<DBGCLKRrs> {
        Dll1OutRstbW::new(self, 8)
    }
    ///Bit 9 - for debug only
    #[inline(always)]
    pub fn dll1_cg_en(&mut self) -> Dll1CgEnW<DBGCLKRrs> {
        Dll1CgEnW::new(self, 9)
    }
    ///Bits 10:11 - for debug only
    #[inline(always)]
    pub fn dll1_out_str(&mut self) -> Dll1OutStrW<DBGCLKRrs> {
        Dll1OutStrW::new(self, 10)
    }
    ///Bit 12 - for debug only
    #[inline(always)]
    pub fn dll2_dbg(&mut self) -> Dll2DbgW<DBGCLKRrs> {
        Dll2DbgW::new(self, 12)
    }
    ///Bit 13 - for debug only
    #[inline(always)]
    pub fn dll2_ldo_en(&mut self) -> Dll2LdoEnW<DBGCLKRrs> {
        Dll2LdoEnW::new(self, 13)
    }
    ///Bit 14 - for debug only
    #[inline(always)]
    pub fn dll2_out_en(&mut self) -> Dll2OutEnW<DBGCLKRrs> {
        Dll2OutEnW::new(self, 14)
    }
    ///Bit 15 - for debug only
    #[inline(always)]
    pub fn dll2_loop_en(&mut self) -> Dll2LoopEnW<DBGCLKRrs> {
        Dll2LoopEnW::new(self, 15)
    }
    ///Bit 16 - for debug only
    #[inline(always)]
    pub fn dll2_out_rstb(&mut self) -> Dll2OutRstbW<DBGCLKRrs> {
        Dll2OutRstbW::new(self, 16)
    }
    ///Bit 17 - for debug only
    #[inline(always)]
    pub fn dll2_cg_en(&mut self) -> Dll2CgEnW<DBGCLKRrs> {
        Dll2CgEnW::new(self, 17)
    }
    ///Bits 18:19 - for debug only
    #[inline(always)]
    pub fn dll2_out_str(&mut self) -> Dll2OutStrW<DBGCLKRrs> {
        Dll2OutStrW::new(self, 18)
    }
}
///Debug Clock Register
///
///You can [`read`](crate::Reg::read) this register and get [`dbgclkr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgclkr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DBGCLKRrs;
impl crate::RegisterSpec for DBGCLKRrs {
    type Ux = u32;
}
///`read()` method returns [`dbgclkr::R`](R) reader structure
impl crate::Readable for DBGCLKRrs {}
///`write(|w| ..)` method takes [`dbgclkr::W`](W) writer structure
impl crate::Writable for DBGCLKRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DBGCLKR to value 0
impl crate::Resettable for DBGCLKRrs {
    const RESET_VALUE: u32 = 0;
}
