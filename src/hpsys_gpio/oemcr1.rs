///Register `OEMCR1` reader
pub type R = crate::R<OEMCR1rs>;
///Register `OEMCR1` writer
pub type W = crate::W<OEMCR1rs>;
///Field `OEMC` reader - output mode Clear of corresponding GPIO\[44:32\]
pub type OemcR = crate::FieldReader<u16>;
///Field `OEMC` writer - output mode Clear of corresponding GPIO\[44:32\]
pub type OemcW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    ///Bits 0:12 - output mode Clear of corresponding GPIO\[44:32\]
    #[inline(always)]
    pub fn oemc(&self) -> OemcR {
        OemcR::new((self.bits & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OEMCR1")
            .field("oemc", &self.oemc())
            .finish()
    }
}
impl W {
    ///Bits 0:12 - output mode Clear of corresponding GPIO\[44:32\]
    #[inline(always)]
    pub fn oemc(&mut self) -> OemcW<OEMCR1rs> {
        OemcW::new(self, 0)
    }
}
///output mode Clear Register
///
///You can [`read`](crate::Reg::read) this register and get [`oemcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oemcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct OEMCR1rs;
impl crate::RegisterSpec for OEMCR1rs {
    type Ux = u32;
}
///`read()` method returns [`oemcr1::R`](R) reader structure
impl crate::Readable for OEMCR1rs {}
///`write(|w| ..)` method takes [`oemcr1::W`](W) writer structure
impl crate::Writable for OEMCR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OEMCR1 to value 0
impl crate::Resettable for OEMCR1rs {
    const RESET_VALUE: u32 = 0;
}
