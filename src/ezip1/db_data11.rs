///Register `DB_DATA11` reader
pub type R = crate::R<DB_DATA11rs>;
///Register `DB_DATA11` writer
pub type W = crate::W<DB_DATA11rs>;
///Field `DB_DATA11` reader - frame_cont_cur
pub type DbData11R = crate::FieldReader<u32>;
///Field `DB_DATA11` writer - frame_cont_cur
pub type DbData11W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - frame_cont_cur
    #[inline(always)]
    pub fn db_data11(&self) -> DbData11R {
        DbData11R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DB_DATA11")
            .field("db_data11", &self.db_data11())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - frame_cont_cur
    #[inline(always)]
    pub fn db_data11(&mut self) -> DbData11W<DB_DATA11rs> {
        DbData11W::new(self, 0)
    }
}
///ezip decoder debug data11
///
///You can [`read`](crate::Reg::read) this register and get [`db_data11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`db_data11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DB_DATA11rs;
impl crate::RegisterSpec for DB_DATA11rs {
    type Ux = u32;
}
///`read()` method returns [`db_data11::R`](R) reader structure
impl crate::Readable for DB_DATA11rs {}
///`write(|w| ..)` method takes [`db_data11::W`](W) writer structure
impl crate::Writable for DB_DATA11rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DB_DATA11 to value 0
impl crate::Resettable for DB_DATA11rs {
    const RESET_VALUE: u32 = 0;
}
