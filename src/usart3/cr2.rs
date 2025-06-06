///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `STOP` reader - Stop bits 0/1: 1 stop bit 2/3: 2 stop bits
pub type StopR = crate::FieldReader;
///Field `STOP` writer - Stop bits 0/1: 1 stop bit 2/3: 2 stop bits
pub type StopW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 12:13 - Stop bits 0/1: 1 stop bit 2/3: 2 stop bits
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2").field("stop", &self.stop()).finish()
    }
}
impl W {
    ///Bits 12:13 - Stop bits 0/1: 1 stop bit 2/3: 2 stop bits
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<CR2rs> {
        StopW::new(self, 12)
    }
}
///Control Register 2
///
///You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0;
}
