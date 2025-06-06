///Register `ANAU_CR` reader
pub type R = crate::R<ANAU_CRrs>;
///Register `ANAU_CR` writer
pub type W = crate::W<ANAU_CRrs>;
///Field `EN_BG` reader - reserved for debug
pub type EnBgR = crate::BitReader;
///Field `EN_BG` writer - reserved for debug
pub type EnBgW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN_VBAT_MON` reader - reserved for debug
pub type EnVbatMonR = crate::BitReader;
///Field `EN_VBAT_MON` writer - reserved for debug
pub type EnVbatMonW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EFUSE_VDD_EN` reader - reserved for debug
pub type EfuseVddEnR = crate::BitReader;
///Field `EFUSE_VDD_EN` writer - reserved for debug
pub type EfuseVddEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EFUSE_VDD_PD` reader - reserved for debug
pub type EfuseVddPdR = crate::BitReader;
///Field `EFUSE_VDD_PD` writer - reserved for debug
pub type EfuseVddPdW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DC_MR` reader - reserved for debug
pub type DcMrR = crate::FieldReader;
///Field `DC_MR` writer - reserved for debug
pub type DcMrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    ///Bit 0 - reserved for debug
    #[inline(always)]
    pub fn en_bg(&self) -> EnBgR {
        EnBgR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - reserved for debug
    #[inline(always)]
    pub fn en_vbat_mon(&self) -> EnVbatMonR {
        EnVbatMonR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - reserved for debug
    #[inline(always)]
    pub fn efuse_vdd_en(&self) -> EfuseVddEnR {
        EfuseVddEnR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - reserved for debug
    #[inline(always)]
    pub fn efuse_vdd_pd(&self) -> EfuseVddPdR {
        EfuseVddPdR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - reserved for debug
    #[inline(always)]
    pub fn dc_mr(&self) -> DcMrR {
        DcMrR::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 7:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANAU_CR")
            .field("rsvd", &self.rsvd())
            .field("dc_mr", &self.dc_mr())
            .field("efuse_vdd_pd", &self.efuse_vdd_pd())
            .field("efuse_vdd_en", &self.efuse_vdd_en())
            .field("en_vbat_mon", &self.en_vbat_mon())
            .field("en_bg", &self.en_bg())
            .finish()
    }
}
impl W {
    ///Bit 0 - reserved for debug
    #[inline(always)]
    pub fn en_bg(&mut self) -> EnBgW<ANAU_CRrs> {
        EnBgW::new(self, 0)
    }
    ///Bit 1 - reserved for debug
    #[inline(always)]
    pub fn en_vbat_mon(&mut self) -> EnVbatMonW<ANAU_CRrs> {
        EnVbatMonW::new(self, 1)
    }
    ///Bit 2 - reserved for debug
    #[inline(always)]
    pub fn efuse_vdd_en(&mut self) -> EfuseVddEnW<ANAU_CRrs> {
        EfuseVddEnW::new(self, 2)
    }
    ///Bit 3 - reserved for debug
    #[inline(always)]
    pub fn efuse_vdd_pd(&mut self) -> EfuseVddPdW<ANAU_CRrs> {
        EfuseVddPdW::new(self, 3)
    }
    ///Bits 4:6 - reserved for debug
    #[inline(always)]
    pub fn dc_mr(&mut self) -> DcMrW<ANAU_CRrs> {
        DcMrW::new(self, 4)
    }
    ///Bits 7:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<ANAU_CRrs> {
        RsvdW::new(self, 7)
    }
}
///ANAU Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`anau_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anau_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ANAU_CRrs;
impl crate::RegisterSpec for ANAU_CRrs {
    type Ux = u32;
}
///`read()` method returns [`anau_cr::R`](R) reader structure
impl crate::Readable for ANAU_CRrs {}
///`write(|w| ..)` method takes [`anau_cr::W`](W) writer structure
impl crate::Writable for ANAU_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ANAU_CR to value 0
impl crate::Resettable for ANAU_CRrs {
    const RESET_VALUE: u32 = 0;
}
