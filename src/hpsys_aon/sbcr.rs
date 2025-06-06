///Register `SBCR` reader
pub type R = crate::R<SBCRrs>;
///Register `SBCR` writer
pub type W = crate::W<SBCRrs>;
///Field `HRC48_REQ` reader - Request hrc48 in Standby mode
pub type Hrc48ReqR = crate::BitReader;
///Field `HRC48_REQ` writer - Request hrc48 in Standby mode
pub type Hrc48ReqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HXT48_REQ` reader - Request hxt48 in Standby mode
pub type Hxt48ReqR = crate::BitReader;
///Field `HXT48_REQ` writer - Request hxt48 in Standby mode
pub type Hxt48ReqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWR_REQ` reader - Request power during Standby mode
pub type PwrReqR = crate::BitReader;
///Field `PWR_REQ` writer - Request power during Standby mode
pub type PwrReqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTPWR_REQ` reader - for debug only
pub type ExtpwrReqR = crate::BitReader;
///Field `EXTPWR_REQ` writer - for debug only
pub type ExtpwrReqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD_RAM0` reader - for debug only
pub type PdRam0R = crate::BitReader;
///Field `PD_RAM0` writer - for debug only
pub type PdRam0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD_RAM1` reader - for debug only
pub type PdRam1R = crate::BitReader;
///Field `PD_RAM1` writer - for debug only
pub type PdRam1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD_RAM2` reader - for debug only
pub type PdRam2R = crate::BitReader;
///Field `PD_RAM2` writer - for debug only
pub type PdRam2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Request hrc48 in Standby mode
    #[inline(always)]
    pub fn hrc48_req(&self) -> Hrc48ReqR {
        Hrc48ReqR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Request hxt48 in Standby mode
    #[inline(always)]
    pub fn hxt48_req(&self) -> Hxt48ReqR {
        Hxt48ReqR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Request power during Standby mode
    #[inline(always)]
    pub fn pwr_req(&self) -> PwrReqR {
        PwrReqR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - for debug only
    #[inline(always)]
    pub fn extpwr_req(&self) -> ExtpwrReqR {
        ExtpwrReqR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 6 - for debug only
    #[inline(always)]
    pub fn pd_ram0(&self) -> PdRam0R {
        PdRam0R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - for debug only
    #[inline(always)]
    pub fn pd_ram1(&self) -> PdRam1R {
        PdRam1R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - for debug only
    #[inline(always)]
    pub fn pd_ram2(&self) -> PdRam2R {
        PdRam2R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SBCR")
            .field("pd_ram2", &self.pd_ram2())
            .field("pd_ram1", &self.pd_ram1())
            .field("pd_ram0", &self.pd_ram0())
            .field("extpwr_req", &self.extpwr_req())
            .field("pwr_req", &self.pwr_req())
            .field("hxt48_req", &self.hxt48_req())
            .field("hrc48_req", &self.hrc48_req())
            .finish()
    }
}
impl W {
    ///Bit 0 - Request hrc48 in Standby mode
    #[inline(always)]
    pub fn hrc48_req(&mut self) -> Hrc48ReqW<SBCRrs> {
        Hrc48ReqW::new(self, 0)
    }
    ///Bit 1 - Request hxt48 in Standby mode
    #[inline(always)]
    pub fn hxt48_req(&mut self) -> Hxt48ReqW<SBCRrs> {
        Hxt48ReqW::new(self, 1)
    }
    ///Bit 2 - Request power during Standby mode
    #[inline(always)]
    pub fn pwr_req(&mut self) -> PwrReqW<SBCRrs> {
        PwrReqW::new(self, 2)
    }
    ///Bit 3 - for debug only
    #[inline(always)]
    pub fn extpwr_req(&mut self) -> ExtpwrReqW<SBCRrs> {
        ExtpwrReqW::new(self, 3)
    }
    ///Bit 6 - for debug only
    #[inline(always)]
    pub fn pd_ram0(&mut self) -> PdRam0W<SBCRrs> {
        PdRam0W::new(self, 6)
    }
    ///Bit 7 - for debug only
    #[inline(always)]
    pub fn pd_ram1(&mut self) -> PdRam1W<SBCRrs> {
        PdRam1W::new(self, 7)
    }
    ///Bit 8 - for debug only
    #[inline(always)]
    pub fn pd_ram2(&mut self) -> PdRam2W<SBCRrs> {
        PdRam2W::new(self, 8)
    }
}
///Standby Mode Ctrl Register
///
///You can [`read`](crate::Reg::read) this register and get [`sbcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SBCRrs;
impl crate::RegisterSpec for SBCRrs {
    type Ux = u32;
}
///`read()` method returns [`sbcr::R`](R) reader structure
impl crate::Readable for SBCRrs {}
///`write(|w| ..)` method takes [`sbcr::W`](W) writer structure
impl crate::Writable for SBCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SBCR to value 0
impl crate::Resettable for SBCRrs {
    const RESET_VALUE: u32 = 0;
}
