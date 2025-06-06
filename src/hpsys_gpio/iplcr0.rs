///Register `IPLCR0` reader
pub type R = crate::R<IPLCR0rs>;
///Register `IPLCR0` writer
pub type W = crate::W<IPLCR0rs>;
///Field `IPLC` reader - set 1 for disable falling edge in edge mode, or low level in level mode of corresponding GPIO\[31:0\]
pub type IplcR = crate::FieldReader<u32>;
///Field `IPLC` writer - set 1 for disable falling edge in edge mode, or low level in level mode of corresponding GPIO\[31:0\]
pub type IplcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - set 1 for disable falling edge in edge mode, or low level in level mode of corresponding GPIO\[31:0\]
    #[inline(always)]
    pub fn iplc(&self) -> IplcR {
        IplcR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPLCR0")
            .field("iplc", &self.iplc())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - set 1 for disable falling edge in edge mode, or low level in level mode of corresponding GPIO\[31:0\]
    #[inline(always)]
    pub fn iplc(&mut self) -> IplcW<IPLCR0rs> {
        IplcW::new(self, 0)
    }
}
///Interrupt Polarity Low Clear Register
///
///You can [`read`](crate::Reg::read) this register and get [`iplcr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iplcr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IPLCR0rs;
impl crate::RegisterSpec for IPLCR0rs {
    type Ux = u32;
}
///`read()` method returns [`iplcr0::R`](R) reader structure
impl crate::Readable for IPLCR0rs {}
///`write(|w| ..)` method takes [`iplcr0::W`](W) writer structure
impl crate::Writable for IPLCR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IPLCR0 to value 0
impl crate::Resettable for IPLCR0rs {
    const RESET_VALUE: u32 = 0;
}
