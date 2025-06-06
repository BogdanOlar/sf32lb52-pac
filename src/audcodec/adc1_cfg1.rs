///Register `ADC1_CFG1` reader
pub type R = crate::R<ADC1_CFG1rs>;
///Register `ADC1_CFG1` writer
pub type W = crate::W<ADC1_CFG1rs>;
///Field `PERI_BM` reader - peripheral circuits biasmode
pub type PeriBmR = crate::FieldReader;
///Field `PERI_BM` writer - peripheral circuits biasmode
pub type PeriBmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CLKOUT_INV` reader - inverse output clock
pub type ClkoutInvR = crate::BitReader;
///Field `CLKOUT_INV` writer - inverse output clock
pub type ClkoutInvW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VCMST` reader - VCM quick settling
pub type VcmstR = crate::BitReader;
///Field `VCMST` writer - VCM quick settling
pub type VcmstW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FCHOP_SEL` reader - chopping frequncy 0x0:÷8 0x1:÷16 0x2:÷32 0x3:÷64
pub type FchopSelR = crate::FieldReader;
///Field `FCHOP_SEL` writer - chopping frequncy 0x0:÷8 0x1:÷16 0x2:÷32 0x3:÷64
pub type FchopSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `VREF_SEL` reader - vref code from proper vcm in flash7 0x0:1.2V 0x1:1.4V 0x7:2.6V
pub type VrefSelR = crate::FieldReader;
///Field `VREF_SEL` writer - vref code from proper vcm in flash7 0x0:1.2V 0x1:1.4V 0x7:2.6V
pub type VrefSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `BM_INT2` reader - bias mode of 2nd and 3rd opamp
pub type BmInt2R = crate::FieldReader;
///Field `BM_INT2` writer - bias mode of 2nd and 3rd opamp
pub type BmInt2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `BM_INT1` reader - bias mode of first opamp
pub type BmInt1R = crate::FieldReader;
///Field `BM_INT1` writer - bias mode of first opamp
pub type BmInt1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `VST_SEL` reader - start voltage 0x0:VCM+200mV 0x7:VCM+550mV
pub type VstSelR = crate::FieldReader;
///Field `VST_SEL` writer - start voltage 0x0:VCM+200mV 0x7:VCM+550mV
pub type VstSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `GC` reader - gaincode: 0x0:-6dB 0x1:0dB ... 0x4:18dB
pub type GcR = crate::FieldReader;
///Field `GC` writer - gaincode: 0x0:-6dB 0x1:0dB ... 0x4:18dB
pub type GcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DACN_EN` reader - enable negative DAC1
pub type DacnEnR = crate::BitReader;
///Field `DACN_EN` writer - enable negative DAC1
pub type DacnEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIFF_EN` reader - enable differential input mode
pub type DiffEnR = crate::BitReader;
///Field `DIFF_EN` writer - enable differential input mode
pub type DiffEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSP` reader - sampling frequency: 0x0:9.6M 0x1:8.82M 0x2:4.8M 0x3:4.41M
pub type FspR = crate::FieldReader;
///Field `FSP` writer - sampling frequency: 0x0:9.6M 0x1:8.82M 0x2:4.8M 0x3:4.41M
pub type FspW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - peripheral circuits biasmode
    #[inline(always)]
    pub fn peri_bm(&self) -> PeriBmR {
        PeriBmR::new((self.bits & 3) as u8)
    }
    ///Bit 2 - inverse output clock
    #[inline(always)]
    pub fn clkout_inv(&self) -> ClkoutInvR {
        ClkoutInvR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - VCM quick settling
    #[inline(always)]
    pub fn vcmst(&self) -> VcmstR {
        VcmstR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - chopping frequncy 0x0:÷8 0x1:÷16 0x2:÷32 0x3:÷64
    #[inline(always)]
    pub fn fchop_sel(&self) -> FchopSelR {
        FchopSelR::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:8 - vref code from proper vcm in flash7 0x0:1.2V 0x1:1.4V 0x7:2.6V
    #[inline(always)]
    pub fn vref_sel(&self) -> VrefSelR {
        VrefSelR::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 9:11 - bias mode of 2nd and 3rd opamp
    #[inline(always)]
    pub fn bm_int2(&self) -> BmInt2R {
        BmInt2R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 12:14 - bias mode of first opamp
    #[inline(always)]
    pub fn bm_int1(&self) -> BmInt1R {
        BmInt1R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 15:17 - start voltage 0x0:VCM+200mV 0x7:VCM+550mV
    #[inline(always)]
    pub fn vst_sel(&self) -> VstSelR {
        VstSelR::new(((self.bits >> 15) & 7) as u8)
    }
    ///Bits 18:20 - gaincode: 0x0:-6dB 0x1:0dB ... 0x4:18dB
    #[inline(always)]
    pub fn gc(&self) -> GcR {
        GcR::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bit 21 - enable negative DAC1
    #[inline(always)]
    pub fn dacn_en(&self) -> DacnEnR {
        DacnEnR::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - enable differential input mode
    #[inline(always)]
    pub fn diff_en(&self) -> DiffEnR {
        DiffEnR::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bits 23:24 - sampling frequency: 0x0:9.6M 0x1:8.82M 0x2:4.8M 0x3:4.41M
    #[inline(always)]
    pub fn fsp(&self) -> FspR {
        FspR::new(((self.bits >> 23) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC1_CFG1")
            .field("fsp", &self.fsp())
            .field("diff_en", &self.diff_en())
            .field("dacn_en", &self.dacn_en())
            .field("gc", &self.gc())
            .field("vst_sel", &self.vst_sel())
            .field("bm_int1", &self.bm_int1())
            .field("bm_int2", &self.bm_int2())
            .field("vref_sel", &self.vref_sel())
            .field("fchop_sel", &self.fchop_sel())
            .field("vcmst", &self.vcmst())
            .field("clkout_inv", &self.clkout_inv())
            .field("peri_bm", &self.peri_bm())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - peripheral circuits biasmode
    #[inline(always)]
    pub fn peri_bm(&mut self) -> PeriBmW<ADC1_CFG1rs> {
        PeriBmW::new(self, 0)
    }
    ///Bit 2 - inverse output clock
    #[inline(always)]
    pub fn clkout_inv(&mut self) -> ClkoutInvW<ADC1_CFG1rs> {
        ClkoutInvW::new(self, 2)
    }
    ///Bit 3 - VCM quick settling
    #[inline(always)]
    pub fn vcmst(&mut self) -> VcmstW<ADC1_CFG1rs> {
        VcmstW::new(self, 3)
    }
    ///Bits 4:5 - chopping frequncy 0x0:÷8 0x1:÷16 0x2:÷32 0x3:÷64
    #[inline(always)]
    pub fn fchop_sel(&mut self) -> FchopSelW<ADC1_CFG1rs> {
        FchopSelW::new(self, 4)
    }
    ///Bits 6:8 - vref code from proper vcm in flash7 0x0:1.2V 0x1:1.4V 0x7:2.6V
    #[inline(always)]
    pub fn vref_sel(&mut self) -> VrefSelW<ADC1_CFG1rs> {
        VrefSelW::new(self, 6)
    }
    ///Bits 9:11 - bias mode of 2nd and 3rd opamp
    #[inline(always)]
    pub fn bm_int2(&mut self) -> BmInt2W<ADC1_CFG1rs> {
        BmInt2W::new(self, 9)
    }
    ///Bits 12:14 - bias mode of first opamp
    #[inline(always)]
    pub fn bm_int1(&mut self) -> BmInt1W<ADC1_CFG1rs> {
        BmInt1W::new(self, 12)
    }
    ///Bits 15:17 - start voltage 0x0:VCM+200mV 0x7:VCM+550mV
    #[inline(always)]
    pub fn vst_sel(&mut self) -> VstSelW<ADC1_CFG1rs> {
        VstSelW::new(self, 15)
    }
    ///Bits 18:20 - gaincode: 0x0:-6dB 0x1:0dB ... 0x4:18dB
    #[inline(always)]
    pub fn gc(&mut self) -> GcW<ADC1_CFG1rs> {
        GcW::new(self, 18)
    }
    ///Bit 21 - enable negative DAC1
    #[inline(always)]
    pub fn dacn_en(&mut self) -> DacnEnW<ADC1_CFG1rs> {
        DacnEnW::new(self, 21)
    }
    ///Bit 22 - enable differential input mode
    #[inline(always)]
    pub fn diff_en(&mut self) -> DiffEnW<ADC1_CFG1rs> {
        DiffEnW::new(self, 22)
    }
    ///Bits 23:24 - sampling frequency: 0x0:9.6M 0x1:8.82M 0x2:4.8M 0x3:4.41M
    #[inline(always)]
    pub fn fsp(&mut self) -> FspW<ADC1_CFG1rs> {
        FspW::new(self, 23)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`adc1_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc1_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ADC1_CFG1rs;
impl crate::RegisterSpec for ADC1_CFG1rs {
    type Ux = u32;
}
///`read()` method returns [`adc1_cfg1::R`](R) reader structure
impl crate::Readable for ADC1_CFG1rs {}
///`write(|w| ..)` method takes [`adc1_cfg1::W`](W) writer structure
impl crate::Writable for ADC1_CFG1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC1_CFG1 to value 0
impl crate::Resettable for ADC1_CFG1rs {
    const RESET_VALUE: u32 = 0;
}
