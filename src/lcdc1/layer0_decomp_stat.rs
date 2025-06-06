///Register `LAYER0_DECOMP_STAT` reader
pub type R = crate::R<LAYER0_DECOMP_STATrs>;
///Register `LAYER0_DECOMP_STAT` writer
pub type W = crate::W<LAYER0_DECOMP_STATrs>;
///Field `BUF_MAX_DEPTH` reader - buf max usage
pub type BufMaxDepthR = crate::FieldReader;
///Field `BUF_MAX_DEPTH` writer - buf max usage
pub type BufMaxDepthW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6 - buf max usage
    #[inline(always)]
    pub fn buf_max_depth(&self) -> BufMaxDepthR {
        BufMaxDepthR::new((self.bits & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LAYER0_DECOMP_STAT")
            .field("buf_max_depth", &self.buf_max_depth())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - buf max usage
    #[inline(always)]
    pub fn buf_max_depth(&mut self) -> BufMaxDepthW<LAYER0_DECOMP_STATrs> {
        BufMaxDepthW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`layer0_decomp_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer0_decomp_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct LAYER0_DECOMP_STATrs;
impl crate::RegisterSpec for LAYER0_DECOMP_STATrs {
    type Ux = u32;
}
///`read()` method returns [`layer0_decomp_stat::R`](R) reader structure
impl crate::Readable for LAYER0_DECOMP_STATrs {}
///`write(|w| ..)` method takes [`layer0_decomp_stat::W`](W) writer structure
impl crate::Writable for LAYER0_DECOMP_STATrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LAYER0_DECOMP_STAT to value 0
impl crate::Resettable for LAYER0_DECOMP_STATrs {
    const RESET_VALUE: u32 = 0;
}
