///Register `IER0` reader
pub type R = crate::R<IER0rs>;
///Register `IER0` writer
pub type W = crate::W<IER0rs>;
///Field `IER` reader - GPIO\[31:0\]
///interrupt enable
pub type IerR = crate::FieldReader<u32>;
///Field `IER` writer - GPIO\[31:0\]
///interrupt enable
pub type IerW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - GPIO\[31:0\]
    ///interrupt enable
    #[inline(always)]
    pub fn ier(&self) -> IerR {
        IerR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER0").field("ier", &self.ier()).finish()
    }
}
impl W {
    ///Bits 0:31 - GPIO\[31:0\]
    ///interrupt enable
    #[inline(always)]
    pub fn ier(&mut self) -> IerW<IER0rs> {
        IerW::new(self, 0)
    }
}
///Interrupt Enable Register
///
///You can [`read`](crate::Reg::read) this register and get [`ier0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IER0rs;
impl crate::RegisterSpec for IER0rs {
    type Ux = u32;
}
///`read()` method returns [`ier0::R`](R) reader structure
impl crate::Readable for IER0rs {}
///`write(|w| ..)` method takes [`ier0::W`](W) writer structure
impl crate::Writable for IER0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IER0 to value 0
impl crate::Resettable for IER0rs {
    const RESET_VALUE: u32 = 0;
}
