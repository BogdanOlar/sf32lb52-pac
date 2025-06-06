///Register `DB_DATA2` reader
pub type R = crate::R<DB_DATA2rs>;
///Register `DB_DATA2` writer
pub type W = crate::W<DB_DATA2rs>;
///Field `DB_DATA2` reader - bit\[31:0\]
///total_len
pub type DbData2R = crate::FieldReader<u32>;
///Field `DB_DATA2` writer - bit\[31:0\]
///total_len
pub type DbData2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - bit\[31:0\]
    ///total_len
    #[inline(always)]
    pub fn db_data2(&self) -> DbData2R {
        DbData2R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DB_DATA2")
            .field("db_data2", &self.db_data2())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - bit\[31:0\]
    ///total_len
    #[inline(always)]
    pub fn db_data2(&mut self) -> DbData2W<DB_DATA2rs> {
        DbData2W::new(self, 0)
    }
}
///ezip decoder debug data2
///
///You can [`read`](crate::Reg::read) this register and get [`db_data2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`db_data2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DB_DATA2rs;
impl crate::RegisterSpec for DB_DATA2rs {
    type Ux = u32;
}
///`read()` method returns [`db_data2::R`](R) reader structure
impl crate::Readable for DB_DATA2rs {}
///`write(|w| ..)` method takes [`db_data2::W`](W) writer structure
impl crate::Writable for DB_DATA2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DB_DATA2 to value 0
impl crate::Resettable for DB_DATA2rs {
    const RESET_VALUE: u32 = 0;
}
