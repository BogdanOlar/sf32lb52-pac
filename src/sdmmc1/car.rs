///Register `CAR` reader
pub type R = crate::R<CARrs>;
///Register `CAR` writer
pub type W = crate::W<CARrs>;
///Field `CMD_ARG` reader - Command argument
pub type CmdArgR = crate::FieldReader<u32>;
///Field `CMD_ARG` writer - Command argument
pub type CmdArgW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Command argument
    #[inline(always)]
    pub fn cmd_arg(&self) -> CmdArgR {
        CmdArgR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAR")
            .field("cmd_arg", &self.cmd_arg())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Command argument
    #[inline(always)]
    pub fn cmd_arg(&mut self) -> CmdArgW<CARrs> {
        CmdArgW::new(self, 0)
    }
}
///command argument register
///
///You can [`read`](crate::Reg::read) this register and get [`car::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`car::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CARrs;
impl crate::RegisterSpec for CARrs {
    type Ux = u32;
}
///`read()` method returns [`car::R`](R) reader structure
impl crate::Readable for CARrs {}
///`write(|w| ..)` method takes [`car::W`](W) writer structure
impl crate::Writable for CARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CAR to value 0
impl crate::Resettable for CARrs {
    const RESET_VALUE: u32 = 0;
}
