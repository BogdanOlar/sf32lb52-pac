///Register `CLK_CTRL` reader
pub type R = crate::R<CLK_CTRLrs>;
///Register `CLK_CTRL` writer
pub type W = crate::W<CLK_CTRLrs>;
///Field `CLK_DIV` reader - div ratio from clk_sys
pub type ClkDivR = crate::FieldReader;
///Field `CLK_DIV` writer - div ratio from clk_sys
pub type ClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `CLK_SEL` reader - 0: select clk_div as clk for SPI controller 1: select clk_sys as clk for SPI controller
pub type ClkSelR = crate::BitReader;
///Field `CLK_SEL` writer - 0: select clk_div as clk for SPI controller 1: select clk_sys as clk for SPI controller
pub type ClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_EN` reader - enable clk for internal logic
pub type ClkEnR = crate::BitReader;
///Field `CLK_EN` writer - enable clk for internal logic
pub type ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI_DI_SEL` reader - Select spi_di source. 0: from port SPI_DI. 1: from port SPI_DIO.
pub type SpiDiSelR = crate::BitReader;
///Field `SPI_DI_SEL` writer - Select spi_di source. 0: from port SPI_DI. 1: from port SPI_DIO.
pub type SpiDiSelW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    ///Bits 0:6 - div ratio from clk_sys
    #[inline(always)]
    pub fn clk_div(&self) -> ClkDivR {
        ClkDivR::new((self.bits & 0x7f) as u8)
    }
    ///Bit 7 - 0: select clk_div as clk for SPI controller 1: select clk_sys as clk for SPI controller
    #[inline(always)]
    pub fn clk_sel(&self) -> ClkSelR {
        ClkSelR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - enable clk for internal logic
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Select spi_di source. 0: from port SPI_DI. 1: from port SPI_DIO.
    #[inline(always)]
    pub fn spi_di_sel(&self) -> SpiDiSelR {
        SpiDiSelR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_CTRL")
            .field("rsvd", &self.rsvd())
            .field("spi_di_sel", &self.spi_di_sel())
            .field("clk_en", &self.clk_en())
            .field("clk_sel", &self.clk_sel())
            .field("clk_div", &self.clk_div())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - div ratio from clk_sys
    #[inline(always)]
    pub fn clk_div(&mut self) -> ClkDivW<CLK_CTRLrs> {
        ClkDivW::new(self, 0)
    }
    ///Bit 7 - 0: select clk_div as clk for SPI controller 1: select clk_sys as clk for SPI controller
    #[inline(always)]
    pub fn clk_sel(&mut self) -> ClkSelW<CLK_CTRLrs> {
        ClkSelW::new(self, 7)
    }
    ///Bit 8 - enable clk for internal logic
    #[inline(always)]
    pub fn clk_en(&mut self) -> ClkEnW<CLK_CTRLrs> {
        ClkEnW::new(self, 8)
    }
    ///Bit 9 - Select spi_di source. 0: from port SPI_DI. 1: from port SPI_DIO.
    #[inline(always)]
    pub fn spi_di_sel(&mut self) -> SpiDiSelW<CLK_CTRLrs> {
        SpiDiSelW::new(self, 9)
    }
    ///Bits 10:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<CLK_CTRLrs> {
        RsvdW::new(self, 10)
    }
}
///CLK Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`clk_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CLK_CTRLrs;
impl crate::RegisterSpec for CLK_CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`clk_ctrl::R`](R) reader structure
impl crate::Readable for CLK_CTRLrs {}
///`write(|w| ..)` method takes [`clk_ctrl::W`](W) writer structure
impl crate::Writable for CLK_CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CLK_CTRL to value 0
impl crate::Resettable for CLK_CTRLrs {
    const RESET_VALUE: u32 = 0;
}
