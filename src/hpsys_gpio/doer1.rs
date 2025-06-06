///Register `DOER1` reader
pub type R = crate::R<DOER1rs>;
///Register `DOER1` writer
pub type W = crate::W<DOER1rs>;
///Field `DOE` reader - GPIO\[44:32\]
///output enable
pub type DoeR = crate::FieldReader<u16>;
///Field `DOE` writer - GPIO\[44:32\]
///output enable
pub type DoeW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    ///Bits 0:12 - GPIO\[44:32\]
    ///output enable
    #[inline(always)]
    pub fn doe(&self) -> DoeR {
        DoeR::new((self.bits & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOER1").field("doe", &self.doe()).finish()
    }
}
impl W {
    ///Bits 0:12 - GPIO\[44:32\]
    ///output enable
    #[inline(always)]
    pub fn doe(&mut self) -> DoeW<DOER1rs> {
        DoeW::new(self, 0)
    }
}
///Data Output Enable Register
///
///You can [`read`](crate::Reg::read) this register and get [`doer1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doer1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DOER1rs;
impl crate::RegisterSpec for DOER1rs {
    type Ux = u32;
}
///`read()` method returns [`doer1::R`](R) reader structure
impl crate::Readable for DOER1rs {}
///`write(|w| ..)` method takes [`doer1::W`](W) writer structure
impl crate::Writable for DOER1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DOER1 to value 0
impl crate::Resettable for DOER1rs {
    const RESET_VALUE: u32 = 0;
}
