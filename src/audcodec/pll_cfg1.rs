///Register `PLL_CFG1` reader
pub type R = crate::R<PLL_CFG1rs>;
///Register `PLL_CFG1` writer
pub type W = crate::W<PLL_CFG1rs>;
///Field `R3_SEL` reader - select R3
pub type R3SelR = crate::FieldReader;
///Field `R3_SEL` writer - select R3
pub type R3SelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RZ_SEL` reader - select Rz
pub type RzSelR = crate::FieldReader;
///Field `RZ_SEL` writer - select Rz
pub type RzSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `C2_SEL` reader - select C2
pub type C2SelR = crate::FieldReader;
///Field `C2_SEL` writer - select C2
pub type C2SelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CZ_SEL` reader - select Cz
pub type CzSelR = crate::FieldReader;
///Field `CZ_SEL` writer - select Cz
pub type CzSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CSD_RST` reader - reset CSD, high active
pub type CsdRstR = crate::BitReader;
///Field `CSD_RST` writer - reset CSD, high active
pub type CsdRstW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSD_EN` reader - enable CSD
pub type CsdEnR = crate::BitReader;
///Field `CSD_EN` writer - enable CSD
pub type CsdEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - select R3
    #[inline(always)]
    pub fn r3_sel(&self) -> R3SelR {
        R3SelR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - select Rz
    #[inline(always)]
    pub fn rz_sel(&self) -> RzSelR {
        RzSelR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:10 - select C2
    #[inline(always)]
    pub fn c2_sel(&self) -> C2SelR {
        C2SelR::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 11:13 - select Cz
    #[inline(always)]
    pub fn cz_sel(&self) -> CzSelR {
        CzSelR::new(((self.bits >> 11) & 7) as u8)
    }
    ///Bit 14 - reset CSD, high active
    #[inline(always)]
    pub fn csd_rst(&self) -> CsdRstR {
        CsdRstR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - enable CSD
    #[inline(always)]
    pub fn csd_en(&self) -> CsdEnR {
        CsdEnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL_CFG1")
            .field("csd_en", &self.csd_en())
            .field("csd_rst", &self.csd_rst())
            .field("cz_sel", &self.cz_sel())
            .field("c2_sel", &self.c2_sel())
            .field("rz_sel", &self.rz_sel())
            .field("r3_sel", &self.r3_sel())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - select R3
    #[inline(always)]
    pub fn r3_sel(&mut self) -> R3SelW<PLL_CFG1rs> {
        R3SelW::new(self, 0)
    }
    ///Bits 4:7 - select Rz
    #[inline(always)]
    pub fn rz_sel(&mut self) -> RzSelW<PLL_CFG1rs> {
        RzSelW::new(self, 4)
    }
    ///Bits 8:10 - select C2
    #[inline(always)]
    pub fn c2_sel(&mut self) -> C2SelW<PLL_CFG1rs> {
        C2SelW::new(self, 8)
    }
    ///Bits 11:13 - select Cz
    #[inline(always)]
    pub fn cz_sel(&mut self) -> CzSelW<PLL_CFG1rs> {
        CzSelW::new(self, 11)
    }
    ///Bit 14 - reset CSD, high active
    #[inline(always)]
    pub fn csd_rst(&mut self) -> CsdRstW<PLL_CFG1rs> {
        CsdRstW::new(self, 14)
    }
    ///Bit 15 - enable CSD
    #[inline(always)]
    pub fn csd_en(&mut self) -> CsdEnW<PLL_CFG1rs> {
        CsdEnW::new(self, 15)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`pll_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PLL_CFG1rs;
impl crate::RegisterSpec for PLL_CFG1rs {
    type Ux = u32;
}
///`read()` method returns [`pll_cfg1::R`](R) reader structure
impl crate::Readable for PLL_CFG1rs {}
///`write(|w| ..)` method takes [`pll_cfg1::W`](W) writer structure
impl crate::Writable for PLL_CFG1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PLL_CFG1 to value 0
impl crate::Resettable for PLL_CFG1rs {
    const RESET_VALUE: u32 = 0;
}
