///Register `MDBGR` reader
pub type R = crate::R<MDBGRrs>;
///Register `MDBGR` writer
pub type W = crate::W<MDBGRrs>;
///Field `LS_RAM0` reader - reserved for debug
pub type LsRam0R = crate::BitReader;
///Field `LS_RAM0` writer - reserved for debug
pub type LsRam0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LS_RAM1` reader - reserved for debug
pub type LsRam1R = crate::BitReader;
///Field `LS_RAM1` writer - reserved for debug
pub type LsRam1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LS_ROM` reader - reserved for debug
pub type LsRomR = crate::BitReader;
///Field `LS_ROM` writer - reserved for debug
pub type LsRomW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD_ROM` reader - reserved for debug
pub type PdRomR = crate::BitReader;
///Field `PD_ROM` writer - reserved for debug
pub type PdRomW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - reserved for debug
    #[inline(always)]
    pub fn ls_ram0(&self) -> LsRam0R {
        LsRam0R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - reserved for debug
    #[inline(always)]
    pub fn ls_ram1(&self) -> LsRam1R {
        LsRam1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - reserved for debug
    #[inline(always)]
    pub fn ls_rom(&self) -> LsRomR {
        LsRomR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - reserved for debug
    #[inline(always)]
    pub fn pd_rom(&self) -> PdRomR {
        PdRomR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDBGR")
            .field("pd_rom", &self.pd_rom())
            .field("ls_rom", &self.ls_rom())
            .field("ls_ram1", &self.ls_ram1())
            .field("ls_ram0", &self.ls_ram0())
            .finish()
    }
}
impl W {
    ///Bit 0 - reserved for debug
    #[inline(always)]
    pub fn ls_ram0(&mut self) -> LsRam0W<MDBGRrs> {
        LsRam0W::new(self, 0)
    }
    ///Bit 1 - reserved for debug
    #[inline(always)]
    pub fn ls_ram1(&mut self) -> LsRam1W<MDBGRrs> {
        LsRam1W::new(self, 1)
    }
    ///Bit 2 - reserved for debug
    #[inline(always)]
    pub fn ls_rom(&mut self) -> LsRomW<MDBGRrs> {
        LsRomW::new(self, 2)
    }
    ///Bit 3 - reserved for debug
    #[inline(always)]
    pub fn pd_rom(&mut self) -> PdRomW<MDBGRrs> {
        PdRomW::new(self, 3)
    }
}
///Memory Debug Register
///
///You can [`read`](crate::Reg::read) this register and get [`mdbgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdbgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct MDBGRrs;
impl crate::RegisterSpec for MDBGRrs {
    type Ux = u32;
}
///`read()` method returns [`mdbgr::R`](R) reader structure
impl crate::Readable for MDBGRrs {}
///`write(|w| ..)` method takes [`mdbgr::W`](W) writer structure
impl crate::Writable for MDBGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MDBGR to value 0
impl crate::Resettable for MDBGRrs {
    const RESET_VALUE: u32 = 0;
}
