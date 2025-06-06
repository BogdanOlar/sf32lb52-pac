///Register `L1_TL_POS` reader
pub type R = crate::R<L1_TL_POSrs>;
///Register `L1_TL_POS` writer
pub type W = crate::W<L1_TL_POSrs>;
///Field `X0` reader - Coordinate X-value
pub type X0R = crate::FieldReader<u16>;
///Field `X0` writer - Coordinate X-value
pub type X0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `Y0` reader - Coordingate Y-value
pub type Y0R = crate::FieldReader<u16>;
///Field `Y0` writer - Coordingate Y-value
pub type Y0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:9 - Coordinate X-value
    #[inline(always)]
    pub fn x0(&self) -> X0R {
        X0R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 10:15
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    ///Bits 16:25 - Coordingate Y-value
    #[inline(always)]
    pub fn y0(&self) -> Y0R {
        Y0R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    ///Bits 26:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_TL_POS")
            .field("rsvd", &self.rsvd())
            .field("y0", &self.y0())
            .field("rsvd2", &self.rsvd2())
            .field("x0", &self.x0())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - Coordinate X-value
    #[inline(always)]
    pub fn x0(&mut self) -> X0W<L1_TL_POSrs> {
        X0W::new(self, 0)
    }
    ///Bits 10:15
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<L1_TL_POSrs> {
        Rsvd2W::new(self, 10)
    }
    ///Bits 16:25 - Coordingate Y-value
    #[inline(always)]
    pub fn y0(&mut self) -> Y0W<L1_TL_POSrs> {
        Y0W::new(self, 16)
    }
    ///Bits 26:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<L1_TL_POSrs> {
        RsvdW::new(self, 26)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`l1_tl_pos::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_tl_pos::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct L1_TL_POSrs;
impl crate::RegisterSpec for L1_TL_POSrs {
    type Ux = u32;
}
///`read()` method returns [`l1_tl_pos::R`](R) reader structure
impl crate::Readable for L1_TL_POSrs {}
///`write(|w| ..)` method takes [`l1_tl_pos::W`](W) writer structure
impl crate::Writable for L1_TL_POSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets L1_TL_POS to value 0
impl crate::Resettable for L1_TL_POSrs {
    const RESET_VALUE: u32 = 0;
}
