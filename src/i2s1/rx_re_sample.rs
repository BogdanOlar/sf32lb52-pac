///Register `RX_RE_SAMPLE` reader
pub type R = crate::R<RX_RE_SAMPLErs>;
///Register `RX_RE_SAMPLE` writer
pub type W = crate::W<RX_RE_SAMPLErs>;
///Field `SMOOTH_EN` reader - 0: Disable RX re-sample smooth filter 1: Enable RX re-sample smooth filter
pub type SmoothEnR = crate::BitReader;
///Field `SMOOTH_EN` writer - 0: Disable RX re-sample smooth filter 1: Enable RX re-sample smooth filter
pub type SmoothEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - 0: Disable RX re-sample smooth filter 1: Enable RX re-sample smooth filter
    #[inline(always)]
    pub fn smooth_en(&self) -> SmoothEnR {
        SmoothEnR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_RE_SAMPLE")
            .field("smooth_en", &self.smooth_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - 0: Disable RX re-sample smooth filter 1: Enable RX re-sample smooth filter
    #[inline(always)]
    pub fn smooth_en(&mut self) -> SmoothEnW<RX_RE_SAMPLErs> {
        SmoothEnW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`rx_re_sample::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_re_sample::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RX_RE_SAMPLErs;
impl crate::RegisterSpec for RX_RE_SAMPLErs {
    type Ux = u32;
}
///`read()` method returns [`rx_re_sample::R`](R) reader structure
impl crate::Readable for RX_RE_SAMPLErs {}
///`write(|w| ..)` method takes [`rx_re_sample::W`](W) writer structure
impl crate::Writable for RX_RE_SAMPLErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RX_RE_SAMPLE to value 0
impl crate::Resettable for RX_RE_SAMPLErs {
    const RESET_VALUE: u32 = 0;
}
