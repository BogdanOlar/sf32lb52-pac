///Register `LXT_CR` reader
pub type R = crate::R<LXT_CRrs>;
///Register `LXT_CR` writer
pub type W = crate::W<LXT_CRrs>;
///Field `EN` reader -
pub type EnR = crate::BitReader;
///Field `EN` writer -
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSN` reader -
pub type RsnR = crate::BitReader;
///Field `RSN` writer -
pub type RsnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BM` reader -
pub type BmR = crate::FieldReader;
///Field `BM` writer -
pub type BmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AMP_BM` reader -
pub type AmpBmR = crate::FieldReader;
///Field `AMP_BM` writer -
pub type AmpBmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `AMPCTRL_ENB` reader -
pub type AmpctrlEnbR = crate::BitReader;
///Field `AMPCTRL_ENB` writer -
pub type AmpctrlEnbW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BMSEL` reader -
pub type BmselR = crate::BitReader;
///Field `BMSEL` writer -
pub type BmselW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BMSTART` reader -
pub type BmstartR = crate::FieldReader;
///Field `BMSTART` writer -
pub type BmstartW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CAP_SEL` reader -
pub type CapSelR = crate::BitReader;
///Field `CAP_SEL` writer -
pub type CapSelW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EXT_EN` reader - use external 32K from Pin
pub type ExtEnR = crate::BitReader;
///Field `EXT_EN` writer - use external 32K from Pin
pub type ExtEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDY` reader -
pub type RdyR = crate::BitReader;
///Field `RDY` writer -
pub type RdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn rsn(&self) -> RsnR {
        RsnR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:5
    #[inline(always)]
    pub fn bm(&self) -> BmR {
        BmR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    ///Bits 6:7
    #[inline(always)]
    pub fn amp_bm(&self) -> AmpBmR {
        AmpBmR::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8
    #[inline(always)]
    pub fn ampctrl_enb(&self) -> AmpctrlEnbR {
        AmpctrlEnbR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9
    #[inline(always)]
    pub fn bmsel(&self) -> BmselR {
        BmselR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:13
    #[inline(always)]
    pub fn bmstart(&self) -> BmstartR {
        BmstartR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    ///Bit 14
    #[inline(always)]
    pub fn cap_sel(&self) -> CapSelR {
        CapSelR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - use external 32K from Pin
    #[inline(always)]
    pub fn ext_en(&self) -> ExtEnR {
        ExtEnR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 31
    #[inline(always)]
    pub fn rdy(&self) -> RdyR {
        RdyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LXT_CR")
            .field("rdy", &self.rdy())
            .field("ext_en", &self.ext_en())
            .field("cap_sel", &self.cap_sel())
            .field("bmstart", &self.bmstart())
            .field("bmsel", &self.bmsel())
            .field("ampctrl_enb", &self.ampctrl_enb())
            .field("amp_bm", &self.amp_bm())
            .field("bm", &self.bm())
            .field("rsn", &self.rsn())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    pub fn en(&mut self) -> EnW<LXT_CRrs> {
        EnW::new(self, 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn rsn(&mut self) -> RsnW<LXT_CRrs> {
        RsnW::new(self, 1)
    }
    ///Bits 2:5
    #[inline(always)]
    pub fn bm(&mut self) -> BmW<LXT_CRrs> {
        BmW::new(self, 2)
    }
    ///Bits 6:7
    #[inline(always)]
    pub fn amp_bm(&mut self) -> AmpBmW<LXT_CRrs> {
        AmpBmW::new(self, 6)
    }
    ///Bit 8
    #[inline(always)]
    pub fn ampctrl_enb(&mut self) -> AmpctrlEnbW<LXT_CRrs> {
        AmpctrlEnbW::new(self, 8)
    }
    ///Bit 9
    #[inline(always)]
    pub fn bmsel(&mut self) -> BmselW<LXT_CRrs> {
        BmselW::new(self, 9)
    }
    ///Bits 10:13
    #[inline(always)]
    pub fn bmstart(&mut self) -> BmstartW<LXT_CRrs> {
        BmstartW::new(self, 10)
    }
    ///Bit 14
    #[inline(always)]
    pub fn cap_sel(&mut self) -> CapSelW<LXT_CRrs> {
        CapSelW::new(self, 14)
    }
    ///Bit 15 - use external 32K from Pin
    #[inline(always)]
    pub fn ext_en(&mut self) -> ExtEnW<LXT_CRrs> {
        ExtEnW::new(self, 15)
    }
    ///Bit 31
    #[inline(always)]
    pub fn rdy(&mut self) -> RdyW<LXT_CRrs> {
        RdyW::new(self, 31)
    }
}
///XTAL32K Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`lxt_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lxt_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct LXT_CRrs;
impl crate::RegisterSpec for LXT_CRrs {
    type Ux = u32;
}
///`read()` method returns [`lxt_cr::R`](R) reader structure
impl crate::Readable for LXT_CRrs {}
///`write(|w| ..)` method takes [`lxt_cr::W`](W) writer structure
impl crate::Writable for LXT_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LXT_CR to value 0
impl crate::Resettable for LXT_CRrs {
    const RESET_VALUE: u32 = 0;
}
