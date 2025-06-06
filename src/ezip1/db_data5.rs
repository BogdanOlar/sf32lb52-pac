///Register `DB_DATA5` reader
pub type R = crate::R<DB_DATA5rs>;
///Register `DB_DATA5` writer
pub type W = crate::W<DB_DATA5rs>;
///Field `DB_DATA5` reader - bit\[31:16\]
///width_cnt_cur bit\[15:0\]
///height_cnt_cur
pub type DbData5R = crate::FieldReader<u32>;
///Field `DB_DATA5` writer - bit\[31:16\]
///width_cnt_cur bit\[15:0\]
///height_cnt_cur
pub type DbData5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - bit\[31:16\]
    ///width_cnt_cur bit\[15:0\]
    ///height_cnt_cur
    #[inline(always)]
    pub fn db_data5(&self) -> DbData5R {
        DbData5R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DB_DATA5")
            .field("db_data5", &self.db_data5())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - bit\[31:16\]
    ///width_cnt_cur bit\[15:0\]
    ///height_cnt_cur
    #[inline(always)]
    pub fn db_data5(&mut self) -> DbData5W<DB_DATA5rs> {
        DbData5W::new(self, 0)
    }
}
///ezip decoder debug data5
///
///You can [`read`](crate::Reg::read) this register and get [`db_data5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`db_data5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DB_DATA5rs;
impl crate::RegisterSpec for DB_DATA5rs {
    type Ux = u32;
}
///`read()` method returns [`db_data5::R`](R) reader structure
impl crate::Readable for DB_DATA5rs {}
///`write(|w| ..)` method takes [`db_data5::W`](W) writer structure
impl crate::Writable for DB_DATA5rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DB_DATA5 to value 0
impl crate::Resettable for DB_DATA5rs {
    const RESET_VALUE: u32 = 0;
}
