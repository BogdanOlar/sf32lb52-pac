///Register `PGA_CFG` reader
pub type R = crate::R<PGA_CFGrs>;
///Register `PGA_CFG` writer
pub type W = crate::W<PGA_CFGrs>;
///Field `PGA_GAIN_L` reader - left channel gain control , the range is -15dB~45dB. Resolution is 0.5dB/LSB
pub type PgaGainLR = crate::FieldReader;
///Field `PGA_GAIN_L` writer - left channel gain control , the range is -15dB~45dB. Resolution is 0.5dB/LSB
pub type PgaGainLW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `PGA_GAIN_R` reader - right channel gain control , the range is -15dB~45dB. Resolution is 0.5dB/LSB
pub type PgaGainRR = crate::FieldReader;
///Field `PGA_GAIN_R` writer - right channel gain control , the range is -15dB~45dB. Resolution is 0.5dB/LSB
pub type PgaGainRW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6 - left channel gain control , the range is -15dB~45dB. Resolution is 0.5dB/LSB
    #[inline(always)]
    pub fn pga_gain_l(&self) -> PgaGainLR {
        PgaGainLR::new((self.bits & 0x7f) as u8)
    }
    ///Bits 7:13 - right channel gain control , the range is -15dB~45dB. Resolution is 0.5dB/LSB
    #[inline(always)]
    pub fn pga_gain_r(&self) -> PgaGainRR {
        PgaGainRR::new(((self.bits >> 7) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PGA_CFG")
            .field("pga_gain_r", &self.pga_gain_r())
            .field("pga_gain_l", &self.pga_gain_l())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - left channel gain control , the range is -15dB~45dB. Resolution is 0.5dB/LSB
    #[inline(always)]
    pub fn pga_gain_l(&mut self) -> PgaGainLW<PGA_CFGrs> {
        PgaGainLW::new(self, 0)
    }
    ///Bits 7:13 - right channel gain control , the range is -15dB~45dB. Resolution is 0.5dB/LSB
    #[inline(always)]
    pub fn pga_gain_r(&mut self) -> PgaGainRW<PGA_CFGrs> {
        PgaGainRW::new(self, 7)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`pga_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pga_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PGA_CFGrs;
impl crate::RegisterSpec for PGA_CFGrs {
    type Ux = u32;
}
///`read()` method returns [`pga_cfg::R`](R) reader structure
impl crate::Readable for PGA_CFGrs {}
///`write(|w| ..)` method takes [`pga_cfg::W`](W) writer structure
impl crate::Writable for PGA_CFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PGA_CFG to value 0
impl crate::Resettable for PGA_CFGrs {
    const RESET_VALUE: u32 = 0;
}
