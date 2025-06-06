///Register `CCR5` reader
pub type R = crate::R<CCR5rs>;
///Register `CCR5` writer
pub type W = crate::W<CCR5rs>;
///Field `CCR5` reader - Capture/Compare 5 value CCR5 is the value to be loaded in the actual capture/compare 5 register (preload value). It is loaded permanently if the preload feature is not selected in the CCMR3 register (bit OC5PE). Else the preload value is copied in the active capture/compare 5 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter CNT and signaled on OC5 output.
pub type Ccr5R = crate::FieldReader<u32>;
///Field `CCR5` writer - Capture/Compare 5 value CCR5 is the value to be loaded in the actual capture/compare 5 register (preload value). It is loaded permanently if the preload feature is not selected in the CCMR3 register (bit OC5PE). Else the preload value is copied in the active capture/compare 5 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter CNT and signaled on OC5 output.
pub type Ccr5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Capture/Compare 5 value CCR5 is the value to be loaded in the actual capture/compare 5 register (preload value). It is loaded permanently if the preload feature is not selected in the CCMR3 register (bit OC5PE). Else the preload value is copied in the active capture/compare 5 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter CNT and signaled on OC5 output.
    #[inline(always)]
    pub fn ccr5(&self) -> Ccr5R {
        Ccr5R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR5").field("ccr5", &self.ccr5()).finish()
    }
}
impl W {
    ///Bits 0:31 - Capture/Compare 5 value CCR5 is the value to be loaded in the actual capture/compare 5 register (preload value). It is loaded permanently if the preload feature is not selected in the CCMR3 register (bit OC5PE). Else the preload value is copied in the active capture/compare 5 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter CNT and signaled on OC5 output.
    #[inline(always)]
    pub fn ccr5(&mut self) -> Ccr5W<CCR5rs> {
        Ccr5W::new(self, 0)
    }
}
///Capture/Compare register 5
///
///You can [`read`](crate::Reg::read) this register and get [`ccr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CCR5rs;
impl crate::RegisterSpec for CCR5rs {
    type Ux = u32;
}
///`read()` method returns [`ccr5::R`](R) reader structure
impl crate::Readable for CCR5rs {}
///`write(|w| ..)` method takes [`ccr5::W`](W) writer structure
impl crate::Writable for CCR5rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CCR5 to value 0
impl crate::Resettable for CCR5rs {
    const RESET_VALUE: u32 = 0;
}
