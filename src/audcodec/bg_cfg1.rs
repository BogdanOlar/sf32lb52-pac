///Register `BG_CFG1` reader
pub type R = crate::R<BG_CFG1rs>;
///Register `BG_CFG1` writer
pub type W = crate::W<BG_CFG1rs>;
///Field `SAMPCLK_HI` reader - bg sample clock high cycle width, based on 0: stop bg sample clock
pub type SampclkHiR = crate::FieldReader<u32>;
///Field `SAMPCLK_HI` writer - bg sample clock high cycle width, based on 0: stop bg sample clock
pub type SampclkHiW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - bg sample clock high cycle width, based on 0: stop bg sample clock
    #[inline(always)]
    pub fn sampclk_hi(&self) -> SampclkHiR {
        SampclkHiR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BG_CFG1")
            .field("sampclk_hi", &self.sampclk_hi())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - bg sample clock high cycle width, based on 0: stop bg sample clock
    #[inline(always)]
    pub fn sampclk_hi(&mut self) -> SampclkHiW<BG_CFG1rs> {
        SampclkHiW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`bg_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bg_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct BG_CFG1rs;
impl crate::RegisterSpec for BG_CFG1rs {
    type Ux = u32;
}
///`read()` method returns [`bg_cfg1::R`](R) reader structure
impl crate::Readable for BG_CFG1rs {}
///`write(|w| ..)` method takes [`bg_cfg1::W`](W) writer structure
impl crate::Writable for BG_CFG1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BG_CFG1 to value 0
impl crate::Resettable for BG_CFG1rs {
    const RESET_VALUE: u32 = 0;
}
