///Register `ITSR1` reader
pub type R = crate::R<ITSR1rs>;
///Register `ITSR1` writer
pub type W = crate::W<ITSR1rs>;
///Field `ITS` reader - set 1 for edge-sensitive interrupt mode of corresponding GPIO\[44:32\]
pub type ItsR = crate::FieldReader<u16>;
///Field `ITS` writer - set 1 for edge-sensitive interrupt mode of corresponding GPIO\[44:32\]
pub type ItsW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    ///Bits 0:12 - set 1 for edge-sensitive interrupt mode of corresponding GPIO\[44:32\]
    #[inline(always)]
    pub fn its(&self) -> ItsR {
        ItsR::new((self.bits & 0x1fff) as u16)
    }
    ///Bits 13:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 13) & 0x0007_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITSR1")
            .field("rsvd", &self.rsvd())
            .field("its", &self.its())
            .finish()
    }
}
impl W {
    ///Bits 0:12 - set 1 for edge-sensitive interrupt mode of corresponding GPIO\[44:32\]
    #[inline(always)]
    pub fn its(&mut self) -> ItsW<ITSR1rs> {
        ItsW::new(self, 0)
    }
    ///Bits 13:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<ITSR1rs> {
        RsvdW::new(self, 13)
    }
}
///Interrupt Type Set Register
///
///You can [`read`](crate::Reg::read) this register and get [`itsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ITSR1rs;
impl crate::RegisterSpec for ITSR1rs {
    type Ux = u32;
}
///`read()` method returns [`itsr1::R`](R) reader structure
impl crate::Readable for ITSR1rs {}
///`write(|w| ..)` method takes [`itsr1::W`](W) writer structure
impl crate::Writable for ITSR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ITSR1 to value 0
impl crate::Resettable for ITSR1rs {
    const RESET_VALUE: u32 = 0;
}
