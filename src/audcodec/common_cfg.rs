///Register `COMMON_CFG` reader
pub type R = crate::R<COMMON_CFGrs>;
///Register `COMMON_CFG` writer
pub type W = crate::W<COMMON_CFGrs>;
///Field `DC_TR` reader - DC test point select
pub type DcTrR = crate::FieldReader;
///Field `DC_TR` writer - DC test point select
pub type DcTrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DC_BR` reader - DC test Block select
pub type DcBrR = crate::FieldReader;
///Field `DC_BR` writer - DC test Block select
pub type DcBrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DC_MR` reader - DC test Macro select
pub type DcMrR = crate::FieldReader;
///Field `DC_MR` writer - DC test Macro select
pub type DcMrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    ///Bits 0:2 - DC test point select
    #[inline(always)]
    pub fn dc_tr(&self) -> DcTrR {
        DcTrR::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - DC test Block select
    #[inline(always)]
    pub fn dc_br(&self) -> DcBrR {
        DcBrR::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:8 - DC test Macro select
    #[inline(always)]
    pub fn dc_mr(&self) -> DcMrR {
        DcMrR::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 9:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMMON_CFG")
            .field("rsvd", &self.rsvd())
            .field("dc_mr", &self.dc_mr())
            .field("dc_br", &self.dc_br())
            .field("dc_tr", &self.dc_tr())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - DC test point select
    #[inline(always)]
    pub fn dc_tr(&mut self) -> DcTrW<COMMON_CFGrs> {
        DcTrW::new(self, 0)
    }
    ///Bits 3:5 - DC test Block select
    #[inline(always)]
    pub fn dc_br(&mut self) -> DcBrW<COMMON_CFGrs> {
        DcBrW::new(self, 3)
    }
    ///Bits 6:8 - DC test Macro select
    #[inline(always)]
    pub fn dc_mr(&mut self) -> DcMrW<COMMON_CFGrs> {
        DcMrW::new(self, 6)
    }
    ///Bits 9:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<COMMON_CFGrs> {
        RsvdW::new(self, 9)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`common_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`common_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct COMMON_CFGrs;
impl crate::RegisterSpec for COMMON_CFGrs {
    type Ux = u32;
}
///`read()` method returns [`common_cfg::R`](R) reader structure
impl crate::Readable for COMMON_CFGrs {}
///`write(|w| ..)` method takes [`common_cfg::W`](W) writer structure
impl crate::Writable for COMMON_CFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets COMMON_CFG to value 0
impl crate::Resettable for COMMON_CFGrs {
    const RESET_VALUE: u32 = 0;
}
