///Register `DPI_CTRL` reader
pub type R = crate::R<DPI_CTRLrs>;
///Register `DPI_CTRL` writer
pub type W = crate::W<DPI_CTRLrs>;
///Field `DPI_EN` reader - dpi interface enable
pub type DpiEnR = crate::BitReader;
///Field `DPI_EN` writer - dpi interface enable
pub type DpiEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DPI_CM` reader - dpi color mode
pub type DpiCmR = crate::BitReader;
///Field `DPI_CM` writer - dpi color mode
pub type DpiCmW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DPI_SD` reader - dpi shutdown
pub type DpiSdR = crate::BitReader;
///Field `DPI_SD` writer - dpi shutdown
pub type DpiSdW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DPI_UC` reader - dpi update config
pub type DpiUcR = crate::BitReader;
///Field `DPI_UC` writer - dpi update config
pub type DpiUcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - dpi interface enable
    #[inline(always)]
    pub fn dpi_en(&self) -> DpiEnR {
        DpiEnR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - dpi color mode
    #[inline(always)]
    pub fn dpi_cm(&self) -> DpiCmR {
        DpiCmR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - dpi shutdown
    #[inline(always)]
    pub fn dpi_sd(&self) -> DpiSdR {
        DpiSdR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - dpi update config
    #[inline(always)]
    pub fn dpi_uc(&self) -> DpiUcR {
        DpiUcR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPI_CTRL")
            .field("dpi_uc", &self.dpi_uc())
            .field("dpi_sd", &self.dpi_sd())
            .field("dpi_cm", &self.dpi_cm())
            .field("dpi_en", &self.dpi_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - dpi interface enable
    #[inline(always)]
    pub fn dpi_en(&mut self) -> DpiEnW<DPI_CTRLrs> {
        DpiEnW::new(self, 0)
    }
    ///Bit 1 - dpi color mode
    #[inline(always)]
    pub fn dpi_cm(&mut self) -> DpiCmW<DPI_CTRLrs> {
        DpiCmW::new(self, 1)
    }
    ///Bit 2 - dpi shutdown
    #[inline(always)]
    pub fn dpi_sd(&mut self) -> DpiSdW<DPI_CTRLrs> {
        DpiSdW::new(self, 2)
    }
    ///Bit 3 - dpi update config
    #[inline(always)]
    pub fn dpi_uc(&mut self) -> DpiUcW<DPI_CTRLrs> {
        DpiUcW::new(self, 3)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`dpi_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DPI_CTRLrs;
impl crate::RegisterSpec for DPI_CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`dpi_ctrl::R`](R) reader structure
impl crate::Readable for DPI_CTRLrs {}
///`write(|w| ..)` method takes [`dpi_ctrl::W`](W) writer structure
impl crate::Writable for DPI_CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DPI_CTRL to value 0
impl crate::Resettable for DPI_CTRLrs {
    const RESET_VALUE: u32 = 0;
}
