///Register `DB_DATA12` reader
pub type R = crate::R<DB_DATA12rs>;
///Register `DB_DATA12` writer
pub type W = crate::W<DB_DATA12rs>;
///Field `DB_DATA12` reader - paly_cont_cur
pub type DbData12R = crate::FieldReader<u32>;
///Field `DB_DATA12` writer - paly_cont_cur
pub type DbData12W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - paly_cont_cur
    #[inline(always)]
    pub fn db_data12(&self) -> DbData12R {
        DbData12R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DB_DATA12")
            .field("db_data12", &self.db_data12())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - paly_cont_cur
    #[inline(always)]
    pub fn db_data12(&mut self) -> DbData12W<DB_DATA12rs> {
        DbData12W::new(self, 0)
    }
}
///ezip decoder debug data12
///
///You can [`read`](crate::Reg::read) this register and get [`db_data12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`db_data12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DB_DATA12rs;
impl crate::RegisterSpec for DB_DATA12rs {
    type Ux = u32;
}
///`read()` method returns [`db_data12::R`](R) reader structure
impl crate::Readable for DB_DATA12rs {}
///`write(|w| ..)` method takes [`db_data12::W`](W) writer structure
impl crate::Writable for DB_DATA12rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DB_DATA12 to value 0
impl crate::Resettable for DB_DATA12rs {
    const RESET_VALUE: u32 = 0;
}
