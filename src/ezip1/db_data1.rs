///Register `DB_DATA1` reader
pub type R = crate::R<DB_DATA1rs>;
///Register `DB_DATA1` writer
pub type W = crate::W<DB_DATA1rs>;
///Field `DB_DATA1` reader - bit\[31:16\]
///width bit\[15:0\]
///height
pub type DbData1R = crate::FieldReader<u32>;
///Field `DB_DATA1` writer - bit\[31:16\]
///width bit\[15:0\]
///height
pub type DbData1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - bit\[31:16\]
    ///width bit\[15:0\]
    ///height
    #[inline(always)]
    pub fn db_data1(&self) -> DbData1R {
        DbData1R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DB_DATA1")
            .field("db_data1", &self.db_data1())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - bit\[31:16\]
    ///width bit\[15:0\]
    ///height
    #[inline(always)]
    pub fn db_data1(&mut self) -> DbData1W<DB_DATA1rs> {
        DbData1W::new(self, 0)
    }
}
///ezip decoder debug data1
///
///You can [`read`](crate::Reg::read) this register and get [`db_data1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`db_data1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DB_DATA1rs;
impl crate::RegisterSpec for DB_DATA1rs {
    type Ux = u32;
}
///`read()` method returns [`db_data1::R`](R) reader structure
impl crate::Readable for DB_DATA1rs {}
///`write(|w| ..)` method takes [`db_data1::W`](W) writer structure
impl crate::Writable for DB_DATA1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DB_DATA1 to value 0
impl crate::Resettable for DB_DATA1rs {
    const RESET_VALUE: u32 = 0;
}
