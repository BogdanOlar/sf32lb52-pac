///Register `HRCCAL2` reader
pub type R = crate::R<HRCCAL2rs>;
///Register `HRCCAL2` writer
pub type W = crate::W<HRCCAL2rs>;
///Field `HRC_CNT` reader - Total clk_hrc48 cycles during calibration
pub type HrcCntR = crate::FieldReader<u16>;
///Field `HRC_CNT` writer - Total clk_hrc48 cycles during calibration
pub type HrcCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `HXT_CNT` reader - Total clk_hxt48 cycles during calibration
pub type HxtCntR = crate::FieldReader<u16>;
///Field `HXT_CNT` writer - Total clk_hxt48 cycles during calibration
pub type HxtCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Total clk_hrc48 cycles during calibration
    #[inline(always)]
    pub fn hrc_cnt(&self) -> HrcCntR {
        HrcCntR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Total clk_hxt48 cycles during calibration
    #[inline(always)]
    pub fn hxt_cnt(&self) -> HxtCntR {
        HxtCntR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HRCCAL2")
            .field("hxt_cnt", &self.hxt_cnt())
            .field("hrc_cnt", &self.hrc_cnt())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Total clk_hrc48 cycles during calibration
    #[inline(always)]
    pub fn hrc_cnt(&mut self) -> HrcCntW<HRCCAL2rs> {
        HrcCntW::new(self, 0)
    }
    ///Bits 16:31 - Total clk_hxt48 cycles during calibration
    #[inline(always)]
    pub fn hxt_cnt(&mut self) -> HxtCntW<HRCCAL2rs> {
        HxtCntW::new(self, 16)
    }
}
///HRC Calibration Register 2
///
///You can [`read`](crate::Reg::read) this register and get [`hrccal2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hrccal2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct HRCCAL2rs;
impl crate::RegisterSpec for HRCCAL2rs {
    type Ux = u32;
}
///`read()` method returns [`hrccal2::R`](R) reader structure
impl crate::Readable for HRCCAL2rs {}
///`write(|w| ..)` method takes [`hrccal2::W`](W) writer structure
impl crate::Writable for HRCCAL2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HRCCAL2 to value 0
impl crate::Resettable for HRCCAL2rs {
    const RESET_VALUE: u32 = 0;
}
