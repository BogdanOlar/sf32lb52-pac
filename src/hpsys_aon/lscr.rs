///Register `LSCR` reader
pub type R = crate::R<LSCRrs>;
///Register `LSCR` writer
pub type W = crate::W<LSCRrs>;
///Field `HRC48_REQ` reader - Request hrc48 in Light Sleep mode
pub type Hrc48ReqR = crate::BitReader;
///Field `HRC48_REQ` writer - Request hrc48 in Light Sleep mode
pub type Hrc48ReqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HXT48_REQ` reader - Request hxt48 in Light Sleep mode
pub type Hxt48ReqR = crate::BitReader;
///Field `HXT48_REQ` writer - Request hxt48 in Light Sleep mode
pub type Hxt48ReqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWR_REQ` reader - Request power during Light Sleep mode
pub type PwrReqR = crate::BitReader;
///Field `PWR_REQ` writer - Request power during Light Sleep mode
pub type PwrReqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTPWR_REQ` reader - for debug only
pub type ExtpwrReqR = crate::BitReader;
///Field `EXTPWR_REQ` writer - for debug only
pub type ExtpwrReqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Request hrc48 in Light Sleep mode
    #[inline(always)]
    pub fn hrc48_req(&self) -> Hrc48ReqR {
        Hrc48ReqR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Request hxt48 in Light Sleep mode
    #[inline(always)]
    pub fn hxt48_req(&self) -> Hxt48ReqR {
        Hxt48ReqR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Request power during Light Sleep mode
    #[inline(always)]
    pub fn pwr_req(&self) -> PwrReqR {
        PwrReqR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - for debug only
    #[inline(always)]
    pub fn extpwr_req(&self) -> ExtpwrReqR {
        ExtpwrReqR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LSCR")
            .field("extpwr_req", &self.extpwr_req())
            .field("pwr_req", &self.pwr_req())
            .field("hxt48_req", &self.hxt48_req())
            .field("hrc48_req", &self.hrc48_req())
            .finish()
    }
}
impl W {
    ///Bit 0 - Request hrc48 in Light Sleep mode
    #[inline(always)]
    pub fn hrc48_req(&mut self) -> Hrc48ReqW<LSCRrs> {
        Hrc48ReqW::new(self, 0)
    }
    ///Bit 1 - Request hxt48 in Light Sleep mode
    #[inline(always)]
    pub fn hxt48_req(&mut self) -> Hxt48ReqW<LSCRrs> {
        Hxt48ReqW::new(self, 1)
    }
    ///Bit 2 - Request power during Light Sleep mode
    #[inline(always)]
    pub fn pwr_req(&mut self) -> PwrReqW<LSCRrs> {
        PwrReqW::new(self, 2)
    }
    ///Bit 3 - for debug only
    #[inline(always)]
    pub fn extpwr_req(&mut self) -> ExtpwrReqW<LSCRrs> {
        ExtpwrReqW::new(self, 3)
    }
}
///Light Sleep Ctrl Register
///
///You can [`read`](crate::Reg::read) this register and get [`lscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct LSCRrs;
impl crate::RegisterSpec for LSCRrs {
    type Ux = u32;
}
///`read()` method returns [`lscr::R`](R) reader structure
impl crate::Readable for LSCRrs {}
///`write(|w| ..)` method takes [`lscr::W`](W) writer structure
impl crate::Writable for LSCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LSCR to value 0
impl crate::Resettable for LSCRrs {
    const RESET_VALUE: u32 = 0;
}
