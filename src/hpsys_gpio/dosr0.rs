///Register `DOSR0` reader
pub type R = crate::R<DOSR0rs>;
///Register `DOSR0` writer
pub type W = crate::W<DOSR0rs>;
///Field `DOS` reader - set 1 to pull up output of corresponding GPIO\[31:0\]
pub type DosR = crate::FieldReader<u32>;
///Field `DOS` writer - set 1 to pull up output of corresponding GPIO\[31:0\]
pub type DosW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - set 1 to pull up output of corresponding GPIO\[31:0\]
    #[inline(always)]
    pub fn dos(&self) -> DosR {
        DosR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOSR0").field("dos", &self.dos()).finish()
    }
}
impl W {
    ///Bits 0:31 - set 1 to pull up output of corresponding GPIO\[31:0\]
    #[inline(always)]
    pub fn dos(&mut self) -> DosW<DOSR0rs> {
        DosW::new(self, 0)
    }
}
///Data Output Set Register
///
///You can [`read`](crate::Reg::read) this register and get [`dosr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dosr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DOSR0rs;
impl crate::RegisterSpec for DOSR0rs {
    type Ux = u32;
}
///`read()` method returns [`dosr0::R`](R) reader structure
impl crate::Readable for DOSR0rs {}
///`write(|w| ..)` method takes [`dosr0::W`](W) writer structure
impl crate::Writable for DOSR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DOSR0 to value 0
impl crate::Resettable for DOSR0rs {
    const RESET_VALUE: u32 = 0;
}
