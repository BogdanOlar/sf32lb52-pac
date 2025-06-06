///Register `ACR` reader
pub type R = crate::R<ACRrs>;
///Register `ACR` writer
pub type W = crate::W<ACRrs>;
///Field `HRC48_REQ` reader - Request hrc48 in active mode
pub type Hrc48ReqR = crate::BitReader;
///Field `HRC48_REQ` writer - Request hrc48 in active mode
pub type Hrc48ReqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HXT48_REQ` reader - Request hxt48 in active mode
pub type Hxt48ReqR = crate::BitReader;
///Field `HXT48_REQ` writer - Request hxt48 in active mode
pub type Hxt48ReqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWR_REQ` reader - Request power during Active mode
pub type PwrReqR = crate::BitReader;
///Field `PWR_REQ` writer - Request power during Active mode
pub type PwrReqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTPWR_REQ` reader - for debug only
pub type ExtpwrReqR = crate::BitReader;
///Field `EXTPWR_REQ` writer - for debug only
pub type ExtpwrReqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HRC48_RDY` reader - Indicate hrc48 is ready
pub type Hrc48RdyR = crate::BitReader;
///Field `HRC48_RDY` writer - Indicate hrc48 is ready
pub type Hrc48RdyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HXT48_RDY` reader - Indicate hxt48 is ready
pub type Hxt48RdyR = crate::BitReader;
///Field `HXT48_RDY` writer - Indicate hxt48 is ready
pub type Hxt48RdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Request hrc48 in active mode
    #[inline(always)]
    pub fn hrc48_req(&self) -> Hrc48ReqR {
        Hrc48ReqR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Request hxt48 in active mode
    #[inline(always)]
    pub fn hxt48_req(&self) -> Hxt48ReqR {
        Hxt48ReqR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Request power during Active mode
    #[inline(always)]
    pub fn pwr_req(&self) -> PwrReqR {
        PwrReqR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - for debug only
    #[inline(always)]
    pub fn extpwr_req(&self) -> ExtpwrReqR {
        ExtpwrReqR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 30 - Indicate hrc48 is ready
    #[inline(always)]
    pub fn hrc48_rdy(&self) -> Hrc48RdyR {
        Hrc48RdyR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Indicate hxt48 is ready
    #[inline(always)]
    pub fn hxt48_rdy(&self) -> Hxt48RdyR {
        Hxt48RdyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACR")
            .field("hxt48_rdy", &self.hxt48_rdy())
            .field("hrc48_rdy", &self.hrc48_rdy())
            .field("extpwr_req", &self.extpwr_req())
            .field("pwr_req", &self.pwr_req())
            .field("hxt48_req", &self.hxt48_req())
            .field("hrc48_req", &self.hrc48_req())
            .finish()
    }
}
impl W {
    ///Bit 0 - Request hrc48 in active mode
    #[inline(always)]
    pub fn hrc48_req(&mut self) -> Hrc48ReqW<ACRrs> {
        Hrc48ReqW::new(self, 0)
    }
    ///Bit 1 - Request hxt48 in active mode
    #[inline(always)]
    pub fn hxt48_req(&mut self) -> Hxt48ReqW<ACRrs> {
        Hxt48ReqW::new(self, 1)
    }
    ///Bit 2 - Request power during Active mode
    #[inline(always)]
    pub fn pwr_req(&mut self) -> PwrReqW<ACRrs> {
        PwrReqW::new(self, 2)
    }
    ///Bit 3 - for debug only
    #[inline(always)]
    pub fn extpwr_req(&mut self) -> ExtpwrReqW<ACRrs> {
        ExtpwrReqW::new(self, 3)
    }
    ///Bit 30 - Indicate hrc48 is ready
    #[inline(always)]
    pub fn hrc48_rdy(&mut self) -> Hrc48RdyW<ACRrs> {
        Hrc48RdyW::new(self, 30)
    }
    ///Bit 31 - Indicate hxt48 is ready
    #[inline(always)]
    pub fn hxt48_rdy(&mut self) -> Hxt48RdyW<ACRrs> {
        Hxt48RdyW::new(self, 31)
    }
}
///Active Mode Control register
///
///You can [`read`](crate::Reg::read) this register and get [`acr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ACRrs;
impl crate::RegisterSpec for ACRrs {
    type Ux = u32;
}
///`read()` method returns [`acr::R`](R) reader structure
impl crate::Readable for ACRrs {}
///`write(|w| ..)` method takes [`acr::W`](W) writer structure
impl crate::Writable for ACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ACR to value 0
impl crate::Resettable for ACRrs {
    const RESET_VALUE: u32 = 0;
}
