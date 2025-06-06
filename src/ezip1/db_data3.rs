///Register `DB_DATA3` reader
pub type R = crate::R<DB_DATA3rs>;
///Register `DB_DATA3` writer
pub type W = crate::W<DB_DATA3rs>;
///Field `DB_DATA3` reader - bit\[31:24\]
///ezip_type bit\[23:20\]
///bfinal bit\[19:16\]
///btype bit\[11:8\]
///ahb_state bit\[7:4\]
///ctrl_state bir\[3:0\]
///build_stste
pub type DbData3R = crate::FieldReader<u32>;
///Field `DB_DATA3` writer - bit\[31:24\]
///ezip_type bit\[23:20\]
///bfinal bit\[19:16\]
///btype bit\[11:8\]
///ahb_state bit\[7:4\]
///ctrl_state bir\[3:0\]
///build_stste
pub type DbData3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - bit\[31:24\]
    ///ezip_type bit\[23:20\]
    ///bfinal bit\[19:16\]
    ///btype bit\[11:8\]
    ///ahb_state bit\[7:4\]
    ///ctrl_state bir\[3:0\]
    ///build_stste
    #[inline(always)]
    pub fn db_data3(&self) -> DbData3R {
        DbData3R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DB_DATA3")
            .field("db_data3", &self.db_data3())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - bit\[31:24\]
    ///ezip_type bit\[23:20\]
    ///bfinal bit\[19:16\]
    ///btype bit\[11:8\]
    ///ahb_state bit\[7:4\]
    ///ctrl_state bir\[3:0\]
    ///build_stste
    #[inline(always)]
    pub fn db_data3(&mut self) -> DbData3W<DB_DATA3rs> {
        DbData3W::new(self, 0)
    }
}
///ezip decoder debug data3
///
///You can [`read`](crate::Reg::read) this register and get [`db_data3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`db_data3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DB_DATA3rs;
impl crate::RegisterSpec for DB_DATA3rs {
    type Ux = u32;
}
///`read()` method returns [`db_data3::R`](R) reader structure
impl crate::Readable for DB_DATA3rs {}
///`write(|w| ..)` method takes [`db_data3::W`](W) writer structure
impl crate::Writable for DB_DATA3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DB_DATA3 to value 0
impl crate::Resettable for DB_DATA3rs {
    const RESET_VALUE: u32 = 0;
}
