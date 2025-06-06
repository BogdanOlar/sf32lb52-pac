///Register `DIR1` reader
pub type R = crate::R<DIR1rs>;
///Register `DIR1` writer
pub type W = crate::W<DIR1rs>;
///Field `IN` reader - GPIO\[44:32\]
///input value
pub type InR = crate::FieldReader<u16>;
///Field `IN` writer - GPIO\[44:32\]
///input value
pub type InW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    ///Bits 0:12 - GPIO\[44:32\]
    ///input value
    #[inline(always)]
    pub fn in_(&self) -> InR {
        InR::new((self.bits & 0x1fff) as u16)
    }
    ///Bits 13:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 13) & 0x0007_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIR1")
            .field("rsvd", &self.rsvd())
            .field("in_", &self.in_())
            .finish()
    }
}
impl W {
    ///Bits 0:12 - GPIO\[44:32\]
    ///input value
    #[inline(always)]
    pub fn in_(&mut self) -> InW<DIR1rs> {
        InW::new(self, 0)
    }
    ///Bits 13:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<DIR1rs> {
        RsvdW::new(self, 13)
    }
}
///Data Input Register
///
///You can [`read`](crate::Reg::read) this register and get [`dir1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dir1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DIR1rs;
impl crate::RegisterSpec for DIR1rs {
    type Ux = u32;
}
///`read()` method returns [`dir1::R`](R) reader structure
impl crate::Readable for DIR1rs {}
///`write(|w| ..)` method takes [`dir1::W`](W) writer structure
impl crate::Writable for DIR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DIR1 to value 0
impl crate::Resettable for DIR1rs {
    const RESET_VALUE: u32 = 0;
}
