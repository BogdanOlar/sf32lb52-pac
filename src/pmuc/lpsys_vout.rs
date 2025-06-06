///Register `LPSYS_VOUT` reader
pub type R = crate::R<LPSYS_VOUTrs>;
///Register `LPSYS_VOUT` writer
pub type W = crate::W<LPSYS_VOUTrs>;
///Field `VOUT` reader - 0x8 - 1.0V, 0x5 - 0.9V
pub type VoutR = crate::FieldReader;
///Field `VOUT` writer - 0x8 - 1.0V, 0x5 - 0.9V
pub type VoutW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    ///Bits 0:3 - 0x8 - 1.0V, 0x5 - 0.9V
    #[inline(always)]
    pub fn vout(&self) -> VoutR {
        VoutR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPSYS_VOUT")
            .field("rsvd", &self.rsvd())
            .field("vout", &self.vout())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - 0x8 - 1.0V, 0x5 - 0.9V
    #[inline(always)]
    pub fn vout(&mut self) -> VoutW<LPSYS_VOUTrs> {
        VoutW::new(self, 0)
    }
    ///Bits 4:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<LPSYS_VOUTrs> {
        RsvdW::new(self, 4)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`lpsys_vout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpsys_vout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct LPSYS_VOUTrs;
impl crate::RegisterSpec for LPSYS_VOUTrs {
    type Ux = u32;
}
///`read()` method returns [`lpsys_vout::R`](R) reader structure
impl crate::Readable for LPSYS_VOUTrs {}
///`write(|w| ..)` method takes [`lpsys_vout::W`](W) writer structure
impl crate::Writable for LPSYS_VOUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LPSYS_VOUT to value 0
impl crate::Resettable for LPSYS_VOUTrs {
    const RESET_VALUE: u32 = 0;
}
