///Register `ROW_SIGN` reader
pub type R = crate::R<ROW_SIGNrs>;
///Register `ROW_SIGN` writer
pub type W = crate::W<ROW_SIGNrs>;
///Field `ROW_SIGN` reader - arrived row sign,ezip can generate a interrupt
pub type RowSignR = crate::FieldReader<u16>;
///Field `ROW_SIGN` writer - arrived row sign,ezip can generate a interrupt
pub type RowSignW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - arrived row sign,ezip can generate a interrupt
    #[inline(always)]
    pub fn row_sign(&self) -> RowSignR {
        RowSignR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROW_SIGN")
            .field("row_sign", &self.row_sign())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - arrived row sign,ezip can generate a interrupt
    #[inline(always)]
    pub fn row_sign(&mut self) -> RowSignW<ROW_SIGNrs> {
        RowSignW::new(self, 0)
    }
}
///ezip decoder row sign
///
///You can [`read`](crate::Reg::read) this register and get [`row_sign::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`row_sign::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ROW_SIGNrs;
impl crate::RegisterSpec for ROW_SIGNrs {
    type Ux = u32;
}
///`read()` method returns [`row_sign::R`](R) reader structure
impl crate::Readable for ROW_SIGNrs {}
///`write(|w| ..)` method takes [`row_sign::W`](W) writer structure
impl crate::Writable for ROW_SIGNrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ROW_SIGN to value 0
impl crate::Resettable for ROW_SIGNrs {
    const RESET_VALUE: u32 = 0;
}
