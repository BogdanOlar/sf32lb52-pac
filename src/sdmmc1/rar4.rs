///Register `RAR4` reader
pub type R = crate::R<RAR4rs>;
///Register `RAR4` writer
pub type W = crate::W<RAR4rs>;
///Field `RSP_ARG4` reader - Long response, it is rsp_arg\[127:104\]
pub type RspArg4R = crate::FieldReader<u32>;
///Field `RSP_ARG4` writer - Long response, it is rsp_arg\[127:104\]
pub type RspArg4W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:23 - Long response, it is rsp_arg\[127:104\]
    #[inline(always)]
    pub fn rsp_arg4(&self) -> RspArg4R {
        RspArg4R::new(self.bits & 0x00ff_ffff)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAR4")
            .field("rsvd", &self.rsvd())
            .field("rsp_arg4", &self.rsp_arg4())
            .finish()
    }
}
impl W {
    ///Bits 0:23 - Long response, it is rsp_arg\[127:104\]
    #[inline(always)]
    pub fn rsp_arg4(&mut self) -> RspArg4W<RAR4rs> {
        RspArg4W::new(self, 0)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<RAR4rs> {
        RsvdW::new(self, 24)
    }
}
///response command argument4 register
///
///You can [`read`](crate::Reg::read) this register and get [`rar4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rar4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RAR4rs;
impl crate::RegisterSpec for RAR4rs {
    type Ux = u32;
}
///`read()` method returns [`rar4::R`](R) reader structure
impl crate::Readable for RAR4rs {}
///`write(|w| ..)` method takes [`rar4::W`](W) writer structure
impl crate::Writable for RAR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RAR4 to value 0
impl crate::Resettable for RAR4rs {
    const RESET_VALUE: u32 = 0;
}
