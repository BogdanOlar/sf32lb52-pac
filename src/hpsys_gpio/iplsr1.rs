///Register `IPLSR1` reader
pub type R = crate::R<IPLSR1rs>;
///Register `IPLSR1` writer
pub type W = crate::W<IPLSR1rs>;
///Field `IPLS` reader - set 1 for falling edge in edge mode, or low level in level mode of corresponding GPIO\[44:32\]
pub type IplsR = crate::FieldReader<u16>;
///Field `IPLS` writer - set 1 for falling edge in edge mode, or low level in level mode of corresponding GPIO\[44:32\]
pub type IplsW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    ///Bits 0:12 - set 1 for falling edge in edge mode, or low level in level mode of corresponding GPIO\[44:32\]
    #[inline(always)]
    pub fn ipls(&self) -> IplsR {
        IplsR::new((self.bits & 0x1fff) as u16)
    }
    ///Bits 13:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 13) & 0x0007_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPLSR1")
            .field("rsvd", &self.rsvd())
            .field("ipls", &self.ipls())
            .finish()
    }
}
impl W {
    ///Bits 0:12 - set 1 for falling edge in edge mode, or low level in level mode of corresponding GPIO\[44:32\]
    #[inline(always)]
    pub fn ipls(&mut self) -> IplsW<IPLSR1rs> {
        IplsW::new(self, 0)
    }
    ///Bits 13:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<IPLSR1rs> {
        RsvdW::new(self, 13)
    }
}
///Interrupt Polarity Low Set Register
///
///You can [`read`](crate::Reg::read) this register and get [`iplsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iplsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IPLSR1rs;
impl crate::RegisterSpec for IPLSR1rs {
    type Ux = u32;
}
///`read()` method returns [`iplsr1::R`](R) reader structure
impl crate::Readable for IPLSR1rs {}
///`write(|w| ..)` method takes [`iplsr1::W`](W) writer structure
impl crate::Writable for IPLSR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IPLSR1 to value 0
impl crate::Resettable for IPLSR1rs {
    const RESET_VALUE: u32 = 0;
}
