///Register `DAC_PATH_CFG3` reader
pub type R = crate::R<DAC_PATH_CFG3rs>;
///Register `DAC_PATH_CFG3` writer
pub type W = crate::W<DAC_PATH_CFG3rs>;
///Field `RAMP_EN_L` reader - dac mixer left channel volume ramp enable
pub type RampEnLR = crate::BitReader;
///Field `RAMP_EN_L` writer - dac mixer left channel volume ramp enable
pub type RampEnLW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAMP_MODE_L` reader - dac mixer left channel volume ramp mode: 1: slowly ramp to target volume. Step is 0.5db 0: directly ramp to target volume.
pub type RampModeLR = crate::BitReader;
///Field `RAMP_MODE_L` writer - dac mixer left channel volume ramp mode: 1: slowly ramp to target volume. Step is 0.5db 0: directly ramp to target volume.
pub type RampModeLW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ZERO_ADJUST_EN_L` reader - dac mixer left channel volume adjustment during 0 volume cross enable
pub type ZeroAdjustEnLR = crate::BitReader;
///Field `ZERO_ADJUST_EN_L` writer - dac mixer left channel volume adjustment during 0 volume cross enable
pub type ZeroAdjustEnLW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAMP_INTERVAL_L` reader - dac mixer left channel volume ramp interval.
pub type RampIntervalLR = crate::FieldReader;
///Field `RAMP_INTERVAL_L` writer - dac mixer left channel volume ramp interval.
pub type RampIntervalLW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RAMP_STAT_L` reader - dac mixer left channel ramp module status
pub type RampStatLR = crate::FieldReader;
///Field `RAMP_STAT_L` writer - dac mixer left channel ramp module status
pub type RampStatLW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RAMP_EN_R` reader - dac mixer right channel volume ramp enable
pub type RampEnRR = crate::BitReader;
///Field `RAMP_EN_R` writer - dac mixer right channel volume ramp enable
pub type RampEnRW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAMP_MODE_R` reader - dac mixer right channel volume ramp mode: 1: slowly ramp to target volume. Step is 0.5db 0: directly ramp to target volume.
pub type RampModeRR = crate::BitReader;
///Field `RAMP_MODE_R` writer - dac mixer right channel volume ramp mode: 1: slowly ramp to target volume. Step is 0.5db 0: directly ramp to target volume.
pub type RampModeRW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ZERO_ADJUST_EN_R` reader - dac mixer right channel volume adjustment during 0 volume cross enable
pub type ZeroAdjustEnRR = crate::BitReader;
///Field `ZERO_ADJUST_EN_R` writer - dac mixer right channel volume adjustment during 0 volume cross enable
pub type ZeroAdjustEnRW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAMP_INTERVAL_R` reader - dac mixer right channel volume ramp interval.
pub type RampIntervalRR = crate::FieldReader;
///Field `RAMP_INTERVAL_R` writer - dac mixer right channel volume ramp interval.
pub type RampIntervalRW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RAMP_STAT_R` reader - dac mixer right channel ramp module status
pub type RampStatRR = crate::FieldReader;
///Field `RAMP_STAT_R` writer - dac mixer right channel ramp module status
pub type RampStatRW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    ///Bit 0 - dac mixer left channel volume ramp enable
    #[inline(always)]
    pub fn ramp_en_l(&self) -> RampEnLR {
        RampEnLR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - dac mixer left channel volume ramp mode: 1: slowly ramp to target volume. Step is 0.5db 0: directly ramp to target volume.
    #[inline(always)]
    pub fn ramp_mode_l(&self) -> RampModeLR {
        RampModeLR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - dac mixer left channel volume adjustment during 0 volume cross enable
    #[inline(always)]
    pub fn zero_adjust_en_l(&self) -> ZeroAdjustEnLR {
        ZeroAdjustEnLR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:6 - dac mixer left channel volume ramp interval.
    #[inline(always)]
    pub fn ramp_interval_l(&self) -> RampIntervalLR {
        RampIntervalLR::new(((self.bits >> 3) & 0x0f) as u8)
    }
    ///Bits 7:8 - dac mixer left channel ramp module status
    #[inline(always)]
    pub fn ramp_stat_l(&self) -> RampStatLR {
        RampStatLR::new(((self.bits >> 7) & 3) as u8)
    }
    ///Bit 9 - dac mixer right channel volume ramp enable
    #[inline(always)]
    pub fn ramp_en_r(&self) -> RampEnRR {
        RampEnRR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - dac mixer right channel volume ramp mode: 1: slowly ramp to target volume. Step is 0.5db 0: directly ramp to target volume.
    #[inline(always)]
    pub fn ramp_mode_r(&self) -> RampModeRR {
        RampModeRR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - dac mixer right channel volume adjustment during 0 volume cross enable
    #[inline(always)]
    pub fn zero_adjust_en_r(&self) -> ZeroAdjustEnRR {
        ZeroAdjustEnRR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:15 - dac mixer right channel volume ramp interval.
    #[inline(always)]
    pub fn ramp_interval_r(&self) -> RampIntervalRR {
        RampIntervalRR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:17 - dac mixer right channel ramp module status
    #[inline(always)]
    pub fn ramp_stat_r(&self) -> RampStatRR {
        RampStatRR::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_PATH_CFG3")
            .field("rsvd", &self.rsvd())
            .field("ramp_stat_r", &self.ramp_stat_r())
            .field("ramp_interval_r", &self.ramp_interval_r())
            .field("zero_adjust_en_r", &self.zero_adjust_en_r())
            .field("ramp_mode_r", &self.ramp_mode_r())
            .field("ramp_en_r", &self.ramp_en_r())
            .field("ramp_stat_l", &self.ramp_stat_l())
            .field("ramp_interval_l", &self.ramp_interval_l())
            .field("zero_adjust_en_l", &self.zero_adjust_en_l())
            .field("ramp_mode_l", &self.ramp_mode_l())
            .field("ramp_en_l", &self.ramp_en_l())
            .finish()
    }
}
impl W {
    ///Bit 0 - dac mixer left channel volume ramp enable
    #[inline(always)]
    pub fn ramp_en_l(&mut self) -> RampEnLW<DAC_PATH_CFG3rs> {
        RampEnLW::new(self, 0)
    }
    ///Bit 1 - dac mixer left channel volume ramp mode: 1: slowly ramp to target volume. Step is 0.5db 0: directly ramp to target volume.
    #[inline(always)]
    pub fn ramp_mode_l(&mut self) -> RampModeLW<DAC_PATH_CFG3rs> {
        RampModeLW::new(self, 1)
    }
    ///Bit 2 - dac mixer left channel volume adjustment during 0 volume cross enable
    #[inline(always)]
    pub fn zero_adjust_en_l(&mut self) -> ZeroAdjustEnLW<DAC_PATH_CFG3rs> {
        ZeroAdjustEnLW::new(self, 2)
    }
    ///Bits 3:6 - dac mixer left channel volume ramp interval.
    #[inline(always)]
    pub fn ramp_interval_l(&mut self) -> RampIntervalLW<DAC_PATH_CFG3rs> {
        RampIntervalLW::new(self, 3)
    }
    ///Bits 7:8 - dac mixer left channel ramp module status
    #[inline(always)]
    pub fn ramp_stat_l(&mut self) -> RampStatLW<DAC_PATH_CFG3rs> {
        RampStatLW::new(self, 7)
    }
    ///Bit 9 - dac mixer right channel volume ramp enable
    #[inline(always)]
    pub fn ramp_en_r(&mut self) -> RampEnRW<DAC_PATH_CFG3rs> {
        RampEnRW::new(self, 9)
    }
    ///Bit 10 - dac mixer right channel volume ramp mode: 1: slowly ramp to target volume. Step is 0.5db 0: directly ramp to target volume.
    #[inline(always)]
    pub fn ramp_mode_r(&mut self) -> RampModeRW<DAC_PATH_CFG3rs> {
        RampModeRW::new(self, 10)
    }
    ///Bit 11 - dac mixer right channel volume adjustment during 0 volume cross enable
    #[inline(always)]
    pub fn zero_adjust_en_r(&mut self) -> ZeroAdjustEnRW<DAC_PATH_CFG3rs> {
        ZeroAdjustEnRW::new(self, 11)
    }
    ///Bits 12:15 - dac mixer right channel volume ramp interval.
    #[inline(always)]
    pub fn ramp_interval_r(&mut self) -> RampIntervalRW<DAC_PATH_CFG3rs> {
        RampIntervalRW::new(self, 12)
    }
    ///Bits 16:17 - dac mixer right channel ramp module status
    #[inline(always)]
    pub fn ramp_stat_r(&mut self) -> RampStatRW<DAC_PATH_CFG3rs> {
        RampStatRW::new(self, 16)
    }
    ///Bits 18:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<DAC_PATH_CFG3rs> {
        RsvdW::new(self, 18)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`dac_path_cfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_path_cfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DAC_PATH_CFG3rs;
impl crate::RegisterSpec for DAC_PATH_CFG3rs {
    type Ux = u32;
}
///`read()` method returns [`dac_path_cfg3::R`](R) reader structure
impl crate::Readable for DAC_PATH_CFG3rs {}
///`write(|w| ..)` method takes [`dac_path_cfg3::W`](W) writer structure
impl crate::Writable for DAC_PATH_CFG3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DAC_PATH_CFG3 to value 0
impl crate::Resettable for DAC_PATH_CFG3rs {
    const RESET_VALUE: u32 = 0;
}
