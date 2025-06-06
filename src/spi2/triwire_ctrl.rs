///Register `TRIWIRE_CTRL` reader
pub type R = crate::R<TRIWIRE_CTRLrs>;
///Register `TRIWIRE_CTRL` writer
pub type W = crate::W<TRIWIRE_CTRLrs>;
///Field `SPI_TRI_WIRE_EN` reader - SPI_THREE_WIRE_MODE_EN 0: normal mode 1: enable TRI-WIRE mode
pub type SpiTriWireEnR = crate::BitReader;
///Field `SPI_TRI_WIRE_EN` writer - SPI_THREE_WIRE_MODE_EN 0: normal mode 1: enable TRI-WIRE mode
pub type SpiTriWireEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXD_OEN` reader - TXD_OEN control when TRI-WIRE mode 1: SPI_DIO is input 0: SPI_DIO is output
pub type TxdOenR = crate::BitReader;
///Field `TXD_OEN` writer - TXD_OEN control when TRI-WIRE mode 1: SPI_DIO is input 0: SPI_DIO is output
pub type TxdOenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WORK_WIDTH_DYN_CHANGE` reader - WORK_WIDTH_DYN_CHNAGE 1: SW can dynamicly change TOP_CTRL\[9:5\]
///without disabling TOP_CTRL\[0\]
///and re-enabling TOP_CTRL\[0\]
pub type WorkWidthDynChangeR = crate::BitReader;
///Field `WORK_WIDTH_DYN_CHANGE` writer - WORK_WIDTH_DYN_CHNAGE 1: SW can dynamicly change TOP_CTRL\[9:5\]
///without disabling TOP_CTRL\[0\]
///and re-enabling TOP_CTRL\[0\]
pub type WorkWidthDynChangeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    ///Bit 0 - SPI_THREE_WIRE_MODE_EN 0: normal mode 1: enable TRI-WIRE mode
    #[inline(always)]
    pub fn spi_tri_wire_en(&self) -> SpiTriWireEnR {
        SpiTriWireEnR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TXD_OEN control when TRI-WIRE mode 1: SPI_DIO is input 0: SPI_DIO is output
    #[inline(always)]
    pub fn txd_oen(&self) -> TxdOenR {
        TxdOenR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - WORK_WIDTH_DYN_CHNAGE 1: SW can dynamicly change TOP_CTRL\[9:5\]
    ///without disabling TOP_CTRL\[0\]
    ///and re-enabling TOP_CTRL\[0\]
    #[inline(always)]
    pub fn work_width_dyn_change(&self) -> WorkWidthDynChangeR {
        WorkWidthDynChangeR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TRIWIRE_CTRL")
            .field("rsvd", &self.rsvd())
            .field("work_width_dyn_change", &self.work_width_dyn_change())
            .field("txd_oen", &self.txd_oen())
            .field("spi_tri_wire_en", &self.spi_tri_wire_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - SPI_THREE_WIRE_MODE_EN 0: normal mode 1: enable TRI-WIRE mode
    #[inline(always)]
    pub fn spi_tri_wire_en(&mut self) -> SpiTriWireEnW<TRIWIRE_CTRLrs> {
        SpiTriWireEnW::new(self, 0)
    }
    ///Bit 1 - TXD_OEN control when TRI-WIRE mode 1: SPI_DIO is input 0: SPI_DIO is output
    #[inline(always)]
    pub fn txd_oen(&mut self) -> TxdOenW<TRIWIRE_CTRLrs> {
        TxdOenW::new(self, 1)
    }
    ///Bit 2 - WORK_WIDTH_DYN_CHNAGE 1: SW can dynamicly change TOP_CTRL\[9:5\]
    ///without disabling TOP_CTRL\[0\]
    ///and re-enabling TOP_CTRL\[0\]
    #[inline(always)]
    pub fn work_width_dyn_change(&mut self) -> WorkWidthDynChangeW<TRIWIRE_CTRLrs> {
        WorkWidthDynChangeW::new(self, 2)
    }
    ///Bits 3:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<TRIWIRE_CTRLrs> {
        RsvdW::new(self, 3)
    }
}
///Three Wire Mode Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`triwire_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`triwire_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TRIWIRE_CTRLrs;
impl crate::RegisterSpec for TRIWIRE_CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`triwire_ctrl::R`](R) reader structure
impl crate::Readable for TRIWIRE_CTRLrs {}
///`write(|w| ..)` method takes [`triwire_ctrl::W`](W) writer structure
impl crate::Writable for TRIWIRE_CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TRIWIRE_CTRL to value 0
impl crate::Resettable for TRIWIRE_CTRLrs {
    const RESET_VALUE: u32 = 0;
}
