///Register `CANVAS_TL_POS` reader
pub type R = crate::R<CANVAS_TL_POSrs>;
///Register `CANVAS_TL_POS` writer
pub type W = crate::W<CANVAS_TL_POSrs>;
///Field `X0` reader -
pub type X0R = crate::FieldReader<u16>;
///Field `X0` writer -
pub type X0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `Y0` reader -
pub type Y0R = crate::FieldReader<u16>;
///Field `Y0` writer -
pub type Y0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9
    #[inline(always)]
    pub fn x0(&self) -> X0R {
        X0R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 16:25
    #[inline(always)]
    pub fn y0(&self) -> Y0R {
        Y0R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CANVAS_TL_POS")
            .field("y0", &self.y0())
            .field("x0", &self.x0())
            .finish()
    }
}
impl W {
    ///Bits 0:9
    #[inline(always)]
    pub fn x0(&mut self) -> X0W<CANVAS_TL_POSrs> {
        X0W::new(self, 0)
    }
    ///Bits 16:25
    #[inline(always)]
    pub fn y0(&mut self) -> Y0W<CANVAS_TL_POSrs> {
        Y0W::new(self, 16)
    }
}
///Top-Left pixel coordinate
///
///You can [`read`](crate::Reg::read) this register and get [`canvas_tl_pos::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`canvas_tl_pos::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CANVAS_TL_POSrs;
impl crate::RegisterSpec for CANVAS_TL_POSrs {
    type Ux = u32;
}
///`read()` method returns [`canvas_tl_pos::R`](R) reader structure
impl crate::Readable for CANVAS_TL_POSrs {}
///`write(|w| ..)` method takes [`canvas_tl_pos::W`](W) writer structure
impl crate::Writable for CANVAS_TL_POSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CANVAS_TL_POS to value 0
impl crate::Resettable for CANVAS_TL_POSrs {
    const RESET_VALUE: u32 = 0;
}
