///Register `INT_MASK` reader
pub type R = crate::R<INT_MASKrs>;
///Register `INT_MASK` writer
pub type W = crate::W<INT_MASKrs>;
///Field `END_INT_MASK` reader - ezip_end _int mask sta/aezip_frame_int_mask_Sta
pub type EndIntMaskR = crate::BitReader;
///Field `END_INT_MASK` writer - ezip_end _int mask sta/aezip_frame_int_mask_Sta
pub type EndIntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ROW_INT_MASK` reader - arrive row sign mask sta
pub type RowIntMaskR = crate::BitReader;
///Field `ROW_INT_MASK` writer - arrive row sign mask sta
pub type RowIntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ROW_ERR_MASK` reader - overflow height mask sta
pub type RowErrMaskR = crate::BitReader;
///Field `ROW_ERR_MASK` writer - overflow height mask sta
pub type RowErrMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BTYPE_ERR_MASK` reader - btype_err_mask sta
pub type BtypeErrMaskR = crate::BitReader;
///Field `BTYPE_ERR_MASK` writer - btype_err_mask sta
pub type BtypeErrMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETYPE_ERR_MASK` reader - ezip_type_err_mask sta
pub type EtypeErrMaskR = crate::BitReader;
///Field `ETYPE_ERR_MASK` writer - ezip_type_err_mask sta
pub type EtypeErrMaskW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AEZIP_INT_MASK` reader - aezip_end_int_mask sta
pub type AezipIntMaskR = crate::BitReader;
///Field `AEZIP_INT_MASK` writer - aezip_end_int_mask sta
pub type AezipIntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ezip_end _int mask sta/aezip_frame_int_mask_Sta
    #[inline(always)]
    pub fn end_int_mask(&self) -> EndIntMaskR {
        EndIntMaskR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - arrive row sign mask sta
    #[inline(always)]
    pub fn row_int_mask(&self) -> RowIntMaskR {
        RowIntMaskR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - overflow height mask sta
    #[inline(always)]
    pub fn row_err_mask(&self) -> RowErrMaskR {
        RowErrMaskR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - btype_err_mask sta
    #[inline(always)]
    pub fn btype_err_mask(&self) -> BtypeErrMaskR {
        BtypeErrMaskR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ezip_type_err_mask sta
    #[inline(always)]
    pub fn etype_err_mask(&self) -> EtypeErrMaskR {
        EtypeErrMaskR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - aezip_end_int_mask sta
    #[inline(always)]
    pub fn aezip_int_mask(&self) -> AezipIntMaskR {
        AezipIntMaskR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_MASK")
            .field("aezip_int_mask", &self.aezip_int_mask())
            .field("etype_err_mask", &self.etype_err_mask())
            .field("btype_err_mask", &self.btype_err_mask())
            .field("row_err_mask", &self.row_err_mask())
            .field("row_int_mask", &self.row_int_mask())
            .field("end_int_mask", &self.end_int_mask())
            .finish()
    }
}
impl W {
    ///Bit 0 - ezip_end _int mask sta/aezip_frame_int_mask_Sta
    #[inline(always)]
    pub fn end_int_mask(&mut self) -> EndIntMaskW<INT_MASKrs> {
        EndIntMaskW::new(self, 0)
    }
    ///Bit 1 - arrive row sign mask sta
    #[inline(always)]
    pub fn row_int_mask(&mut self) -> RowIntMaskW<INT_MASKrs> {
        RowIntMaskW::new(self, 1)
    }
    ///Bit 2 - overflow height mask sta
    #[inline(always)]
    pub fn row_err_mask(&mut self) -> RowErrMaskW<INT_MASKrs> {
        RowErrMaskW::new(self, 2)
    }
    ///Bit 3 - btype_err_mask sta
    #[inline(always)]
    pub fn btype_err_mask(&mut self) -> BtypeErrMaskW<INT_MASKrs> {
        BtypeErrMaskW::new(self, 3)
    }
    ///Bit 4 - ezip_type_err_mask sta
    #[inline(always)]
    pub fn etype_err_mask(&mut self) -> EtypeErrMaskW<INT_MASKrs> {
        EtypeErrMaskW::new(self, 4)
    }
    ///Bit 5 - aezip_end_int_mask sta
    #[inline(always)]
    pub fn aezip_int_mask(&mut self) -> AezipIntMaskW<INT_MASKrs> {
        AezipIntMaskW::new(self, 5)
    }
}
///ezip decoder int mask state
///
///You can [`read`](crate::Reg::read) this register and get [`int_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct INT_MASKrs;
impl crate::RegisterSpec for INT_MASKrs {
    type Ux = u32;
}
///`read()` method returns [`int_mask::R`](R) reader structure
impl crate::Readable for INT_MASKrs {}
///`write(|w| ..)` method takes [`int_mask::W`](W) writer structure
impl crate::Writable for INT_MASKrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INT_MASK to value 0
impl crate::Resettable for INT_MASKrs {
    const RESET_VALUE: u32 = 0;
}
