///Register `DAC_PATH_CFG2` reader
pub type R = crate::R<DAC_PATH_CFG2rs>;
///Register `DAC_PATH_CFG2` writer
pub type W = crate::W<DAC_PATH_CFG2rs>;
///Field `SINC_RATIO` reader - sinc filter ratio, s31.30 format. Range from 0~2
pub type SincRatioR = crate::FieldReader<u32>;
///Field `SINC_RATIO` writer - sinc filter ratio, s31.30 format. Range from 0~2
pub type SincRatioW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
///Field `SRC_SINC_EN` reader - sinc filter enable
pub type SrcSincEnR = crate::BitReader;
///Field `SRC_SINC_EN` writer - sinc filter enable
pub type SrcSincEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:30 - sinc filter ratio, s31.30 format. Range from 0~2
    #[inline(always)]
    pub fn sinc_ratio(&self) -> SincRatioR {
        SincRatioR::new(self.bits & 0x7fff_ffff)
    }
    ///Bit 31 - sinc filter enable
    #[inline(always)]
    pub fn src_sinc_en(&self) -> SrcSincEnR {
        SrcSincEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_PATH_CFG2")
            .field("src_sinc_en", &self.src_sinc_en())
            .field("sinc_ratio", &self.sinc_ratio())
            .finish()
    }
}
impl W {
    ///Bits 0:30 - sinc filter ratio, s31.30 format. Range from 0~2
    #[inline(always)]
    pub fn sinc_ratio(&mut self) -> SincRatioW<DAC_PATH_CFG2rs> {
        SincRatioW::new(self, 0)
    }
    ///Bit 31 - sinc filter enable
    #[inline(always)]
    pub fn src_sinc_en(&mut self) -> SrcSincEnW<DAC_PATH_CFG2rs> {
        SrcSincEnW::new(self, 31)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`dac_path_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_path_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DAC_PATH_CFG2rs;
impl crate::RegisterSpec for DAC_PATH_CFG2rs {
    type Ux = u32;
}
///`read()` method returns [`dac_path_cfg2::R`](R) reader structure
impl crate::Readable for DAC_PATH_CFG2rs {}
///`write(|w| ..)` method takes [`dac_path_cfg2::W`](W) writer structure
impl crate::Writable for DAC_PATH_CFG2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DAC_PATH_CFG2 to value 0
impl crate::Resettable for DAC_PATH_CFG2rs {
    const RESET_VALUE: u32 = 0;
}
