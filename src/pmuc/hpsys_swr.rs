///Register `HPSYS_SWR` reader
pub type R = crate::R<HPSYS_SWRrs>;
///Register `HPSYS_SWR` writer
pub type W = crate::W<HPSYS_SWRrs>;
///Field `PSW` reader - 0\]
///- RET_LDO; \[1\]
///- HPSYS_LDO
pub type PswR = crate::FieldReader;
///Field `PSW` writer - 0\]
///- RET_LDO; \[1\]
///- HPSYS_LDO
pub type PswW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PSW_RET` reader - PSW value during DS/SB
pub type PswRetR = crate::FieldReader;
///Field `PSW_RET` writer - PSW value during DS/SB
pub type PswRetW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DLY` reader - wait for N cycles before asserting RDY
pub type DlyR = crate::FieldReader;
///Field `DLY` writer - wait for N cycles before asserting RDY
pub type DlyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `NORET` reader - Cut off VHPMEM entirely during standby. No retention
pub type NoretR = crate::BitReader;
///Field `NORET` writer - Cut off VHPMEM entirely during standby. No retention
pub type NoretW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDY` reader -
pub type RdyR = crate::BitReader;
///Field `RDY` writer -
pub type RdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - 0\]
    ///- RET_LDO; \[1\]
    ///- HPSYS_LDO
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
    ///Bit 7 - Cut off VHPMEM entirely during standby. No retention
    #[inline(always)]
    pub fn noret(&self) -> NoretR {
        NoretR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 31
    #[inline(always)]
    pub fn rdy(&self) -> RdyR {
        RdyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPSYS_SWR")
            .field("rdy", &self.rdy())
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
    ///- HPSYS_LDO
    #[inline(always)]
    pub fn psw(&mut self) -> PswW<HPSYS_SWRrs> {
        PswW::new(self, 0)
    }
    ///Bits 2:3 - PSW value during DS/SB
    #[inline(always)]
    pub fn psw_ret(&mut self) -> PswRetW<HPSYS_SWRrs> {
        PswRetW::new(self, 2)
    }
    ///Bits 4:6 - wait for N cycles before asserting RDY
    #[inline(always)]
    pub fn dly(&mut self) -> DlyW<HPSYS_SWRrs> {
        DlyW::new(self, 4)
    }
    ///Bit 7 - Cut off VHPMEM entirely during standby. No retention
    #[inline(always)]
    pub fn noret(&mut self) -> NoretW<HPSYS_SWRrs> {
        NoretW::new(self, 7)
    }
    ///Bit 31
    #[inline(always)]
    pub fn rdy(&mut self) -> RdyW<HPSYS_SWRrs> {
        RdyW::new(self, 31)
    }
}
///HPSYS Switch Register
///
///You can [`read`](crate::Reg::read) this register and get [`hpsys_swr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpsys_swr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct HPSYS_SWRrs;
impl crate::RegisterSpec for HPSYS_SWRrs {
    type Ux = u32;
}
///`read()` method returns [`hpsys_swr::R`](R) reader structure
impl crate::Readable for HPSYS_SWRrs {}
///`write(|w| ..)` method takes [`hpsys_swr::W`](W) writer structure
impl crate::Writable for HPSYS_SWRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HPSYS_SWR to value 0
impl crate::Resettable for HPSYS_SWRrs {
    const RESET_VALUE: u32 = 0;
}
