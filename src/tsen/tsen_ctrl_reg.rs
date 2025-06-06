///Register `TSEN_CTRL_REG` reader
pub type R = crate::R<TSEN_CTRL_REGrs>;
///Register `TSEN_CTRL_REG` writer
pub type W = crate::W<TSEN_CTRL_REGrs>;
///Field `ANAU_TSEN_PU` reader - power up tsen
pub type AnauTsenPuR = crate::BitReader;
///Field `ANAU_TSEN_PU` writer - power up tsen
pub type AnauTsenPuW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ANAU_TSEN_RSTB` reader - resetb for tsen
pub type AnauTsenRstbR = crate::BitReader;
///Field `ANAU_TSEN_RSTB` writer - resetb for tsen
pub type AnauTsenRstbW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ANAU_TSEN_RUN` reader - enable tsen run
pub type AnauTsenRunR = crate::BitReader;
///Field `ANAU_TSEN_RUN` writer - enable tsen run
pub type AnauTsenRunW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ANAU_TSEN_IG_VBE` reader - bias current selection to tune vba
pub type AnauTsenIgVbeR = crate::FieldReader;
///Field `ANAU_TSEN_IG_VBE` writer - bias current selection to tune vba
pub type AnauTsenIgVbeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ANAU_TSEN_FCK_SEL` reader - select internal clock frequency
pub type AnauTsenFckSelR = crate::FieldReader;
///Field `ANAU_TSEN_FCK_SEL` writer - select internal clock frequency
pub type AnauTsenFckSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ANAU_TSEN_SGN_EN` reader - signature-mode enable
pub type AnauTsenSgnEnR = crate::BitReader;
///Field `ANAU_TSEN_SGN_EN` writer - signature-mode enable
pub type AnauTsenSgnEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ANAU_TSEN_SER_PAR_SEL` reader - serial-parallel output selection
pub type AnauTsenSerParSelR = crate::BitReader;
///Field `ANAU_TSEN_SER_PAR_SEL` writer - serial-parallel output selection
pub type AnauTsenSerParSelW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ANAU_TSEN_RDY` reader - tsen ready
pub type AnauTsenRdyR = crate::BitReader;
///Field `ANAU_TSEN_RDY` writer - tsen ready
pub type AnauTsenRdyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ANAU_TSEN_EN` reader - Enable tsen digital module
pub type AnauTsenEnR = crate::BitReader;
///Field `ANAU_TSEN_EN` writer - Enable tsen digital module
pub type AnauTsenEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ANAU_TSEN_CLK_DIV` reader - gen tsen clk by divide hclk by anau_tsen_clk_div
pub type AnauTsenClkDivR = crate::FieldReader;
///Field `ANAU_TSEN_CLK_DIV` writer - gen tsen clk by divide hclk by anau_tsen_clk_div
pub type AnauTsenClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    ///Bit 0 - power up tsen
    #[inline(always)]
    pub fn anau_tsen_pu(&self) -> AnauTsenPuR {
        AnauTsenPuR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - resetb for tsen
    #[inline(always)]
    pub fn anau_tsen_rstb(&self) -> AnauTsenRstbR {
        AnauTsenRstbR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - enable tsen run
    #[inline(always)]
    pub fn anau_tsen_run(&self) -> AnauTsenRunR {
        AnauTsenRunR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:5 - bias current selection to tune vba
    #[inline(always)]
    pub fn anau_tsen_ig_vbe(&self) -> AnauTsenIgVbeR {
        AnauTsenIgVbeR::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:7 - select internal clock frequency
    #[inline(always)]
    pub fn anau_tsen_fck_sel(&self) -> AnauTsenFckSelR {
        AnauTsenFckSelR::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8 - signature-mode enable
    #[inline(always)]
    pub fn anau_tsen_sgn_en(&self) -> AnauTsenSgnEnR {
        AnauTsenSgnEnR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - serial-parallel output selection
    #[inline(always)]
    pub fn anau_tsen_ser_par_sel(&self) -> AnauTsenSerParSelR {
        AnauTsenSerParSelR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - tsen ready
    #[inline(always)]
    pub fn anau_tsen_rdy(&self) -> AnauTsenRdyR {
        AnauTsenRdyR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Enable tsen digital module
    #[inline(always)]
    pub fn anau_tsen_en(&self) -> AnauTsenEnR {
        AnauTsenEnR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:17 - gen tsen clk by divide hclk by anau_tsen_clk_div
    #[inline(always)]
    pub fn anau_tsen_clk_div(&self) -> AnauTsenClkDivR {
        AnauTsenClkDivR::new(((self.bits >> 12) & 0x3f) as u8)
    }
    ///Bits 18:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSEN_CTRL_REG")
            .field("rsvd", &self.rsvd())
            .field("anau_tsen_clk_div", &self.anau_tsen_clk_div())
            .field("anau_tsen_en", &self.anau_tsen_en())
            .field("anau_tsen_rdy", &self.anau_tsen_rdy())
            .field("anau_tsen_ser_par_sel", &self.anau_tsen_ser_par_sel())
            .field("anau_tsen_sgn_en", &self.anau_tsen_sgn_en())
            .field("anau_tsen_fck_sel", &self.anau_tsen_fck_sel())
            .field("anau_tsen_ig_vbe", &self.anau_tsen_ig_vbe())
            .field("anau_tsen_run", &self.anau_tsen_run())
            .field("anau_tsen_rstb", &self.anau_tsen_rstb())
            .field("anau_tsen_pu", &self.anau_tsen_pu())
            .finish()
    }
}
impl W {
    ///Bit 0 - power up tsen
    #[inline(always)]
    pub fn anau_tsen_pu(&mut self) -> AnauTsenPuW<TSEN_CTRL_REGrs> {
        AnauTsenPuW::new(self, 0)
    }
    ///Bit 1 - resetb for tsen
    #[inline(always)]
    pub fn anau_tsen_rstb(&mut self) -> AnauTsenRstbW<TSEN_CTRL_REGrs> {
        AnauTsenRstbW::new(self, 1)
    }
    ///Bit 2 - enable tsen run
    #[inline(always)]
    pub fn anau_tsen_run(&mut self) -> AnauTsenRunW<TSEN_CTRL_REGrs> {
        AnauTsenRunW::new(self, 2)
    }
    ///Bits 3:5 - bias current selection to tune vba
    #[inline(always)]
    pub fn anau_tsen_ig_vbe(&mut self) -> AnauTsenIgVbeW<TSEN_CTRL_REGrs> {
        AnauTsenIgVbeW::new(self, 3)
    }
    ///Bits 6:7 - select internal clock frequency
    #[inline(always)]
    pub fn anau_tsen_fck_sel(&mut self) -> AnauTsenFckSelW<TSEN_CTRL_REGrs> {
        AnauTsenFckSelW::new(self, 6)
    }
    ///Bit 8 - signature-mode enable
    #[inline(always)]
    pub fn anau_tsen_sgn_en(&mut self) -> AnauTsenSgnEnW<TSEN_CTRL_REGrs> {
        AnauTsenSgnEnW::new(self, 8)
    }
    ///Bit 9 - serial-parallel output selection
    #[inline(always)]
    pub fn anau_tsen_ser_par_sel(&mut self) -> AnauTsenSerParSelW<TSEN_CTRL_REGrs> {
        AnauTsenSerParSelW::new(self, 9)
    }
    ///Bit 10 - tsen ready
    #[inline(always)]
    pub fn anau_tsen_rdy(&mut self) -> AnauTsenRdyW<TSEN_CTRL_REGrs> {
        AnauTsenRdyW::new(self, 10)
    }
    ///Bit 11 - Enable tsen digital module
    #[inline(always)]
    pub fn anau_tsen_en(&mut self) -> AnauTsenEnW<TSEN_CTRL_REGrs> {
        AnauTsenEnW::new(self, 11)
    }
    ///Bits 12:17 - gen tsen clk by divide hclk by anau_tsen_clk_div
    #[inline(always)]
    pub fn anau_tsen_clk_div(&mut self) -> AnauTsenClkDivW<TSEN_CTRL_REGrs> {
        AnauTsenClkDivW::new(self, 12)
    }
    ///Bits 18:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<TSEN_CTRL_REGrs> {
        RsvdW::new(self, 18)
    }
}
///TSEN Analog Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`tsen_ctrl_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsen_ctrl_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TSEN_CTRL_REGrs;
impl crate::RegisterSpec for TSEN_CTRL_REGrs {
    type Ux = u32;
}
///`read()` method returns [`tsen_ctrl_reg::R`](R) reader structure
impl crate::Readable for TSEN_CTRL_REGrs {}
///`write(|w| ..)` method takes [`tsen_ctrl_reg::W`](W) writer structure
impl crate::Writable for TSEN_CTRL_REGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TSEN_CTRL_REG to value 0x0471_4a44
impl crate::Resettable for TSEN_CTRL_REGrs {
    const RESET_VALUE: u32 = 0x0471_4a44;
}
