///Register `RAR3` reader
pub type R = crate::R<RAR3rs>;
///Register `RAR3` writer
pub type W = crate::W<RAR3rs>;
///Field `RSP_ARG3` reader - Long response, it is rsp_arg\[103:72\]
pub type RspArg3R = crate::FieldReader<u32>;
///Field `RSP_ARG3` writer - Long response, it is rsp_arg\[103:72\]
pub type RspArg3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Long response, it is rsp_arg\[103:72\]
    #[inline(always)]
    pub fn rsp_arg3(&self) -> RspArg3R {
        RspArg3R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAR3")
            .field("rsp_arg3", &self.rsp_arg3())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Long response, it is rsp_arg\[103:72\]
    #[inline(always)]
    pub fn rsp_arg3(&mut self) -> RspArg3W<RAR3rs> {
        RspArg3W::new(self, 0)
    }
}
///response command argument3 register
///
///You can [`read`](crate::Reg::read) this register and get [`rar3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rar3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RAR3rs;
impl crate::RegisterSpec for RAR3rs {
    type Ux = u32;
}
///`read()` method returns [`rar3::R`](R) reader structure
impl crate::Readable for RAR3rs {}
///`write(|w| ..)` method takes [`rar3::W`](W) writer structure
impl crate::Writable for RAR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RAR3 to value 0
impl crate::Resettable for RAR3rs {
    const RESET_VALUE: u32 = 0;
}
