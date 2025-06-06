///Register `INT_STA` reader
pub type R = crate::R<INT_STArs>;
///Register `INT_STA` writer
pub type W = crate::W<INT_STArs>;
///Field `END_INT_STA` reader - ezip_end _int_sta/aezip_frame_int_sta
pub type EndIntStaR = crate::BitReader;
///Field `END_INT_STA` writer - ezip_end _int_sta/aezip_frame_int_sta
pub type EndIntStaW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ROW_INT_STA` reader - arrive row sign sta
pub type RowIntStaR = crate::BitReader;
///Field `ROW_INT_STA` writer - arrive row sign sta
pub type RowIntStaW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ROW_ERR_STA` reader - overflow height sta
pub type RowErrStaR = crate::BitReader;
///Field `ROW_ERR_STA` writer - overflow height sta
pub type RowErrStaW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BTYPE_ERR_STA` reader - btype_err_sta
pub type BtypeErrStaR = crate::BitReader;
///Field `BTYPE_ERR_STA` writer - btype_err_sta
pub type BtypeErrStaW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETYPE_ERR_STA` reader - ezip_type_err_sta
pub type EtypeErrStaR = crate::BitReader;
///Field `ETYPE_ERR_STA` writer - ezip_type_err_sta
pub type EtypeErrStaW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AEZIP_INT_STA` reader - aezip_end_int_sta
pub type AezipIntStaR = crate::BitReader;
///Field `AEZIP_INT_STA` writer - aezip_end_int_sta
pub type AezipIntStaW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    ///Bit 0 - ezip_end _int_sta/aezip_frame_int_sta
    #[inline(always)]
    pub fn end_int_sta(&self) -> EndIntStaR {
        EndIntStaR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - arrive row sign sta
    #[inline(always)]
    pub fn row_int_sta(&self) -> RowIntStaR {
        RowIntStaR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - overflow height sta
    #[inline(always)]
    pub fn row_err_sta(&self) -> RowErrStaR {
        RowErrStaR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - btype_err_sta
    #[inline(always)]
    pub fn btype_err_sta(&self) -> BtypeErrStaR {
        BtypeErrStaR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ezip_type_err_sta
    #[inline(always)]
    pub fn etype_err_sta(&self) -> EtypeErrStaR {
        EtypeErrStaR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - aezip_end_int_sta
    #[inline(always)]
    pub fn aezip_int_sta(&self) -> AezipIntStaR {
        AezipIntStaR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_STA")
            .field("rsvd", &self.rsvd())
            .field("aezip_int_sta", &self.aezip_int_sta())
            .field("etype_err_sta", &self.etype_err_sta())
            .field("btype_err_sta", &self.btype_err_sta())
            .field("row_err_sta", &self.row_err_sta())
            .field("row_int_sta", &self.row_int_sta())
            .field("end_int_sta", &self.end_int_sta())
            .finish()
    }
}
impl W {
    ///Bit 0 - ezip_end _int_sta/aezip_frame_int_sta
    #[inline(always)]
    pub fn end_int_sta(&mut self) -> EndIntStaW<INT_STArs> {
        EndIntStaW::new(self, 0)
    }
    ///Bit 1 - arrive row sign sta
    #[inline(always)]
    pub fn row_int_sta(&mut self) -> RowIntStaW<INT_STArs> {
        RowIntStaW::new(self, 1)
    }
    ///Bit 2 - overflow height sta
    #[inline(always)]
    pub fn row_err_sta(&mut self) -> RowErrStaW<INT_STArs> {
        RowErrStaW::new(self, 2)
    }
    ///Bit 3 - btype_err_sta
    #[inline(always)]
    pub fn btype_err_sta(&mut self) -> BtypeErrStaW<INT_STArs> {
        BtypeErrStaW::new(self, 3)
    }
    ///Bit 4 - ezip_type_err_sta
    #[inline(always)]
    pub fn etype_err_sta(&mut self) -> EtypeErrStaW<INT_STArs> {
        EtypeErrStaW::new(self, 4)
    }
    ///Bit 5 - aezip_end_int_sta
    #[inline(always)]
    pub fn aezip_int_sta(&mut self) -> AezipIntStaW<INT_STArs> {
        AezipIntStaW::new(self, 5)
    }
    ///Bits 6:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<INT_STArs> {
        RsvdW::new(self, 6)
    }
}
///ezip decoder _int_sta
///
///You can [`read`](crate::Reg::read) this register and get [`int_sta::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_sta::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct INT_STArs;
impl crate::RegisterSpec for INT_STArs {
    type Ux = u32;
}
///`read()` method returns [`int_sta::R`](R) reader structure
impl crate::Readable for INT_STArs {}
///`write(|w| ..)` method takes [`int_sta::W`](W) writer structure
impl crate::Writable for INT_STArs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INT_STA to value 0
impl crate::Resettable for INT_STArs {
    const RESET_VALUE: u32 = 0;
}
