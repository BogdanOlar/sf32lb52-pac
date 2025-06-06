///Register `LAYER0_BR_POS` reader
pub type R = crate::R<LAYER0_BR_POSrs>;
///Register `LAYER0_BR_POS` writer
pub type W = crate::W<LAYER0_BR_POSrs>;
///Field `X1` reader - Coordinate X-value
pub type X1R = crate::FieldReader<u16>;
///Field `X1` writer - Coordinate X-value
pub type X1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `Y1` reader - Coordingate Y-value
pub type Y1R = crate::FieldReader<u16>;
///Field `Y1` writer - Coordingate Y-value
pub type Y1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    ///Bits 0:10 - Coordinate X-value
    #[inline(always)]
    pub fn x1(&self) -> X1R {
        X1R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - Coordingate Y-value
    #[inline(always)]
    pub fn y1(&self) -> Y1R {
        Y1R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LAYER0_BR_POS")
            .field("y1", &self.y1())
            .field("x1", &self.x1())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - Coordinate X-value
    #[inline(always)]
    pub fn x1(&mut self) -> X1W<LAYER0_BR_POSrs> {
        X1W::new(self, 0)
    }
    ///Bits 16:26 - Coordingate Y-value
    #[inline(always)]
    pub fn y1(&mut self) -> Y1W<LAYER0_BR_POSrs> {
        Y1W::new(self, 16)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`layer0_br_pos::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer0_br_pos::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct LAYER0_BR_POSrs;
impl crate::RegisterSpec for LAYER0_BR_POSrs {
    type Ux = u32;
}
///`read()` method returns [`layer0_br_pos::R`](R) reader structure
impl crate::Readable for LAYER0_BR_POSrs {}
///`write(|w| ..)` method takes [`layer0_br_pos::W`](W) writer structure
impl crate::Writable for LAYER0_BR_POSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LAYER0_BR_POS to value 0
impl crate::Resettable for LAYER0_BR_POSrs {
    const RESET_VALUE: u32 = 0;
}
