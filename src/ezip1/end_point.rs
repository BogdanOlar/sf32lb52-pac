///Register `END_POINT` reader
pub type R = crate::R<END_POINTrs>;
///Register `END_POINT` writer
pub type W = crate::W<END_POINTrs>;
///Field `END_ROW` reader - ezip end row
pub type EndRowR = crate::FieldReader<u16>;
///Field `END_ROW` writer - ezip end row
pub type EndRowW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `END_COL` reader - ezip end col
pub type EndColR = crate::FieldReader<u16>;
///Field `END_COL` writer - ezip end col
pub type EndColW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - ezip end row
    #[inline(always)]
    pub fn end_row(&self) -> EndRowR {
        EndRowR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - ezip end col
    #[inline(always)]
    pub fn end_col(&self) -> EndColR {
        EndColR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("END_POINT")
            .field("end_col", &self.end_col())
            .field("end_row", &self.end_row())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - ezip end row
    #[inline(always)]
    pub fn end_row(&mut self) -> EndRowW<END_POINTrs> {
        EndRowW::new(self, 0)
    }
    ///Bits 16:31 - ezip end col
    #[inline(always)]
    pub fn end_col(&mut self) -> EndColW<END_POINTrs> {
        EndColW::new(self, 16)
    }
}
///ezip decoder end point
///
///You can [`read`](crate::Reg::read) this register and get [`end_point::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`end_point::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct END_POINTrs;
impl crate::RegisterSpec for END_POINTrs {
    type Ux = u32;
}
///`read()` method returns [`end_point::R`](R) reader structure
impl crate::Readable for END_POINTrs {}
///`write(|w| ..)` method takes [`end_point::W`](W) writer structure
impl crate::Writable for END_POINTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets END_POINT to value 0
impl crate::Resettable for END_POINTrs {
    const RESET_VALUE: u32 = 0;
}
