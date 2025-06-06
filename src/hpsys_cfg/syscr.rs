///Register `SYSCR` reader
pub type R = crate::R<SYSCRrs>;
///Register `SYSCR` writer
pub type W = crate::W<SYSCRrs>;
///Field `WDT1_REBOOT` reader - If set to 1, WDT1 reset will reboot the whole chip
pub type Wdt1RebootR = crate::BitReader;
///Field `WDT1_REBOOT` writer - If set to 1, WDT1 reset will reboot the whole chip
pub type Wdt1RebootW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDNAND` reader - 0: MPI2 AHB space is allocated to MPI2 1: MPI2 AHB space is allocated to SDMMC1
pub type SdnandR = crate::BitReader;
///Field `SDNAND` writer - 0: MPI2 AHB space is allocated to MPI2 1: MPI2 AHB space is allocated to SDMMC1
pub type SdnandW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LDO_VSEL` reader - select work mode 0: enhanced mode 1: base mode
pub type LdoVselR = crate::BitReader;
///Field `LDO_VSEL` writer - select work mode 0: enhanced mode 1: base mode
pub type LdoVselW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    ///Bit 0 - If set to 1, WDT1 reset will reboot the whole chip
    #[inline(always)]
    pub fn wdt1_reboot(&self) -> Wdt1RebootR {
        Wdt1RebootR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - 0: MPI2 AHB space is allocated to MPI2 1: MPI2 AHB space is allocated to SDMMC1
    #[inline(always)]
    pub fn sdnand(&self) -> SdnandR {
        SdnandR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - select work mode 0: enhanced mode 1: base mode
    #[inline(always)]
    pub fn ldo_vsel(&self) -> LdoVselR {
        LdoVselR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCR")
            .field("rsvd", &self.rsvd())
            .field("ldo_vsel", &self.ldo_vsel())
            .field("sdnand", &self.sdnand())
            .field("wdt1_reboot", &self.wdt1_reboot())
            .finish()
    }
}
impl W {
    ///Bit 0 - If set to 1, WDT1 reset will reboot the whole chip
    #[inline(always)]
    pub fn wdt1_reboot(&mut self) -> Wdt1RebootW<SYSCRrs> {
        Wdt1RebootW::new(self, 0)
    }
    ///Bit 1 - 0: MPI2 AHB space is allocated to MPI2 1: MPI2 AHB space is allocated to SDMMC1
    #[inline(always)]
    pub fn sdnand(&mut self) -> SdnandW<SYSCRrs> {
        SdnandW::new(self, 1)
    }
    ///Bit 2 - select work mode 0: enhanced mode 1: base mode
    #[inline(always)]
    pub fn ldo_vsel(&mut self) -> LdoVselW<SYSCRrs> {
        LdoVselW::new(self, 2)
    }
    ///Bits 3:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<SYSCRrs> {
        RsvdW::new(self, 3)
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
