///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Register `ISR` writer
pub type W = crate::W<ISRrs>;
///Field `GIF1` reader - channel global interrupt flag. Set when any of TEIF/HTIF/TCIF asserted. Cleared when TEIF/HTIF/TCIF all cleared.
pub type Gif1R = crate::BitReader;
///Field `GIF1` writer - channel global interrupt flag. Set when any of TEIF/HTIF/TCIF asserted. Cleared when TEIF/HTIF/TCIF all cleared.
pub type Gif1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIF1` reader - channel transfer complete flag. Set when all NDT are transferred. Cleared when write 1 to CTCIF or CGIF.
pub type Tcif1R = crate::BitReader;
///Field `TCIF1` writer - channel transfer complete flag. Set when all NDT are transferred. Cleared when write 1 to CTCIF or CGIF.
pub type Tcif1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HTIF1` reader - channel half transfer flag. Set when half NDT are transferred. Cleared when write 1 to CHTIF or CGIF.
pub type Htif1R = crate::BitReader;
///Field `HTIF1` writer - channel half transfer flag. Set when half NDT are transferred. Cleared when write 1 to CHTIF or CGIF.
pub type Htif1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEIF1` reader - channel transfer error flag. Set when bus error detected. Cleared when write 1 to CTEIF or CGIF.
pub type Teif1R = crate::BitReader;
///Field `TEIF1` writer - channel transfer error flag. Set when bus error detected. Cleared when write 1 to CTEIF or CGIF.
pub type Teif1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GIF2` reader - channel global interrupt flag
pub type Gif2R = crate::BitReader;
///Field `GIF2` writer - channel global interrupt flag
pub type Gif2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIF2` reader - channel transfer complete flag
pub type Tcif2R = crate::BitReader;
///Field `TCIF2` writer - channel transfer complete flag
pub type Tcif2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HTIF2` reader - channel half transfer flag
pub type Htif2R = crate::BitReader;
///Field `HTIF2` writer - channel half transfer flag
pub type Htif2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEIF2` reader - channel transfer error flag
pub type Teif2R = crate::BitReader;
///Field `TEIF2` writer - channel transfer error flag
pub type Teif2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GIF3` reader - channel global interrupt flag
pub type Gif3R = crate::BitReader;
///Field `GIF3` writer - channel global interrupt flag
pub type Gif3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIF3` reader - channel transfer complete flag
pub type Tcif3R = crate::BitReader;
///Field `TCIF3` writer - channel transfer complete flag
pub type Tcif3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HTIF3` reader - channel half transfer flag
pub type Htif3R = crate::BitReader;
///Field `HTIF3` writer - channel half transfer flag
pub type Htif3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEIF3` reader - channel transfer error flag
pub type Teif3R = crate::BitReader;
///Field `TEIF3` writer - channel transfer error flag
pub type Teif3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GIF4` reader - channel global interrupt flag
pub type Gif4R = crate::BitReader;
///Field `GIF4` writer - channel global interrupt flag
pub type Gif4W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIF4` reader - channel transfer complete flag
pub type Tcif4R = crate::BitReader;
///Field `TCIF4` writer - channel transfer complete flag
pub type Tcif4W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HTIF4` reader - channel half transfer flag
pub type Htif4R = crate::BitReader;
///Field `HTIF4` writer - channel half transfer flag
pub type Htif4W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEIF4` reader - channel transfer error flag
pub type Teif4R = crate::BitReader;
///Field `TEIF4` writer - channel transfer error flag
pub type Teif4W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GIF5` reader - channel global interrupt flag
pub type Gif5R = crate::BitReader;
///Field `GIF5` writer - channel global interrupt flag
pub type Gif5W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIF5` reader - channel transfer complete flag
pub type Tcif5R = crate::BitReader;
///Field `TCIF5` writer - channel transfer complete flag
pub type Tcif5W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HTIF5` reader - channel half transfer flag
pub type Htif5R = crate::BitReader;
///Field `HTIF5` writer - channel half transfer flag
pub type Htif5W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEIF5` reader - channel transfer error flag
pub type Teif5R = crate::BitReader;
///Field `TEIF5` writer - channel transfer error flag
pub type Teif5W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GIF6` reader - channel global interrupt flag
pub type Gif6R = crate::BitReader;
///Field `GIF6` writer - channel global interrupt flag
pub type Gif6W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIF6` reader - channel transfer complete flag
pub type Tcif6R = crate::BitReader;
///Field `TCIF6` writer - channel transfer complete flag
pub type Tcif6W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HTIF6` reader - channel half transfer flag
pub type Htif6R = crate::BitReader;
///Field `HTIF6` writer - channel half transfer flag
pub type Htif6W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEIF6` reader - channel transfer error flag
pub type Teif6R = crate::BitReader;
///Field `TEIF6` writer - channel transfer error flag
pub type Teif6W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GIF7` reader - channel global interrupt flag
pub type Gif7R = crate::BitReader;
///Field `GIF7` writer - channel global interrupt flag
pub type Gif7W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIF7` reader - channel transfer complete flag
pub type Tcif7R = crate::BitReader;
///Field `TCIF7` writer - channel transfer complete flag
pub type Tcif7W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HTIF7` reader - channel half transfer flag
pub type Htif7R = crate::BitReader;
///Field `HTIF7` writer - channel half transfer flag
pub type Htif7W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEIF7` reader - channel transfer error flag
pub type Teif7R = crate::BitReader;
///Field `TEIF7` writer - channel transfer error flag
pub type Teif7W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GIF8` reader - channel global interrupt flag
pub type Gif8R = crate::BitReader;
///Field `GIF8` writer - channel global interrupt flag
pub type Gif8W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIF8` reader - channel transfer complete flag
pub type Tcif8R = crate::BitReader;
///Field `TCIF8` writer - channel transfer complete flag
pub type Tcif8W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HTIF8` reader - channel half transfer flag
pub type Htif8R = crate::BitReader;
///Field `HTIF8` writer - channel half transfer flag
pub type Htif8W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEIF8` reader - channel transfer error flag
pub type Teif8R = crate::BitReader;
///Field `TEIF8` writer - channel transfer error flag
pub type Teif8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - channel global interrupt flag. Set when any of TEIF/HTIF/TCIF asserted. Cleared when TEIF/HTIF/TCIF all cleared.
    #[inline(always)]
    pub fn gif1(&self) -> Gif1R {
        Gif1R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - channel transfer complete flag. Set when all NDT are transferred. Cleared when write 1 to CTCIF or CGIF.
    #[inline(always)]
    pub fn tcif1(&self) -> Tcif1R {
        Tcif1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - channel half transfer flag. Set when half NDT are transferred. Cleared when write 1 to CHTIF or CGIF.
    #[inline(always)]
    pub fn htif1(&self) -> Htif1R {
        Htif1R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - channel transfer error flag. Set when bus error detected. Cleared when write 1 to CTEIF or CGIF.
    #[inline(always)]
    pub fn teif1(&self) -> Teif1R {
        Teif1R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - channel global interrupt flag
    #[inline(always)]
    pub fn gif2(&self) -> Gif2R {
        Gif2R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - channel transfer complete flag
    #[inline(always)]
    pub fn tcif2(&self) -> Tcif2R {
        Tcif2R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - channel half transfer flag
    #[inline(always)]
    pub fn htif2(&self) -> Htif2R {
        Htif2R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - channel transfer error flag
    #[inline(always)]
    pub fn teif2(&self) -> Teif2R {
        Teif2R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - channel global interrupt flag
    #[inline(always)]
    pub fn gif3(&self) -> Gif3R {
        Gif3R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - channel transfer complete flag
    #[inline(always)]
    pub fn tcif3(&self) -> Tcif3R {
        Tcif3R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - channel half transfer flag
    #[inline(always)]
    pub fn htif3(&self) -> Htif3R {
        Htif3R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - channel transfer error flag
    #[inline(always)]
    pub fn teif3(&self) -> Teif3R {
        Teif3R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - channel global interrupt flag
    #[inline(always)]
    pub fn gif4(&self) -> Gif4R {
        Gif4R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - channel transfer complete flag
    #[inline(always)]
    pub fn tcif4(&self) -> Tcif4R {
        Tcif4R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - channel half transfer flag
    #[inline(always)]
    pub fn htif4(&self) -> Htif4R {
        Htif4R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - channel transfer error flag
    #[inline(always)]
    pub fn teif4(&self) -> Teif4R {
        Teif4R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - channel global interrupt flag
    #[inline(always)]
    pub fn gif5(&self) -> Gif5R {
        Gif5R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - channel transfer complete flag
    #[inline(always)]
    pub fn tcif5(&self) -> Tcif5R {
        Tcif5R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - channel half transfer flag
    #[inline(always)]
    pub fn htif5(&self) -> Htif5R {
        Htif5R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - channel transfer error flag
    #[inline(always)]
    pub fn teif5(&self) -> Teif5R {
        Teif5R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - channel global interrupt flag
    #[inline(always)]
    pub fn gif6(&self) -> Gif6R {
        Gif6R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - channel transfer complete flag
    #[inline(always)]
    pub fn tcif6(&self) -> Tcif6R {
        Tcif6R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - channel half transfer flag
    #[inline(always)]
    pub fn htif6(&self) -> Htif6R {
        Htif6R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - channel transfer error flag
    #[inline(always)]
    pub fn teif6(&self) -> Teif6R {
        Teif6R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - channel global interrupt flag
    #[inline(always)]
    pub fn gif7(&self) -> Gif7R {
        Gif7R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - channel transfer complete flag
    #[inline(always)]
    pub fn tcif7(&self) -> Tcif7R {
        Tcif7R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - channel half transfer flag
    #[inline(always)]
    pub fn htif7(&self) -> Htif7R {
        Htif7R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - channel transfer error flag
    #[inline(always)]
    pub fn teif7(&self) -> Teif7R {
        Teif7R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - channel global interrupt flag
    #[inline(always)]
    pub fn gif8(&self) -> Gif8R {
        Gif8R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - channel transfer complete flag
    #[inline(always)]
    pub fn tcif8(&self) -> Tcif8R {
        Tcif8R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - channel half transfer flag
    #[inline(always)]
    pub fn htif8(&self) -> Htif8R {
        Htif8R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - channel transfer error flag
    #[inline(always)]
    pub fn teif8(&self) -> Teif8R {
        Teif8R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("teif8", &self.teif8())
            .field("htif8", &self.htif8())
            .field("tcif8", &self.tcif8())
            .field("gif8", &self.gif8())
            .field("teif7", &self.teif7())
            .field("htif7", &self.htif7())
            .field("tcif7", &self.tcif7())
            .field("gif7", &self.gif7())
            .field("teif6", &self.teif6())
            .field("htif6", &self.htif6())
            .field("tcif6", &self.tcif6())
            .field("gif6", &self.gif6())
            .field("teif5", &self.teif5())
            .field("htif5", &self.htif5())
            .field("tcif5", &self.tcif5())
            .field("gif5", &self.gif5())
            .field("teif4", &self.teif4())
            .field("htif4", &self.htif4())
            .field("tcif4", &self.tcif4())
            .field("gif4", &self.gif4())
            .field("teif3", &self.teif3())
            .field("htif3", &self.htif3())
            .field("tcif3", &self.tcif3())
            .field("gif3", &self.gif3())
            .field("teif2", &self.teif2())
            .field("htif2", &self.htif2())
            .field("tcif2", &self.tcif2())
            .field("gif2", &self.gif2())
            .field("teif1", &self.teif1())
            .field("htif1", &self.htif1())
            .field("tcif1", &self.tcif1())
            .field("gif1", &self.gif1())
            .finish()
    }
}
impl W {
    ///Bit 0 - channel global interrupt flag. Set when any of TEIF/HTIF/TCIF asserted. Cleared when TEIF/HTIF/TCIF all cleared.
    #[inline(always)]
    pub fn gif1(&mut self) -> Gif1W<ISRrs> {
        Gif1W::new(self, 0)
    }
    ///Bit 1 - channel transfer complete flag. Set when all NDT are transferred. Cleared when write 1 to CTCIF or CGIF.
    #[inline(always)]
    pub fn tcif1(&mut self) -> Tcif1W<ISRrs> {
        Tcif1W::new(self, 1)
    }
    ///Bit 2 - channel half transfer flag. Set when half NDT are transferred. Cleared when write 1 to CHTIF or CGIF.
    #[inline(always)]
    pub fn htif1(&mut self) -> Htif1W<ISRrs> {
        Htif1W::new(self, 2)
    }
    ///Bit 3 - channel transfer error flag. Set when bus error detected. Cleared when write 1 to CTEIF or CGIF.
    #[inline(always)]
    pub fn teif1(&mut self) -> Teif1W<ISRrs> {
        Teif1W::new(self, 3)
    }
    ///Bit 4 - channel global interrupt flag
    #[inline(always)]
    pub fn gif2(&mut self) -> Gif2W<ISRrs> {
        Gif2W::new(self, 4)
    }
    ///Bit 5 - channel transfer complete flag
    #[inline(always)]
    pub fn tcif2(&mut self) -> Tcif2W<ISRrs> {
        Tcif2W::new(self, 5)
    }
    ///Bit 6 - channel half transfer flag
    #[inline(always)]
    pub fn htif2(&mut self) -> Htif2W<ISRrs> {
        Htif2W::new(self, 6)
    }
    ///Bit 7 - channel transfer error flag
    #[inline(always)]
    pub fn teif2(&mut self) -> Teif2W<ISRrs> {
        Teif2W::new(self, 7)
    }
    ///Bit 8 - channel global interrupt flag
    #[inline(always)]
    pub fn gif3(&mut self) -> Gif3W<ISRrs> {
        Gif3W::new(self, 8)
    }
    ///Bit 9 - channel transfer complete flag
    #[inline(always)]
    pub fn tcif3(&mut self) -> Tcif3W<ISRrs> {
        Tcif3W::new(self, 9)
    }
    ///Bit 10 - channel half transfer flag
    #[inline(always)]
    pub fn htif3(&mut self) -> Htif3W<ISRrs> {
        Htif3W::new(self, 10)
    }
    ///Bit 11 - channel transfer error flag
    #[inline(always)]
    pub fn teif3(&mut self) -> Teif3W<ISRrs> {
        Teif3W::new(self, 11)
    }
    ///Bit 12 - channel global interrupt flag
    #[inline(always)]
    pub fn gif4(&mut self) -> Gif4W<ISRrs> {
        Gif4W::new(self, 12)
    }
    ///Bit 13 - channel transfer complete flag
    #[inline(always)]
    pub fn tcif4(&mut self) -> Tcif4W<ISRrs> {
        Tcif4W::new(self, 13)
    }
    ///Bit 14 - channel half transfer flag
    #[inline(always)]
    pub fn htif4(&mut self) -> Htif4W<ISRrs> {
        Htif4W::new(self, 14)
    }
    ///Bit 15 - channel transfer error flag
    #[inline(always)]
    pub fn teif4(&mut self) -> Teif4W<ISRrs> {
        Teif4W::new(self, 15)
    }
    ///Bit 16 - channel global interrupt flag
    #[inline(always)]
    pub fn gif5(&mut self) -> Gif5W<ISRrs> {
        Gif5W::new(self, 16)
    }
    ///Bit 17 - channel transfer complete flag
    #[inline(always)]
    pub fn tcif5(&mut self) -> Tcif5W<ISRrs> {
        Tcif5W::new(self, 17)
    }
    ///Bit 18 - channel half transfer flag
    #[inline(always)]
    pub fn htif5(&mut self) -> Htif5W<ISRrs> {
        Htif5W::new(self, 18)
    }
    ///Bit 19 - channel transfer error flag
    #[inline(always)]
    pub fn teif5(&mut self) -> Teif5W<ISRrs> {
        Teif5W::new(self, 19)
    }
    ///Bit 20 - channel global interrupt flag
    #[inline(always)]
    pub fn gif6(&mut self) -> Gif6W<ISRrs> {
        Gif6W::new(self, 20)
    }
    ///Bit 21 - channel transfer complete flag
    #[inline(always)]
    pub fn tcif6(&mut self) -> Tcif6W<ISRrs> {
        Tcif6W::new(self, 21)
    }
    ///Bit 22 - channel half transfer flag
    #[inline(always)]
    pub fn htif6(&mut self) -> Htif6W<ISRrs> {
        Htif6W::new(self, 22)
    }
    ///Bit 23 - channel transfer error flag
    #[inline(always)]
    pub fn teif6(&mut self) -> Teif6W<ISRrs> {
        Teif6W::new(self, 23)
    }
    ///Bit 24 - channel global interrupt flag
    #[inline(always)]
    pub fn gif7(&mut self) -> Gif7W<ISRrs> {
        Gif7W::new(self, 24)
    }
    ///Bit 25 - channel transfer complete flag
    #[inline(always)]
    pub fn tcif7(&mut self) -> Tcif7W<ISRrs> {
        Tcif7W::new(self, 25)
    }
    ///Bit 26 - channel half transfer flag
    #[inline(always)]
    pub fn htif7(&mut self) -> Htif7W<ISRrs> {
        Htif7W::new(self, 26)
    }
    ///Bit 27 - channel transfer error flag
    #[inline(always)]
    pub fn teif7(&mut self) -> Teif7W<ISRrs> {
        Teif7W::new(self, 27)
    }
    ///Bit 28 - channel global interrupt flag
    #[inline(always)]
    pub fn gif8(&mut self) -> Gif8W<ISRrs> {
        Gif8W::new(self, 28)
    }
    ///Bit 29 - channel transfer complete flag
    #[inline(always)]
    pub fn tcif8(&mut self) -> Tcif8W<ISRrs> {
        Tcif8W::new(self, 29)
    }
    ///Bit 30 - channel half transfer flag
    #[inline(always)]
    pub fn htif8(&mut self) -> Htif8W<ISRrs> {
        Htif8W::new(self, 30)
    }
    ///Bit 31 - channel transfer error flag
    #[inline(always)]
    pub fn teif8(&mut self) -> Teif8W<ISRrs> {
        Teif8W::new(self, 31)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`write(|w| ..)` method takes [`isr::W`](W) writer structure
impl crate::Writable for ISRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0;
}
