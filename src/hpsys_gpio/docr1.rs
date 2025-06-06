///Register `DOCR1` reader
pub type R = crate::R<DOCR1rs>;
///Register `DOCR1` writer
pub type W = crate::W<DOCR1rs>;
///Field `DOC` reader - set 1 to pull down output of corresponding GPIO\[44:32\]
pub type DocR = crate::FieldReader<u16>;
///Field `DOC` writer - set 1 to pull down output of corresponding GPIO\[44:32\]
pub type DocW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    ///Bits 0:12 - set 1 to pull down output of corresponding GPIO\[44:32\]
    #[inline(always)]
    pub fn doc(&self) -> DocR {
        DocR::new((self.bits & 0x1fff) as u16)
    }
    ///Bits 13:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 13) & 0x0007_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOCR1")
            .field("rsvd", &self.rsvd())
            .field("doc", &self.doc())
            .finish()
    }
}
impl W {
    ///Bits 0:12 - set 1 to pull down output of corresponding GPIO\[44:32\]
    #[inline(always)]
    pub fn doc(&mut self) -> DocW<DOCR1rs> {
        DocW::new(self, 0)
    }
    ///Bits 13:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<DOCR1rs> {
        RsvdW::new(self, 13)
    }
}
///Data Output Clear Register
///
///You can [`read`](crate::Reg::read) this register and get [`docr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`docr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DOCR1rs;
impl crate::RegisterSpec for DOCR1rs {
    type Ux = u32;
}
///`read()` method returns [`docr1::R`](R) reader structure
impl crate::Readable for DOCR1rs {}
///`write(|w| ..)` method takes [`docr1::W`](W) writer structure
impl crate::Writable for DOCR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DOCR1 to value 0
impl crate::Resettable for DOCR1rs {
    const RESET_VALUE: u32 = 0;
}
