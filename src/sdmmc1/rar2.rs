///Register `RAR2` reader
pub type R = crate::R<RAR2rs>;
///Register `RAR2` writer
pub type W = crate::W<RAR2rs>;
///Field `RSP_ARG2` reader - Long response, it is rsp_arg\[71:40\]
pub type RspArg2R = crate::FieldReader<u32>;
///Field `RSP_ARG2` writer - Long response, it is rsp_arg\[71:40\]
pub type RspArg2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Long response, it is rsp_arg\[71:40\]
    #[inline(always)]
    pub fn rsp_arg2(&self) -> RspArg2R {
        RspArg2R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAR2")
            .field("rsp_arg2", &self.rsp_arg2())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Long response, it is rsp_arg\[71:40\]
    #[inline(always)]
    pub fn rsp_arg2(&mut self) -> RspArg2W<RAR2rs> {
        RspArg2W::new(self, 0)
    }
}
///response command argument2 register
///
///You can [`read`](crate::Reg::read) this register and get [`rar2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rar2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RAR2rs;
impl crate::RegisterSpec for RAR2rs {
    type Ux = u32;
}
///`read()` method returns [`rar2::R`](R) reader structure
impl crate::Readable for RAR2rs {}
///`write(|w| ..)` method takes [`rar2::W`](W) writer structure
impl crate::Writable for RAR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RAR2 to value 0
impl crate::Resettable for RAR2rs {
    const RESET_VALUE: u32 = 0;
}
