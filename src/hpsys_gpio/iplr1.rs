///Register `IPLR1` reader
pub type R = crate::R<IPLR1rs>;
///Register `IPLR1` writer
pub type W = crate::W<IPLR1rs>;
///Field `IPL` reader - falling edge in edge mode, or low level in level mode of corresponding GPIO\[44:32\]
pub type IplR = crate::FieldReader<u16>;
///Field `IPL` writer - falling edge in edge mode, or low level in level mode of corresponding GPIO\[44:32\]
pub type IplW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    ///Bits 0:12 - falling edge in edge mode, or low level in level mode of corresponding GPIO\[44:32\]
    #[inline(always)]
    pub fn ipl(&self) -> IplR {
        IplR::new((self.bits & 0x1fff) as u16)
    }
    ///Bits 13:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 13) & 0x0007_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPLR1")
            .field("rsvd", &self.rsvd())
            .field("ipl", &self.ipl())
            .finish()
    }
}
impl W {
    ///Bits 0:12 - falling edge in edge mode, or low level in level mode of corresponding GPIO\[44:32\]
    #[inline(always)]
    pub fn ipl(&mut self) -> IplW<IPLR1rs> {
        IplW::new(self, 0)
    }
    ///Bits 13:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<IPLR1rs> {
        RsvdW::new(self, 13)
    }
}
///Interrupt Polarity Low Register
///
///You can [`read`](crate::Reg::read) this register and get [`iplr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iplr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IPLR1rs;
impl crate::RegisterSpec for IPLR1rs {
    type Ux = u32;
}
///`read()` method returns [`iplr1::R`](R) reader structure
impl crate::Readable for IPLR1rs {}
///`write(|w| ..)` method takes [`iplr1::W`](W) writer structure
impl crate::Writable for IPLR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IPLR1 to value 0
impl crate::Resettable for IPLR1rs {
    const RESET_VALUE: u32 = 0;
}
