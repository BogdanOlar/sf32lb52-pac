///Register `DB_SEL` reader
pub type R = crate::R<DB_SELrs>;
///Register `DB_SEL` writer
pub type W = crate::W<DB_SELrs>;
///Field `DB_SEL` reader - debug signal select
pub type DbSelR = crate::FieldReader<u32>;
///Field `DB_SEL` writer - debug signal select
pub type DbSelW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - debug signal select
    #[inline(always)]
    pub fn db_sel(&self) -> DbSelR {
        DbSelR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DB_SEL")
            .field("db_sel", &self.db_sel())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - debug signal select
    #[inline(always)]
    pub fn db_sel(&mut self) -> DbSelW<DB_SELrs> {
        DbSelW::new(self, 0)
    }
}
///debug signal select
///
///You can [`read`](crate::Reg::read) this register and get [`db_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`db_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DB_SELrs;
impl crate::RegisterSpec for DB_SELrs {
    type Ux = u32;
}
///`read()` method returns [`db_sel::R`](R) reader structure
impl crate::Readable for DB_SELrs {}
///`write(|w| ..)` method takes [`db_sel::W`](W) writer structure
impl crate::Writable for DB_SELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DB_SEL to value 0
impl crate::Resettable for DB_SELrs {
    const RESET_VALUE: u32 = 0;
}
