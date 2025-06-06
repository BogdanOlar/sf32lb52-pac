///Register `SPR` reader
pub type R = crate::R<SPRrs>;
///Register `SPR` writer
pub type W = crate::W<SPRrs>;
///Field `SP` reader - LCPU stack pointer address
pub type SpR = crate::FieldReader<u32>;
///Field `SP` writer - LCPU stack pointer address
pub type SpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - LCPU stack pointer address
    #[inline(always)]
    pub fn sp(&self) -> SpR {
        SpR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPR").field("sp", &self.sp()).finish()
    }
}
impl W {
    ///Bits 0:31 - LCPU stack pointer address
    #[inline(always)]
    pub fn sp(&mut self) -> SpW<SPRrs> {
        SpW::new(self, 0)
    }
}
///Stack Pointer Register
///
///You can [`read`](crate::Reg::read) this register and get [`spr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SPRrs;
impl crate::RegisterSpec for SPRrs {
    type Ux = u32;
}
///`read()` method returns [`spr::R`](R) reader structure
impl crate::Readable for SPRrs {}
///`write(|w| ..)` method takes [`spr::W`](W) writer structure
impl crate::Writable for SPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SPR to value 0
impl crate::Resettable for SPRrs {
    const RESET_VALUE: u32 = 0;
}
