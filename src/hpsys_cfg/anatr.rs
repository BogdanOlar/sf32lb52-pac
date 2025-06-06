///Register `ANATR` reader
pub type R = crate::R<ANATRrs>;
///Register `ANATR` writer
pub type W = crate::W<ANATRrs>;
///Field `DC_TE_ATEST0` reader - reserved for debug
pub type DcTeAtest0R = crate::BitReader;
///Field `DC_TE_ATEST0` writer - reserved for debug
pub type DcTeAtest0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DC_UR_ATEST0` reader - reserved for debug
pub type DcUrAtest0R = crate::FieldReader;
///Field `DC_UR_ATEST0` writer - reserved for debug
pub type DcUrAtest0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DC_TE_ATEST1` reader - reserved for debug
pub type DcTeAtest1R = crate::BitReader;
///Field `DC_TE_ATEST1` writer - reserved for debug
pub type DcTeAtest1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DC_UR_ATEST1` reader - reserved for debug
pub type DcUrAtest1R = crate::FieldReader;
///Field `DC_UR_ATEST1` writer - reserved for debug
pub type DcUrAtest1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bit 0 - reserved for debug
    #[inline(always)]
    pub fn dc_te_atest0(&self) -> DcTeAtest0R {
        DcTeAtest0R::new((self.bits & 1) != 0)
    }
    ///Bits 1:3 - reserved for debug
    #[inline(always)]
    pub fn dc_ur_atest0(&self) -> DcUrAtest0R {
        DcUrAtest0R::new(((self.bits >> 1) & 7) as u8)
    }
    ///Bit 4 - reserved for debug
    #[inline(always)]
    pub fn dc_te_atest1(&self) -> DcTeAtest1R {
        DcTeAtest1R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:7 - reserved for debug
    #[inline(always)]
    pub fn dc_ur_atest1(&self) -> DcUrAtest1R {
        DcUrAtest1R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bits 8:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANATR")
            .field("rsvd", &self.rsvd())
            .field("dc_ur_atest1", &self.dc_ur_atest1())
            .field("dc_te_atest1", &self.dc_te_atest1())
            .field("dc_ur_atest0", &self.dc_ur_atest0())
            .field("dc_te_atest0", &self.dc_te_atest0())
            .finish()
    }
}
impl W {
    ///Bit 0 - reserved for debug
    #[inline(always)]
    pub fn dc_te_atest0(&mut self) -> DcTeAtest0W<ANATRrs> {
        DcTeAtest0W::new(self, 0)
    }
    ///Bits 1:3 - reserved for debug
    #[inline(always)]
    pub fn dc_ur_atest0(&mut self) -> DcUrAtest0W<ANATRrs> {
        DcUrAtest0W::new(self, 1)
    }
    ///Bit 4 - reserved for debug
    #[inline(always)]
    pub fn dc_te_atest1(&mut self) -> DcTeAtest1W<ANATRrs> {
        DcTeAtest1W::new(self, 4)
    }
    ///Bits 5:7 - reserved for debug
    #[inline(always)]
    pub fn dc_ur_atest1(&mut self) -> DcUrAtest1W<ANATRrs> {
        DcUrAtest1W::new(self, 5)
    }
    ///Bits 8:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<ANATRrs> {
        RsvdW::new(self, 8)
    }
}
///Analog Test Register
///
///You can [`read`](crate::Reg::read) this register and get [`anatr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anatr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ANATRrs;
impl crate::RegisterSpec for ANATRrs {
    type Ux = u32;
}
///`read()` method returns [`anatr::R`](R) reader structure
impl crate::Readable for ANATRrs {}
///`write(|w| ..)` method takes [`anatr::W`](W) writer structure
impl crate::Writable for ANATRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ANATR to value 0
impl crate::Resettable for ANATRrs {
    const RESET_VALUE: u32 = 0;
}
