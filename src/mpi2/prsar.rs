///Register `PRSAR` reader
pub type R = crate::R<PRSARrs>;
///Register `PRSAR` writer
pub type W = crate::W<PRSARrs>;
///Field `SA` reader - Starting address of the prefetch area If prefetch is enabled and the read address falls into \[PRSAR, PREAR), controller will prefetch the following data
pub type SaR = crate::FieldReader<u32>;
///Field `SA` writer - Starting address of the prefetch area If prefetch is enabled and the read address falls into \[PRSAR, PREAR), controller will prefetch the following data
pub type SaW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    ///Bits 10:31 - Starting address of the prefetch area If prefetch is enabled and the read address falls into \[PRSAR, PREAR), controller will prefetch the following data
    #[inline(always)]
    pub fn sa(&self) -> SaR {
        SaR::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRSAR").field("sa", &self.sa()).finish()
    }
}
impl W {
    ///Bits 10:31 - Starting address of the prefetch area If prefetch is enabled and the read address falls into \[PRSAR, PREAR), controller will prefetch the following data
    #[inline(always)]
    pub fn sa(&mut self) -> SaW<PRSARrs> {
        SaW::new(self, 10)
    }
}
///Prefetch Starting Address Register
///
///You can [`read`](crate::Reg::read) this register and get [`prsar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prsar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PRSARrs;
impl crate::RegisterSpec for PRSARrs {
    type Ux = u32;
}
///`read()` method returns [`prsar::R`](R) reader structure
impl crate::Readable for PRSARrs {}
///`write(|w| ..)` method takes [`prsar::W`](W) writer structure
impl crate::Writable for PRSARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PRSAR to value 0
impl crate::Resettable for PRSARrs {
    const RESET_VALUE: u32 = 0;
}
