///Register `IECR1` reader
pub type R = crate::R<IECR1rs>;
///Register `IECR1` writer
pub type W = crate::W<IECR1rs>;
///Field `IEC` reader - set 1 to disable interrupt of corresponding GPIO\[44:32\]
pub type IecR = crate::FieldReader<u16>;
///Field `IEC` writer - set 1 to disable interrupt of corresponding GPIO\[44:32\]
pub type IecW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    ///Bits 0:12 - set 1 to disable interrupt of corresponding GPIO\[44:32\]
    #[inline(always)]
    pub fn iec(&self) -> IecR {
        IecR::new((self.bits & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IECR1").field("iec", &self.iec()).finish()
    }
}
impl W {
    ///Bits 0:12 - set 1 to disable interrupt of corresponding GPIO\[44:32\]
    #[inline(always)]
    pub fn iec(&mut self) -> IecW<IECR1rs> {
        IecW::new(self, 0)
    }
}
///Interrupt Enable Clear Register
///
///You can [`read`](crate::Reg::read) this register and get [`iecr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iecr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IECR1rs;
impl crate::RegisterSpec for IECR1rs {
    type Ux = u32;
}
///`read()` method returns [`iecr1::R`](R) reader structure
impl crate::Readable for IECR1rs {}
///`write(|w| ..)` method takes [`iecr1::W`](W) writer structure
impl crate::Writable for IECR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IECR1 to value 0
impl crate::Resettable for IECR1rs {
    const RESET_VALUE: u32 = 0;
}
