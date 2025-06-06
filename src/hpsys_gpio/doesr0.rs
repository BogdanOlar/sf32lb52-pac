///Register `DOESR0` reader
pub type R = crate::R<DOESR0rs>;
///Register `DOESR0` writer
pub type W = crate::W<DOESR0rs>;
///Field `DOES` reader - set 1 to enable output of corresponding GPIO\[31:0\]
pub type DoesR = crate::FieldReader<u32>;
///Field `DOES` writer - set 1 to enable output of corresponding GPIO\[31:0\]
pub type DoesW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - set 1 to enable output of corresponding GPIO\[31:0\]
    #[inline(always)]
    pub fn does(&self) -> DoesR {
        DoesR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOESR0")
            .field("does", &self.does())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - set 1 to enable output of corresponding GPIO\[31:0\]
    #[inline(always)]
    pub fn does(&mut self) -> DoesW<DOESR0rs> {
        DoesW::new(self, 0)
    }
}
///Data Output Enable Set Register
///
///You can [`read`](crate::Reg::read) this register and get [`doesr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doesr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DOESR0rs;
impl crate::RegisterSpec for DOESR0rs {
    type Ux = u32;
}
///`read()` method returns [`doesr0::R`](R) reader structure
impl crate::Readable for DOESR0rs {}
///`write(|w| ..)` method takes [`doesr0::W`](W) writer structure
impl crate::Writable for DOESR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DOESR0 to value 0
impl crate::Resettable for DOESR0rs {
    const RESET_VALUE: u32 = 0;
}
