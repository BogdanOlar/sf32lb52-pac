///Register `BUCK_VOUT` reader
pub type R = crate::R<BUCK_VOUTrs>;
///Register `BUCK_VOUT` writer
pub type W = crate::W<BUCK_VOUTrs>;
///Field `VOUT` reader - 0xF - 1.35V, 0xD - 1.25V, 0x9 - 1.05V, 0x6 - 0.9V, 0x2 - 0.7V
pub type VoutR = crate::FieldReader;
///Field `VOUT` writer - 0xF - 1.35V, 0xD - 1.25V, 0x9 - 1.05V, 0x6 - 0.9V, 0x2 - 0.7V
pub type VoutW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - 0xF - 1.35V, 0xD - 1.25V, 0x9 - 1.05V, 0x6 - 0.9V, 0x2 - 0.7V
    #[inline(always)]
    pub fn vout(&self) -> VoutR {
        VoutR::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUCK_VOUT")
            .field("vout", &self.vout())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - 0xF - 1.35V, 0xD - 1.25V, 0x9 - 1.05V, 0x6 - 0.9V, 0x2 - 0.7V
    #[inline(always)]
    pub fn vout(&mut self) -> VoutW<BUCK_VOUTrs> {
        VoutW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`buck_vout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buck_vout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct BUCK_VOUTrs;
impl crate::RegisterSpec for BUCK_VOUTrs {
    type Ux = u32;
}
///`read()` method returns [`buck_vout::R`](R) reader structure
impl crate::Readable for BUCK_VOUTrs {}
///`write(|w| ..)` method takes [`buck_vout::W`](W) writer structure
impl crate::Writable for BUCK_VOUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BUCK_VOUT to value 0
impl crate::Resettable for BUCK_VOUTrs {
    const RESET_VALUE: u32 = 0;
}
