///Register `DB_DATA13` reader
pub type R = crate::R<DB_DATA13rs>;
///Register `DB_DATA13` writer
pub type W = crate::W<DB_DATA13rs>;
///Field `DB_DATA13` reader - frame_size_cur
pub type DbData13R = crate::FieldReader<u32>;
///Field `DB_DATA13` writer - frame_size_cur
pub type DbData13W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - frame_size_cur
    #[inline(always)]
    pub fn db_data13(&self) -> DbData13R {
        DbData13R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DB_DATA13")
            .field("db_data13", &self.db_data13())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - frame_size_cur
    #[inline(always)]
    pub fn db_data13(&mut self) -> DbData13W<DB_DATA13rs> {
        DbData13W::new(self, 0)
    }
}
///ezip decoder debug data13
///
///You can [`read`](crate::Reg::read) this register and get [`db_data13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`db_data13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DB_DATA13rs;
impl crate::RegisterSpec for DB_DATA13rs {
    type Ux = u32;
}
///`read()` method returns [`db_data13::R`](R) reader structure
impl crate::Readable for DB_DATA13rs {}
///`write(|w| ..)` method takes [`db_data13::W`](W) writer structure
impl crate::Writable for DB_DATA13rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DB_DATA13 to value 0
impl crate::Resettable for DB_DATA13rs {
    const RESET_VALUE: u32 = 0;
}
