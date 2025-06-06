///Register `CMDR1` reader
pub type R = crate::R<CMDR1rs>;
///Register `CMDR1` writer
pub type W = crate::W<CMDR1rs>;
///Field `CMD` reader - Command. Write to this register will trigger the sequence specified in CCR1
pub type CmdR = crate::FieldReader;
///Field `CMD` writer - Command. Write to this register will trigger the sequence specified in CCR1
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Command. Write to this register will trigger the sequence specified in CCR1
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDR1").field("cmd", &self.cmd()).finish()
    }
}
impl W {
    ///Bits 0:7 - Command. Write to this register will trigger the sequence specified in CCR1
    #[inline(always)]
    pub fn cmd(&mut self) -> CmdW<CMDR1rs> {
        CmdW::new(self, 0)
    }
}
///Command Register
///
///You can [`read`](crate::Reg::read) this register and get [`cmdr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CMDR1rs;
impl crate::RegisterSpec for CMDR1rs {
    type Ux = u32;
}
///`read()` method returns [`cmdr1::R`](R) reader structure
impl crate::Readable for CMDR1rs {}
///`write(|w| ..)` method takes [`cmdr1::W`](W) writer structure
impl crate::Writable for CMDR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CMDR1 to value 0
impl crate::Resettable for CMDR1rs {
    const RESET_VALUE: u32 = 0;
}
