///Register `PERF_CNT` reader
pub type R = crate::R<PERF_CNTrs>;
///Register `PERF_CNT` writer
pub type W = crate::W<PERF_CNTrs>;
///Field `VAL` reader - lcdc performance counter
pub type ValR = crate::FieldReader<u32>;
///Field `VAL` writer - lcdc performance counter
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - lcdc performance counter
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERF_CNT")
            .field("val", &self.val())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - lcdc performance counter
    #[inline(always)]
    pub fn val(&mut self) -> ValW<PERF_CNTrs> {
        ValW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`perf_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perf_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PERF_CNTrs;
impl crate::RegisterSpec for PERF_CNTrs {
    type Ux = u32;
}
///`read()` method returns [`perf_cnt::R`](R) reader structure
impl crate::Readable for PERF_CNTrs {}
///`write(|w| ..)` method takes [`perf_cnt::W`](W) writer structure
impl crate::Writable for PERF_CNTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PERF_CNT to value 0
impl crate::Resettable for PERF_CNTrs {
    const RESET_VALUE: u32 = 0;
}
