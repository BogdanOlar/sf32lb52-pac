///Register `AAOAR` reader
pub type R = crate::R<AAOARrs>;
///Register `AAOAR` writer
pub type W = crate::W<AAOARrs>;
///Field `OA` reader - The offset to be added to the original address
pub type OaR = crate::FieldReader<u32>;
///Field `OA` writer - The offset to be added to the original address
pub type OaW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    ///Bits 10:31 - The offset to be added to the original address
    #[inline(always)]
    pub fn oa(&self) -> OaR {
        OaR::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AAOAR").field("oa", &self.oa()).finish()
    }
}
impl W {
    ///Bits 10:31 - The offset to be added to the original address
    #[inline(always)]
    pub fn oa(&mut self) -> OaW<AAOARrs> {
        OaW::new(self, 10)
    }
}
///Address Aliasing Offset Address Register
///
///You can [`read`](crate::Reg::read) this register and get [`aaoar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aaoar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct AAOARrs;
impl crate::RegisterSpec for AAOARrs {
    type Ux = u32;
}
///`read()` method returns [`aaoar::R`](R) reader structure
impl crate::Readable for AAOARrs {}
///`write(|w| ..)` method takes [`aaoar::W`](W) writer structure
impl crate::Writable for AAOARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AAOAR to value 0
impl crate::Resettable for AAOARrs {
    const RESET_VALUE: u32 = 0;
}
