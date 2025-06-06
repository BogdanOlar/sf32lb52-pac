///Register `L0_BR_POS` reader
pub type R = crate::R<L0_BR_POSrs>;
///Register `L0_BR_POS` writer
pub type W = crate::W<L0_BR_POSrs>;
///Field `X1` reader - Coordinate X-value
pub type X1R = crate::FieldReader<u16>;
///Field `X1` writer - Coordinate X-value
pub type X1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `Y1` reader - Coordingate Y-value
pub type Y1R = crate::FieldReader<u16>;
///Field `Y1` writer - Coordingate Y-value
pub type Y1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:9 - Coordinate X-value
    #[inline(always)]
    pub fn x1(&self) -> X1R {
        X1R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 10:15
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    ///Bits 16:25 - Coordingate Y-value
    #[inline(always)]
    pub fn y1(&self) -> Y1R {
        Y1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    ///Bits 26:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L0_BR_POS")
            .field("rsvd", &self.rsvd())
            .field("y1", &self.y1())
            .field("rsvd2", &self.rsvd2())
            .field("x1", &self.x1())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - Coordinate X-value
    #[inline(always)]
    pub fn x1(&mut self) -> X1W<L0_BR_POSrs> {
        X1W::new(self, 0)
    }
    ///Bits 10:15
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<L0_BR_POSrs> {
        Rsvd2W::new(self, 10)
    }
    ///Bits 16:25 - Coordingate Y-value
    #[inline(always)]
    pub fn y1(&mut self) -> Y1W<L0_BR_POSrs> {
        Y1W::new(self, 16)
    }
    ///Bits 26:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<L0_BR_POSrs> {
        RsvdW::new(self, 26)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`l0_br_pos::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l0_br_pos::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct L0_BR_POSrs;
impl crate::RegisterSpec for L0_BR_POSrs {
    type Ux = u32;
}
///`read()` method returns [`l0_br_pos::R`](R) reader structure
impl crate::Readable for L0_BR_POSrs {}
///`write(|w| ..)` method takes [`l0_br_pos::W`](W) writer structure
impl crate::Writable for L0_BR_POSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets L0_BR_POS to value 0
impl crate::Resettable for L0_BR_POSrs {
    const RESET_VALUE: u32 = 0;
}
