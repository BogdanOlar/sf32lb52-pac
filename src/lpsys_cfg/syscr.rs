///Register `SYSCR` reader
pub type R = crate::R<SYSCRrs>;
///Register `SYSCR` writer
pub type W = crate::W<SYSCRrs>;
///Field `WDT2_REBOOT` reader - If set to 1, WDT2 reset will reboot the whole chip
pub type Wdt2RebootR = crate::BitReader;
///Field `WDT2_REBOOT` writer - If set to 1, WDT2 reset will reboot the whole chip
pub type Wdt2RebootW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_SWAP` reader - reserved for debug
pub type DbgSwapR = crate::FieldReader;
///Field `DBG_SWAP` writer - reserved for debug
pub type DbgSwapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LDO_VSEL` reader - select work mode 0: D 1: S
pub type LdoVselR = crate::BitReader;
///Field `LDO_VSEL` writer - select work mode 0: D 1: S
pub type LdoVselW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    ///Bit 0 - If set to 1, WDT2 reset will reboot the whole chip
    #[inline(always)]
    pub fn wdt2_reboot(&self) -> Wdt2RebootR {
        Wdt2RebootR::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - reserved for debug
    #[inline(always)]
    pub fn dbg_swap(&self) -> DbgSwapR {
        DbgSwapR::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 3 - select work mode 0: D 1: S
    #[inline(always)]
    pub fn ldo_vsel(&self) -> LdoVselR {
        LdoVselR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCR")
            .field("rsvd", &self.rsvd())
            .field("ldo_vsel", &self.ldo_vsel())
            .field("dbg_swap", &self.dbg_swap())
            .field("wdt2_reboot", &self.wdt2_reboot())
            .finish()
    }
}
impl W {
    ///Bit 0 - If set to 1, WDT2 reset will reboot the whole chip
    #[inline(always)]
    pub fn wdt2_reboot(&mut self) -> Wdt2RebootW<SYSCRrs> {
        Wdt2RebootW::new(self, 0)
    }
    ///Bits 1:2 - reserved for debug
    #[inline(always)]
    pub fn dbg_swap(&mut self) -> DbgSwapW<SYSCRrs> {
        DbgSwapW::new(self, 1)
    }
    ///Bit 3 - select work mode 0: D 1: S
    #[inline(always)]
    pub fn ldo_vsel(&mut self) -> LdoVselW<SYSCRrs> {
        LdoVselW::new(self, 3)
    }
    ///Bits 4:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<SYSCRrs> {
        RsvdW::new(self, 4)
    }
}
///System Configure Register
///
///You can [`read`](crate::Reg::read) this register and get [`syscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SYSCRrs;
impl crate::RegisterSpec for SYSCRrs {
    type Ux = u32;
}
///`read()` method returns [`syscr::R`](R) reader structure
impl crate::Readable for SYSCRrs {}
///`write(|w| ..)` method takes [`syscr::W`](W) writer structure
impl crate::Writable for SYSCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SYSCR to value 0
impl crate::Resettable for SYSCRrs {
    const RESET_VALUE: u32 = 0;
}
