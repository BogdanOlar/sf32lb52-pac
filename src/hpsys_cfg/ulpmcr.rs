///Register `ULPMCR` reader
pub type R = crate::R<ULPMCRrs>;
///Register `ULPMCR` writer
pub type W = crate::W<ULPMCRrs>;
///Field `RAM_RM` reader - reserved for debug
pub type RamRmR = crate::FieldReader;
///Field `RAM_RM` writer - reserved for debug
pub type RamRmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RAM_RME` reader - reserved for debug
pub type RamRmeR = crate::BitReader;
///Field `RAM_RME` writer - reserved for debug
pub type RamRmeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAM_RA` reader - reserved for debug
pub type RamRaR = crate::FieldReader;
///Field `RAM_RA` writer - reserved for debug
pub type RamRaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RAM_WA` reader - reserved for debug
pub type RamWaR = crate::FieldReader;
///Field `RAM_WA` writer - reserved for debug
pub type RamWaW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RAM_WPULSE` reader - reserved for debug
pub type RamWpulseR = crate::FieldReader;
///Field `RAM_WPULSE` writer - reserved for debug
pub type RamWpulseW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ROM_RM` reader - reserved for debug
pub type RomRmR = crate::FieldReader;
///Field `ROM_RM` writer - reserved for debug
pub type RomRmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ROM_RME` reader - reserved for debug
pub type RomRmeR = crate::BitReader;
///Field `ROM_RME` writer - reserved for debug
pub type RomRmeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ROM_DIS` reader - reserved for debug
pub type RomDisR = crate::BitReader;
///Field `ROM_DIS` writer - reserved for debug
pub type RomDisW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FORCE_ON` reader - reserved for debug
pub type ForceOnR = crate::BitReader;
///Field `FORCE_ON` writer - reserved for debug
pub type ForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - reserved for debug
    #[inline(always)]
    pub fn ram_rm(&self) -> RamRmR {
        RamRmR::new((self.bits & 3) as u8)
    }
    ///Bit 4 - reserved for debug
    #[inline(always)]
    pub fn ram_rme(&self) -> RamRmeR {
        RamRmeR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:6 - reserved for debug
    #[inline(always)]
    pub fn ram_ra(&self) -> RamRaR {
        RamRaR::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bits 7:9 - reserved for debug
    #[inline(always)]
    pub fn ram_wa(&self) -> RamWaR {
        RamWaR::new(((self.bits >> 7) & 7) as u8)
    }
    ///Bits 10:12 - reserved for debug
    #[inline(always)]
    pub fn ram_wpulse(&self) -> RamWpulseR {
        RamWpulseR::new(((self.bits >> 10) & 7) as u8)
    }
    ///Bits 16:17 - reserved for debug
    #[inline(always)]
    pub fn rom_rm(&self) -> RomRmR {
        RomRmR::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 20 - reserved for debug
    #[inline(always)]
    pub fn rom_rme(&self) -> RomRmeR {
        RomRmeR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 30 - reserved for debug
    #[inline(always)]
    pub fn rom_dis(&self) -> RomDisR {
        RomDisR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - reserved for debug
    #[inline(always)]
    pub fn force_on(&self) -> ForceOnR {
        ForceOnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ULPMCR")
            .field("force_on", &self.force_on())
            .field("rom_dis", &self.rom_dis())
            .field("rom_rme", &self.rom_rme())
            .field("rom_rm", &self.rom_rm())
            .field("ram_wpulse", &self.ram_wpulse())
            .field("ram_wa", &self.ram_wa())
            .field("ram_ra", &self.ram_ra())
            .field("ram_rme", &self.ram_rme())
            .field("ram_rm", &self.ram_rm())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - reserved for debug
    #[inline(always)]
    pub fn ram_rm(&mut self) -> RamRmW<ULPMCRrs> {
        RamRmW::new(self, 0)
    }
    ///Bit 4 - reserved for debug
    #[inline(always)]
    pub fn ram_rme(&mut self) -> RamRmeW<ULPMCRrs> {
        RamRmeW::new(self, 4)
    }
    ///Bits 5:6 - reserved for debug
    #[inline(always)]
    pub fn ram_ra(&mut self) -> RamRaW<ULPMCRrs> {
        RamRaW::new(self, 5)
    }
    ///Bits 7:9 - reserved for debug
    #[inline(always)]
    pub fn ram_wa(&mut self) -> RamWaW<ULPMCRrs> {
        RamWaW::new(self, 7)
    }
    ///Bits 10:12 - reserved for debug
    #[inline(always)]
    pub fn ram_wpulse(&mut self) -> RamWpulseW<ULPMCRrs> {
        RamWpulseW::new(self, 10)
    }
    ///Bits 16:17 - reserved for debug
    #[inline(always)]
    pub fn rom_rm(&mut self) -> RomRmW<ULPMCRrs> {
        RomRmW::new(self, 16)
    }
    ///Bit 20 - reserved for debug
    #[inline(always)]
    pub fn rom_rme(&mut self) -> RomRmeW<ULPMCRrs> {
        RomRmeW::new(self, 20)
    }
    ///Bit 30 - reserved for debug
    #[inline(always)]
    pub fn rom_dis(&mut self) -> RomDisW<ULPMCRrs> {
        RomDisW::new(self, 30)
    }
    ///Bit 31 - reserved for debug
    #[inline(always)]
    pub fn force_on(&mut self) -> ForceOnW<ULPMCRrs> {
        ForceOnW::new(self, 31)
    }
}
///ULP Memory Control register
///
///You can [`read`](crate::Reg::read) this register and get [`ulpmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ulpmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ULPMCRrs;
impl crate::RegisterSpec for ULPMCRrs {
    type Ux = u32;
}
///`read()` method returns [`ulpmcr::R`](R) reader structure
impl crate::Readable for ULPMCRrs {}
///`write(|w| ..)` method takes [`ulpmcr::W`](W) writer structure
impl crate::Writable for ULPMCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ULPMCR to value 0
impl crate::Resettable for ULPMCRrs {
    const RESET_VALUE: u32 = 0;
}
