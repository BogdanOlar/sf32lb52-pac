///Register `ITCR0` reader
pub type R = crate::R<ITCR0rs>;
///Register `ITCR0` writer
pub type W = crate::W<ITCR0rs>;
///Field `ITC` reader - set 1 for level-sensitive interrupt mode of corresponding GPIO\[31:0\]
pub type ItcR = crate::FieldReader<u32>;
///Field `ITC` writer - set 1 for level-sensitive interrupt mode of corresponding GPIO\[31:0\]
pub type ItcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - set 1 for level-sensitive interrupt mode of corresponding GPIO\[31:0\]
    #[inline(always)]
    pub fn itc(&self) -> ItcR {
        ItcR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITCR0").field("itc", &self.itc()).finish()
    }
}
impl W {
    ///Bits 0:31 - set 1 for level-sensitive interrupt mode of corresponding GPIO\[31:0\]
    #[inline(always)]
    pub fn itc(&mut self) -> ItcW<ITCR0rs> {
        ItcW::new(self, 0)
    }
}
///Interrupt Type Clear Register
///
///You can [`read`](crate::Reg::read) this register and get [`itcr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itcr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ITCR0rs;
impl crate::RegisterSpec for ITCR0rs {
    type Ux = u32;
}
///`read()` method returns [`itcr0::R`](R) reader structure
impl crate::Readable for ITCR0rs {}
///`write(|w| ..)` method takes [`itcr0::W`](W) writer structure
impl crate::Writable for ITCR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ITCR0 to value 0
impl crate::Resettable for ITCR0rs {
    const RESET_VALUE: u32 = 0;
}
