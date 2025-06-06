///Register `DB_DATA0` reader
pub type R = crate::R<DB_DATA0rs>;
///Register `DB_DATA0` writer
pub type W = crate::W<DB_DATA0rs>;
///Field `DB_DATA0` reader - bit\[31:24\]
///bit_depth bit\[23:16\]
///color_type bit\[15:0\]
///block number
pub type DbData0R = crate::FieldReader<u32>;
///Field `DB_DATA0` writer - bit\[31:24\]
///bit_depth bit\[23:16\]
///color_type bit\[15:0\]
///block number
pub type DbData0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - bit\[31:24\]
    ///bit_depth bit\[23:16\]
    ///color_type bit\[15:0\]
    ///block number
    #[inline(always)]
    pub fn db_data0(&self) -> DbData0R {
        DbData0R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DB_DATA0")
            .field("db_data0", &self.db_data0())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - bit\[31:24\]
    ///bit_depth bit\[23:16\]
    ///color_type bit\[15:0\]
    ///block number
    #[inline(always)]
    pub fn db_data0(&mut self) -> DbData0W<DB_DATA0rs> {
        DbData0W::new(self, 0)
    }
}
///ezip decoder debug data0
///
///You can [`read`](crate::Reg::read) this register and get [`db_data0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`db_data0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DB_DATA0rs;
impl crate::RegisterSpec for DB_DATA0rs {
    type Ux = u32;
}
///`read()` method returns [`db_data0::R`](R) reader structure
impl crate::Readable for DB_DATA0rs {}
///`write(|w| ..)` method takes [`db_data0::W`](W) writer structure
impl crate::Writable for DB_DATA0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DB_DATA0 to value 0
impl crate::Resettable for DB_DATA0rs {
    const RESET_VALUE: u32 = 0;
}
