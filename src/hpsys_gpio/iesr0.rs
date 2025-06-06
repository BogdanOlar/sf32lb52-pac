///Register `IESR0` reader
pub type R = crate::R<IESR0rs>;
///Register `IESR0` writer
pub type W = crate::W<IESR0rs>;
///Field `IES` reader - set 1 to enable interrupt of corresponding GPIO\[31:0\]
pub type IesR = crate::FieldReader<u32>;
///Field `IES` writer - set 1 to enable interrupt of corresponding GPIO\[31:0\]
pub type IesW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - set 1 to enable interrupt of corresponding GPIO\[31:0\]
    #[inline(always)]
    pub fn ies(&self) -> IesR {
        IesR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IESR0").field("ies", &self.ies()).finish()
    }
}
impl W {
    ///Bits 0:31 - set 1 to enable interrupt of corresponding GPIO\[31:0\]
    #[inline(always)]
    pub fn ies(&mut self) -> IesW<IESR0rs> {
        IesW::new(self, 0)
    }
}
///Interrupt Enable Set Register
///
///You can [`read`](crate::Reg::read) this register and get [`iesr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iesr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IESR0rs;
impl crate::RegisterSpec for IESR0rs {
    type Ux = u32;
}
///`read()` method returns [`iesr0::R`](R) reader structure
impl crate::Readable for IESR0rs {}
///`write(|w| ..)` method takes [`iesr0::W`](W) writer structure
impl crate::Writable for IESR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IESR0 to value 0
impl crate::Resettable for IESR0rs {
    const RESET_VALUE: u32 = 0;
}
