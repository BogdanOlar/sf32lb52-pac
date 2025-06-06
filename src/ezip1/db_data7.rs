///Register `DB_DATA7` reader
pub type R = crate::R<DB_DATA7rs>;
///Register `DB_DATA7` writer
pub type W = crate::W<DB_DATA7rs>;
///Field `DB_DATA7` reader - bit\[31:16\]
///frame_width_cur bit\[15:0\]
///frame_height_cur
pub type DbData7R = crate::FieldReader<u32>;
///Field `DB_DATA7` writer - bit\[31:16\]
///frame_width_cur bit\[15:0\]
///frame_height_cur
pub type DbData7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - bit\[31:16\]
    ///frame_width_cur bit\[15:0\]
    ///frame_height_cur
    #[inline(always)]
    pub fn db_data7(&self) -> DbData7R {
        DbData7R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DB_DATA7")
            .field("db_data7", &self.db_data7())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - bit\[31:16\]
    ///frame_width_cur bit\[15:0\]
    ///frame_height_cur
    #[inline(always)]
    pub fn db_data7(&mut self) -> DbData7W<DB_DATA7rs> {
        DbData7W::new(self, 0)
    }
}
///ezip decoder debug data7
///
///You can [`read`](crate::Reg::read) this register and get [`db_data7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`db_data7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DB_DATA7rs;
impl crate::RegisterSpec for DB_DATA7rs {
    type Ux = u32;
}
///`read()` method returns [`db_data7::R`](R) reader structure
impl crate::Readable for DB_DATA7rs {}
///`write(|w| ..)` method takes [`db_data7::W`](W) writer structure
impl crate::Writable for DB_DATA7rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DB_DATA7 to value 0
impl crate::Resettable for DB_DATA7rs {
    const RESET_VALUE: u32 = 0;
}
