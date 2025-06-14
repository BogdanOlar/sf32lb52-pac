///Register `ITCR1` reader
pub type R = crate::R<ITCR1rs>;
///Register `ITCR1` writer
pub type W = crate::W<ITCR1rs>;
///Field `ITC` reader - set 1 for level-sensitive interrupt mode of corresponding GPIO\[44:32\]
pub type ItcR = crate::FieldReader<u16>;
///Field `ITC` writer - set 1 for level-sensitive interrupt mode of corresponding GPIO\[44:32\]
pub type ItcW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    ///Bits 0:12 - set 1 for level-sensitive interrupt mode of corresponding GPIO\[44:32\]
    #[inline(always)]
    pub fn itc(&self) -> ItcR {
        ItcR::new((self.bits & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITCR1").field("itc", &self.itc()).finish()
    }
}
impl W {
    ///Bits 0:12 - set 1 for level-sensitive interrupt mode of corresponding GPIO\[44:32\]
    #[inline(always)]
    pub fn itc(&mut self) -> ItcW<ITCR1rs> {
        ItcW::new(self, 0)
    }
}
///Interrupt Type Clear Register
///
///You can [`read`](crate::Reg::read) this register and get [`itcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ITCR1rs;
impl crate::RegisterSpec for ITCR1rs {
    type Ux = u32;
}
///`read()` method returns [`itcr1::R`](R) reader structure
impl crate::Readable for ITCR1rs {}
///`write(|w| ..)` method takes [`itcr1::W`](W) writer structure
impl crate::Writable for ITCR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ITCR1 to value 0
impl crate::Resettable for ITCR1rs {
    const RESET_VALUE: u32 = 0;
}
