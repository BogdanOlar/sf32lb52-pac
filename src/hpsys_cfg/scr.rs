///Register `SCR` reader
pub type R = crate::R<SCRrs>;
///Register `SCR` writer
pub type W = crate::W<SCRrs>;
///Field `FKEY_MODE` reader - reserved for debug
pub type FkeyModeR = crate::BitReader;
///Field `FKEY_MODE` writer - reserved for debug
pub type FkeyModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - reserved for debug
    #[inline(always)]
    pub fn fkey_mode(&self) -> FkeyModeR {
        FkeyModeR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCR")
            .field("fkey_mode", &self.fkey_mode())
            .finish()
    }
}
impl W {
    ///Bit 0 - reserved for debug
    #[inline(always)]
    pub fn fkey_mode(&mut self) -> FkeyModeW<SCRrs> {
        FkeyModeW::new(self, 0)
    }
}
///Security Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`scr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SCRrs;
impl crate::RegisterSpec for SCRrs {
    type Ux = u32;
}
///`read()` method returns [`scr::R`](R) reader structure
impl crate::Readable for SCRrs {}
///`write(|w| ..)` method takes [`scr::W`](W) writer structure
impl crate::Writable for SCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SCR to value 0
impl crate::Resettable for SCRrs {
    const RESET_VALUE: u32 = 0;
}
