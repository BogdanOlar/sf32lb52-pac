///Register `CTREAR` reader
pub type R = crate::R<CTREARrs>;
///Register `CTREAR` writer
pub type W = crate::W<CTREARrs>;
///Field `EA` reader - Ending address of the AES decryption area
pub type EaR = crate::FieldReader<u32>;
///Field `EA` writer - Ending address of the AES decryption area
pub type EaW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    ///Bits 10:31 - Ending address of the AES decryption area
    #[inline(always)]
    pub fn ea(&self) -> EaR {
        EaR::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTREAR").field("ea", &self.ea()).finish()
    }
}
impl W {
    ///Bits 10:31 - Ending address of the AES decryption area
    #[inline(always)]
    pub fn ea(&mut self) -> EaW<CTREARrs> {
        EaW::new(self, 10)
    }
}
///CTR Ending Address Register
///
///You can [`read`](crate::Reg::read) this register and get [`ctrear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CTREARrs;
impl crate::RegisterSpec for CTREARrs {
    type Ux = u32;
}
///`read()` method returns [`ctrear::R`](R) reader structure
impl crate::Readable for CTREARrs {}
///`write(|w| ..)` method takes [`ctrear::W`](W) writer structure
impl crate::Writable for CTREARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CTREAR to value 0
impl crate::Resettable for CTREARrs {
    const RESET_VALUE: u32 = 0;
}
