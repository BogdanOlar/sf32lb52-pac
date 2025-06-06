///Register `CAU_BGR` reader
pub type R = crate::R<CAU_BGRrs>;
///Register `CAU_BGR` writer
pub type W = crate::W<CAU_BGRrs>;
///Field `HPBG_VDDPSW_EN` reader -
pub type HpbgVddpswEnR = crate::BitReader;
///Field `HPBG_VDDPSW_EN` writer -
pub type HpbgVddpswEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HPBG_EN` reader -
pub type HpbgEnR = crate::BitReader;
///Field `HPBG_EN` writer -
pub type HpbgEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPBG_EN` reader -
pub type LpbgEnR = crate::BitReader;
///Field `LPBG_EN` writer -
pub type LpbgEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPBG_VREF06` reader -
pub type LpbgVref06R = crate::FieldReader;
///Field `LPBG_VREF06` writer -
pub type LpbgVref06W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `LPBG_VREF12` reader -
pub type LpbgVref12R = crate::FieldReader;
///Field `LPBG_VREF12` writer -
pub type LpbgVref12W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn hpbg_vddpsw_en(&self) -> HpbgVddpswEnR {
        HpbgVddpswEnR::new((self.bits & 1) != 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn hpbg_en(&self) -> HpbgEnR {
        HpbgEnR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2
    #[inline(always)]
    pub fn lpbg_en(&self) -> LpbgEnR {
        LpbgEnR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:6
    #[inline(always)]
    pub fn lpbg_vref06(&self) -> LpbgVref06R {
        LpbgVref06R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    ///Bits 7:10
    #[inline(always)]
    pub fn lpbg_vref12(&self) -> LpbgVref12R {
        LpbgVref12R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    ///Bits 11:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAU_BGR")
            .field("rsvd", &self.rsvd())
            .field("lpbg_vref12", &self.lpbg_vref12())
            .field("lpbg_vref06", &self.lpbg_vref06())
            .field("lpbg_en", &self.lpbg_en())
            .field("hpbg_en", &self.hpbg_en())
            .field("hpbg_vddpsw_en", &self.hpbg_vddpsw_en())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    pub fn hpbg_vddpsw_en(&mut self) -> HpbgVddpswEnW<CAU_BGRrs> {
        HpbgVddpswEnW::new(self, 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn hpbg_en(&mut self) -> HpbgEnW<CAU_BGRrs> {
        HpbgEnW::new(self, 1)
    }
    ///Bit 2
    #[inline(always)]
    pub fn lpbg_en(&mut self) -> LpbgEnW<CAU_BGRrs> {
        LpbgEnW::new(self, 2)
    }
    ///Bits 3:6
    #[inline(always)]
    pub fn lpbg_vref06(&mut self) -> LpbgVref06W<CAU_BGRrs> {
        LpbgVref06W::new(self, 3)
    }
    ///Bits 7:10
    #[inline(always)]
    pub fn lpbg_vref12(&mut self) -> LpbgVref12W<CAU_BGRrs> {
        LpbgVref12W::new(self, 7)
    }
    ///Bits 11:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<CAU_BGRrs> {
        RsvdW::new(self, 11)
    }
}
///CAU Bandgap Register
///
///You can [`read`](crate::Reg::read) this register and get [`cau_bgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cau_bgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CAU_BGRrs;
impl crate::RegisterSpec for CAU_BGRrs {
    type Ux = u32;
}
///`read()` method returns [`cau_bgr::R`](R) reader structure
impl crate::Readable for CAU_BGRrs {}
///`write(|w| ..)` method takes [`cau_bgr::W`](W) writer structure
impl crate::Writable for CAU_BGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CAU_BGR to value 0
impl crate::Resettable for CAU_BGRrs {
    const RESET_VALUE: u32 = 0;
}
