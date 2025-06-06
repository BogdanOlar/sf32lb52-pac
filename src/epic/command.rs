///Register `COMMAND` reader
pub type R = crate::R<COMMANDrs>;
///Register `COMMAND` writer
pub type W = crate::W<COMMANDrs>;
///Field `START` reader - write 1 to trigger the lcd interface block
pub type StartR = crate::BitReader;
///Field `START` writer - write 1 to trigger the lcd interface block
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RESET` reader - 1: reset the whole graphics 0: release the reset
pub type ResetR = crate::BitReader;
///Field `RESET` writer - 1: reset the whole graphics 0: release the reset
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - write 1 to trigger the lcd interface block
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - 1: reset the whole graphics 0: release the reset
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMMAND")
            .field("reset", &self.reset())
            .field("start", &self.start())
            .finish()
    }
}
impl W {
    ///Bit 0 - write 1 to trigger the lcd interface block
    #[inline(always)]
    pub fn start(&mut self) -> StartW<COMMANDrs> {
        StartW::new(self, 0)
    }
    ///Bit 1 - 1: reset the whole graphics 0: release the reset
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<COMMANDrs> {
        ResetW::new(self, 1)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`command::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`command::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct COMMANDrs;
impl crate::RegisterSpec for COMMANDrs {
    type Ux = u32;
}
///`read()` method returns [`command::R`](R) reader structure
impl crate::Readable for COMMANDrs {}
///`write(|w| ..)` method takes [`command::W`](W) writer structure
impl crate::Writable for COMMANDrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets COMMAND to value 0
impl crate::Resettable for COMMANDrs {
    const RESET_VALUE: u32 = 0;
}
