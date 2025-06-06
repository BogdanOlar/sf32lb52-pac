///Register `ISR0` reader
pub type R = crate::R<ISR0rs>;
///Register `ISR0` writer
pub type W = crate::W<ISR0rs>;
///Field `IS` reader - Interrupt status. Write 1 will clear interrupt status of corresponding GPIO\[31:0\]
pub type IsR = crate::FieldReader<u32>;
///Field `IS` writer - Interrupt status. Write 1 will clear interrupt status of corresponding GPIO\[31:0\]
pub type IsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Interrupt status. Write 1 will clear interrupt status of corresponding GPIO\[31:0\]
    #[inline(always)]
    pub fn is(&self) -> IsR {
        IsR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR0").field("is", &self.is()).finish()
    }
}
impl W {
    ///Bits 0:31 - Interrupt status. Write 1 will clear interrupt status of corresponding GPIO\[31:0\]
    #[inline(always)]
    pub fn is(&mut self) -> IsW<ISR0rs> {
        IsW::new(self, 0)
    }
}
///Interrupt Status Register
///
///You can [`read`](crate::Reg::read) this register and get [`isr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ISR0rs;
impl crate::RegisterSpec for ISR0rs {
    type Ux = u32;
}
///`read()` method returns [`isr0::R`](R) reader structure
impl crate::Readable for ISR0rs {}
///`write(|w| ..)` method takes [`isr0::W`](W) writer structure
impl crate::Writable for ISR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ISR0 to value 0
impl crate::Resettable for ISR0rs {
    const RESET_VALUE: u32 = 0;
}
