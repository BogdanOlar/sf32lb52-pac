///Register `CAU2_CR` reader
pub type R = crate::R<CAU2_CRrs>;
///Register `CAU2_CR` writer
pub type W = crate::W<CAU2_CRrs>;
///Field `HPBG_VDDPSW_EN` reader - reserved for debug
pub type HpbgVddpswEnR = crate::BitReader;
///Field `HPBG_VDDPSW_EN` writer - reserved for debug
pub type HpbgVddpswEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HPBG_EN` reader - reserved for debug
pub type HpbgEnR = crate::BitReader;
///Field `HPBG_EN` writer - reserved for debug
pub type HpbgEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DC_TR` reader - reserved for debug
pub type DcTrR = crate::FieldReader;
///Field `DC_TR` writer - reserved for debug
pub type DcTrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DC_BR` reader - reserved for debug
pub type DcBrR = crate::FieldReader;
///Field `DC_BR` writer - reserved for debug
pub type DcBrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DC_MR` reader - reserved for debug
pub type DcMrR = crate::FieldReader;
///Field `DC_MR` writer - reserved for debug
pub type DcMrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0 - reserved for debug
    #[inline(always)]
    pub fn hpbg_vddpsw_en(&self) -> HpbgVddpswEnR {
        HpbgVddpswEnR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - reserved for debug
    #[inline(always)]
    pub fn hpbg_en(&self) -> HpbgEnR {
        HpbgEnR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 4:6 - reserved for debug
    #[inline(always)]
    pub fn dc_tr(&self) -> DcTrR {
        DcTrR::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 7:9 - reserved for debug
    #[inline(always)]
    pub fn dc_br(&self) -> DcBrR {
        DcBrR::new(((self.bits >> 7) & 7) as u8)
    }
    ///Bits 10:12 - reserved for debug
    #[inline(always)]
    pub fn dc_mr(&self) -> DcMrR {
        DcMrR::new(((self.bits >> 10) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAU2_CR")
            .field("dc_mr", &self.dc_mr())
            .field("dc_br", &self.dc_br())
            .field("dc_tr", &self.dc_tr())
            .field("hpbg_en", &self.hpbg_en())
            .field("hpbg_vddpsw_en", &self.hpbg_vddpsw_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - reserved for debug
    #[inline(always)]
    pub fn hpbg_vddpsw_en(&mut self) -> HpbgVddpswEnW<CAU2_CRrs> {
        HpbgVddpswEnW::new(self, 0)
    }
    ///Bit 1 - reserved for debug
    #[inline(always)]
    pub fn hpbg_en(&mut self) -> HpbgEnW<CAU2_CRrs> {
        HpbgEnW::new(self, 1)
    }
    ///Bits 4:6 - reserved for debug
    #[inline(always)]
    pub fn dc_tr(&mut self) -> DcTrW<CAU2_CRrs> {
        DcTrW::new(self, 4)
    }
    ///Bits 7:9 - reserved for debug
    #[inline(always)]
    pub fn dc_br(&mut self) -> DcBrW<CAU2_CRrs> {
        DcBrW::new(self, 7)
    }
    ///Bits 10:12 - reserved for debug
    #[inline(always)]
    pub fn dc_mr(&mut self) -> DcMrW<CAU2_CRrs> {
        DcMrW::new(self, 10)
    }
}
///CAU2 Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`cau2_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cau2_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CAU2_CRrs;
impl crate::RegisterSpec for CAU2_CRrs {
    type Ux = u32;
}
///`read()` method returns [`cau2_cr::R`](R) reader structure
impl crate::Readable for CAU2_CRrs {}
///`write(|w| ..)` method takes [`cau2_cr::W`](W) writer structure
impl crate::Writable for CAU2_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CAU2_CR to value 0
impl crate::Resettable for CAU2_CRrs {
    const RESET_VALUE: u32 = 0;
}
