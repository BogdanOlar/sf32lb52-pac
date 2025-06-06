///Register `WCR` reader
pub type R = crate::R<WCRrs>;
///Register `WCR` writer
pub type W = crate::W<WCRrs>;
///Field `CNT` reader - Controls the counter values defining the setup and hold times in standard and fast mode Tvddat=Thddat=Tfclk*(CNT+2) Tsudat=max(Tlow-Thddat,Thddat) Lower counter values may violate setup and hold times.
pub type CntR = crate::FieldReader;
///Field `CNT` writer - Controls the counter values defining the setup and hold times in standard and fast mode Tvddat=Thddat=Tfclk*(CNT+2) Tsudat=max(Tlow-Thddat,Thddat) Lower counter values may violate setup and hold times.
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Controls the counter values defining the setup and hold times in standard and fast mode Tvddat=Thddat=Tfclk*(CNT+2) Tsudat=max(Tlow-Thddat,Thddat) Lower counter values may violate setup and hold times.
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WCR").field("cnt", &self.cnt()).finish()
    }
}
impl W {
    ///Bits 0:7 - Controls the counter values defining the setup and hold times in standard and fast mode Tvddat=Thddat=Tfclk*(CNT+2) Tsudat=max(Tlow-Thddat,Thddat) Lower counter values may violate setup and hold times.
    #[inline(always)]
    pub fn cnt(&mut self) -> CntW<WCRrs> {
        CntW::new(self, 0)
    }
}
///Wait Count Register
///
///You can [`read`](crate::Reg::read) this register and get [`wcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct WCRrs;
impl crate::RegisterSpec for WCRrs {
    type Ux = u32;
}
///`read()` method returns [`wcr::R`](R) reader structure
impl crate::Readable for WCRrs {}
///`write(|w| ..)` method takes [`wcr::W`](W) writer structure
impl crate::Writable for WCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets WCR to value 0x0a
impl crate::Resettable for WCRrs {
    const RESET_VALUE: u32 = 0x0a;
}
