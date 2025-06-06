///Register `CANVAS_STAT0` reader
pub type R = crate::R<CANVAS_STAT0rs>;
///Register `CANVAS_STAT0` writer
pub type W = crate::W<CANVAS_STAT0rs>;
///Field `X_COR` reader - canvas x cordinate
pub type XCorR = crate::FieldReader<u16>;
///Field `X_COR` writer - canvas x cordinate
pub type XCorW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `Y_COR` reader - canvas y cordinate
pub type YCorR = crate::FieldReader<u16>;
///Field `Y_COR` writer - canvas y cordinate
pub type YCorW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    ///Bits 0:10 - canvas x cordinate
    #[inline(always)]
    pub fn x_cor(&self) -> XCorR {
        XCorR::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - canvas y cordinate
    #[inline(always)]
    pub fn y_cor(&self) -> YCorR {
        YCorR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CANVAS_STAT0")
            .field("y_cor", &self.y_cor())
            .field("x_cor", &self.x_cor())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - canvas x cordinate
    #[inline(always)]
    pub fn x_cor(&mut self) -> XCorW<CANVAS_STAT0rs> {
        XCorW::new(self, 0)
    }
    ///Bits 16:26 - canvas y cordinate
    #[inline(always)]
    pub fn y_cor(&mut self) -> YCorW<CANVAS_STAT0rs> {
        YCorW::new(self, 16)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`canvas_stat0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`canvas_stat0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CANVAS_STAT0rs;
impl crate::RegisterSpec for CANVAS_STAT0rs {
    type Ux = u32;
}
///`read()` method returns [`canvas_stat0::R`](R) reader structure
impl crate::Readable for CANVAS_STAT0rs {}
///`write(|w| ..)` method takes [`canvas_stat0::W`](W) writer structure
impl crate::Writable for CANVAS_STAT0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CANVAS_STAT0 to value 0
impl crate::Resettable for CANVAS_STAT0rs {
    const RESET_VALUE: u32 = 0;
}
