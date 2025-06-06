///Register `CAL_RESULT` reader
pub type R = crate::R<CAL_RESULTrs>;
///Register `CAL_RESULT` writer
pub type W = crate::W<CAL_RESULTrs>;
///Field `PCLK_CNT` reader - pclk calibration counter result
pub type PclkCntR = crate::FieldReader<u16>;
///Field `PCLK_CNT` writer - pclk calibration counter result
pub type PclkCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `OSC_CNT` reader - osc clock calibration counter result
pub type OscCntR = crate::FieldReader<u16>;
///Field `OSC_CNT` writer - osc clock calibration counter result
pub type OscCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - pclk calibration counter result
    #[inline(always)]
    pub fn pclk_cnt(&self) -> PclkCntR {
        PclkCntR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - osc clock calibration counter result
    #[inline(always)]
    pub fn osc_cnt(&self) -> OscCntR {
        OscCntR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAL_RESULT")
            .field("osc_cnt", &self.osc_cnt())
            .field("pclk_cnt", &self.pclk_cnt())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - pclk calibration counter result
    #[inline(always)]
    pub fn pclk_cnt(&mut self) -> PclkCntW<CAL_RESULTrs> {
        PclkCntW::new(self, 0)
    }
    ///Bits 16:31 - osc clock calibration counter result
    #[inline(always)]
    pub fn osc_cnt(&mut self) -> OscCntW<CAL_RESULTrs> {
        OscCntW::new(self, 16)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`cal_result::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal_result::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CAL_RESULTrs;
impl crate::RegisterSpec for CAL_RESULTrs {
    type Ux = u32;
}
///`read()` method returns [`cal_result::R`](R) reader structure
impl crate::Readable for CAL_RESULTrs {}
///`write(|w| ..)` method takes [`cal_result::W`](W) writer structure
impl crate::Writable for CAL_RESULTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CAL_RESULT to value 0
impl crate::Resettable for CAL_RESULTrs {
    const RESET_VALUE: u32 = 0;
}
