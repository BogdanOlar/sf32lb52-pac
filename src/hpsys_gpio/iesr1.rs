///Register `IESR1` reader
pub type R = crate::R<IESR1rs>;
///Register `IESR1` writer
pub type W = crate::W<IESR1rs>;
///Field `IES` reader - set 1 to enable interrupt of corresponding GPIO\[44:32\]
pub type IesR = crate::FieldReader<u16>;
///Field `IES` writer - set 1 to enable interrupt of corresponding GPIO\[44:32\]
pub type IesW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    ///Bits 0:12 - set 1 to enable interrupt of corresponding GPIO\[44:32\]
    #[inline(always)]
    pub fn ies(&self) -> IesR {
        IesR::new((self.bits & 0x1fff) as u16)
    }
    ///Bits 13:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 13) & 0x0007_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IESR1")
            .field("rsvd", &self.rsvd())
            .field("ies", &self.ies())
            .finish()
    }
}
impl W {
    ///Bits 0:12 - set 1 to enable interrupt of corresponding GPIO\[44:32\]
    #[inline(always)]
    pub fn ies(&mut self) -> IesW<IESR1rs> {
        IesW::new(self, 0)
    }
    ///Bits 13:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<IESR1rs> {
        RsvdW::new(self, 13)
    }
}
///Interrupt Enable Set Register
///
///You can [`read`](crate::Reg::read) this register and get [`iesr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iesr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IESR1rs;
impl crate::RegisterSpec for IESR1rs {
    type Ux = u32;
}
///`read()` method returns [`iesr1::R`](R) reader structure
impl crate::Readable for IESR1rs {}
///`write(|w| ..)` method takes [`iesr1::W`](W) writer structure
impl crate::Writable for IESR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IESR1 to value 0
impl crate::Resettable for IESR1rs {
    const RESET_VALUE: u32 = 0;
}
