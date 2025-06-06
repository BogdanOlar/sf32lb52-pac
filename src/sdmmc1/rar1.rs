///Register `RAR1` reader
pub type R = crate::R<RAR1rs>;
///Register `RAR1` writer
pub type W = crate::W<RAR1rs>;
///Field `RSP_ARG1` reader - Response command content If long response, it is rsp_arg\[39:8\]
pub type RspArg1R = crate::FieldReader<u32>;
///Field `RSP_ARG1` writer - Response command content If long response, it is rsp_arg\[39:8\]
pub type RspArg1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Response command content If long response, it is rsp_arg\[39:8\]
    #[inline(always)]
    pub fn rsp_arg1(&self) -> RspArg1R {
        RspArg1R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAR1")
            .field("rsp_arg1", &self.rsp_arg1())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Response command content If long response, it is rsp_arg\[39:8\]
    #[inline(always)]
    pub fn rsp_arg1(&mut self) -> RspArg1W<RAR1rs> {
        RspArg1W::new(self, 0)
    }
}
///response command argument1 register
///
///You can [`read`](crate::Reg::read) this register and get [`rar1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rar1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RAR1rs;
impl crate::RegisterSpec for RAR1rs {
    type Ux = u32;
}
///`read()` method returns [`rar1::R`](R) reader structure
impl crate::Readable for RAR1rs {}
///`write(|w| ..)` method takes [`rar1::W`](W) writer structure
impl crate::Writable for RAR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RAR1 to value 0
impl crate::Resettable for RAR1rs {
    const RESET_VALUE: u32 = 0;
}
