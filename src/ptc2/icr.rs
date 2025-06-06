///Register `ICR` reader
pub type R = crate::R<ICRrs>;
///Register `ICR` writer
pub type W = crate::W<ICRrs>;
///Field `CTCIF1` reader - clear task complete interrupt flag for task 1
pub type Ctcif1R = crate::BitReader;
///Field `CTCIF1` writer - clear task complete interrupt flag for task 1
pub type Ctcif1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF2` reader - clear task complete interrupt flag for task 2
pub type Ctcif2R = crate::BitReader;
///Field `CTCIF2` writer - clear task complete interrupt flag for task 2
pub type Ctcif2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF3` reader - clear task complete interrupt flag for task 3
pub type Ctcif3R = crate::BitReader;
///Field `CTCIF3` writer - clear task complete interrupt flag for task 3
pub type Ctcif3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF4` reader - clear task complete interrupt flag for task 4
pub type Ctcif4R = crate::BitReader;
///Field `CTCIF4` writer - clear task complete interrupt flag for task 4
pub type Ctcif4W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF5` reader - clear task complete interrupt flag for task 5
pub type Ctcif5R = crate::BitReader;
///Field `CTCIF5` writer - clear task complete interrupt flag for task 5
pub type Ctcif5W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF6` reader - clear task complete interrupt flag for task 6
pub type Ctcif6R = crate::BitReader;
///Field `CTCIF6` writer - clear task complete interrupt flag for task 6
pub type Ctcif6W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF7` reader - clear task complete interrupt flag for task 7
pub type Ctcif7R = crate::BitReader;
///Field `CTCIF7` writer - clear task complete interrupt flag for task 7
pub type Ctcif7W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTCIF8` reader - clear task complete interrupt flag for task 8
pub type Ctcif8R = crate::BitReader;
///Field `CTCIF8` writer - clear task complete interrupt flag for task 8
pub type Ctcif8W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTEIF` reader - clear transfer error flag
pub type CteifR = crate::BitReader;
///Field `CTEIF` writer - clear transfer error flag
pub type CteifW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - clear task complete interrupt flag for task 1
    #[inline(always)]
    pub fn ctcif1(&self) -> Ctcif1R {
        Ctcif1R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - clear task complete interrupt flag for task 2
    #[inline(always)]
    pub fn ctcif2(&self) -> Ctcif2R {
        Ctcif2R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - clear task complete interrupt flag for task 3
    #[inline(always)]
    pub fn ctcif3(&self) -> Ctcif3R {
        Ctcif3R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - clear task complete interrupt flag for task 4
    #[inline(always)]
    pub fn ctcif4(&self) -> Ctcif4R {
        Ctcif4R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - clear task complete interrupt flag for task 5
    #[inline(always)]
    pub fn ctcif5(&self) -> Ctcif5R {
        Ctcif5R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - clear task complete interrupt flag for task 6
    #[inline(always)]
    pub fn ctcif6(&self) -> Ctcif6R {
        Ctcif6R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - clear task complete interrupt flag for task 7
    #[inline(always)]
    pub fn ctcif7(&self) -> Ctcif7R {
        Ctcif7R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - clear task complete interrupt flag for task 8
    #[inline(always)]
    pub fn ctcif8(&self) -> Ctcif8R {
        Ctcif8R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 16 - clear transfer error flag
    #[inline(always)]
    pub fn cteif(&self) -> CteifR {
        CteifR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICR")
            .field("cteif", &self.cteif())
            .field("ctcif8", &self.ctcif8())
            .field("ctcif7", &self.ctcif7())
            .field("ctcif6", &self.ctcif6())
            .field("ctcif5", &self.ctcif5())
            .field("ctcif4", &self.ctcif4())
            .field("ctcif3", &self.ctcif3())
            .field("ctcif2", &self.ctcif2())
            .field("ctcif1", &self.ctcif1())
            .finish()
    }
}
impl W {
    ///Bit 0 - clear task complete interrupt flag for task 1
    #[inline(always)]
    pub fn ctcif1(&mut self) -> Ctcif1W<ICRrs> {
        Ctcif1W::new(self, 0)
    }
    ///Bit 1 - clear task complete interrupt flag for task 2
    #[inline(always)]
    pub fn ctcif2(&mut self) -> Ctcif2W<ICRrs> {
        Ctcif2W::new(self, 1)
    }
    ///Bit 2 - clear task complete interrupt flag for task 3
    #[inline(always)]
    pub fn ctcif3(&mut self) -> Ctcif3W<ICRrs> {
        Ctcif3W::new(self, 2)
    }
    ///Bit 3 - clear task complete interrupt flag for task 4
    #[inline(always)]
    pub fn ctcif4(&mut self) -> Ctcif4W<ICRrs> {
        Ctcif4W::new(self, 3)
    }
    ///Bit 4 - clear task complete interrupt flag for task 5
    #[inline(always)]
    pub fn ctcif5(&mut self) -> Ctcif5W<ICRrs> {
        Ctcif5W::new(self, 4)
    }
    ///Bit 5 - clear task complete interrupt flag for task 6
    #[inline(always)]
    pub fn ctcif6(&mut self) -> Ctcif6W<ICRrs> {
        Ctcif6W::new(self, 5)
    }
    ///Bit 6 - clear task complete interrupt flag for task 7
    #[inline(always)]
    pub fn ctcif7(&mut self) -> Ctcif7W<ICRrs> {
        Ctcif7W::new(self, 6)
    }
    ///Bit 7 - clear task complete interrupt flag for task 8
    #[inline(always)]
    pub fn ctcif8(&mut self) -> Ctcif8W<ICRrs> {
        Ctcif8W::new(self, 7)
    }
    ///Bit 16 - clear transfer error flag
    #[inline(always)]
    pub fn cteif(&mut self) -> CteifW<ICRrs> {
        CteifW::new(self, 16)
    }
}
///interrupt clear register
///
///You can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`read()` method returns [`icr::R`](R) reader structure
impl crate::Readable for ICRrs {}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {
    const RESET_VALUE: u32 = 0;
}
