///Register `ITSR0` reader
pub type R = crate::R<ITSR0rs>;
///Register `ITSR0` writer
pub type W = crate::W<ITSR0rs>;
///Field `ITS` reader - set 1 for edge-sensitive interrupt mode of corresponding GPIO\[31:0\]
pub type ItsR = crate::FieldReader<u32>;
///Field `ITS` writer - set 1 for edge-sensitive interrupt mode of corresponding GPIO\[31:0\]
pub type ItsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - set 1 for edge-sensitive interrupt mode of corresponding GPIO\[31:0\]
    #[inline(always)]
    pub fn its(&self) -> ItsR {
        ItsR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITSR0").field("its", &self.its()).finish()
    }
}
impl W {
    ///Bits 0:31 - set 1 for edge-sensitive interrupt mode of corresponding GPIO\[31:0\]
    #[inline(always)]
    pub fn its(&mut self) -> ItsW<ITSR0rs> {
        ItsW::new(self, 0)
    }
}
///Interrupt Type Set Register
///
///You can [`read`](crate::Reg::read) this register and get [`itsr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itsr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ITSR0rs;
impl crate::RegisterSpec for ITSR0rs {
    type Ux = u32;
}
///`read()` method returns [`itsr0::R`](R) reader structure
impl crate::Readable for ITSR0rs {}
///`write(|w| ..)` method takes [`itsr0::W`](W) writer structure
impl crate::Writable for ITSR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ITSR0 to value 0
impl crate::Resettable for ITSR0rs {
    const RESET_VALUE: u32 = 0;
}
