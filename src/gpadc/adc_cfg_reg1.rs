///Register `ADC_CFG_REG1` reader
pub type R = crate::R<ADC_CFG_REG1rs>;
///Register `ADC_CFG_REG1` writer
pub type W = crate::W<ADC_CFG_REG1rs>;
///Field `ANAU_GPADC_CMREF_FAST_EN` reader -
pub type AnauGpadcCmrefFastEnR = crate::BitReader;
///Field `ANAU_GPADC_CMREF_FAST_EN` writer -
pub type AnauGpadcCmrefFastEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::BitReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ANAU_GPADC_P_INT_EN` reader -
pub type AnauGpadcPIntEnR = crate::BitReader;
///Field `ANAU_GPADC_P_INT_EN` writer -
pub type AnauGpadcPIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ANAU_GPADC_CL_DLY` reader -
pub type AnauGpadcClDlyR = crate::FieldReader;
///Field `ANAU_GPADC_CL_DLY` writer -
pub type AnauGpadcClDlyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ANAU_GPADC_EN_V18` reader -
pub type AnauGpadcEnV18R = crate::BitReader;
///Field `ANAU_GPADC_EN_V18` writer -
pub type AnauGpadcEnV18W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ANAU_GPADC_SE` reader - Set GPADC in single-ended mode, signal range at P-input: 0 ~ VREF
pub type AnauGpadcSeR = crate::BitReader;
///Field `ANAU_GPADC_SE` writer - Set GPADC in single-ended mode, signal range at P-input: 0 ~ VREF
pub type AnauGpadcSeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ANAU_GPADC_MUTE` reader - Short GPADC P and N input to CMREF, i.e., VREF/2
pub type AnauGpadcMuteR = crate::BitReader;
///Field `ANAU_GPADC_MUTE` writer - Short GPADC P and N input to CMREF, i.e., VREF/2
pub type AnauGpadcMuteW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ANAU_GPADC_SEL_NCH` reader - Select N-side input channel for GPADC, 0 for channel 0, 7 for channel 7, effective when force on
pub type AnauGpadcSelNchR = crate::FieldReader;
///Field `ANAU_GPADC_SEL_NCH` writer - Select N-side input channel for GPADC, 0 for channel 0, 7 for channel 7, effective when force on
pub type AnauGpadcSelNchW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ANAU_GPADC_SEL_PCH` reader - Select P-side input channel for GPADC, 0 for channel 0, 7 for channel 7, effective when force on
pub type AnauGpadcSelPchR = crate::FieldReader;
///Field `ANAU_GPADC_SEL_PCH` writer - Select P-side input channel for GPADC, 0 for channel 0, 7 for channel 7, effective when force on
pub type AnauGpadcSelPchW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ANAU_GPADC_LDOVREF_SEL` reader - Set reference voltage for LDOREF, range = 0.35V(0) ~ 0.65V(15), step = 20mV
pub type AnauGpadcLdovrefSelR = crate::FieldReader;
///Field `ANAU_GPADC_LDOVREF_SEL` writer - Set reference voltage for LDOREF, range = 0.35V(0) ~ 0.65V(15), step = 20mV
pub type AnauGpadcLdovrefSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ANAU_GPADC_LDOREF_EN` reader - Enable LDORF for ADC VREF
pub type AnauGpadcLdorefEnR = crate::BitReader;
///Field `ANAU_GPADC_LDOREF_EN` writer - Enable LDORF for ADC VREF
pub type AnauGpadcLdorefEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ANAU_GPADC_VSP` reader - Set comparator input CM in sampling phase, 0.539V (0) / 0.578V (1) / 0.642V (2) / 0.784V (3)
pub type AnauGpadcVspR = crate::FieldReader;
///Field `ANAU_GPADC_VSP` writer - Set comparator input CM in sampling phase, 0.539V (0) / 0.578V (1) / 0.642V (2) / 0.784V (3)
pub type AnauGpadcVspW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ANAU_GPADC_CMPCL` reader - Tune ADC comparator CL= 3: 40f, range: 10fF (0) ~ 80fF (7) / 10fF step
pub type AnauGpadcCmpclR = crate::FieldReader;
///Field `ANAU_GPADC_CMPCL` writer - Tune ADC comparator CL= 3: 40f, range: 10fF (0) ~ 80fF (7) / 10fF step
pub type AnauGpadcCmpclW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ANAU_GPADC_CMM` reader - Tune CDAC CM voltage 375mV range (increasing) / 25mV step, 8: for 0.5V Vcm,in
pub type AnauGpadcCmmR = crate::FieldReader;
///Field `ANAU_GPADC_CMM` writer - Tune CDAC CM voltage 375mV range (increasing) / 25mV step, 8: for 0.5V Vcm,in
pub type AnauGpadcCmmW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn anau_gpadc_cmref_fast_en(&self) -> AnauGpadcCmrefFastEnR {
        AnauGpadcCmrefFastEnR::new((self.bits & 1) != 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2
    #[inline(always)]
    pub fn anau_gpadc_p_int_en(&self) -> AnauGpadcPIntEnR {
        AnauGpadcPIntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:5
    #[inline(always)]
    pub fn anau_gpadc_cl_dly(&self) -> AnauGpadcClDlyR {
        AnauGpadcClDlyR::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bit 6
    #[inline(always)]
    pub fn anau_gpadc_en_v18(&self) -> AnauGpadcEnV18R {
        AnauGpadcEnV18R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Set GPADC in single-ended mode, signal range at P-input: 0 ~ VREF
    #[inline(always)]
    pub fn anau_gpadc_se(&self) -> AnauGpadcSeR {
        AnauGpadcSeR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Short GPADC P and N input to CMREF, i.e., VREF/2
    #[inline(always)]
    pub fn anau_gpadc_mute(&self) -> AnauGpadcMuteR {
        AnauGpadcMuteR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:11 - Select N-side input channel for GPADC, 0 for channel 0, 7 for channel 7, effective when force on
    #[inline(always)]
    pub fn anau_gpadc_sel_nch(&self) -> AnauGpadcSelNchR {
        AnauGpadcSelNchR::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 12:14 - Select P-side input channel for GPADC, 0 for channel 0, 7 for channel 7, effective when force on
    #[inline(always)]
    pub fn anau_gpadc_sel_pch(&self) -> AnauGpadcSelPchR {
        AnauGpadcSelPchR::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 15:18 - Set reference voltage for LDOREF, range = 0.35V(0) ~ 0.65V(15), step = 20mV
    #[inline(always)]
    pub fn anau_gpadc_ldovref_sel(&self) -> AnauGpadcLdovrefSelR {
        AnauGpadcLdovrefSelR::new(((self.bits >> 15) & 0x0f) as u8)
    }
    ///Bit 19 - Enable LDORF for ADC VREF
    #[inline(always)]
    pub fn anau_gpadc_ldoref_en(&self) -> AnauGpadcLdorefEnR {
        AnauGpadcLdorefEnR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:21 - Set comparator input CM in sampling phase, 0.539V (0) / 0.578V (1) / 0.642V (2) / 0.784V (3)
    #[inline(always)]
    pub fn anau_gpadc_vsp(&self) -> AnauGpadcVspR {
        AnauGpadcVspR::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:24 - Tune ADC comparator CL= 3: 40f, range: 10fF (0) ~ 80fF (7) / 10fF step
    #[inline(always)]
    pub fn anau_gpadc_cmpcl(&self) -> AnauGpadcCmpclR {
        AnauGpadcCmpclR::new(((self.bits >> 22) & 7) as u8)
    }
    ///Bits 25:29 - Tune CDAC CM voltage 375mV range (increasing) / 25mV step, 8: for 0.5V Vcm,in
    #[inline(always)]
    pub fn anau_gpadc_cmm(&self) -> AnauGpadcCmmR {
        AnauGpadcCmmR::new(((self.bits >> 25) & 0x1f) as u8)
    }
    ///Bits 30:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CFG_REG1")
            .field("rsvd", &self.rsvd())
            .field("anau_gpadc_cmm", &self.anau_gpadc_cmm())
            .field("anau_gpadc_cmpcl", &self.anau_gpadc_cmpcl())
            .field("anau_gpadc_vsp", &self.anau_gpadc_vsp())
            .field("anau_gpadc_ldoref_en", &self.anau_gpadc_ldoref_en())
            .field("anau_gpadc_ldovref_sel", &self.anau_gpadc_ldovref_sel())
            .field("anau_gpadc_sel_pch", &self.anau_gpadc_sel_pch())
            .field("anau_gpadc_sel_nch", &self.anau_gpadc_sel_nch())
            .field("anau_gpadc_mute", &self.anau_gpadc_mute())
            .field("anau_gpadc_se", &self.anau_gpadc_se())
            .field("anau_gpadc_en_v18", &self.anau_gpadc_en_v18())
            .field("anau_gpadc_cl_dly", &self.anau_gpadc_cl_dly())
            .field("anau_gpadc_p_int_en", &self.anau_gpadc_p_int_en())
            .field("rsvd2", &self.rsvd2())
            .field("anau_gpadc_cmref_fast_en", &self.anau_gpadc_cmref_fast_en())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    pub fn anau_gpadc_cmref_fast_en(&mut self) -> AnauGpadcCmrefFastEnW<ADC_CFG_REG1rs> {
        AnauGpadcCmrefFastEnW::new(self, 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<ADC_CFG_REG1rs> {
        Rsvd2W::new(self, 1)
    }
    ///Bit 2
    #[inline(always)]
    pub fn anau_gpadc_p_int_en(&mut self) -> AnauGpadcPIntEnW<ADC_CFG_REG1rs> {
        AnauGpadcPIntEnW::new(self, 2)
    }
    ///Bits 3:5
    #[inline(always)]
    pub fn anau_gpadc_cl_dly(&mut self) -> AnauGpadcClDlyW<ADC_CFG_REG1rs> {
        AnauGpadcClDlyW::new(self, 3)
    }
    ///Bit 6
    #[inline(always)]
    pub fn anau_gpadc_en_v18(&mut self) -> AnauGpadcEnV18W<ADC_CFG_REG1rs> {
        AnauGpadcEnV18W::new(self, 6)
    }
    ///Bit 7 - Set GPADC in single-ended mode, signal range at P-input: 0 ~ VREF
    #[inline(always)]
    pub fn anau_gpadc_se(&mut self) -> AnauGpadcSeW<ADC_CFG_REG1rs> {
        AnauGpadcSeW::new(self, 7)
    }
    ///Bit 8 - Short GPADC P and N input to CMREF, i.e., VREF/2
    #[inline(always)]
    pub fn anau_gpadc_mute(&mut self) -> AnauGpadcMuteW<ADC_CFG_REG1rs> {
        AnauGpadcMuteW::new(self, 8)
    }
    ///Bits 9:11 - Select N-side input channel for GPADC, 0 for channel 0, 7 for channel 7, effective when force on
    #[inline(always)]
    pub fn anau_gpadc_sel_nch(&mut self) -> AnauGpadcSelNchW<ADC_CFG_REG1rs> {
        AnauGpadcSelNchW::new(self, 9)
    }
    ///Bits 12:14 - Select P-side input channel for GPADC, 0 for channel 0, 7 for channel 7, effective when force on
    #[inline(always)]
    pub fn anau_gpadc_sel_pch(&mut self) -> AnauGpadcSelPchW<ADC_CFG_REG1rs> {
        AnauGpadcSelPchW::new(self, 12)
    }
    ///Bits 15:18 - Set reference voltage for LDOREF, range = 0.35V(0) ~ 0.65V(15), step = 20mV
    #[inline(always)]
    pub fn anau_gpadc_ldovref_sel(&mut self) -> AnauGpadcLdovrefSelW<ADC_CFG_REG1rs> {
        AnauGpadcLdovrefSelW::new(self, 15)
    }
    ///Bit 19 - Enable LDORF for ADC VREF
    #[inline(always)]
    pub fn anau_gpadc_ldoref_en(&mut self) -> AnauGpadcLdorefEnW<ADC_CFG_REG1rs> {
        AnauGpadcLdorefEnW::new(self, 19)
    }
    ///Bits 20:21 - Set comparator input CM in sampling phase, 0.539V (0) / 0.578V (1) / 0.642V (2) / 0.784V (3)
    #[inline(always)]
    pub fn anau_gpadc_vsp(&mut self) -> AnauGpadcVspW<ADC_CFG_REG1rs> {
        AnauGpadcVspW::new(self, 20)
    }
    ///Bits 22:24 - Tune ADC comparator CL= 3: 40f, range: 10fF (0) ~ 80fF (7) / 10fF step
    #[inline(always)]
    pub fn anau_gpadc_cmpcl(&mut self) -> AnauGpadcCmpclW<ADC_CFG_REG1rs> {
        AnauGpadcCmpclW::new(self, 22)
    }
    ///Bits 25:29 - Tune CDAC CM voltage 375mV range (increasing) / 25mV step, 8: for 0.5V Vcm,in
    #[inline(always)]
    pub fn anau_gpadc_cmm(&mut self) -> AnauGpadcCmmW<ADC_CFG_REG1rs> {
        AnauGpadcCmmW::new(self, 25)
    }
    ///Bits 30:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<ADC_CFG_REG1rs> {
        RsvdW::new(self, 30)
    }
}
///ADC Analog Config Register 1
///
///You can [`read`](crate::Reg::read) this register and get [`adc_cfg_reg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_cfg_reg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ADC_CFG_REG1rs;
impl crate::RegisterSpec for ADC_CFG_REG1rs {
    type Ux = u32;
}
///`read()` method returns [`adc_cfg_reg1::R`](R) reader structure
impl crate::Readable for ADC_CFG_REG1rs {}
///`write(|w| ..)` method takes [`adc_cfg_reg1::W`](W) writer structure
impl crate::Writable for ADC_CFG_REG1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC_CFG_REG1 to value 0x0471_4a44
impl crate::Resettable for ADC_CFG_REG1rs {
    const RESET_VALUE: u32 = 0x0471_4a44;
}
