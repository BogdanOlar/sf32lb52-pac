///Register `DB_DATA4` reader
pub type R = crate::R<DB_DATA4rs>;
///Register `DB_DATA4` writer
pub type W = crate::W<DB_DATA4rs>;
///Field `DB_DATA4` reader - bit\[9\]:ezip_buf_full bit\[8\]:ezip_buf_empty bit\[7\]:dec_buf_full bit\[6\]:dec_buf_empty bit\[5\]:bypass_on bit\[4\]:dec_on bit\[3\]:ind3_on bit\[2\]:ind2_on bit\[1\]:ind1_on bit\[0\]:ezip_on
pub type DbData4R = crate::FieldReader<u32>;
///Field `DB_DATA4` writer - bit\[9\]:ezip_buf_full bit\[8\]:ezip_buf_empty bit\[7\]:dec_buf_full bit\[6\]:dec_buf_empty bit\[5\]:bypass_on bit\[4\]:dec_on bit\[3\]:ind3_on bit\[2\]:ind2_on bit\[1\]:ind1_on bit\[0\]:ezip_on
pub type DbData4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - bit\[9\]:ezip_buf_full bit\[8\]:ezip_buf_empty bit\[7\]:dec_buf_full bit\[6\]:dec_buf_empty bit\[5\]:bypass_on bit\[4\]:dec_on bit\[3\]:ind3_on bit\[2\]:ind2_on bit\[1\]:ind1_on bit\[0\]:ezip_on
    #[inline(always)]
    pub fn db_data4(&self) -> DbData4R {
        DbData4R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DB_DATA4")
            .field("db_data4", &self.db_data4())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - bit\[9\]:ezip_buf_full bit\[8\]:ezip_buf_empty bit\[7\]:dec_buf_full bit\[6\]:dec_buf_empty bit\[5\]:bypass_on bit\[4\]:dec_on bit\[3\]:ind3_on bit\[2\]:ind2_on bit\[1\]:ind1_on bit\[0\]:ezip_on
    #[inline(always)]
    pub fn db_data4(&mut self) -> DbData4W<DB_DATA4rs> {
        DbData4W::new(self, 0)
    }
}
///ezip decoder debug data4
///
///You can [`read`](crate::Reg::read) this register and get [`db_data4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`db_data4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DB_DATA4rs;
impl crate::RegisterSpec for DB_DATA4rs {
    type Ux = u32;
}
///`read()` method returns [`db_data4::R`](R) reader structure
impl crate::Readable for DB_DATA4rs {}
///`write(|w| ..)` method takes [`db_data4::W`](W) writer structure
impl crate::Writable for DB_DATA4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DB_DATA4 to value 0
impl crate::Resettable for DB_DATA4rs {
    const RESET_VALUE: u32 = 0;
}
