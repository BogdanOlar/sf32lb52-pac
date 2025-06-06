///Register `HRC_CR` reader
pub type R = crate::R<HRC_CRrs>;
///Register `HRC_CR` writer
pub type W = crate::W<HRC_CRrs>;
///Field `EN` reader -
pub type EnR = crate::BitReader;
///Field `EN` writer -
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LDO_VREF` reader -
pub type LdoVrefR = crate::FieldReader;
///Field `LDO_VREF` writer -
pub type LdoVrefW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `FREQ_TRIM` reader -
pub type FreqTrimR = crate::FieldReader<u16>;
///Field `FREQ_TRIM` writer -
pub type FreqTrimW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `TEMP_TRIM` reader -
pub type TempTrimR = crate::FieldReader;
///Field `TEMP_TRIM` writer -
pub type TempTrimW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CLK96M_EN` reader -
pub type Clk96mEnR = crate::BitReader;
///Field `CLK96M_EN` writer -
pub type Clk96mEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::BitReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLKHP_EN` reader -
pub type ClkhpEnR = crate::BitReader;
///Field `CLKHP_EN` writer -
pub type ClkhpEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLKHP_SEL` reader -
pub type ClkhpSelR = crate::FieldReader;
///Field `CLKHP_SEL` writer -
pub type ClkhpSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CLKHP_STR` reader -
pub type ClkhpStrR = crate::FieldReader;
///Field `CLKHP_STR` writer -
pub type ClkhpStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CLKLP_EN` reader -
pub type ClklpEnR = crate::BitReader;
///Field `CLKLP_EN` writer -
pub type ClklpEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLKLP_SEL` reader -
pub type ClklpSelR = crate::FieldReader;
///Field `CLKLP_SEL` writer -
pub type ClklpSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CLKLP_STR` reader -
pub type ClklpStrR = crate::FieldReader;
///Field `CLKLP_STR` writer -
pub type ClklpStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RSVD` reader -
pub type RsvdR = crate::BitReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DLY` reader - number of cycles for BG ready. 0 - one cycle of CLK_LP; 1 - two cycles of CLK_LP
pub type DlyR = crate::BitReader;
///Field `DLY` writer - number of cycles for BG ready. 0 - one cycle of CLK_LP; 1 - two cycles of CLK_LP
pub type DlyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    ///Bits 1:4
    #[inline(always)]
    pub fn ldo_vref(&self) -> LdoVrefR {
        LdoVrefR::new(((self.bits >> 1) & 0x0f) as u8)
    }
    ///Bits 5:14
    #[inline(always)]
    pub fn freq_trim(&self) -> FreqTrimR {
        FreqTrimR::new(((self.bits >> 5) & 0x03ff) as u16)
    }
    ///Bits 15:17
    #[inline(always)]
    pub fn temp_trim(&self) -> TempTrimR {
        TempTrimR::new(((self.bits >> 15) & 7) as u8)
    }
    ///Bit 18
    #[inline(always)]
    pub fn clk96m_en(&self) -> Clk96mEnR {
        Clk96mEnR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20
    #[inline(always)]
    pub fn clkhp_en(&self) -> ClkhpEnR {
        ClkhpEnR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22
    #[inline(always)]
    pub fn clkhp_sel(&self) -> ClkhpSelR {
        ClkhpSelR::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bits 23:24
    #[inline(always)]
    pub fn clkhp_str(&self) -> ClkhpStrR {
        ClkhpStrR::new(((self.bits >> 23) & 3) as u8)
    }
    ///Bit 25
    #[inline(always)]
    pub fn clklp_en(&self) -> ClklpEnR {
        ClklpEnR::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bits 26:27
    #[inline(always)]
    pub fn clklp_sel(&self) -> ClklpSelR {
        ClklpSelR::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29
    #[inline(always)]
    pub fn clklp_str(&self) -> ClklpStrR {
        ClklpStrR::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bit 30
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - number of cycles for BG ready. 0 - one cycle of CLK_LP; 1 - two cycles of CLK_LP
    #[inline(always)]
    pub fn dly(&self) -> DlyR {
        DlyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HRC_CR")
            .field("dly", &self.dly())
            .field("rsvd", &self.rsvd())
            .field("clklp_str", &self.clklp_str())
            .field("clklp_sel", &self.clklp_sel())
            .field("clklp_en", &self.clklp_en())
            .field("clkhp_str", &self.clkhp_str())
            .field("clkhp_sel", &self.clkhp_sel())
            .field("clkhp_en", &self.clkhp_en())
            .field("rsvd2", &self.rsvd2())
            .field("clk96m_en", &self.clk96m_en())
            .field("temp_trim", &self.temp_trim())
            .field("freq_trim", &self.freq_trim())
            .field("ldo_vref", &self.ldo_vref())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    pub fn en(&mut self) -> EnW<HRC_CRrs> {
        EnW::new(self, 0)
    }
    ///Bits 1:4
    #[inline(always)]
    pub fn ldo_vref(&mut self) -> LdoVrefW<HRC_CRrs> {
        LdoVrefW::new(self, 1)
    }
    ///Bits 5:14
    #[inline(always)]
    pub fn freq_trim(&mut self) -> FreqTrimW<HRC_CRrs> {
        FreqTrimW::new(self, 5)
    }
    ///Bits 15:17
    #[inline(always)]
    pub fn temp_trim(&mut self) -> TempTrimW<HRC_CRrs> {
        TempTrimW::new(self, 15)
    }
    ///Bit 18
    #[inline(always)]
    pub fn clk96m_en(&mut self) -> Clk96mEnW<HRC_CRrs> {
        Clk96mEnW::new(self, 18)
    }
    ///Bit 19
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<HRC_CRrs> {
        Rsvd2W::new(self, 19)
    }
    ///Bit 20
    #[inline(always)]
    pub fn clkhp_en(&mut self) -> ClkhpEnW<HRC_CRrs> {
        ClkhpEnW::new(self, 20)
    }
    ///Bits 21:22
    #[inline(always)]
    pub fn clkhp_sel(&mut self) -> ClkhpSelW<HRC_CRrs> {
        ClkhpSelW::new(self, 21)
    }
    ///Bits 23:24
    #[inline(always)]
    pub fn clkhp_str(&mut self) -> ClkhpStrW<HRC_CRrs> {
        ClkhpStrW::new(self, 23)
    }
    ///Bit 25
    #[inline(always)]
    pub fn clklp_en(&mut self) -> ClklpEnW<HRC_CRrs> {
        ClklpEnW::new(self, 25)
    }
    ///Bits 26:27
    #[inline(always)]
    pub fn clklp_sel(&mut self) -> ClklpSelW<HRC_CRrs> {
        ClklpSelW::new(self, 26)
    }
    ///Bits 28:29
    #[inline(always)]
    pub fn clklp_str(&mut self) -> ClklpStrW<HRC_CRrs> {
        ClklpStrW::new(self, 28)
    }
    ///Bit 30
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<HRC_CRrs> {
        RsvdW::new(self, 30)
    }
    ///Bit 31 - number of cycles for BG ready. 0 - one cycle of CLK_LP; 1 - two cycles of CLK_LP
    #[inline(always)]
    pub fn dly(&mut self) -> DlyW<HRC_CRrs> {
        DlyW::new(self, 31)
    }
}
///HRC48 Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`hrc_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hrc_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct HRC_CRrs;
impl crate::RegisterSpec for HRC_CRrs {
    type Ux = u32;
}
///`read()` method returns [`hrc_cr::R`](R) reader structure
impl crate::Readable for HRC_CRrs {}
///`write(|w| ..)` method takes [`hrc_cr::W`](W) writer structure
impl crate::Writable for HRC_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HRC_CR to value 0
impl crate::Resettable for HRC_CRrs {
    const RESET_VALUE: u32 = 0;
}
