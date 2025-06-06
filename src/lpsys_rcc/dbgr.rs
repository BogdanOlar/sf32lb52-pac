///Register `DBGR` reader
pub type R = crate::R<DBGRrs>;
///Register `DBGR` writer
pub type W = crate::W<DBGRrs>;
///Field `SYSCLK_AON` reader - for debug only
pub type SysclkAonR = crate::BitReader;
///Field `SYSCLK_AON` writer - for debug only
pub type SysclkAonW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSCLK_SWLP` reader - for debug only
pub type SysclkSwlpR = crate::BitReader;
///Field `SYSCLK_SWLP` writer - for debug only
pub type SysclkSwlpW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FORCE_BUS` reader - for debug only
pub type ForceBusR = crate::BitReader;
///Field `FORCE_BUS` writer - for debug only
pub type ForceBusW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FORCE_MAC` reader - for debug only
pub type ForceMacR = crate::BitReader;
///Field `FORCE_MAC` writer - for debug only
pub type ForceMacW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FORCE_GPIO` reader - for debug only
pub type ForceGpioR = crate::BitReader;
///Field `FORCE_GPIO` writer - for debug only
pub type ForceGpioW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSCLK_SWBT` reader - If set to 1, clk_lpsys will: switch from clk_hrc48 to clk_hxt48 when MAC active; switch from clk_hxt48 to clk_hrc48 when MAC sleep;
pub type SysclkSwbtR = crate::BitReader;
///Field `SYSCLK_SWBT` writer - If set to 1, clk_lpsys will: switch from clk_hrc48 to clk_hxt48 when MAC active; switch from clk_hxt48 to clk_hrc48 when MAC sleep;
pub type SysclkSwbtW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    ///Bit 0 - for debug only
    #[inline(always)]
    pub fn sysclk_aon(&self) -> SysclkAonR {
        SysclkAonR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - for debug only
    #[inline(always)]
    pub fn sysclk_swlp(&self) -> SysclkSwlpR {
        SysclkSwlpR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - for debug only
    #[inline(always)]
    pub fn force_bus(&self) -> ForceBusR {
        ForceBusR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - for debug only
    #[inline(always)]
    pub fn force_mac(&self) -> ForceMacR {
        ForceMacR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - for debug only
    #[inline(always)]
    pub fn force_gpio(&self) -> ForceGpioR {
        ForceGpioR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - If set to 1, clk_lpsys will: switch from clk_hrc48 to clk_hxt48 when MAC active; switch from clk_hxt48 to clk_hrc48 when MAC sleep;
    #[inline(always)]
    pub fn sysclk_swbt(&self) -> SysclkSwbtR {
        SysclkSwbtR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGR")
            .field("rsvd", &self.rsvd())
            .field("sysclk_swbt", &self.sysclk_swbt())
            .field("force_gpio", &self.force_gpio())
            .field("force_mac", &self.force_mac())
            .field("force_bus", &self.force_bus())
            .field("sysclk_swlp", &self.sysclk_swlp())
            .field("sysclk_aon", &self.sysclk_aon())
            .finish()
    }
}
impl W {
    ///Bit 0 - for debug only
    #[inline(always)]
    pub fn sysclk_aon(&mut self) -> SysclkAonW<DBGRrs> {
        SysclkAonW::new(self, 0)
    }
    ///Bit 1 - for debug only
    #[inline(always)]
    pub fn sysclk_swlp(&mut self) -> SysclkSwlpW<DBGRrs> {
        SysclkSwlpW::new(self, 1)
    }
    ///Bit 2 - for debug only
    #[inline(always)]
    pub fn force_bus(&mut self) -> ForceBusW<DBGRrs> {
        ForceBusW::new(self, 2)
    }
    ///Bit 3 - for debug only
    #[inline(always)]
    pub fn force_mac(&mut self) -> ForceMacW<DBGRrs> {
        ForceMacW::new(self, 3)
    }
    ///Bit 4 - for debug only
    #[inline(always)]
    pub fn force_gpio(&mut self) -> ForceGpioW<DBGRrs> {
        ForceGpioW::new(self, 4)
    }
    ///Bit 5 - If set to 1, clk_lpsys will: switch from clk_hrc48 to clk_hxt48 when MAC active; switch from clk_hxt48 to clk_hrc48 when MAC sleep;
    #[inline(always)]
    pub fn sysclk_swbt(&mut self) -> SysclkSwbtW<DBGRrs> {
        SysclkSwbtW::new(self, 5)
    }
    ///Bits 6:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<DBGRrs> {
        RsvdW::new(self, 6)
    }
}
///Debug Register
///
///You can [`read`](crate::Reg::read) this register and get [`dbgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DBGRrs;
impl crate::RegisterSpec for DBGRrs {
    type Ux = u32;
}
///`read()` method returns [`dbgr::R`](R) reader structure
impl crate::Readable for DBGRrs {}
///`write(|w| ..)` method takes [`dbgr::W`](W) writer structure
impl crate::Writable for DBGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DBGR to value 0
impl crate::Resettable for DBGRrs {
    const RESET_VALUE: u32 = 0;
}
