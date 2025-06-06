///Register `LCD_IF_CONF` reader
pub type R = crate::R<LCD_IF_CONFrs>;
///Register `LCD_IF_CONF` writer
pub type W = crate::W<LCD_IF_CONFrs>;
///Field `TAS` reader - setup cycles, delay from LCD_CS active to LCD_WR/LCD_RD active
pub type TasR = crate::FieldReader;
///Field `TAS` writer - setup cycles, delay from LCD_CS active to LCD_WR/LCD_RD active
pub type TasW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TAH` reader - hold cycles, delay from LCD_WR/LCD_RD inactive to LCD_CS inactive
pub type TahR = crate::FieldReader;
///Field `TAH` writer - hold cycles, delay from LCD_WR/LCD_RD inactive to LCD_CS inactive
pub type TahW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PWL` reader - active cycles of LCD_WR/LCD_RD
pub type PwlR = crate::FieldReader;
///Field `PWL` writer - active cycles of LCD_WR/LCD_RD
pub type PwlW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `PWH` reader - inactive cycles of LCD_WR/LCD_RD for consecutive write/read operation
pub type PwhR = crate::FieldReader;
///Field `PWH` writer - inactive cycles of LCD_WR/LCD_RD for consecutive write/read operation
pub type PwhW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `CS0_POL` reader - LCD1 CS pin polarity. CS is 0 for LCD chip select if polarity bit is set as 0. CS bit definition is opposite if polarity bit is set as 1.
pub type Cs0PolR = crate::BitReader;
///Field `CS0_POL` writer - LCD1 CS pin polarity. CS is 0 for LCD chip select if polarity bit is set as 0. CS bit definition is opposite if polarity bit is set as 1.
pub type Cs0PolW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CS1_POL` reader - LCD0 CS pin polarity. CS is 0 for LCD chip select if polarity bit is set as 0. CS bit definition is opposite if polarity bit is set as 1.
pub type Cs1PolR = crate::BitReader;
///Field `CS1_POL` writer - LCD0 CS pin polarity. CS is 0 for LCD chip select if polarity bit is set as 0. CS bit definition is opposite if polarity bit is set as 1.
pub type Cs1PolW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RS_POL` reader - LCD RS pin polarity. RS is 1 for data access, 0 for command access if polarity bit is set as 0. RS bit definition is opposite if polarity bit is set as 1.
pub type RsPolR = crate::BitReader;
///Field `RS_POL` writer - LCD RS pin polarity. RS is 1 for data access, 0 for command access if polarity bit is set as 0. RS bit definition is opposite if polarity bit is set as 1.
pub type RsPolW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WR_POL` reader - LCD WR pin polarity. WR is 0 for write operation, 1 for idle if polarity bit is set as 0. WR bit definition is opposite if polarity bit is set as 1.
pub type WrPolR = crate::BitReader;
///Field `WR_POL` writer - LCD WR pin polarity. WR is 0 for write operation, 1 for idle if polarity bit is set as 0. WR bit definition is opposite if polarity bit is set as 1.
pub type WrPolW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RD_POL` reader - LCD RD pin polarity. RD is 0 for write operation, 1 for idle if polarity bit is set as 0. RD bit definition is opposite if polarity bit is set as 1.
pub type RdPolR = crate::BitReader;
///Field `RD_POL` writer - LCD RD pin polarity. RD is 0 for write operation, 1 for idle if polarity bit is set as 0. RD bit definition is opposite if polarity bit is set as 1.
pub type RdPolW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCD_RSTB` reader - LCD RSTB pin, direct to output
pub type LcdRstbR = crate::BitReader;
///Field `LCD_RSTB` writer - LCD RSTB pin, direct to output
pub type LcdRstbW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DO_DLY_SET` reader - if this bit is set to 1, LCD data output will be delayed for 1 lcdc clock cycle
pub type DoDlySetR = crate::BitReader;
///Field `DO_DLY_SET` writer - if this bit is set to 1, LCD data output will be delayed for 1 lcdc clock cycle
pub type DoDlySetW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTRL_DLY_SET` reader - if this bit is set to 1, LCD control output will be delayed for 1 lcdc clock cycle
pub type CtrlDlySetR = crate::BitReader;
///Field `CTRL_DLY_SET` writer - if this bit is set to 1, LCD control output will be delayed for 1 lcdc clock cycle
pub type CtrlDlySetW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:2 - setup cycles, delay from LCD_CS active to LCD_WR/LCD_RD active
    #[inline(always)]
    pub fn tas(&self) -> TasR {
        TasR::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - hold cycles, delay from LCD_WR/LCD_RD inactive to LCD_CS inactive
    #[inline(always)]
    pub fn tah(&self) -> TahR {
        TahR::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:11 - active cycles of LCD_WR/LCD_RD
    #[inline(always)]
    pub fn pwl(&self) -> PwlR {
        PwlR::new(((self.bits >> 6) & 0x3f) as u8)
    }
    ///Bits 12:17 - inactive cycles of LCD_WR/LCD_RD for consecutive write/read operation
    #[inline(always)]
    pub fn pwh(&self) -> PwhR {
        PwhR::new(((self.bits >> 12) & 0x3f) as u8)
    }
    ///Bit 18 - LCD1 CS pin polarity. CS is 0 for LCD chip select if polarity bit is set as 0. CS bit definition is opposite if polarity bit is set as 1.
    #[inline(always)]
    pub fn cs0_pol(&self) -> Cs0PolR {
        Cs0PolR::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - LCD0 CS pin polarity. CS is 0 for LCD chip select if polarity bit is set as 0. CS bit definition is opposite if polarity bit is set as 1.
    #[inline(always)]
    pub fn cs1_pol(&self) -> Cs1PolR {
        Cs1PolR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - LCD RS pin polarity. RS is 1 for data access, 0 for command access if polarity bit is set as 0. RS bit definition is opposite if polarity bit is set as 1.
    #[inline(always)]
    pub fn rs_pol(&self) -> RsPolR {
        RsPolR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - LCD WR pin polarity. WR is 0 for write operation, 1 for idle if polarity bit is set as 0. WR bit definition is opposite if polarity bit is set as 1.
    #[inline(always)]
    pub fn wr_pol(&self) -> WrPolR {
        WrPolR::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - LCD RD pin polarity. RD is 0 for write operation, 1 for idle if polarity bit is set as 0. RD bit definition is opposite if polarity bit is set as 1.
    #[inline(always)]
    pub fn rd_pol(&self) -> RdPolR {
        RdPolR::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - LCD RSTB pin, direct to output
    #[inline(always)]
    pub fn lcd_rstb(&self) -> LcdRstbR {
        LcdRstbR::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - if this bit is set to 1, LCD data output will be delayed for 1 lcdc clock cycle
    #[inline(always)]
    pub fn do_dly_set(&self) -> DoDlySetR {
        DoDlySetR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - if this bit is set to 1, LCD control output will be delayed for 1 lcdc clock cycle
    #[inline(always)]
    pub fn ctrl_dly_set(&self) -> CtrlDlySetR {
        CtrlDlySetR::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bits 26:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCD_IF_CONF")
            .field("rsvd", &self.rsvd())
            .field("ctrl_dly_set", &self.ctrl_dly_set())
            .field("do_dly_set", &self.do_dly_set())
            .field("lcd_rstb", &self.lcd_rstb())
            .field("rd_pol", &self.rd_pol())
            .field("wr_pol", &self.wr_pol())
            .field("rs_pol", &self.rs_pol())
            .field("cs1_pol", &self.cs1_pol())
            .field("cs0_pol", &self.cs0_pol())
            .field("pwh", &self.pwh())
            .field("pwl", &self.pwl())
            .field("tah", &self.tah())
            .field("tas", &self.tas())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - setup cycles, delay from LCD_CS active to LCD_WR/LCD_RD active
    #[inline(always)]
    pub fn tas(&mut self) -> TasW<LCD_IF_CONFrs> {
        TasW::new(self, 0)
    }
    ///Bits 3:5 - hold cycles, delay from LCD_WR/LCD_RD inactive to LCD_CS inactive
    #[inline(always)]
    pub fn tah(&mut self) -> TahW<LCD_IF_CONFrs> {
        TahW::new(self, 3)
    }
    ///Bits 6:11 - active cycles of LCD_WR/LCD_RD
    #[inline(always)]
    pub fn pwl(&mut self) -> PwlW<LCD_IF_CONFrs> {
        PwlW::new(self, 6)
    }
    ///Bits 12:17 - inactive cycles of LCD_WR/LCD_RD for consecutive write/read operation
    #[inline(always)]
    pub fn pwh(&mut self) -> PwhW<LCD_IF_CONFrs> {
        PwhW::new(self, 12)
    }
    ///Bit 18 - LCD1 CS pin polarity. CS is 0 for LCD chip select if polarity bit is set as 0. CS bit definition is opposite if polarity bit is set as 1.
    #[inline(always)]
    pub fn cs0_pol(&mut self) -> Cs0PolW<LCD_IF_CONFrs> {
        Cs0PolW::new(self, 18)
    }
    ///Bit 19 - LCD0 CS pin polarity. CS is 0 for LCD chip select if polarity bit is set as 0. CS bit definition is opposite if polarity bit is set as 1.
    #[inline(always)]
    pub fn cs1_pol(&mut self) -> Cs1PolW<LCD_IF_CONFrs> {
        Cs1PolW::new(self, 19)
    }
    ///Bit 20 - LCD RS pin polarity. RS is 1 for data access, 0 for command access if polarity bit is set as 0. RS bit definition is opposite if polarity bit is set as 1.
    #[inline(always)]
    pub fn rs_pol(&mut self) -> RsPolW<LCD_IF_CONFrs> {
        RsPolW::new(self, 20)
    }
    ///Bit 21 - LCD WR pin polarity. WR is 0 for write operation, 1 for idle if polarity bit is set as 0. WR bit definition is opposite if polarity bit is set as 1.
    #[inline(always)]
    pub fn wr_pol(&mut self) -> WrPolW<LCD_IF_CONFrs> {
        WrPolW::new(self, 21)
    }
    ///Bit 22 - LCD RD pin polarity. RD is 0 for write operation, 1 for idle if polarity bit is set as 0. RD bit definition is opposite if polarity bit is set as 1.
    #[inline(always)]
    pub fn rd_pol(&mut self) -> RdPolW<LCD_IF_CONFrs> {
        RdPolW::new(self, 22)
    }
    ///Bit 23 - LCD RSTB pin, direct to output
    #[inline(always)]
    pub fn lcd_rstb(&mut self) -> LcdRstbW<LCD_IF_CONFrs> {
        LcdRstbW::new(self, 23)
    }
    ///Bit 24 - if this bit is set to 1, LCD data output will be delayed for 1 lcdc clock cycle
    #[inline(always)]
    pub fn do_dly_set(&mut self) -> DoDlySetW<LCD_IF_CONFrs> {
        DoDlySetW::new(self, 24)
    }
    ///Bit 25 - if this bit is set to 1, LCD control output will be delayed for 1 lcdc clock cycle
    #[inline(always)]
    pub fn ctrl_dly_set(&mut self) -> CtrlDlySetW<LCD_IF_CONFrs> {
        CtrlDlySetW::new(self, 25)
    }
    ///Bits 26:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<LCD_IF_CONFrs> {
        RsvdW::new(self, 26)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`lcd_if_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_if_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct LCD_IF_CONFrs;
impl crate::RegisterSpec for LCD_IF_CONFrs {
    type Ux = u32;
}
///`read()` method returns [`lcd_if_conf::R`](R) reader structure
impl crate::Readable for LCD_IF_CONFrs {}
///`write(|w| ..)` method takes [`lcd_if_conf::W`](W) writer structure
impl crate::Writable for LCD_IF_CONFrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LCD_IF_CONF to value 0
impl crate::Resettable for LCD_IF_CONFrs {
    const RESET_VALUE: u32 = 0;
}
