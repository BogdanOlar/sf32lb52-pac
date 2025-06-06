///Register `DSCR` reader
pub type R = crate::R<DSCRrs>;
///Register `DSCR` writer
pub type W = crate::W<DSCRrs>;
///Field `HRC48_REQ` reader - Request hrc48 in Deep Sleep mode
pub type Hrc48ReqR = crate::BitReader;
///Field `HRC48_REQ` writer - Request hrc48 in Deep Sleep mode
pub type Hrc48ReqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HXT48_REQ` reader - Request hxt48 in Deep Sleep mode
pub type Hxt48ReqR = crate::BitReader;
///Field `HXT48_REQ` writer - Request hxt48 in Deep Sleep mode
pub type Hxt48ReqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWR_REQ` reader - Request power during Deep Sleep mode
pub type PwrReqR = crate::BitReader;
///Field `PWR_REQ` writer - Request power during Deep Sleep mode
pub type PwrReqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXTPWR_REQ` reader - for debug only
pub type ExtpwrReqR = crate::BitReader;
///Field `EXTPWR_REQ` writer - for debug only
pub type ExtpwrReqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    ///Bit 0 - Request hrc48 in Deep Sleep mode
    #[inline(always)]
    pub fn hrc48_req(&self) -> Hrc48ReqR {
        Hrc48ReqR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Request hxt48 in Deep Sleep mode
    #[inline(always)]
    pub fn hxt48_req(&self) -> Hxt48ReqR {
        Hxt48ReqR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Request power during Deep Sleep mode
    #[inline(always)]
    pub fn pwr_req(&self) -> PwrReqR {
        PwrReqR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - for debug only
    #[inline(always)]
    pub fn extpwr_req(&self) -> ExtpwrReqR {
        ExtpwrReqR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSCR")
            .field("rsvd", &self.rsvd())
            .field("extpwr_req", &self.extpwr_req())
            .field("pwr_req", &self.pwr_req())
            .field("hxt48_req", &self.hxt48_req())
            .field("hrc48_req", &self.hrc48_req())
            .finish()
    }
}
impl W {
    ///Bit 0 - Request hrc48 in Deep Sleep mode
    #[inline(always)]
    pub fn hrc48_req(&mut self) -> Hrc48ReqW<DSCRrs> {
        Hrc48ReqW::new(self, 0)
    }
    ///Bit 1 - Request hxt48 in Deep Sleep mode
    #[inline(always)]
    pub fn hxt48_req(&mut self) -> Hxt48ReqW<DSCRrs> {
        Hxt48ReqW::new(self, 1)
    }
    ///Bit 2 - Request power during Deep Sleep mode
    #[inline(always)]
    pub fn pwr_req(&mut self) -> PwrReqW<DSCRrs> {
        PwrReqW::new(self, 2)
    }
    ///Bit 3 - for debug only
    #[inline(always)]
    pub fn extpwr_req(&mut self) -> ExtpwrReqW<DSCRrs> {
        ExtpwrReqW::new(self, 3)
    }
    ///Bits 4:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<DSCRrs> {
        RsvdW::new(self, 4)
    }
}
///Deep Sleep Ctrl Register
///
///You can [`read`](crate::Reg::read) this register and get [`dscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DSCRrs;
impl crate::RegisterSpec for DSCRrs {
    type Ux = u32;
}
///`read()` method returns [`dscr::R`](R) reader structure
impl crate::Readable for DSCRrs {}
///`write(|w| ..)` method takes [`dscr::W`](W) writer structure
impl crate::Writable for DSCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DSCR to value 0
impl crate::Resettable for DSCRrs {
    const RESET_VALUE: u32 = 0;
}
