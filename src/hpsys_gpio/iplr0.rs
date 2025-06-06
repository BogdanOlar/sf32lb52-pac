///Register `IPLR0` reader
pub type R = crate::R<IPLR0rs>;
///Register `IPLR0` writer
pub type W = crate::W<IPLR0rs>;
///Field `IPL` reader - falling edge in edge mode, or low level in level mode of corresponding GPIO\[31:0\]
pub type IplR = crate::FieldReader<u32>;
///Field `IPL` writer - falling edge in edge mode, or low level in level mode of corresponding GPIO\[31:0\]
pub type IplW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - falling edge in edge mode, or low level in level mode of corresponding GPIO\[31:0\]
    #[inline(always)]
    pub fn ipl(&self) -> IplR {
        IplR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPLR0").field("ipl", &self.ipl()).finish()
    }
}
impl W {
    ///Bits 0:31 - falling edge in edge mode, or low level in level mode of corresponding GPIO\[31:0\]
    #[inline(always)]
    pub fn ipl(&mut self) -> IplW<IPLR0rs> {
        IplW::new(self, 0)
    }
}
///Interrupt Polarity Low Register
///
///You can [`read`](crate::Reg::read) this register and get [`iplr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iplr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IPLR0rs;
impl crate::RegisterSpec for IPLR0rs {
    type Ux = u32;
}
///`read()` method returns [`iplr0::R`](R) reader structure
impl crate::Readable for IPLR0rs {}
///`write(|w| ..)` method takes [`iplr0::W`](W) writer structure
impl crate::Writable for IPLR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IPLR0 to value 0
impl crate::Resettable for IPLR0rs {
    const RESET_VALUE: u32 = 0;
}
