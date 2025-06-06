///Register `DAC_CH1_CFG_EXT` reader
pub type R = crate::R<DAC_CH1_CFG_EXTrs>;
///Register `DAC_CH1_CFG_EXT` writer
pub type W = crate::W<DAC_CH1_CFG_EXTrs>;
///Field `RAMP_EN` reader - volume ramp enable
pub type RampEnR = crate::BitReader;
///Field `RAMP_EN` writer - volume ramp enable
pub type RampEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAMP_MODE` reader - volume ramp mode: 1: slowly ramp to target volume. Step is 0.5db 0: directly ramp to target volume.
pub type RampModeR = crate::BitReader;
///Field `RAMP_MODE` writer - volume ramp mode: 1: slowly ramp to target volume. Step is 0.5db 0: directly ramp to target volume.
pub type RampModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ZERO_ADJUST_EN` reader - enable volume adjustment during 0 volume cross.
pub type ZeroAdjustEnR = crate::BitReader;
///Field `ZERO_ADJUST_EN` writer - enable volume adjustment during 0 volume cross.
pub type ZeroAdjustEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAMP_INTERVAL` reader - volume ramp interval.
pub type RampIntervalR = crate::FieldReader;
///Field `RAMP_INTERVAL` writer - volume ramp interval.
pub type RampIntervalW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RAMP_STAT` reader - ramp module status
pub type RampStatR = crate::FieldReader;
///Field `RAMP_STAT` writer - ramp module status
pub type RampStatW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    ///Bit 0 - volume ramp enable
    #[inline(always)]
    pub fn ramp_en(&self) -> RampEnR {
        RampEnR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - volume ramp mode: 1: slowly ramp to target volume. Step is 0.5db 0: directly ramp to target volume.
    #[inline(always)]
    pub fn ramp_mode(&self) -> RampModeR {
        RampModeR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - enable volume adjustment during 0 volume cross.
    #[inline(always)]
    pub fn zero_adjust_en(&self) -> ZeroAdjustEnR {
        ZeroAdjustEnR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:6 - volume ramp interval.
    #[inline(always)]
    pub fn ramp_interval(&self) -> RampIntervalR {
        RampIntervalR::new(((self.bits >> 3) & 0x0f) as u8)
    }
    ///Bits 7:8 - ramp module status
    #[inline(always)]
    pub fn ramp_stat(&self) -> RampStatR {
        RampStatR::new(((self.bits >> 7) & 3) as u8)
    }
    ///Bits 9:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_CH1_CFG_EXT")
            .field("rsvd", &self.rsvd())
            .field("ramp_stat", &self.ramp_stat())
            .field("ramp_interval", &self.ramp_interval())
            .field("zero_adjust_en", &self.zero_adjust_en())
            .field("ramp_mode", &self.ramp_mode())
            .field("ramp_en", &self.ramp_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - volume ramp enable
    #[inline(always)]
    pub fn ramp_en(&mut self) -> RampEnW<DAC_CH1_CFG_EXTrs> {
        RampEnW::new(self, 0)
    }
    ///Bit 1 - volume ramp mode: 1: slowly ramp to target volume. Step is 0.5db 0: directly ramp to target volume.
    #[inline(always)]
    pub fn ramp_mode(&mut self) -> RampModeW<DAC_CH1_CFG_EXTrs> {
        RampModeW::new(self, 1)
    }
    ///Bit 2 - enable volume adjustment during 0 volume cross.
    #[inline(always)]
    pub fn zero_adjust_en(&mut self) -> ZeroAdjustEnW<DAC_CH1_CFG_EXTrs> {
        ZeroAdjustEnW::new(self, 2)
    }
    ///Bits 3:6 - volume ramp interval.
    #[inline(always)]
    pub fn ramp_interval(&mut self) -> RampIntervalW<DAC_CH1_CFG_EXTrs> {
        RampIntervalW::new(self, 3)
    }
    ///Bits 7:8 - ramp module status
    #[inline(always)]
    pub fn ramp_stat(&mut self) -> RampStatW<DAC_CH1_CFG_EXTrs> {
        RampStatW::new(self, 7)
    }
    ///Bits 9:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<DAC_CH1_CFG_EXTrs> {
        RsvdW::new(self, 9)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`dac_ch1_cfg_ext::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_ch1_cfg_ext::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DAC_CH1_CFG_EXTrs;
impl crate::RegisterSpec for DAC_CH1_CFG_EXTrs {
    type Ux = u32;
}
///`read()` method returns [`dac_ch1_cfg_ext::R`](R) reader structure
impl crate::Readable for DAC_CH1_CFG_EXTrs {}
///`write(|w| ..)` method takes [`dac_ch1_cfg_ext::W`](W) writer structure
impl crate::Writable for DAC_CH1_CFG_EXTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DAC_CH1_CFG_EXT to value 0
impl crate::Resettable for DAC_CH1_CFG_EXTrs {
    const RESET_VALUE: u32 = 0;
}
