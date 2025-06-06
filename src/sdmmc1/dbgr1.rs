///Register `DBGR1` reader
pub type R = crate::R<DBGR1rs>;
///Register `DBGR1` writer
pub type W = crate::W<DBGR1rs>;
///Field `CMD_ST` reader - command state for debug only
pub type CmdStR = crate::FieldReader<u16>;
///Field `CMD_ST` writer - command state for debug only
pub type CmdStW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `DATA_ST` reader - data state for debug only
pub type DataStR = crate::FieldReader<u16>;
///Field `DATA_ST` writer - data state for debug only
pub type DataStW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
///Field `RSVD` reader -
pub type RsvdR = crate::BitReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - command state for debug only
    #[inline(always)]
    pub fn cmd_st(&self) -> CmdStR {
        CmdStR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:30 - data state for debug only
    #[inline(always)]
    pub fn data_st(&self) -> DataStR {
        DataStR::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    ///Bit 31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGR1")
            .field("rsvd", &self.rsvd())
            .field("data_st", &self.data_st())
            .field("cmd_st", &self.cmd_st())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - command state for debug only
    #[inline(always)]
    pub fn cmd_st(&mut self) -> CmdStW<DBGR1rs> {
        CmdStW::new(self, 0)
    }
    ///Bits 16:30 - data state for debug only
    #[inline(always)]
    pub fn data_st(&mut self) -> DataStW<DBGR1rs> {
        DataStW::new(self, 16)
    }
    ///Bit 31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<DBGR1rs> {
        RsvdW::new(self, 31)
    }
}
///card debug port1 register
///
///You can [`read`](crate::Reg::read) this register and get [`dbgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DBGR1rs;
impl crate::RegisterSpec for DBGR1rs {
    type Ux = u32;
}
///`read()` method returns [`dbgr1::R`](R) reader structure
impl crate::Readable for DBGR1rs {}
///`write(|w| ..)` method takes [`dbgr1::W`](W) writer structure
impl crate::Writable for DBGR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DBGR1 to value 0
impl crate::Resettable for DBGR1rs {
    const RESET_VALUE: u32 = 0;
}
