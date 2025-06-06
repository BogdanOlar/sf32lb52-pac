///Register `PLL_CAL_RESULT` reader
pub type R = crate::R<PLL_CAL_RESULTrs>;
///Register `PLL_CAL_RESULT` writer
pub type W = crate::W<PLL_CAL_RESULTrs>;
///Field `XTAL_CNT` reader - xtal calibration counter result
pub type XtalCntR = crate::FieldReader<u16>;
///Field `XTAL_CNT` writer - xtal calibration counter result
pub type XtalCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `PLL_CNT` reader - pll calibration counter result
pub type PllCntR = crate::FieldReader<u16>;
///Field `PLL_CNT` writer - pll calibration counter result
pub type PllCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - xtal calibration counter result
    #[inline(always)]
    pub fn xtal_cnt(&self) -> XtalCntR {
        XtalCntR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - pll calibration counter result
    #[inline(always)]
    pub fn pll_cnt(&self) -> PllCntR {
        PllCntR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL_CAL_RESULT")
            .field("pll_cnt", &self.pll_cnt())
            .field("xtal_cnt", &self.xtal_cnt())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - xtal calibration counter result
    #[inline(always)]
    pub fn xtal_cnt(&mut self) -> XtalCntW<PLL_CAL_RESULTrs> {
        XtalCntW::new(self, 0)
    }
    ///Bits 16:31 - pll calibration counter result
    #[inline(always)]
    pub fn pll_cnt(&mut self) -> PllCntW<PLL_CAL_RESULTrs> {
        PllCntW::new(self, 16)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`pll_cal_result::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_cal_result::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PLL_CAL_RESULTrs;
impl crate::RegisterSpec for PLL_CAL_RESULTrs {
    type Ux = u32;
}
///`read()` method returns [`pll_cal_result::R`](R) reader structure
impl crate::Readable for PLL_CAL_RESULTrs {}
///`write(|w| ..)` method takes [`pll_cal_result::W`](W) writer structure
impl crate::Writable for PLL_CAL_RESULTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PLL_CAL_RESULT to value 0
impl crate::Resettable for PLL_CAL_RESULTrs {
    const RESET_VALUE: u32 = 0;
}
