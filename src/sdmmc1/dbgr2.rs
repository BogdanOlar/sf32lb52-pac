///Register `DBGR2` reader
pub type R = crate::R<DBGR2rs>;
///Register `DBGR2` writer
pub type W = crate::W<DBGR2rs>;
///Field `HOST_WORD_COUNTER` reader - for debug only
pub type HostWordCounterR = crate::FieldReader<u16>;
///Field `HOST_WORD_COUNTER` writer - for debug only
pub type HostWordCounterW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `VALID_DATA_COU` reader - for debug only
pub type ValidDataCouR = crate::FieldReader<u16>;
///Field `VALID_DATA_COU` writer - for debug only
pub type ValidDataCouW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DBG_SEL` reader - for debug only
pub type DbgSelR = crate::FieldReader;
///Field `DBG_SEL` writer - for debug only
pub type DbgSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:13 - for debug only
    #[inline(always)]
    pub fn host_word_counter(&self) -> HostWordCounterR {
        HostWordCounterR::new((self.bits & 0x3fff) as u16)
    }
    ///Bits 14:15
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:25 - for debug only
    #[inline(always)]
    pub fn valid_data_cou(&self) -> ValidDataCouR {
        ValidDataCouR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    ///Bits 26:29
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 26) & 0x0f) as u8)
    }
    ///Bits 30:31 - for debug only
    #[inline(always)]
    pub fn dbg_sel(&self) -> DbgSelR {
        DbgSelR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGR2")
            .field("dbg_sel", &self.dbg_sel())
            .field("rsvd", &self.rsvd())
            .field("valid_data_cou", &self.valid_data_cou())
            .field("rsvd2", &self.rsvd2())
            .field("host_word_counter", &self.host_word_counter())
            .finish()
    }
}
impl W {
    ///Bits 0:13 - for debug only
    #[inline(always)]
    pub fn host_word_counter(&mut self) -> HostWordCounterW<DBGR2rs> {
        HostWordCounterW::new(self, 0)
    }
    ///Bits 14:15
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<DBGR2rs> {
        Rsvd2W::new(self, 14)
    }
    ///Bits 16:25 - for debug only
    #[inline(always)]
    pub fn valid_data_cou(&mut self) -> ValidDataCouW<DBGR2rs> {
        ValidDataCouW::new(self, 16)
    }
    ///Bits 26:29
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<DBGR2rs> {
        RsvdW::new(self, 26)
    }
    ///Bits 30:31 - for debug only
    #[inline(always)]
    pub fn dbg_sel(&mut self) -> DbgSelW<DBGR2rs> {
        DbgSelW::new(self, 30)
    }
}
///card debug port2 register
///
///You can [`read`](crate::Reg::read) this register and get [`dbgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DBGR2rs;
impl crate::RegisterSpec for DBGR2rs {
    type Ux = u32;
}
///`read()` method returns [`dbgr2::R`](R) reader structure
impl crate::Readable for DBGR2rs {}
///`write(|w| ..)` method takes [`dbgr2::W`](W) writer structure
impl crate::Writable for DBGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DBGR2 to value 0
impl crate::Resettable for DBGR2rs {
    const RESET_VALUE: u32 = 0;
}
