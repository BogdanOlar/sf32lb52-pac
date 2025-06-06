///Register `DPI_IF_CONF5` reader
pub type R = crate::R<DPI_IF_CONF5rs>;
///Register `DPI_IF_CONF5` writer
pub type W = crate::W<DPI_IF_CONF5rs>;
///Field `PCLK_DIV` reader - pixel clock divider
pub type PclkDivR = crate::FieldReader;
///Field `PCLK_DIV` writer - pixel clock divider
pub type PclkDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `PCLKPOL` reader - pixel clock polarity
pub type PclkpolR = crate::BitReader;
///Field `PCLKPOL` writer - pixel clock polarity
pub type PclkpolW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEPOL` reader - de polarity
pub type DepolR = crate::BitReader;
///Field `DEPOL` writer - de polarity
pub type DepolW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VSPOL` reader - vsync polarity
pub type VspolR = crate::BitReader;
///Field `VSPOL` writer - vsync polarity
pub type VspolW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSPOL` reader - hsync polarity
pub type HspolR = crate::BitReader;
///Field `HSPOL` writer - hsync polarity
pub type HspolW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INT_LINE_NUM` reader - DPI interrupt line number
pub type IntLineNumR = crate::FieldReader<u16>;
///Field `INT_LINE_NUM` writer - DPI interrupt line number
pub type IntLineNumW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `CLK_FORCE_ON` reader - 1: force DPI clock on 0: DPI clock is controlled by hardware
pub type ClkForceOnR = crate::BitReader;
///Field `CLK_FORCE_ON` writer - 1: force DPI clock on 0: DPI clock is controlled by hardware
pub type ClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - pixel clock divider
    #[inline(always)]
    pub fn pclk_div(&self) -> PclkDivR {
        PclkDivR::new((self.bits & 0xff) as u8)
    }
    ///Bit 8 - pixel clock polarity
    #[inline(always)]
    pub fn pclkpol(&self) -> PclkpolR {
        PclkpolR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - de polarity
    #[inline(always)]
    pub fn depol(&self) -> DepolR {
        DepolR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - vsync polarity
    #[inline(always)]
    pub fn vspol(&self) -> VspolR {
        VspolR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - hsync polarity
    #[inline(always)]
    pub fn hspol(&self) -> HspolR {
        HspolR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:22 - DPI interrupt line number
    #[inline(always)]
    pub fn int_line_num(&self) -> IntLineNumR {
        IntLineNumR::new(((self.bits >> 12) & 0x07ff) as u16)
    }
    ///Bit 23 - 1: force DPI clock on 0: DPI clock is controlled by hardware
    #[inline(always)]
    pub fn clk_force_on(&self) -> ClkForceOnR {
        ClkForceOnR::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPI_IF_CONF5")
            .field("rsvd", &self.rsvd())
            .field("clk_force_on", &self.clk_force_on())
            .field("int_line_num", &self.int_line_num())
            .field("hspol", &self.hspol())
            .field("vspol", &self.vspol())
            .field("depol", &self.depol())
            .field("pclkpol", &self.pclkpol())
            .field("pclk_div", &self.pclk_div())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - pixel clock divider
    #[inline(always)]
    pub fn pclk_div(&mut self) -> PclkDivW<DPI_IF_CONF5rs> {
        PclkDivW::new(self, 0)
    }
    ///Bit 8 - pixel clock polarity
    #[inline(always)]
    pub fn pclkpol(&mut self) -> PclkpolW<DPI_IF_CONF5rs> {
        PclkpolW::new(self, 8)
    }
    ///Bit 9 - de polarity
    #[inline(always)]
    pub fn depol(&mut self) -> DepolW<DPI_IF_CONF5rs> {
        DepolW::new(self, 9)
    }
    ///Bit 10 - vsync polarity
    #[inline(always)]
    pub fn vspol(&mut self) -> VspolW<DPI_IF_CONF5rs> {
        VspolW::new(self, 10)
    }
    ///Bit 11 - hsync polarity
    #[inline(always)]
    pub fn hspol(&mut self) -> HspolW<DPI_IF_CONF5rs> {
        HspolW::new(self, 11)
    }
    ///Bits 12:22 - DPI interrupt line number
    #[inline(always)]
    pub fn int_line_num(&mut self) -> IntLineNumW<DPI_IF_CONF5rs> {
        IntLineNumW::new(self, 12)
    }
    ///Bit 23 - 1: force DPI clock on 0: DPI clock is controlled by hardware
    #[inline(always)]
    pub fn clk_force_on(&mut self) -> ClkForceOnW<DPI_IF_CONF5rs> {
        ClkForceOnW::new(self, 23)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<DPI_IF_CONF5rs> {
        RsvdW::new(self, 24)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`dpi_if_conf5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi_if_conf5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DPI_IF_CONF5rs;
impl crate::RegisterSpec for DPI_IF_CONF5rs {
    type Ux = u32;
}
///`read()` method returns [`dpi_if_conf5::R`](R) reader structure
impl crate::Readable for DPI_IF_CONF5rs {}
///`write(|w| ..)` method takes [`dpi_if_conf5::W`](W) writer structure
impl crate::Writable for DPI_IF_CONF5rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DPI_IF_CONF5 to value 0
impl crate::Resettable for DPI_IF_CONF5rs {
    const RESET_VALUE: u32 = 0;
}
