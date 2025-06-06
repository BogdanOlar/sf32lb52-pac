///Register `SPI_IF_CONF` reader
pub type R = crate::R<SPI_IF_CONFrs>;
///Register `SPI_IF_CONF` writer
pub type W = crate::W<SPI_IF_CONFrs>;
///Field `WAIT_CYCLE` reader - SPI line wait cycle, wait cycle is after each line and is according to SPI clock. 0 refers to no wait cycle.
pub type WaitCycleR = crate::FieldReader;
///Field `WAIT_CYCLE` writer - SPI line wait cycle, wait cycle is after each line and is according to SPI clock. 0 refers to no wait cycle.
pub type WaitCycleW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `CLK_DIV` reader - SPI clock divider
pub type ClkDivR = crate::FieldReader;
///Field `CLK_DIV` writer - SPI clock divider
pub type ClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DUMMY_CYCLE` reader - SPI transaction dummy cycle
pub type DummyCycleR = crate::FieldReader;
///Field `DUMMY_CYCLE` writer - SPI transaction dummy cycle
pub type DummyCycleW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `LINE` reader - SPI line mode 0: 4-line 1: 4-line with 2 data line(support RGB565 and RGB888) 2: 4-line with 4 data line(support RGB565 and RGB888) 3: reserved 4: 3-line 5: 3-line with 2 data line(support RGB565 and RGB888) 6: 3-line with 4 data line(support RGB565 and RGB888) 7: reserved
pub type LineR = crate::FieldReader;
///Field `LINE` writer - SPI line mode 0: 4-line 1: 4-line with 2 data line(support RGB565 and RGB888) 2: 4-line with 4 data line(support RGB565 and RGB888) 3: reserved 4: 3-line 5: 3-line with 2 data line(support RGB565 and RGB888) 6: 3-line with 4 data line(support RGB565 and RGB888) 7: reserved
pub type LineW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RD_LEN` reader - SPI read data length(single access)
pub type RdLenR = crate::FieldReader;
///Field `RD_LEN` writer - SPI read data length(single access)
pub type RdLenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WR_LEN` reader - SPI write data length(single access)
pub type WrLenR = crate::FieldReader;
///Field `WR_LEN` writer - SPI write data length(single access)
pub type WrLenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SPI_RD_MODE` reader - SPI read mode: 1'b0: normal read. Send write request before read. 1'b1: direct read. Read data without write request.
pub type SpiRdModeR = crate::BitReader;
///Field `SPI_RD_MODE` writer - SPI read mode: 1'b0: normal read. Send write request before read. 1'b1: direct read. Read data without write request.
pub type SpiRdModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI_CLK_AUTO_DIS` reader - 1: SPI clock auto disable in wait state during data transaction 0: SPI clock is always on in wait state during data transaction
pub type SpiClkAutoDisR = crate::BitReader;
///Field `SPI_CLK_AUTO_DIS` writer - 1: SPI clock auto disable in wait state during data transaction 0: SPI clock is always on in wait state during data transaction
pub type SpiClkAutoDisW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI_CS_NO_IDLE` reader - 1: SPI CS is always active during data transaction 0: SPI CS is IDLE in wait state during data transaction
pub type SpiCsNoIdleR = crate::BitReader;
///Field `SPI_CS_NO_IDLE` writer - 1: SPI CS is always active during data transaction 0: SPI CS is IDLE in wait state during data transaction
pub type SpiCsNoIdleW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI_CS_AUTO_DIS` reader - 1: SPI CS is automatically disabled after data transaction 0: SPI CS is not disabled after data transaction
pub type SpiCsAutoDisR = crate::BitReader;
///Field `SPI_CS_AUTO_DIS` writer - 1: SPI CS is automatically disabled after data transaction 0: SPI CS is not disabled after data transaction
pub type SpiCsAutoDisW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI_CS_POL` reader - SPI CS polarity 0: low active 1: high active
pub type SpiCsPolR = crate::BitReader;
///Field `SPI_CS_POL` writer - SPI CS polarity 0: low active 1: high active
pub type SpiCsPolW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI_CLK_POL` reader - SPI CLK polarity 1'h0: normal 1'h1: inverted
pub type SpiClkPolR = crate::BitReader;
///Field `SPI_CLK_POL` writer - SPI CLK polarity 1'h0: normal 1'h1: inverted
pub type SpiClkPolW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI_CLK_INIT` reader - SPI CLK idle state value 1'h0: high 1'h1: low
pub type SpiClkInitR = crate::BitReader;
///Field `SPI_CLK_INIT` writer - SPI CLK idle state value 1'h0: high 1'h1: low
pub type SpiClkInitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:5 - SPI line wait cycle, wait cycle is after each line and is according to SPI clock. 0 refers to no wait cycle.
    #[inline(always)]
    pub fn wait_cycle(&self) -> WaitCycleR {
        WaitCycleR::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:13 - SPI clock divider
    #[inline(always)]
    pub fn clk_div(&self) -> ClkDivR {
        ClkDivR::new(((self.bits >> 6) & 0xff) as u8)
    }
    ///Bits 14:16 - SPI transaction dummy cycle
    #[inline(always)]
    pub fn dummy_cycle(&self) -> DummyCycleR {
        DummyCycleR::new(((self.bits >> 14) & 7) as u8)
    }
    ///Bits 17:19 - SPI line mode 0: 4-line 1: 4-line with 2 data line(support RGB565 and RGB888) 2: 4-line with 4 data line(support RGB565 and RGB888) 3: reserved 4: 3-line 5: 3-line with 2 data line(support RGB565 and RGB888) 6: 3-line with 4 data line(support RGB565 and RGB888) 7: reserved
    #[inline(always)]
    pub fn line(&self) -> LineR {
        LineR::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bits 20:21 - SPI read data length(single access)
    #[inline(always)]
    pub fn rd_len(&self) -> RdLenR {
        RdLenR::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - SPI write data length(single access)
    #[inline(always)]
    pub fn wr_len(&self) -> WrLenR {
        WrLenR::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bit 24 - SPI read mode: 1'b0: normal read. Send write request before read. 1'b1: direct read. Read data without write request.
    #[inline(always)]
    pub fn spi_rd_mode(&self) -> SpiRdModeR {
        SpiRdModeR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - 1: SPI clock auto disable in wait state during data transaction 0: SPI clock is always on in wait state during data transaction
    #[inline(always)]
    pub fn spi_clk_auto_dis(&self) -> SpiClkAutoDisR {
        SpiClkAutoDisR::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - 1: SPI CS is always active during data transaction 0: SPI CS is IDLE in wait state during data transaction
    #[inline(always)]
    pub fn spi_cs_no_idle(&self) -> SpiCsNoIdleR {
        SpiCsNoIdleR::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - 1: SPI CS is automatically disabled after data transaction 0: SPI CS is not disabled after data transaction
    #[inline(always)]
    pub fn spi_cs_auto_dis(&self) -> SpiCsAutoDisR {
        SpiCsAutoDisR::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - SPI CS polarity 0: low active 1: high active
    #[inline(always)]
    pub fn spi_cs_pol(&self) -> SpiCsPolR {
        SpiCsPolR::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - SPI CLK polarity 1'h0: normal 1'h1: inverted
    #[inline(always)]
    pub fn spi_clk_pol(&self) -> SpiClkPolR {
        SpiClkPolR::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - SPI CLK idle state value 1'h0: high 1'h1: low
    #[inline(always)]
    pub fn spi_clk_init(&self) -> SpiClkInitR {
        SpiClkInitR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_IF_CONF")
            .field("spi_clk_init", &self.spi_clk_init())
            .field("spi_clk_pol", &self.spi_clk_pol())
            .field("spi_cs_pol", &self.spi_cs_pol())
            .field("spi_cs_auto_dis", &self.spi_cs_auto_dis())
            .field("spi_cs_no_idle", &self.spi_cs_no_idle())
            .field("spi_clk_auto_dis", &self.spi_clk_auto_dis())
            .field("spi_rd_mode", &self.spi_rd_mode())
            .field("wr_len", &self.wr_len())
            .field("rd_len", &self.rd_len())
            .field("line", &self.line())
            .field("dummy_cycle", &self.dummy_cycle())
            .field("clk_div", &self.clk_div())
            .field("wait_cycle", &self.wait_cycle())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - SPI line wait cycle, wait cycle is after each line and is according to SPI clock. 0 refers to no wait cycle.
    #[inline(always)]
    pub fn wait_cycle(&mut self) -> WaitCycleW<SPI_IF_CONFrs> {
        WaitCycleW::new(self, 0)
    }
    ///Bits 6:13 - SPI clock divider
    #[inline(always)]
    pub fn clk_div(&mut self) -> ClkDivW<SPI_IF_CONFrs> {
        ClkDivW::new(self, 6)
    }
    ///Bits 14:16 - SPI transaction dummy cycle
    #[inline(always)]
    pub fn dummy_cycle(&mut self) -> DummyCycleW<SPI_IF_CONFrs> {
        DummyCycleW::new(self, 14)
    }
    ///Bits 17:19 - SPI line mode 0: 4-line 1: 4-line with 2 data line(support RGB565 and RGB888) 2: 4-line with 4 data line(support RGB565 and RGB888) 3: reserved 4: 3-line 5: 3-line with 2 data line(support RGB565 and RGB888) 6: 3-line with 4 data line(support RGB565 and RGB888) 7: reserved
    #[inline(always)]
    pub fn line(&mut self) -> LineW<SPI_IF_CONFrs> {
        LineW::new(self, 17)
    }
    ///Bits 20:21 - SPI read data length(single access)
    #[inline(always)]
    pub fn rd_len(&mut self) -> RdLenW<SPI_IF_CONFrs> {
        RdLenW::new(self, 20)
    }
    ///Bits 22:23 - SPI write data length(single access)
    #[inline(always)]
    pub fn wr_len(&mut self) -> WrLenW<SPI_IF_CONFrs> {
        WrLenW::new(self, 22)
    }
    ///Bit 24 - SPI read mode: 1'b0: normal read. Send write request before read. 1'b1: direct read. Read data without write request.
    #[inline(always)]
    pub fn spi_rd_mode(&mut self) -> SpiRdModeW<SPI_IF_CONFrs> {
        SpiRdModeW::new(self, 24)
    }
    ///Bit 25 - 1: SPI clock auto disable in wait state during data transaction 0: SPI clock is always on in wait state during data transaction
    #[inline(always)]
    pub fn spi_clk_auto_dis(&mut self) -> SpiClkAutoDisW<SPI_IF_CONFrs> {
        SpiClkAutoDisW::new(self, 25)
    }
    ///Bit 26 - 1: SPI CS is always active during data transaction 0: SPI CS is IDLE in wait state during data transaction
    #[inline(always)]
    pub fn spi_cs_no_idle(&mut self) -> SpiCsNoIdleW<SPI_IF_CONFrs> {
        SpiCsNoIdleW::new(self, 26)
    }
    ///Bit 27 - 1: SPI CS is automatically disabled after data transaction 0: SPI CS is not disabled after data transaction
    #[inline(always)]
    pub fn spi_cs_auto_dis(&mut self) -> SpiCsAutoDisW<SPI_IF_CONFrs> {
        SpiCsAutoDisW::new(self, 27)
    }
    ///Bit 28 - SPI CS polarity 0: low active 1: high active
    #[inline(always)]
    pub fn spi_cs_pol(&mut self) -> SpiCsPolW<SPI_IF_CONFrs> {
        SpiCsPolW::new(self, 28)
    }
    ///Bit 29 - SPI CLK polarity 1'h0: normal 1'h1: inverted
    #[inline(always)]
    pub fn spi_clk_pol(&mut self) -> SpiClkPolW<SPI_IF_CONFrs> {
        SpiClkPolW::new(self, 29)
    }
    ///Bit 30 - SPI CLK idle state value 1'h0: high 1'h1: low
    #[inline(always)]
    pub fn spi_clk_init(&mut self) -> SpiClkInitW<SPI_IF_CONFrs> {
        SpiClkInitW::new(self, 30)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`spi_if_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_if_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SPI_IF_CONFrs;
impl crate::RegisterSpec for SPI_IF_CONFrs {
    type Ux = u32;
}
///`read()` method returns [`spi_if_conf::R`](R) reader structure
impl crate::Readable for SPI_IF_CONFrs {}
///`write(|w| ..)` method takes [`spi_if_conf::W`](W) writer structure
impl crate::Writable for SPI_IF_CONFrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SPI_IF_CONF to value 0
impl crate::Resettable for SPI_IF_CONFrs {
    const RESET_VALUE: u32 = 0;
}
