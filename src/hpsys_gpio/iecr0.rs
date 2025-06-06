///Register `IECR0` reader
pub type R = crate::R<IECR0rs>;
///Register `IECR0` writer
pub type W = crate::W<IECR0rs>;
///Field `IEC` reader - set 1 to disable interrupt of corresponding GPIO\[31:0\]
pub type IecR = crate::FieldReader<u32>;
///Field `IEC` writer - set 1 to disable interrupt of corresponding GPIO\[31:0\]
pub type IecW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - set 1 to disable interrupt of corresponding GPIO\[31:0\]
    #[inline(always)]
    pub fn iec(&self) -> IecR {
        IecR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IECR0").field("iec", &self.iec()).finish()
    }
}
impl W {
    ///Bits 0:31 - set 1 to disable interrupt of corresponding GPIO\[31:0\]
    #[inline(always)]
    pub fn iec(&mut self) -> IecW<IECR0rs> {
        IecW::new(self, 0)
    }
}
///Interrupt Enable Clear Register
///
///You can [`read`](crate::Reg::read) this register and get [`iecr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iecr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IECR0rs;
impl crate::RegisterSpec for IECR0rs {
    type Ux = u32;
}
///`read()` method returns [`iecr0::R`](R) reader structure
impl crate::Readable for IECR0rs {}
///`write(|w| ..)` method takes [`iecr0::W`](W) writer structure
impl crate::Writable for IECR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IECR0 to value 0
impl crate::Resettable for IECR0rs {
    const RESET_VALUE: u32 = 0;
}
