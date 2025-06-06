///Register `BG_CFG2` reader
pub type R = crate::R<BG_CFG2rs>;
///Register `BG_CFG2` writer
pub type W = crate::W<BG_CFG2rs>;
///Field `SAMPCLK_LO` reader - bg sample clock low cycle width. 0: stop bg sample clock
pub type SampclkLoR = crate::FieldReader<u32>;
///Field `SAMPCLK_LO` writer - bg sample clock low cycle width. 0: stop bg sample clock
pub type SampclkLoW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - bg sample clock low cycle width. 0: stop bg sample clock
    #[inline(always)]
    pub fn sampclk_lo(&self) -> SampclkLoR {
        SampclkLoR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BG_CFG2")
            .field("sampclk_lo", &self.sampclk_lo())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - bg sample clock low cycle width. 0: stop bg sample clock
    #[inline(always)]
    pub fn sampclk_lo(&mut self) -> SampclkLoW<BG_CFG2rs> {
        SampclkLoW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`bg_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bg_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct BG_CFG2rs;
impl crate::RegisterSpec for BG_CFG2rs {
    type Ux = u32;
}
///`read()` method returns [`bg_cfg2::R`](R) reader structure
impl crate::Readable for BG_CFG2rs {}
///`write(|w| ..)` method takes [`bg_cfg2::W`](W) writer structure
impl crate::Writable for BG_CFG2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BG_CFG2 to value 0
impl crate::Resettable for BG_CFG2rs {
    const RESET_VALUE: u32 = 0;
}
