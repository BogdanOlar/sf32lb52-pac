///Register `DOECR1` reader
pub type R = crate::R<DOECR1rs>;
///Register `DOECR1` writer
pub type W = crate::W<DOECR1rs>;
///Field `DOEC` reader - set 1 to disable output of corresponding GPIO\[44:32\]
pub type DoecR = crate::FieldReader<u16>;
///Field `DOEC` writer - set 1 to disable output of corresponding GPIO\[44:32\]
pub type DoecW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    ///Bits 0:12 - set 1 to disable output of corresponding GPIO\[44:32\]
    #[inline(always)]
    pub fn doec(&self) -> DoecR {
        DoecR::new((self.bits & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOECR1")
            .field("doec", &self.doec())
            .finish()
    }
}
impl W {
    ///Bits 0:12 - set 1 to disable output of corresponding GPIO\[44:32\]
    #[inline(always)]
    pub fn doec(&mut self) -> DoecW<DOECR1rs> {
        DoecW::new(self, 0)
    }
}
///Data Output Enable Clear Register
///
///You can [`read`](crate::Reg::read) this register and get [`doecr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doecr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DOECR1rs;
impl crate::RegisterSpec for DOECR1rs {
    type Ux = u32;
}
///`read()` method returns [`doecr1::R`](R) reader structure
impl crate::Readable for DOECR1rs {}
///`write(|w| ..)` method takes [`doecr1::W`](W) writer structure
impl crate::Writable for DOECR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DOECR1 to value 0
impl crate::Resettable for DOECR1rs {
    const RESET_VALUE: u32 = 0;
}
