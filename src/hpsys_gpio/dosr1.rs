///Register `DOSR1` reader
pub type R = crate::R<DOSR1rs>;
///Register `DOSR1` writer
pub type W = crate::W<DOSR1rs>;
///Field `DOS` reader - set 1 to pull up output of corresponding GPIO\[44:32\]
pub type DosR = crate::FieldReader<u16>;
///Field `DOS` writer - set 1 to pull up output of corresponding GPIO\[44:32\]
pub type DosW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    ///Bits 0:12 - set 1 to pull up output of corresponding GPIO\[44:32\]
    #[inline(always)]
    pub fn dos(&self) -> DosR {
        DosR::new((self.bits & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOSR1").field("dos", &self.dos()).finish()
    }
}
impl W {
    ///Bits 0:12 - set 1 to pull up output of corresponding GPIO\[44:32\]
    #[inline(always)]
    pub fn dos(&mut self) -> DosW<DOSR1rs> {
        DosW::new(self, 0)
    }
}
///Data Output Set Register
///
///You can [`read`](crate::Reg::read) this register and get [`dosr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dosr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DOSR1rs;
impl crate::RegisterSpec for DOSR1rs {
    type Ux = u32;
}
///`read()` method returns [`dosr1::R`](R) reader structure
impl crate::Readable for DOSR1rs {}
///`write(|w| ..)` method takes [`dosr1::W`](W) writer structure
impl crate::Writable for DOSR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DOSR1 to value 0
impl crate::Resettable for DOSR1rs {
    const RESET_VALUE: u32 = 0;
}
