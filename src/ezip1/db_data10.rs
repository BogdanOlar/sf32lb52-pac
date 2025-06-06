///Register `DB_DATA10` reader
pub type R = crate::R<DB_DATA10rs>;
///Register `DB_DATA10` writer
pub type W = crate::W<DB_DATA10rs>;
///Field `DB_DATA10` reader - bit\[15:8 dispos_op_cur bit\[7:0\]
///blend_op_cur
pub type DbData10R = crate::FieldReader<u32>;
///Field `DB_DATA10` writer - bit\[15:8 dispos_op_cur bit\[7:0\]
///blend_op_cur
pub type DbData10W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - bit\[15:8 dispos_op_cur bit\[7:0\]
    ///blend_op_cur
    #[inline(always)]
    pub fn db_data10(&self) -> DbData10R {
        DbData10R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DB_DATA10")
            .field("db_data10", &self.db_data10())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - bit\[15:8 dispos_op_cur bit\[7:0\]
    ///blend_op_cur
    #[inline(always)]
    pub fn db_data10(&mut self) -> DbData10W<DB_DATA10rs> {
        DbData10W::new(self, 0)
    }
}
///ezip decoder debug data10
///
///You can [`read`](crate::Reg::read) this register and get [`db_data10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`db_data10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DB_DATA10rs;
impl crate::RegisterSpec for DB_DATA10rs {
    type Ux = u32;
}
///`read()` method returns [`db_data10::R`](R) reader structure
impl crate::Readable for DB_DATA10rs {}
///`write(|w| ..)` method takes [`db_data10::W`](W) writer structure
impl crate::Writable for DB_DATA10rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DB_DATA10 to value 0
impl crate::Resettable for DB_DATA10rs {
    const RESET_VALUE: u32 = 0;
}
