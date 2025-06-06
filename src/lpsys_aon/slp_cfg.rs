///Register `SLP_CFG` reader
pub type R = crate::R<SLP_CFGrs>;
///Register `SLP_CFG` writer
pub type W = crate::W<SLP_CFGrs>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `XTAL_ALWAYS_ON` reader - for debug only
pub type XtalAlwaysOnR = crate::BitReader;
///Field `XTAL_ALWAYS_ON` writer - for debug only
pub type XtalAlwaysOnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XTAL_FORCE_OFF` reader - for debug only
pub type XtalForceOffR = crate::BitReader;
///Field `XTAL_FORCE_OFF` writer - for debug only
pub type XtalForceOffW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    ///Bits 0:1
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - for debug only
    #[inline(always)]
    pub fn xtal_always_on(&self) -> XtalAlwaysOnR {
        XtalAlwaysOnR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - for debug only
    #[inline(always)]
    pub fn xtal_force_off(&self) -> XtalForceOffR {
        XtalForceOffR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLP_CFG")
            .field("rsvd", &self.rsvd())
            .field("xtal_force_off", &self.xtal_force_off())
            .field("xtal_always_on", &self.xtal_always_on())
            .field("rsvd2", &self.rsvd2())
            .finish()
    }
}
impl W {
    ///Bits 0:1
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<SLP_CFGrs> {
        Rsvd2W::new(self, 0)
    }
    ///Bit 2 - for debug only
    #[inline(always)]
    pub fn xtal_always_on(&mut self) -> XtalAlwaysOnW<SLP_CFGrs> {
        XtalAlwaysOnW::new(self, 2)
    }
    ///Bit 3 - for debug only
    #[inline(always)]
    pub fn xtal_force_off(&mut self) -> XtalForceOffW<SLP_CFGrs> {
        XtalForceOffW::new(self, 3)
    }
    ///Bits 4:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<SLP_CFGrs> {
        RsvdW::new(self, 4)
    }
}
///BT sleep configuration
///
///You can [`read`](crate::Reg::read) this register and get [`slp_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SLP_CFGrs;
impl crate::RegisterSpec for SLP_CFGrs {
    type Ux = u32;
}
///`read()` method returns [`slp_cfg::R`](R) reader structure
impl crate::Readable for SLP_CFGrs {}
///`write(|w| ..)` method takes [`slp_cfg::W`](W) writer structure
impl crate::Writable for SLP_CFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SLP_CFG to value 0
impl crate::Resettable for SLP_CFGrs {
    const RESET_VALUE: u32 = 0;
}
