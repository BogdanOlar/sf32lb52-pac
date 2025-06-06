///Register `DB_DATA9` reader
pub type R = crate::R<DB_DATA9rs>;
///Register `DB_DATA9` writer
pub type W = crate::W<DB_DATA9rs>;
///Field `DB_DATA9` reader - bit\[31:16 \]delay_num_cur bit\[15:0\]
///delay_den_cur
pub type DbData9R = crate::FieldReader<u32>;
///Field `DB_DATA9` writer - bit\[31:16 \]delay_num_cur bit\[15:0\]
///delay_den_cur
pub type DbData9W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - bit\[31:16 \]delay_num_cur bit\[15:0\]
    ///delay_den_cur
    #[inline(always)]
    pub fn db_data9(&self) -> DbData9R {
        DbData9R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DB_DATA9")
            .field("db_data9", &self.db_data9())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - bit\[31:16 \]delay_num_cur bit\[15:0\]
    ///delay_den_cur
    #[inline(always)]
    pub fn db_data9(&mut self) -> DbData9W<DB_DATA9rs> {
        DbData9W::new(self, 0)
    }
}
///ezip decoder debug data9
///
///You can [`read`](crate::Reg::read) this register and get [`db_data9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`db_data9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DB_DATA9rs;
impl crate::RegisterSpec for DB_DATA9rs {
    type Ux = u32;
}
///`read()` method returns [`db_data9::R`](R) reader structure
impl crate::Readable for DB_DATA9rs {}
///`write(|w| ..)` method takes [`db_data9::W`](W) writer structure
impl crate::Writable for DB_DATA9rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DB_DATA9 to value 0
impl crate::Resettable for DB_DATA9rs {
    const RESET_VALUE: u32 = 0;
}
