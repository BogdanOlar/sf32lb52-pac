///Register `HXT_CR2` reader
pub type R = crate::R<HXT_CR2rs>;
///Register `HXT_CR2` writer
pub type W = crate::W<HXT_CR2rs>;
///Field `AGC_EN` reader -
pub type AgcEnR = crate::BitReader;
///Field `AGC_EN` writer -
pub type AgcEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AGC_ISTART_SEL` reader -
pub type AgcIstartSelR = crate::BitReader;
///Field `AGC_ISTART_SEL` writer -
pub type AgcIstartSelW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AGC_VTH` reader -
pub type AgcVthR = crate::FieldReader;
///Field `AGC_VTH` writer -
pub type AgcVthW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AGC_VINDC` reader -
pub type AgcVindcR = crate::FieldReader;
///Field `AGC_VINDC` writer -
pub type AgcVindcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ACBUF_SEL` reader -
pub type AcbufSelR = crate::FieldReader;
///Field `ACBUF_SEL` writer -
pub type AcbufSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ACBUF_RSEL` reader -
pub type AcbufRselR = crate::BitReader;
///Field `ACBUF_RSEL` writer -
pub type AcbufRselW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUF_SEL2` reader -
pub type BufSel2R = crate::FieldReader;
///Field `BUF_SEL2` writer -
pub type BufSel2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BUF_SEL3` reader -
pub type BufSel3R = crate::FieldReader;
///Field `BUF_SEL3` writer -
pub type BufSel3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IDAC_EN` reader -
pub type IdacEnR = crate::BitReader;
///Field `IDAC_EN` writer -
pub type IdacEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDAC` reader -
pub type IdacR = crate::FieldReader<u16>;
///Field `IDAC` writer -
pub type IdacW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `SDADC_CLKIN_EN` reader -
pub type SdadcClkinEnR = crate::BitReader;
///Field `SDADC_CLKIN_EN` writer -
pub type SdadcClkinEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDADC_CLKDIV1_SEL` reader -
pub type SdadcClkdiv1SelR = crate::FieldReader;
///Field `SDADC_CLKDIV1_SEL` writer -
pub type SdadcClkdiv1SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SDADC_CLKDIV2_SEL` reader -
pub type SdadcClkdiv2SelR = crate::FieldReader;
///Field `SDADC_CLKDIV2_SEL` writer -
pub type SdadcClkdiv2SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SLEEP_EN` reader -
pub type SleepEnR = crate::BitReader;
///Field `SLEEP_EN` writer -
pub type SleepEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn agc_en(&self) -> AgcEnR {
        AgcEnR::new((self.bits & 1) != 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn agc_istart_sel(&self) -> AgcIstartSelR {
        AgcIstartSelR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:5
    #[inline(always)]
    pub fn agc_vth(&self) -> AgcVthR {
        AgcVthR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    ///Bits 6:7
    #[inline(always)]
    pub fn agc_vindc(&self) -> AgcVindcR {
        AgcVindcR::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9
    #[inline(always)]
    pub fn acbuf_sel(&self) -> AcbufSelR {
        AcbufSelR::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10
    #[inline(always)]
    pub fn acbuf_rsel(&self) -> AcbufRselR {
        AcbufRselR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 11:12
    #[inline(always)]
    pub fn buf_sel2(&self) -> BufSel2R {
        BufSel2R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bits 13:14
    #[inline(always)]
    pub fn buf_sel3(&self) -> BufSel3R {
        BufSel3R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 15
    #[inline(always)]
    pub fn idac_en(&self) -> IdacEnR {
        IdacEnR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:25
    #[inline(always)]
    pub fn idac(&self) -> IdacR {
        IdacR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    ///Bit 26
    #[inline(always)]
    pub fn sdadc_clkin_en(&self) -> SdadcClkinEnR {
        SdadcClkinEnR::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bits 27:28
    #[inline(always)]
    pub fn sdadc_clkdiv1_sel(&self) -> SdadcClkdiv1SelR {
        SdadcClkdiv1SelR::new(((self.bits >> 27) & 3) as u8)
    }
    ///Bits 29:30
    #[inline(always)]
    pub fn sdadc_clkdiv2_sel(&self) -> SdadcClkdiv2SelR {
        SdadcClkdiv2SelR::new(((self.bits >> 29) & 3) as u8)
    }
    ///Bit 31
    #[inline(always)]
    pub fn sleep_en(&self) -> SleepEnR {
        SleepEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HXT_CR2")
            .field("sleep_en", &self.sleep_en())
            .field("sdadc_clkdiv2_sel", &self.sdadc_clkdiv2_sel())
            .field("sdadc_clkdiv1_sel", &self.sdadc_clkdiv1_sel())
            .field("sdadc_clkin_en", &self.sdadc_clkin_en())
            .field("idac", &self.idac())
            .field("idac_en", &self.idac_en())
            .field("buf_sel3", &self.buf_sel3())
            .field("buf_sel2", &self.buf_sel2())
            .field("acbuf_rsel", &self.acbuf_rsel())
            .field("acbuf_sel", &self.acbuf_sel())
            .field("agc_vindc", &self.agc_vindc())
            .field("agc_vth", &self.agc_vth())
            .field("agc_istart_sel", &self.agc_istart_sel())
            .field("agc_en", &self.agc_en())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    pub fn agc_en(&mut self) -> AgcEnW<HXT_CR2rs> {
        AgcEnW::new(self, 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn agc_istart_sel(&mut self) -> AgcIstartSelW<HXT_CR2rs> {
        AgcIstartSelW::new(self, 1)
    }
    ///Bits 2:5
    #[inline(always)]
    pub fn agc_vth(&mut self) -> AgcVthW<HXT_CR2rs> {
        AgcVthW::new(self, 2)
    }
    ///Bits 6:7
    #[inline(always)]
    pub fn agc_vindc(&mut self) -> AgcVindcW<HXT_CR2rs> {
        AgcVindcW::new(self, 6)
    }
    ///Bits 8:9
    #[inline(always)]
    pub fn acbuf_sel(&mut self) -> AcbufSelW<HXT_CR2rs> {
        AcbufSelW::new(self, 8)
    }
    ///Bit 10
    #[inline(always)]
    pub fn acbuf_rsel(&mut self) -> AcbufRselW<HXT_CR2rs> {
        AcbufRselW::new(self, 10)
    }
    ///Bits 11:12
    #[inline(always)]
    pub fn buf_sel2(&mut self) -> BufSel2W<HXT_CR2rs> {
        BufSel2W::new(self, 11)
    }
    ///Bits 13:14
    #[inline(always)]
    pub fn buf_sel3(&mut self) -> BufSel3W<HXT_CR2rs> {
        BufSel3W::new(self, 13)
    }
    ///Bit 15
    #[inline(always)]
    pub fn idac_en(&mut self) -> IdacEnW<HXT_CR2rs> {
        IdacEnW::new(self, 15)
    }
    ///Bits 16:25
    #[inline(always)]
    pub fn idac(&mut self) -> IdacW<HXT_CR2rs> {
        IdacW::new(self, 16)
    }
    ///Bit 26
    #[inline(always)]
    pub fn sdadc_clkin_en(&mut self) -> SdadcClkinEnW<HXT_CR2rs> {
        SdadcClkinEnW::new(self, 26)
    }
    ///Bits 27:28
    #[inline(always)]
    pub fn sdadc_clkdiv1_sel(&mut self) -> SdadcClkdiv1SelW<HXT_CR2rs> {
        SdadcClkdiv1SelW::new(self, 27)
    }
    ///Bits 29:30
    #[inline(always)]
    pub fn sdadc_clkdiv2_sel(&mut self) -> SdadcClkdiv2SelW<HXT_CR2rs> {
        SdadcClkdiv2SelW::new(self, 29)
    }
    ///Bit 31
    #[inline(always)]
    pub fn sleep_en(&mut self) -> SleepEnW<HXT_CR2rs> {
        SleepEnW::new(self, 31)
    }
}
///HXT48 Control Register 2
///
///You can [`read`](crate::Reg::read) this register and get [`hxt_cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hxt_cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct HXT_CR2rs;
impl crate::RegisterSpec for HXT_CR2rs {
    type Ux = u32;
}
///`read()` method returns [`hxt_cr2::R`](R) reader structure
impl crate::Readable for HXT_CR2rs {}
///`write(|w| ..)` method takes [`hxt_cr2::W`](W) writer structure
impl crate::Writable for HXT_CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HXT_CR2 to value 0
impl crate::Resettable for HXT_CR2rs {
    const RESET_VALUE: u32 = 0;
}
