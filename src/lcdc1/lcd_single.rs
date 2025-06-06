///Register `LCD_SINGLE` reader
pub type R = crate::R<LCD_SINGLErs>;
///Register `LCD_SINGLE` writer
pub type W = crate::W<LCD_SINGLErs>;
///Field `TYPE` reader - LCD access type, this bit could affect all LCD interface including SPI, parellel and AHB 1'b0: command 1'b1: data
pub type TypeR = crate::BitReader;
///Field `TYPE` writer - LCD access type, this bit could affect all LCD interface including SPI, parellel and AHB 1'b0: command 1'b1: data
pub type TypeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WR_TRIG` reader - Single write operation trigger
pub type WrTrigR = crate::BitReader;
///Field `WR_TRIG` writer - Single write operation trigger
pub type WrTrigW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RD_TRIG` reader - Single read operation trigger
pub type RdTrigR = crate::BitReader;
///Field `RD_TRIG` writer - Single read operation trigger
pub type RdTrigW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCD_BUSY` reader - LCD/SPI LCD interface is busy for single access
pub type LcdBusyR = crate::BitReader;
///Field `LCD_BUSY` writer - LCD/SPI LCD interface is busy for single access
pub type LcdBusyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LCD access type, this bit could affect all LCD interface including SPI, parellel and AHB 1'b0: command 1'b1: data
    #[inline(always)]
    pub fn type_(&self) -> TypeR {
        TypeR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Single write operation trigger
    #[inline(always)]
    pub fn wr_trig(&self) -> WrTrigR {
        WrTrigR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Single read operation trigger
    #[inline(always)]
    pub fn rd_trig(&self) -> RdTrigR {
        RdTrigR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - LCD/SPI LCD interface is busy for single access
    #[inline(always)]
    pub fn lcd_busy(&self) -> LcdBusyR {
        LcdBusyR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCD_SINGLE")
            .field("lcd_busy", &self.lcd_busy())
            .field("rd_trig", &self.rd_trig())
            .field("wr_trig", &self.wr_trig())
            .field("type_", &self.type_())
            .finish()
    }
}
impl W {
    ///Bit 0 - LCD access type, this bit could affect all LCD interface including SPI, parellel and AHB 1'b0: command 1'b1: data
    #[inline(always)]
    pub fn type_(&mut self) -> TypeW<LCD_SINGLErs> {
        TypeW::new(self, 0)
    }
    ///Bit 1 - Single write operation trigger
    #[inline(always)]
    pub fn wr_trig(&mut self) -> WrTrigW<LCD_SINGLErs> {
        WrTrigW::new(self, 1)
    }
    ///Bit 2 - Single read operation trigger
    #[inline(always)]
    pub fn rd_trig(&mut self) -> RdTrigW<LCD_SINGLErs> {
        RdTrigW::new(self, 2)
    }
    ///Bit 3 - LCD/SPI LCD interface is busy for single access
    #[inline(always)]
    pub fn lcd_busy(&mut self) -> LcdBusyW<LCD_SINGLErs> {
        LcdBusyW::new(self, 3)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`lcd_single::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_single::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct LCD_SINGLErs;
impl crate::RegisterSpec for LCD_SINGLErs {
    type Ux = u32;
}
///`read()` method returns [`lcd_single::R`](R) reader structure
impl crate::Readable for LCD_SINGLErs {}
///`write(|w| ..)` method takes [`lcd_single::W`](W) writer structure
impl crate::Writable for LCD_SINGLErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LCD_SINGLE to value 0
impl crate::Resettable for LCD_SINGLErs {
    const RESET_VALUE: u32 = 0;
}
