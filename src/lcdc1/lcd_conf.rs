///Register `LCD_CONF` reader
pub type R = crate::R<LCD_CONFrs>;
///Register `LCD_CONF` writer
pub type W = crate::W<LCD_CONFrs>;
///Field `TARGET_LCD` reader - The Data can be sent to four destinations: 2'b00: LCD panel 0 2'b01: LCD panel 1 2'b10: AHB LCD 2'b11: AHB RAM
pub type TargetLcdR = crate::FieldReader;
///Field `TARGET_LCD` writer - The Data can be sent to four destinations: 2'b00: LCD panel 0 2'b01: LCD panel 1 2'b10: AHB LCD 2'b11: AHB RAM
pub type TargetLcdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LCD_INTF_SEL` reader - 3'b000: 8080 DBI Type B 3'b001: SPI interface 3'b010: DBI to DSI interface 3'b011: DPI interface 3'b100: JDI serial interface 3'b101: JDI parallel interface 3'b110: 8080 DBI Type A 3'b111: DPI to DSI interface
pub type LcdIntfSelR = crate::FieldReader;
///Field `LCD_INTF_SEL` writer - 3'b000: 8080 DBI Type B 3'b001: SPI interface 3'b010: DBI to DSI interface 3'b011: DPI interface 3'b100: JDI serial interface 3'b101: JDI parallel interface 3'b110: 8080 DBI Type A 3'b111: DPI to DSI interface
pub type LcdIntfSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `LCD_FORMAT` reader - LCD output format: 3'b000: 8-bit RGB 3:3:2 3'b001: 16-bit RGB 5:6:5 over 8-bit bus, 2 cycles/pixel 3'b010: 12-bit RGB 4:4:4 3'b011: 16-bit RGB 5:6:5 3'b100: 18-bit RGB 6:6:6 3'b101: 24-bit RGB 8:8:8 3'b110: 24-bit RGB 8:8:8 over 16-bit bus, 1.5 cycles/pixel 3'b111: 24-bit RGB 8:8:8 over 8-bit bus, 3cycles/pixel others: Reserved
pub type LcdFormatR = crate::FieldReader;
///Field `LCD_FORMAT` writer - LCD output format: 3'b000: 8-bit RGB 3:3:2 3'b001: 16-bit RGB 5:6:5 over 8-bit bus, 2 cycles/pixel 3'b010: 12-bit RGB 4:4:4 3'b011: 16-bit RGB 5:6:5 3'b100: 18-bit RGB 6:6:6 3'b101: 24-bit RGB 8:8:8 3'b110: 24-bit RGB 8:8:8 over 16-bit bus, 1.5 cycles/pixel 3'b111: 24-bit RGB 8:8:8 over 8-bit bus, 3cycles/pixel others: Reserved
pub type LcdFormatW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `AHB_FORMAT` reader - AHB LCD/RAM output format: 0: RGB565 1: RGB888 2: ARGB8888 3: RGB332
pub type AhbFormatR = crate::FieldReader;
///Field `AHB_FORMAT` writer - AHB LCD/RAM output format: 0: RGB565 1: RGB888 2: ARGB8888 3: RGB332
pub type AhbFormatW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SPI_LCD_FORMAT` reader - SPI LCD format 2'b00: 8-bit RGB 3:3:2 2'b01: 16-bit RGB 5:6:5 2'b10: 24-bit RGB 8:8:8 2'b11: Reserved
pub type SpiLcdFormatR = crate::FieldReader;
///Field `SPI_LCD_FORMAT` writer - SPI LCD format 2'b00: 8-bit RGB 3:3:2 2'b01: 16-bit RGB 5:6:5 2'b10: 24-bit RGB 8:8:8 2'b11: Reserved
pub type SpiLcdFormatW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DPI_LCD_FORMAT` reader - DPI LCD format 3'b000: 16-bit conf1 3'b001: 16-bit conf2 3'b010: 16-bit conf3 3'b011: 18-bit conf1 3'b100: 18-bit conf2 3'b101: 24-bit others: Reserved
pub type DpiLcdFormatR = crate::FieldReader;
///Field `DPI_LCD_FORMAT` writer - DPI LCD format 3'b000: 16-bit conf1 3'b001: 16-bit conf2 3'b010: 16-bit conf3 3'b011: 18-bit conf1 3'b100: 18-bit conf2 3'b101: 24-bit others: Reserved
pub type DpiLcdFormatW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `JDI_SER_FORMAT` reader - JDI serial format 2'b00: 3-bit mode 2'b01: 4-bit mode 2'b10: 1-bit mode 2'b11: reserved
pub type JdiSerFormatR = crate::FieldReader;
///Field `JDI_SER_FORMAT` writer - JDI serial format 2'b00: 3-bit mode 2'b01: 4-bit mode 2'b10: 1-bit mode 2'b11: reserved
pub type JdiSerFormatW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DIRECT_INTF_EN` reader - when the target LCD is AHB LCD, this bit enable the direct interface to DSI module. Direct interface has higher bandwidth and speed than AHB interface.
pub type DirectIntfEnR = crate::BitReader;
///Field `DIRECT_INTF_EN` writer - when the target LCD is AHB LCD, this bit enable the direct interface to DSI module. Direct interface has higher bandwidth and speed than AHB interface.
pub type DirectIntfEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENDIAN` reader - LCD 565 data format endian, this bit would affect SPI, DPI, DBI and AHB interface 565 format 0: {R\[4:0\], G\[5:3\], G\[2:0\], B\[4:0\]} 1: {G\[2:0\], R\[4:0\], B\[4:0\], G\[5:3\]}
pub type EndianR = crate::BitReader;
///Field `ENDIAN` writer - LCD 565 data format endian, this bit would affect SPI, DPI, DBI and AHB interface 565 format 0: {R\[4:0\], G\[5:3\], G\[2:0\], B\[4:0\]} 1: {G\[2:0\], R\[4:0\], B\[4:0\], G\[5:3\]}
pub type EndianW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI_RD_SEL` reader - spi read line select. 0: select line 0 1: select line 1 2: select line 2 3: select line 3
pub type SpiRdSelR = crate::FieldReader;
///Field `SPI_RD_SEL` writer - spi read line select. 0: select line 0 1: select line 1 2: select line 2 3: select line 3
pub type SpiRdSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - The Data can be sent to four destinations: 2'b00: LCD panel 0 2'b01: LCD panel 1 2'b10: AHB LCD 2'b11: AHB RAM
    #[inline(always)]
    pub fn target_lcd(&self) -> TargetLcdR {
        TargetLcdR::new((self.bits & 3) as u8)
    }
    ///Bits 2:4 - 3'b000: 8080 DBI Type B 3'b001: SPI interface 3'b010: DBI to DSI interface 3'b011: DPI interface 3'b100: JDI serial interface 3'b101: JDI parallel interface 3'b110: 8080 DBI Type A 3'b111: DPI to DSI interface
    #[inline(always)]
    pub fn lcd_intf_sel(&self) -> LcdIntfSelR {
        LcdIntfSelR::new(((self.bits >> 2) & 7) as u8)
    }
    ///Bits 5:7 - LCD output format: 3'b000: 8-bit RGB 3:3:2 3'b001: 16-bit RGB 5:6:5 over 8-bit bus, 2 cycles/pixel 3'b010: 12-bit RGB 4:4:4 3'b011: 16-bit RGB 5:6:5 3'b100: 18-bit RGB 6:6:6 3'b101: 24-bit RGB 8:8:8 3'b110: 24-bit RGB 8:8:8 over 16-bit bus, 1.5 cycles/pixel 3'b111: 24-bit RGB 8:8:8 over 8-bit bus, 3cycles/pixel others: Reserved
    #[inline(always)]
    pub fn lcd_format(&self) -> LcdFormatR {
        LcdFormatR::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bits 8:9 - AHB LCD/RAM output format: 0: RGB565 1: RGB888 2: ARGB8888 3: RGB332
    #[inline(always)]
    pub fn ahb_format(&self) -> AhbFormatR {
        AhbFormatR::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - SPI LCD format 2'b00: 8-bit RGB 3:3:2 2'b01: 16-bit RGB 5:6:5 2'b10: 24-bit RGB 8:8:8 2'b11: Reserved
    #[inline(always)]
    pub fn spi_lcd_format(&self) -> SpiLcdFormatR {
        SpiLcdFormatR::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:14 - DPI LCD format 3'b000: 16-bit conf1 3'b001: 16-bit conf2 3'b010: 16-bit conf3 3'b011: 18-bit conf1 3'b100: 18-bit conf2 3'b101: 24-bit others: Reserved
    #[inline(always)]
    pub fn dpi_lcd_format(&self) -> DpiLcdFormatR {
        DpiLcdFormatR::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 15:16 - JDI serial format 2'b00: 3-bit mode 2'b01: 4-bit mode 2'b10: 1-bit mode 2'b11: reserved
    #[inline(always)]
    pub fn jdi_ser_format(&self) -> JdiSerFormatR {
        JdiSerFormatR::new(((self.bits >> 15) & 3) as u8)
    }
    ///Bit 17 - when the target LCD is AHB LCD, this bit enable the direct interface to DSI module. Direct interface has higher bandwidth and speed than AHB interface.
    #[inline(always)]
    pub fn direct_intf_en(&self) -> DirectIntfEnR {
        DirectIntfEnR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - LCD 565 data format endian, this bit would affect SPI, DPI, DBI and AHB interface 565 format 0: {R\[4:0\], G\[5:3\], G\[2:0\], B\[4:0\]} 1: {G\[2:0\], R\[4:0\], B\[4:0\], G\[5:3\]}
    #[inline(always)]
    pub fn endian(&self) -> EndianR {
        EndianR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 19:20 - spi read line select. 0: select line 0 1: select line 1 2: select line 2 3: select line 3
    #[inline(always)]
    pub fn spi_rd_sel(&self) -> SpiRdSelR {
        SpiRdSelR::new(((self.bits >> 19) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCD_CONF")
            .field("spi_rd_sel", &self.spi_rd_sel())
            .field("endian", &self.endian())
            .field("direct_intf_en", &self.direct_intf_en())
            .field("jdi_ser_format", &self.jdi_ser_format())
            .field("dpi_lcd_format", &self.dpi_lcd_format())
            .field("spi_lcd_format", &self.spi_lcd_format())
            .field("ahb_format", &self.ahb_format())
            .field("lcd_format", &self.lcd_format())
            .field("lcd_intf_sel", &self.lcd_intf_sel())
            .field("target_lcd", &self.target_lcd())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - The Data can be sent to four destinations: 2'b00: LCD panel 0 2'b01: LCD panel 1 2'b10: AHB LCD 2'b11: AHB RAM
    #[inline(always)]
    pub fn target_lcd(&mut self) -> TargetLcdW<LCD_CONFrs> {
        TargetLcdW::new(self, 0)
    }
    ///Bits 2:4 - 3'b000: 8080 DBI Type B 3'b001: SPI interface 3'b010: DBI to DSI interface 3'b011: DPI interface 3'b100: JDI serial interface 3'b101: JDI parallel interface 3'b110: 8080 DBI Type A 3'b111: DPI to DSI interface
    #[inline(always)]
    pub fn lcd_intf_sel(&mut self) -> LcdIntfSelW<LCD_CONFrs> {
        LcdIntfSelW::new(self, 2)
    }
    ///Bits 5:7 - LCD output format: 3'b000: 8-bit RGB 3:3:2 3'b001: 16-bit RGB 5:6:5 over 8-bit bus, 2 cycles/pixel 3'b010: 12-bit RGB 4:4:4 3'b011: 16-bit RGB 5:6:5 3'b100: 18-bit RGB 6:6:6 3'b101: 24-bit RGB 8:8:8 3'b110: 24-bit RGB 8:8:8 over 16-bit bus, 1.5 cycles/pixel 3'b111: 24-bit RGB 8:8:8 over 8-bit bus, 3cycles/pixel others: Reserved
    #[inline(always)]
    pub fn lcd_format(&mut self) -> LcdFormatW<LCD_CONFrs> {
        LcdFormatW::new(self, 5)
    }
    ///Bits 8:9 - AHB LCD/RAM output format: 0: RGB565 1: RGB888 2: ARGB8888 3: RGB332
    #[inline(always)]
    pub fn ahb_format(&mut self) -> AhbFormatW<LCD_CONFrs> {
        AhbFormatW::new(self, 8)
    }
    ///Bits 10:11 - SPI LCD format 2'b00: 8-bit RGB 3:3:2 2'b01: 16-bit RGB 5:6:5 2'b10: 24-bit RGB 8:8:8 2'b11: Reserved
    #[inline(always)]
    pub fn spi_lcd_format(&mut self) -> SpiLcdFormatW<LCD_CONFrs> {
        SpiLcdFormatW::new(self, 10)
    }
    ///Bits 12:14 - DPI LCD format 3'b000: 16-bit conf1 3'b001: 16-bit conf2 3'b010: 16-bit conf3 3'b011: 18-bit conf1 3'b100: 18-bit conf2 3'b101: 24-bit others: Reserved
    #[inline(always)]
    pub fn dpi_lcd_format(&mut self) -> DpiLcdFormatW<LCD_CONFrs> {
        DpiLcdFormatW::new(self, 12)
    }
    ///Bits 15:16 - JDI serial format 2'b00: 3-bit mode 2'b01: 4-bit mode 2'b10: 1-bit mode 2'b11: reserved
    #[inline(always)]
    pub fn jdi_ser_format(&mut self) -> JdiSerFormatW<LCD_CONFrs> {
        JdiSerFormatW::new(self, 15)
    }
    ///Bit 17 - when the target LCD is AHB LCD, this bit enable the direct interface to DSI module. Direct interface has higher bandwidth and speed than AHB interface.
    #[inline(always)]
    pub fn direct_intf_en(&mut self) -> DirectIntfEnW<LCD_CONFrs> {
        DirectIntfEnW::new(self, 17)
    }
    ///Bit 18 - LCD 565 data format endian, this bit would affect SPI, DPI, DBI and AHB interface 565 format 0: {R\[4:0\], G\[5:3\], G\[2:0\], B\[4:0\]} 1: {G\[2:0\], R\[4:0\], B\[4:0\], G\[5:3\]}
    #[inline(always)]
    pub fn endian(&mut self) -> EndianW<LCD_CONFrs> {
        EndianW::new(self, 18)
    }
    ///Bits 19:20 - spi read line select. 0: select line 0 1: select line 1 2: select line 2 3: select line 3
    #[inline(always)]
    pub fn spi_rd_sel(&mut self) -> SpiRdSelW<LCD_CONFrs> {
        SpiRdSelW::new(self, 19)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`lcd_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct LCD_CONFrs;
impl crate::RegisterSpec for LCD_CONFrs {
    type Ux = u32;
}
///`read()` method returns [`lcd_conf::R`](R) reader structure
impl crate::Readable for LCD_CONFrs {}
///`write(|w| ..)` method takes [`lcd_conf::W`](W) writer structure
impl crate::Writable for LCD_CONFrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LCD_CONF to value 0
impl crate::Resettable for LCD_CONFrs {
    const RESET_VALUE: u32 = 0;
}
