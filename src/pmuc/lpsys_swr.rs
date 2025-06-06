///Register `LPSYS_SWR` reader
pub type R = crate::R<LPSYS_SWRrs>;
///Register `LPSYS_SWR` writer
pub type W = crate::W<LPSYS_SWRrs>;
///Field `PSW` reader - 0\]
///- RET_LDO; \[1\]
///- LPSYS_LDO
pub type PswR = crate::FieldReader;
///Field `PSW` writer - 0\]
///- RET_LDO; \[1\]
///- LPSYS_LDO
pub type PswW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PSW_RET` reader - PSW value during DS/SB
pub type PswRetR = crate::FieldReader;
///Field `PSW_RET` writer - PSW value during DS/SB
pub type PswRetW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DLY` reader - wait for N cycles before asserting RDY
pub type DlyR = crate::FieldReader;
///Field `DLY` writer - wait for N cycles before asserting RDY
pub type DlyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `NORET` reader - Cut off VLPMEM entirely during standby. No retention
pub type NoretR = crate::BitReader;
///Field `NORET` writer - Cut off VLPMEM entirely during standby. No retention
pub type NoretW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
///Field `RDY` reader -
pub type RdyR = crate::BitReader;
///Field `RDY` writer -
pub type RdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - 0\]
    ///- RET_LDO; \[1\]
    ///- LPSYS_LDO
    #[inline(always)]
    pub fn psw(&self) -> PswR {
        PswR::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - PSW value during DS/SB
    #[inline(always)]
    pub fn psw_ret(&self) -> PswRetR {
        PswRetR::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:6 - wait for N cycles before asserting RDY
    #[inline(always)]
    pub fn dly(&self) -> DlyR {
        DlyR::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Cut off VLPMEM entirely during standby. No retention
    #[inline(always)]
    pub fn noret(&self) -> NoretR {
        NoretR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:30
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 8) & 0x007f_ffff)
    }
    ///Bit 31
    #[inline(always)]
    pub fn rdy(&self) -> RdyR {
        RdyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPSYS_SWR")
            .field("rdy", &self.rdy())
            .field("rsvd", &self.rsvd())
            .field("noret", &self.noret())
            .field("dly", &self.dly())
            .field("psw_ret", &self.psw_ret())
            .field("psw", &self.psw())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - 0\]
    ///- RET_LDO; \[1\]
    ///- LPSYS_LDO
    #[inline(always)]
    pub fn psw(&mut self) -> PswW<LPSYS_SWRrs> {
        PswW::new(self, 0)
    }
    ///Bits 2:3 - PSW value during DS/SB
    #[inline(always)]
    pub fn psw_ret(&mut self) -> PswRetW<LPSYS_SWRrs> {
        PswRetW::new(self, 2)
    }
    ///Bits 4:6 - wait for N cycles before asserting RDY
    #[inline(always)]
    pub fn dly(&mut self) -> DlyW<LPSYS_SWRrs> {
        DlyW::new(self, 4)
    }
    ///Bit 7 - Cut off VLPMEM entirely during standby. No retention
    #[inline(always)]
    pub fn noret(&mut self) -> NoretW<LPSYS_SWRrs> {
        NoretW::new(self, 7)
    }
    ///Bits 8:30
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<LPSYS_SWRrs> {
        RsvdW::new(self, 8)
    }
    ///Bit 31
    #[inline(always)]
    pub fn rdy(&mut self) -> RdyW<LPSYS_SWRrs> {
        RdyW::new(self, 31)
    }
}
///LPSYS Switch Register
///
///You can [`read`](crate::Reg::read) this register and get [`lpsys_swr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpsys_swr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct LPSYS_SWRrs;
impl crate::RegisterSpec for LPSYS_SWRrs {
    type Ux = u32;
}
///`read()` method returns [`lpsys_swr::R`](R) reader structure
impl crate::Readable for LPSYS_SWRrs {}
///`write(|w| ..)` method takes [`lpsys_swr::W`](W) writer structure
impl crate::Writable for LPSYS_SWRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LPSYS_SWR to value 0
impl crate::Resettable for LPSYS_SWRrs {
    const RESET_VALUE: u32 = 0;
}
