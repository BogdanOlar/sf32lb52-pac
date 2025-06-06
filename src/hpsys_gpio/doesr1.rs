///Register `DOESR1` reader
pub type R = crate::R<DOESR1rs>;
///Register `DOESR1` writer
pub type W = crate::W<DOESR1rs>;
///Field `DOES` reader - set 1 to enable output of corresponding GPIO\[44:32\]
pub type DoesR = crate::FieldReader<u16>;
///Field `DOES` writer - set 1 to enable output of corresponding GPIO\[44:32\]
pub type DoesW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    ///Bits 0:12 - set 1 to enable output of corresponding GPIO\[44:32\]
    #[inline(always)]
    pub fn does(&self) -> DoesR {
        DoesR::new((self.bits & 0x1fff) as u16)
    }
    ///Bits 13:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 13) & 0x0007_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOESR1")
            .field("rsvd", &self.rsvd())
            .field("does", &self.does())
            .finish()
    }
}
impl W {
    ///Bits 0:12 - set 1 to enable output of corresponding GPIO\[44:32\]
    #[inline(always)]
    pub fn does(&mut self) -> DoesW<DOESR1rs> {
        DoesW::new(self, 0)
    }
    ///Bits 13:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<DOESR1rs> {
        RsvdW::new(self, 13)
    }
}
///Data Output Enable Set Register
///
///You can [`read`](crate::Reg::read) this register and get [`doesr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doesr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DOESR1rs;
impl crate::RegisterSpec for DOESR1rs {
    type Ux = u32;
}
///`read()` method returns [`doesr1::R`](R) reader structure
impl crate::Readable for DOESR1rs {}
///`write(|w| ..)` method takes [`doesr1::W`](W) writer structure
impl crate::Writable for DOESR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DOESR1 to value 0
impl crate::Resettable for DOESR1rs {
    const RESET_VALUE: u32 = 0;
}
