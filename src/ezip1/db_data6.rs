///Register `DB_DATA6` reader
pub type R = crate::R<DB_DATA6rs>;
///Register `DB_DATA6` writer
pub type W = crate::W<DB_DATA6rs>;
///Field `DB_DATA6` reader - seq_num
pub type DbData6R = crate::FieldReader<u32>;
///Field `DB_DATA6` writer - seq_num
pub type DbData6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - seq_num
    #[inline(always)]
    pub fn db_data6(&self) -> DbData6R {
        DbData6R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DB_DATA6")
            .field("db_data6", &self.db_data6())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - seq_num
    #[inline(always)]
    pub fn db_data6(&mut self) -> DbData6W<DB_DATA6rs> {
        DbData6W::new(self, 0)
    }
}
///ezip decoder debug data6
///
///You can [`read`](crate::Reg::read) this register and get [`db_data6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`db_data6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DB_DATA6rs;
impl crate::RegisterSpec for DB_DATA6rs {
    type Ux = u32;
}
///`read()` method returns [`db_data6::R`](R) reader structure
impl crate::Readable for DB_DATA6rs {}
///`write(|w| ..)` method takes [`db_data6::W`](W) writer structure
impl crate::Writable for DB_DATA6rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DB_DATA6 to value 0
impl crate::Resettable for DB_DATA6rs {
    const RESET_VALUE: u32 = 0;
}
