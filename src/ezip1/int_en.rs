///Register `INT_EN` reader
pub type R = crate::R<INT_ENrs>;
///Register `INT_EN` writer
pub type W = crate::W<INT_ENrs>;
///Field `END_INT_EN` reader - ezip_end _int_en
pub type EndIntEnR = crate::BitReader;
///Field `END_INT_EN` writer - ezip_end _int_en
pub type EndIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ROW_INT_EN` reader - row_int_en
pub type RowIntEnR = crate::BitReader;
///Field `ROW_INT_EN` writer - row_int_en
pub type RowIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ROW_ERR_EN` reader - row_err_en
pub type RowErrEnR = crate::BitReader;
///Field `ROW_ERR_EN` writer - row_err_en
pub type RowErrEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BTYPE_ERR_EN` reader - btype_err_en
pub type BtypeErrEnR = crate::BitReader;
///Field `BTYPE_ERR_EN` writer - btype_err_en
pub type BtypeErrEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETYPE_ERR_EN` reader - ezip_type_err_en
pub type EtypeErrEnR = crate::BitReader;
///Field `ETYPE_ERR_EN` writer - ezip_type_err_en
pub type EtypeErrEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AEZIP_INT_EN` reader - aezip_frame_int_en
pub type AezipIntEnR = crate::BitReader;
///Field `AEZIP_INT_EN` writer - aezip_frame_int_en
pub type AezipIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    ///Bit 0 - ezip_end _int_en
    #[inline(always)]
    pub fn end_int_en(&self) -> EndIntEnR {
        EndIntEnR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - row_int_en
    #[inline(always)]
    pub fn row_int_en(&self) -> RowIntEnR {
        RowIntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - row_err_en
    #[inline(always)]
    pub fn row_err_en(&self) -> RowErrEnR {
        RowErrEnR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - btype_err_en
    #[inline(always)]
    pub fn btype_err_en(&self) -> BtypeErrEnR {
        BtypeErrEnR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ezip_type_err_en
    #[inline(always)]
    pub fn etype_err_en(&self) -> EtypeErrEnR {
        EtypeErrEnR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - aezip_frame_int_en
    #[inline(always)]
    pub fn aezip_int_en(&self) -> AezipIntEnR {
        AezipIntEnR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_EN")
            .field("rsvd", &self.rsvd())
            .field("aezip_int_en", &self.aezip_int_en())
            .field("etype_err_en", &self.etype_err_en())
            .field("btype_err_en", &self.btype_err_en())
            .field("row_err_en", &self.row_err_en())
            .field("row_int_en", &self.row_int_en())
            .field("end_int_en", &self.end_int_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - ezip_end _int_en
    #[inline(always)]
    pub fn end_int_en(&mut self) -> EndIntEnW<INT_ENrs> {
        EndIntEnW::new(self, 0)
    }
    ///Bit 1 - row_int_en
    #[inline(always)]
    pub fn row_int_en(&mut self) -> RowIntEnW<INT_ENrs> {
        RowIntEnW::new(self, 1)
    }
    ///Bit 2 - row_err_en
    #[inline(always)]
    pub fn row_err_en(&mut self) -> RowErrEnW<INT_ENrs> {
        RowErrEnW::new(self, 2)
    }
    ///Bit 3 - btype_err_en
    #[inline(always)]
    pub fn btype_err_en(&mut self) -> BtypeErrEnW<INT_ENrs> {
        BtypeErrEnW::new(self, 3)
    }
    ///Bit 4 - ezip_type_err_en
    #[inline(always)]
    pub fn etype_err_en(&mut self) -> EtypeErrEnW<INT_ENrs> {
        EtypeErrEnW::new(self, 4)
    }
    ///Bit 5 - aezip_frame_int_en
    #[inline(always)]
    pub fn aezip_int_en(&mut self) -> AezipIntEnW<INT_ENrs> {
        AezipIntEnW::new(self, 5)
    }
    ///Bits 6:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<INT_ENrs> {
        RsvdW::new(self, 6)
    }
}
///ezip decoder _int_en
///
///You can [`read`](crate::Reg::read) this register and get [`int_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct INT_ENrs;
impl crate::RegisterSpec for INT_ENrs {
    type Ux = u32;
}
///`read()` method returns [`int_en::R`](R) reader structure
impl crate::Readable for INT_ENrs {}
///`write(|w| ..)` method takes [`int_en::W`](W) writer structure
impl crate::Writable for INT_ENrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INT_EN to value 0
impl crate::Resettable for INT_ENrs {
    const RESET_VALUE: u32 = 0;
}
