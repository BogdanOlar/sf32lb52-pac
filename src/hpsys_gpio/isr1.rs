///Register `ISR1` reader
pub type R = crate::R<ISR1rs>;
///Register `ISR1` writer
pub type W = crate::W<ISR1rs>;
///Field `IS` reader - Interrupt status. Write 1 will clear interrupt status of corresponding GPIO\[44:32\]
pub type IsR = crate::FieldReader<u16>;
///Field `IS` writer - Interrupt status. Write 1 will clear interrupt status of corresponding GPIO\[44:32\]
pub type IsW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    ///Bits 0:12 - Interrupt status. Write 1 will clear interrupt status of corresponding GPIO\[44:32\]
    #[inline(always)]
    pub fn is(&self) -> IsR {
        IsR::new((self.bits & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR1").field("is", &self.is()).finish()
    }
}
impl W {
    ///Bits 0:12 - Interrupt status. Write 1 will clear interrupt status of corresponding GPIO\[44:32\]
    #[inline(always)]
    pub fn is(&mut self) -> IsW<ISR1rs> {
        IsW::new(self, 0)
    }
}
///Interrupt Status Register
///
///You can [`read`](crate::Reg::read) this register and get [`isr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ISR1rs;
impl crate::RegisterSpec for ISR1rs {
    type Ux = u32;
}
///`read()` method returns [`isr1::R`](R) reader structure
impl crate::Readable for ISR1rs {}
///`write(|w| ..)` method takes [`isr1::W`](W) writer structure
impl crate::Writable for ISR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ISR1 to value 0
impl crate::Resettable for ISR1rs {
    const RESET_VALUE: u32 = 0;
}
