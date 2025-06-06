///Register `DBGR` reader
pub type R = crate::R<DBGRrs>;
///Register `DBGR` writer
pub type W = crate::W<DBGRrs>;
///Field `SEL_L` reader - reserved for debug
pub type SelLR = crate::FieldReader;
///Field `SEL_L` writer - reserved for debug
pub type SelLW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SEL_H` reader - reserved for debug
pub type SelHR = crate::FieldReader;
///Field `SEL_H` writer - reserved for debug
pub type SelHW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `BITEN_L` reader - reserved for debug
pub type BitenLR = crate::FieldReader;
///Field `BITEN_L` writer - reserved for debug
pub type BitenLW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BITEN_H` reader - reserved for debug
pub type BitenHR = crate::FieldReader;
///Field `BITEN_H` writer - reserved for debug
pub type BitenHW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CLK_SEL` reader - reserved for debug
pub type ClkSelR = crate::FieldReader;
///Field `CLK_SEL` writer - reserved for debug
pub type ClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CLK_EN` reader - reserved for debug
pub type ClkEnR = crate::BitReader;
///Field `CLK_EN` writer - reserved for debug
pub type ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HP2LP_NMI` reader - set 1 to send NMI interrupt to LCPU
pub type Hp2lpNmiR = crate::BitReader;
///Field `HP2LP_NMI` writer - set 1 to send NMI interrupt to LCPU
pub type Hp2lpNmiW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP2HP_NMIE` reader - LP2HP NMI interrupt enable
pub type Lp2hpNmieR = crate::BitReader;
///Field `LP2HP_NMIE` writer - LP2HP NMI interrupt enable
pub type Lp2hpNmieW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP2HP_NMIF` reader - LP2HP NMI interrupt flag
pub type Lp2hpNmifR = crate::BitReader;
///Field `LP2HP_NMIF` writer - LP2HP NMI interrupt flag
pub type Lp2hpNmifW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWAP` reader - reserved for debug
pub type SwapR = crate::BitReader;
///Field `SWAP` writer - reserved for debug
pub type SwapW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - reserved for debug
    #[inline(always)]
    pub fn sel_l(&self) -> SelLR {
        SelLR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - reserved for debug
    #[inline(always)]
    pub fn sel_h(&self) -> SelHR {
        SelHR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:15 - reserved for debug
    #[inline(always)]
    pub fn biten_l(&self) -> BitenLR {
        BitenLR::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - reserved for debug
    #[inline(always)]
    pub fn biten_h(&self) -> BitenHR {
        BitenHR::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:26 - reserved for debug
    #[inline(always)]
    pub fn clk_sel(&self) -> ClkSelR {
        ClkSelR::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bit 27 - reserved for debug
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - set 1 to send NMI interrupt to LCPU
    #[inline(always)]
    pub fn hp2lp_nmi(&self) -> Hp2lpNmiR {
        Hp2lpNmiR::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - LP2HP NMI interrupt enable
    #[inline(always)]
    pub fn lp2hp_nmie(&self) -> Lp2hpNmieR {
        Lp2hpNmieR::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - LP2HP NMI interrupt flag
    #[inline(always)]
    pub fn lp2hp_nmif(&self) -> Lp2hpNmifR {
        Lp2hpNmifR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - reserved for debug
    #[inline(always)]
    pub fn swap(&self) -> SwapR {
        SwapR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGR")
            .field("swap", &self.swap())
            .field("lp2hp_nmif", &self.lp2hp_nmif())
            .field("lp2hp_nmie", &self.lp2hp_nmie())
            .field("hp2lp_nmi", &self.hp2lp_nmi())
            .field("clk_en", &self.clk_en())
            .field("clk_sel", &self.clk_sel())
            .field("biten_h", &self.biten_h())
            .field("biten_l", &self.biten_l())
            .field("sel_h", &self.sel_h())
            .field("sel_l", &self.sel_l())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - reserved for debug
    #[inline(always)]
    pub fn sel_l(&mut self) -> SelLW<DBGRrs> {
        SelLW::new(self, 0)
    }
    ///Bits 4:7 - reserved for debug
    #[inline(always)]
    pub fn sel_h(&mut self) -> SelHW<DBGRrs> {
        SelHW::new(self, 4)
    }
    ///Bits 8:15 - reserved for debug
    #[inline(always)]
    pub fn biten_l(&mut self) -> BitenLW<DBGRrs> {
        BitenLW::new(self, 8)
    }
    ///Bits 16:23 - reserved for debug
    #[inline(always)]
    pub fn biten_h(&mut self) -> BitenHW<DBGRrs> {
        BitenHW::new(self, 16)
    }
    ///Bits 24:26 - reserved for debug
    #[inline(always)]
    pub fn clk_sel(&mut self) -> ClkSelW<DBGRrs> {
        ClkSelW::new(self, 24)
    }
    ///Bit 27 - reserved for debug
    #[inline(always)]
    pub fn clk_en(&mut self) -> ClkEnW<DBGRrs> {
        ClkEnW::new(self, 27)
    }
    ///Bit 28 - set 1 to send NMI interrupt to LCPU
    #[inline(always)]
    pub fn hp2lp_nmi(&mut self) -> Hp2lpNmiW<DBGRrs> {
        Hp2lpNmiW::new(self, 28)
    }
    ///Bit 29 - LP2HP NMI interrupt enable
    #[inline(always)]
    pub fn lp2hp_nmie(&mut self) -> Lp2hpNmieW<DBGRrs> {
        Lp2hpNmieW::new(self, 29)
    }
    ///Bit 30 - LP2HP NMI interrupt flag
    #[inline(always)]
    pub fn lp2hp_nmif(&mut self) -> Lp2hpNmifW<DBGRrs> {
        Lp2hpNmifW::new(self, 30)
    }
    ///Bit 31 - reserved for debug
    #[inline(always)]
    pub fn swap(&mut self) -> SwapW<DBGRrs> {
        SwapW::new(self, 31)
    }
}
///Debug Select Register
///
///You can [`read`](crate::Reg::read) this register and get [`dbgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DBGRrs;
impl crate::RegisterSpec for DBGRrs {
    type Ux = u32;
}
///`read()` method returns [`dbgr::R`](R) reader structure
impl crate::Readable for DBGRrs {}
///`write(|w| ..)` method takes [`dbgr::W`](W) writer structure
impl crate::Writable for DBGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DBGR to value 0
impl crate::Resettable for DBGRrs {
    const RESET_VALUE: u32 = 0;
}
