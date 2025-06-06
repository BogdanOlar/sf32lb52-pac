///Register `HPSYS_VOUT` reader
pub type R = crate::R<HPSYS_VOUTrs>;
///Register `HPSYS_VOUT` writer
pub type W = crate::W<HPSYS_VOUTrs>;
///Field `VOUT` reader - 0xD - 1.2V, 0xA - 1.1V, 0x8 - 1.0V, 0x5 - 0.9V
pub type VoutR = crate::FieldReader;
///Field `VOUT` writer - 0xD - 1.2V, 0xA - 1.1V, 0x8 - 1.0V, 0x5 - 0.9V
pub type VoutW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - 0xD - 1.2V, 0xA - 1.1V, 0x8 - 1.0V, 0x5 - 0.9V
    #[inline(always)]
    pub fn vout(&self) -> VoutR {
        VoutR::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPSYS_VOUT")
            .field("vout", &self.vout())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - 0xD - 1.2V, 0xA - 1.1V, 0x8 - 1.0V, 0x5 - 0.9V
    #[inline(always)]
    pub fn vout(&mut self) -> VoutW<HPSYS_VOUTrs> {
        VoutW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`hpsys_vout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpsys_vout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct HPSYS_VOUTrs;
impl crate::RegisterSpec for HPSYS_VOUTrs {
    type Ux = u32;
}
///`read()` method returns [`hpsys_vout::R`](R) reader structure
impl crate::Readable for HPSYS_VOUTrs {}
///`write(|w| ..)` method takes [`hpsys_vout::W`](W) writer structure
impl crate::Writable for HPSYS_VOUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HPSYS_VOUT to value 0
impl crate::Resettable for HPSYS_VOUTrs {
    const RESET_VALUE: u32 = 0;
}
