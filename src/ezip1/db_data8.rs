///Register `DB_DATA8` reader
pub type R = crate::R<DB_DATA8rs>;
///Register `DB_DATA8` writer
pub type W = crate::W<DB_DATA8rs>;
///Field `DB_DATA8` reader - bit\[31:16 \]frame_offsetx_cur bit\[15:0\]
///frame_offsety_cur
pub type DbData8R = crate::FieldReader<u32>;
///Field `DB_DATA8` writer - bit\[31:16 \]frame_offsetx_cur bit\[15:0\]
///frame_offsety_cur
pub type DbData8W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - bit\[31:16 \]frame_offsetx_cur bit\[15:0\]
    ///frame_offsety_cur
    #[inline(always)]
    pub fn db_data8(&self) -> DbData8R {
        DbData8R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DB_DATA8")
            .field("db_data8", &self.db_data8())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - bit\[31:16 \]frame_offsetx_cur bit\[15:0\]
    ///frame_offsety_cur
    #[inline(always)]
    pub fn db_data8(&mut self) -> DbData8W<DB_DATA8rs> {
        DbData8W::new(self, 0)
    }
}
///ezip decoder debug data8
///
///You can [`read`](crate::Reg::read) this register and get [`db_data8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`db_data8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DB_DATA8rs;
impl crate::RegisterSpec for DB_DATA8rs {
    type Ux = u32;
}
///`read()` method returns [`db_data8::R`](R) reader structure
impl crate::Readable for DB_DATA8rs {}
///`write(|w| ..)` method takes [`db_data8::W`](W) writer structure
impl crate::Writable for DB_DATA8rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DB_DATA8 to value 0
impl crate::Resettable for DB_DATA8rs {
    const RESET_VALUE: u32 = 0;
}
