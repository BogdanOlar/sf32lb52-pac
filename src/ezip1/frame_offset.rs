///Register `FRAME_OFFSET` reader
pub type R = crate::R<FRAME_OFFSETrs>;
///Register `FRAME_OFFSET` writer
pub type W = crate::W<FRAME_OFFSETrs>;
///Field `OFFEST_ROW` reader - AEZIP frame offset row
pub type OffestRowR = crate::FieldReader<u16>;
///Field `OFFEST_ROW` writer - AEZIP frame offset row
pub type OffestRowW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `OFFSET_COL` reader - AEZIP frame offset col
pub type OffsetColR = crate::FieldReader<u16>;
///Field `OFFSET_COL` writer - AEZIP frame offset col
pub type OffsetColW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - AEZIP frame offset row
    #[inline(always)]
    pub fn offest_row(&self) -> OffestRowR {
        OffestRowR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - AEZIP frame offset col
    #[inline(always)]
    pub fn offset_col(&self) -> OffsetColR {
        OffsetColR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRAME_OFFSET")
            .field("offset_col", &self.offset_col())
            .field("offest_row", &self.offest_row())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - AEZIP frame offset row
    #[inline(always)]
    pub fn offest_row(&mut self) -> OffestRowW<FRAME_OFFSETrs> {
        OffestRowW::new(self, 0)
    }
    ///Bits 16:31 - AEZIP frame offset col
    #[inline(always)]
    pub fn offset_col(&mut self) -> OffsetColW<FRAME_OFFSETrs> {
        OffsetColW::new(self, 16)
    }
}
///Aezip frame area
///
///You can [`read`](crate::Reg::read) this register and get [`frame_offset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame_offset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct FRAME_OFFSETrs;
impl crate::RegisterSpec for FRAME_OFFSETrs {
    type Ux = u32;
}
///`read()` method returns [`frame_offset::R`](R) reader structure
impl crate::Readable for FRAME_OFFSETrs {}
///`write(|w| ..)` method takes [`frame_offset::W`](W) writer structure
impl crate::Writable for FRAME_OFFSETrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FRAME_OFFSET to value 0
impl crate::Resettable for FRAME_OFFSETrs {
    const RESET_VALUE: u32 = 0;
}
