///Register `CMDR2` reader
pub type R = crate::R<CMDR2rs>;
///Register `CMDR2` writer
pub type W = crate::W<CMDR2rs>;
///Field `CMD` reader - Command 2. If CMD2E is enabled, the CMD2 sequence will be issued after CMD1 as specified in CCR2 Note: CMD2 sequence cannot be issue individually
pub type CmdR = crate::FieldReader;
///Field `CMD` writer - Command 2. If CMD2E is enabled, the CMD2 sequence will be issued after CMD1 as specified in CCR2 Note: CMD2 sequence cannot be issue individually
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Command 2. If CMD2E is enabled, the CMD2 sequence will be issued after CMD1 as specified in CCR2 Note: CMD2 sequence cannot be issue individually
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDR2").field("cmd", &self.cmd()).finish()
    }
}
impl W {
    ///Bits 0:7 - Command 2. If CMD2E is enabled, the CMD2 sequence will be issued after CMD1 as specified in CCR2 Note: CMD2 sequence cannot be issue individually
    #[inline(always)]
    pub fn cmd(&mut self) -> CmdW<CMDR2rs> {
        CmdW::new(self, 0)
    }
}
///Command Register
///
///You can [`read`](crate::Reg::read) this register and get [`cmdr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CMDR2rs;
impl crate::RegisterSpec for CMDR2rs {
    type Ux = u32;
}
///`read()` method returns [`cmdr2::R`](R) reader structure
impl crate::Readable for CMDR2rs {}
///`write(|w| ..)` method takes [`cmdr2::W`](W) writer structure
impl crate::Writable for CMDR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CMDR2 to value 0
impl crate::Resettable for CMDR2rs {
    const RESET_VALUE: u32 = 0;
}
