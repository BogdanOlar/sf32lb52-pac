///Register `IFCR` reader
pub type R = crate::R<IFCRrs>;
///Register `IFCR` writer
pub type W = crate::W<IFCRrs>;
///Field `CGIF1` reader - CGIF, global interrupt flag clear. Write 1 to clear all TEIF/HTIF/TCIF.
pub type Cgif1R = crate::BitReader;
///Field `CGIF1` writer - CGIF, global interrupt flag clear. Write 1 to clear all TEIF/HTIF/TCIF.
pub type Cgif1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF1` reader - CTCIF, transfer complete flag clear. Write 1 to clear TCIF.
pub type Ctcif1R = crate::BitReader;
///Field `CTCIF1` writer - CTCIF, transfer complete flag clear. Write 1 to clear TCIF.
pub type Ctcif1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF1` reader - CHTIF, half transfer flag clear. Write 1 to clear HTIF.
pub type Chtif1R = crate::BitReader;
///Field `CHTIF1` writer - CHTIF, half transfer flag clear. Write 1 to clear HTIF.
pub type Chtif1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF1` reader - CTEIF, transfer error flag clear. Write 1 to clear TEIF.
pub type Cteif1R = crate::BitReader;
///Field `CTEIF1` writer - CTEIF, transfer error flag clear. Write 1 to clear TEIF.
pub type Cteif1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CGIF2` reader - CGIF, global interrupt flag clear
pub type Cgif2R = crate::BitReader;
///Field `CGIF2` writer - CGIF, global interrupt flag clear
pub type Cgif2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF2` reader - CTCIF, transfer complete flag clear
pub type Ctcif2R = crate::BitReader;
///Field `CTCIF2` writer - CTCIF, transfer complete flag clear
pub type Ctcif2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF2` reader - CHTIF, half transfer flag clear
pub type Chtif2R = crate::BitReader;
///Field `CHTIF2` writer - CHTIF, half transfer flag clear
pub type Chtif2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF2` reader - CTEIF, transfer error flag clear
pub type Cteif2R = crate::BitReader;
///Field `CTEIF2` writer - CTEIF, transfer error flag clear
pub type Cteif2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CGIF3` reader - CGIF, global interrupt flag clear
pub type Cgif3R = crate::BitReader;
///Field `CGIF3` writer - CGIF, global interrupt flag clear
pub type Cgif3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF3` reader - CTCIF, transfer complete flag clear
pub type Ctcif3R = crate::BitReader;
///Field `CTCIF3` writer - CTCIF, transfer complete flag clear
pub type Ctcif3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF3` reader - CHTIF, half transfer flag clear
pub type Chtif3R = crate::BitReader;
///Field `CHTIF3` writer - CHTIF, half transfer flag clear
pub type Chtif3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF3` reader - CTEIF, transfer error flag clear
pub type Cteif3R = crate::BitReader;
///Field `CTEIF3` writer - CTEIF, transfer error flag clear
pub type Cteif3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CGIF4` reader - CGIF, global interrupt flag clear
pub type Cgif4R = crate::BitReader;
///Field `CGIF4` writer - CGIF, global interrupt flag clear
pub type Cgif4W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF4` reader - CTCIF, transfer complete flag clear
pub type Ctcif4R = crate::BitReader;
///Field `CTCIF4` writer - CTCIF, transfer complete flag clear
pub type Ctcif4W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF4` reader - CHTIF, half transfer flag clear
pub type Chtif4R = crate::BitReader;
///Field `CHTIF4` writer - CHTIF, half transfer flag clear
pub type Chtif4W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF4` reader - CTEIF, transfer error flag clear
pub type Cteif4R = crate::BitReader;
///Field `CTEIF4` writer - CTEIF, transfer error flag clear
pub type Cteif4W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CGIF5` reader - CGIF, global interrupt flag clear
pub type Cgif5R = crate::BitReader;
///Field `CGIF5` writer - CGIF, global interrupt flag clear
pub type Cgif5W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF5` reader - CTCIF, transfer complete flag clear
pub type Ctcif5R = crate::BitReader;
///Field `CTCIF5` writer - CTCIF, transfer complete flag clear
pub type Ctcif5W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF5` reader - CHTIF, half transfer flag clear
pub type Chtif5R = crate::BitReader;
///Field `CHTIF5` writer - CHTIF, half transfer flag clear
pub type Chtif5W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF5` reader - CTEIF, transfer error flag clear
pub type Cteif5R = crate::BitReader;
///Field `CTEIF5` writer - CTEIF, transfer error flag clear
pub type Cteif5W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CGIF6` reader - CGIF, global interrupt flag clear
pub type Cgif6R = crate::BitReader;
///Field `CGIF6` writer - CGIF, global interrupt flag clear
pub type Cgif6W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF6` reader - CTCIF, transfer complete flag clear
pub type Ctcif6R = crate::BitReader;
///Field `CTCIF6` writer - CTCIF, transfer complete flag clear
pub type Ctcif6W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF6` reader - CHTIF, half transfer flag clear
pub type Chtif6R = crate::BitReader;
///Field `CHTIF6` writer - CHTIF, half transfer flag clear
pub type Chtif6W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF6` reader - CTEIF, transfer error flag clear
pub type Cteif6R = crate::BitReader;
///Field `CTEIF6` writer - CTEIF, transfer error flag clear
pub type Cteif6W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CGIF7` reader - CGIF, global interrupt flag clear
pub type Cgif7R = crate::BitReader;
///Field `CGIF7` writer - CGIF, global interrupt flag clear
pub type Cgif7W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF7` reader - CTCIF, transfer complete flag clear
pub type Ctcif7R = crate::BitReader;
///Field `CTCIF7` writer - CTCIF, transfer complete flag clear
pub type Ctcif7W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF7` reader - CHTIF, half transfer flag clear
pub type Chtif7R = crate::BitReader;
///Field `CHTIF7` writer - CHTIF, half transfer flag clear
pub type Chtif7W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF7` reader - CTEIF, transfer error flag clear
pub type Cteif7R = crate::BitReader;
///Field `CTEIF7` writer - CTEIF, transfer error flag clear
pub type Cteif7W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CGIF8` reader - CGIF, global interrupt flag clear
pub type Cgif8R = crate::BitReader;
///Field `CGIF8` writer - CGIF, global interrupt flag clear
pub type Cgif8W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF8` reader - CTCIF, transfer complete flag clear
pub type Ctcif8R = crate::BitReader;
///Field `CTCIF8` writer - CTCIF, transfer complete flag clear
pub type Ctcif8W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHTIF8` reader - CHTIF, half transfer flag clear
pub type Chtif8R = crate::BitReader;
///Field `CHTIF8` writer - CHTIF, half transfer flag clear
pub type Chtif8W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF8` reader - CTEIF, transfer error flag clear
pub type Cteif8R = crate::BitReader;
///Field `CTEIF8` writer - CTEIF, transfer error flag clear
pub type Cteif8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CGIF, global interrupt flag clear. Write 1 to clear all TEIF/HTIF/TCIF.
    #[inline(always)]
    pub fn cgif1(&self) -> Cgif1R {
        Cgif1R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CTCIF, transfer complete flag clear. Write 1 to clear TCIF.
    #[inline(always)]
    pub fn ctcif1(&self) -> Ctcif1R {
        Ctcif1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CHTIF, half transfer flag clear. Write 1 to clear HTIF.
    #[inline(always)]
    pub fn chtif1(&self) -> Chtif1R {
        Chtif1R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CTEIF, transfer error flag clear. Write 1 to clear TEIF.
    #[inline(always)]
    pub fn cteif1(&self) -> Cteif1R {
        Cteif1R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CGIF, global interrupt flag clear
    #[inline(always)]
    pub fn cgif2(&self) -> Cgif2R {
        Cgif2R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CTCIF, transfer complete flag clear
    #[inline(always)]
    pub fn ctcif2(&self) -> Ctcif2R {
        Ctcif2R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CHTIF, half transfer flag clear
    #[inline(always)]
    pub fn chtif2(&self) -> Chtif2R {
        Chtif2R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CTEIF, transfer error flag clear
    #[inline(always)]
    pub fn cteif2(&self) -> Cteif2R {
        Cteif2R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - CGIF, global interrupt flag clear
    #[inline(always)]
    pub fn cgif3(&self) -> Cgif3R {
        Cgif3R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CTCIF, transfer complete flag clear
    #[inline(always)]
    pub fn ctcif3(&self) -> Ctcif3R {
        Ctcif3R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CHTIF, half transfer flag clear
    #[inline(always)]
    pub fn chtif3(&self) -> Chtif3R {
        Chtif3R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CTEIF, transfer error flag clear
    #[inline(always)]
    pub fn cteif3(&self) -> Cteif3R {
        Cteif3R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CGIF, global interrupt flag clear
    #[inline(always)]
    pub fn cgif4(&self) -> Cgif4R {
        Cgif4R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - CTCIF, transfer complete flag clear
    #[inline(always)]
    pub fn ctcif4(&self) -> Ctcif4R {
        Ctcif4R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - CHTIF, half transfer flag clear
    #[inline(always)]
    pub fn chtif4(&self) -> Chtif4R {
        Chtif4R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - CTEIF, transfer error flag clear
    #[inline(always)]
    pub fn cteif4(&self) -> Cteif4R {
        Cteif4R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - CGIF, global interrupt flag clear
    #[inline(always)]
    pub fn cgif5(&self) -> Cgif5R {
        Cgif5R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - CTCIF, transfer complete flag clear
    #[inline(always)]
    pub fn ctcif5(&self) -> Ctcif5R {
        Ctcif5R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - CHTIF, half transfer flag clear
    #[inline(always)]
    pub fn chtif5(&self) -> Chtif5R {
        Chtif5R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - CTEIF, transfer error flag clear
    #[inline(always)]
    pub fn cteif5(&self) -> Cteif5R {
        Cteif5R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - CGIF, global interrupt flag clear
    #[inline(always)]
    pub fn cgif6(&self) -> Cgif6R {
        Cgif6R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - CTCIF, transfer complete flag clear
    #[inline(always)]
    pub fn ctcif6(&self) -> Ctcif6R {
        Ctcif6R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - CHTIF, half transfer flag clear
    #[inline(always)]
    pub fn chtif6(&self) -> Chtif6R {
        Chtif6R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - CTEIF, transfer error flag clear
    #[inline(always)]
    pub fn cteif6(&self) -> Cteif6R {
        Cteif6R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - CGIF, global interrupt flag clear
    #[inline(always)]
    pub fn cgif7(&self) -> Cgif7R {
        Cgif7R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - CTCIF, transfer complete flag clear
    #[inline(always)]
    pub fn ctcif7(&self) -> Ctcif7R {
        Ctcif7R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - CHTIF, half transfer flag clear
    #[inline(always)]
    pub fn chtif7(&self) -> Chtif7R {
        Chtif7R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - CTEIF, transfer error flag clear
    #[inline(always)]
    pub fn cteif7(&self) -> Cteif7R {
        Cteif7R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - CGIF, global interrupt flag clear
    #[inline(always)]
    pub fn cgif8(&self) -> Cgif8R {
        Cgif8R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - CTCIF, transfer complete flag clear
    #[inline(always)]
    pub fn ctcif8(&self) -> Ctcif8R {
        Ctcif8R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - CHTIF, half transfer flag clear
    #[inline(always)]
    pub fn chtif8(&self) -> Chtif8R {
        Chtif8R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - CTEIF, transfer error flag clear
    #[inline(always)]
    pub fn cteif8(&self) -> Cteif8R {
        Cteif8R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IFCR")
            .field("cteif8", &self.cteif8())
            .field("chtif8", &self.chtif8())
            .field("ctcif8", &self.ctcif8())
            .field("cgif8", &self.cgif8())
            .field("cteif7", &self.cteif7())
            .field("chtif7", &self.chtif7())
            .field("ctcif7", &self.ctcif7())
            .field("cgif7", &self.cgif7())
            .field("cteif6", &self.cteif6())
            .field("chtif6", &self.chtif6())
            .field("ctcif6", &self.ctcif6())
            .field("cgif6", &self.cgif6())
            .field("cteif5", &self.cteif5())
            .field("chtif5", &self.chtif5())
            .field("ctcif5", &self.ctcif5())
            .field("cgif5", &self.cgif5())
            .field("cteif4", &self.cteif4())
            .field("chtif4", &self.chtif4())
            .field("ctcif4", &self.ctcif4())
            .field("cgif4", &self.cgif4())
            .field("cteif3", &self.cteif3())
            .field("chtif3", &self.chtif3())
            .field("ctcif3", &self.ctcif3())
            .field("cgif3", &self.cgif3())
            .field("cteif2", &self.cteif2())
            .field("chtif2", &self.chtif2())
            .field("ctcif2", &self.ctcif2())
            .field("cgif2", &self.cgif2())
            .field("cteif1", &self.cteif1())
            .field("chtif1", &self.chtif1())
            .field("ctcif1", &self.ctcif1())
            .field("cgif1", &self.cgif1())
            .finish()
    }
}
impl W {
    ///Bit 0 - CGIF, global interrupt flag clear. Write 1 to clear all TEIF/HTIF/TCIF.
    #[inline(always)]
    pub fn cgif1(&mut self) -> Cgif1W<IFCRrs> {
        Cgif1W::new(self, 0)
    }
    ///Bit 1 - CTCIF, transfer complete flag clear. Write 1 to clear TCIF.
    #[inline(always)]
    pub fn ctcif1(&mut self) -> Ctcif1W<IFCRrs> {
        Ctcif1W::new(self, 1)
    }
    ///Bit 2 - CHTIF, half transfer flag clear. Write 1 to clear HTIF.
    #[inline(always)]
    pub fn chtif1(&mut self) -> Chtif1W<IFCRrs> {
        Chtif1W::new(self, 2)
    }
    ///Bit 3 - CTEIF, transfer error flag clear. Write 1 to clear TEIF.
    #[inline(always)]
    pub fn cteif1(&mut self) -> Cteif1W<IFCRrs> {
        Cteif1W::new(self, 3)
    }
    ///Bit 4 - CGIF, global interrupt flag clear
    #[inline(always)]
    pub fn cgif2(&mut self) -> Cgif2W<IFCRrs> {
        Cgif2W::new(self, 4)
    }
    ///Bit 5 - CTCIF, transfer complete flag clear
    #[inline(always)]
    pub fn ctcif2(&mut self) -> Ctcif2W<IFCRrs> {
        Ctcif2W::new(self, 5)
    }
    ///Bit 6 - CHTIF, half transfer flag clear
    #[inline(always)]
    pub fn chtif2(&mut self) -> Chtif2W<IFCRrs> {
        Chtif2W::new(self, 6)
    }
    ///Bit 7 - CTEIF, transfer error flag clear
    #[inline(always)]
    pub fn cteif2(&mut self) -> Cteif2W<IFCRrs> {
        Cteif2W::new(self, 7)
    }
    ///Bit 8 - CGIF, global interrupt flag clear
    #[inline(always)]
    pub fn cgif3(&mut self) -> Cgif3W<IFCRrs> {
        Cgif3W::new(self, 8)
    }
    ///Bit 9 - CTCIF, transfer complete flag clear
    #[inline(always)]
    pub fn ctcif3(&mut self) -> Ctcif3W<IFCRrs> {
        Ctcif3W::new(self, 9)
    }
    ///Bit 10 - CHTIF, half transfer flag clear
    #[inline(always)]
    pub fn chtif3(&mut self) -> Chtif3W<IFCRrs> {
        Chtif3W::new(self, 10)
    }
    ///Bit 11 - CTEIF, transfer error flag clear
    #[inline(always)]
    pub fn cteif3(&mut self) -> Cteif3W<IFCRrs> {
        Cteif3W::new(self, 11)
    }
    ///Bit 12 - CGIF, global interrupt flag clear
    #[inline(always)]
    pub fn cgif4(&mut self) -> Cgif4W<IFCRrs> {
        Cgif4W::new(self, 12)
    }
    ///Bit 13 - CTCIF, transfer complete flag clear
    #[inline(always)]
    pub fn ctcif4(&mut self) -> Ctcif4W<IFCRrs> {
        Ctcif4W::new(self, 13)
    }
    ///Bit 14 - CHTIF, half transfer flag clear
    #[inline(always)]
    pub fn chtif4(&mut self) -> Chtif4W<IFCRrs> {
        Chtif4W::new(self, 14)
    }
    ///Bit 15 - CTEIF, transfer error flag clear
    #[inline(always)]
    pub fn cteif4(&mut self) -> Cteif4W<IFCRrs> {
        Cteif4W::new(self, 15)
    }
    ///Bit 16 - CGIF, global interrupt flag clear
    #[inline(always)]
    pub fn cgif5(&mut self) -> Cgif5W<IFCRrs> {
        Cgif5W::new(self, 16)
    }
    ///Bit 17 - CTCIF, transfer complete flag clear
    #[inline(always)]
    pub fn ctcif5(&mut self) -> Ctcif5W<IFCRrs> {
        Ctcif5W::new(self, 17)
    }
    ///Bit 18 - CHTIF, half transfer flag clear
    #[inline(always)]
    pub fn chtif5(&mut self) -> Chtif5W<IFCRrs> {
        Chtif5W::new(self, 18)
    }
    ///Bit 19 - CTEIF, transfer error flag clear
    #[inline(always)]
    pub fn cteif5(&mut self) -> Cteif5W<IFCRrs> {
        Cteif5W::new(self, 19)
    }
    ///Bit 20 - CGIF, global interrupt flag clear
    #[inline(always)]
    pub fn cgif6(&mut self) -> Cgif6W<IFCRrs> {
        Cgif6W::new(self, 20)
    }
    ///Bit 21 - CTCIF, transfer complete flag clear
    #[inline(always)]
    pub fn ctcif6(&mut self) -> Ctcif6W<IFCRrs> {
        Ctcif6W::new(self, 21)
    }
    ///Bit 22 - CHTIF, half transfer flag clear
    #[inline(always)]
    pub fn chtif6(&mut self) -> Chtif6W<IFCRrs> {
        Chtif6W::new(self, 22)
    }
    ///Bit 23 - CTEIF, transfer error flag clear
    #[inline(always)]
    pub fn cteif6(&mut self) -> Cteif6W<IFCRrs> {
        Cteif6W::new(self, 23)
    }
    ///Bit 24 - CGIF, global interrupt flag clear
    #[inline(always)]
    pub fn cgif7(&mut self) -> Cgif7W<IFCRrs> {
        Cgif7W::new(self, 24)
    }
    ///Bit 25 - CTCIF, transfer complete flag clear
    #[inline(always)]
    pub fn ctcif7(&mut self) -> Ctcif7W<IFCRrs> {
        Ctcif7W::new(self, 25)
    }
    ///Bit 26 - CHTIF, half transfer flag clear
    #[inline(always)]
    pub fn chtif7(&mut self) -> Chtif7W<IFCRrs> {
        Chtif7W::new(self, 26)
    }
    ///Bit 27 - CTEIF, transfer error flag clear
    #[inline(always)]
    pub fn cteif7(&mut self) -> Cteif7W<IFCRrs> {
        Cteif7W::new(self, 27)
    }
    ///Bit 28 - CGIF, global interrupt flag clear
    #[inline(always)]
    pub fn cgif8(&mut self) -> Cgif8W<IFCRrs> {
        Cgif8W::new(self, 28)
    }
    ///Bit 29 - CTCIF, transfer complete flag clear
    #[inline(always)]
    pub fn ctcif8(&mut self) -> Ctcif8W<IFCRrs> {
        Ctcif8W::new(self, 29)
    }
    ///Bit 30 - CHTIF, half transfer flag clear
    #[inline(always)]
    pub fn chtif8(&mut self) -> Chtif8W<IFCRrs> {
        Chtif8W::new(self, 30)
    }
    ///Bit 31 - CTEIF, transfer error flag clear
    #[inline(always)]
    pub fn cteif8(&mut self) -> Cteif8W<IFCRrs> {
        Cteif8W::new(self, 31)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`ifcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IFCRrs;
impl crate::RegisterSpec for IFCRrs {
    type Ux = u32;
}
///`read()` method returns [`ifcr::R`](R) reader structure
impl crate::Readable for IFCRrs {}
///`write(|w| ..)` method takes [`ifcr::W`](W) writer structure
impl crate::Writable for IFCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IFCR to value 0
impl crate::Resettable for IFCRrs {
    const RESET_VALUE: u32 = 0;
}
