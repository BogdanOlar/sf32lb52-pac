///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
///Field `TCIE1` reader - enable task complete interrupt for task 1
pub type Tcie1R = crate::BitReader;
///Field `TCIE1` writer - enable task complete interrupt for task 1
pub type Tcie1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIE2` reader - enable task complete interrupt for task 2
pub type Tcie2R = crate::BitReader;
///Field `TCIE2` writer - enable task complete interrupt for task 2
pub type Tcie2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIE3` reader - enable task complete interrupt for task 3
pub type Tcie3R = crate::BitReader;
///Field `TCIE3` writer - enable task complete interrupt for task 3
pub type Tcie3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIE4` reader - enable task complete interrupt for task 4
pub type Tcie4R = crate::BitReader;
///Field `TCIE4` writer - enable task complete interrupt for task 4
pub type Tcie4W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIE5` reader - enable task complete interrupt for task 5
pub type Tcie5R = crate::BitReader;
///Field `TCIE5` writer - enable task complete interrupt for task 5
pub type Tcie5W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIE6` reader - enable task complete interrupt for task 6
pub type Tcie6R = crate::BitReader;
///Field `TCIE6` writer - enable task complete interrupt for task 6
pub type Tcie6W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIE7` reader - enable task complete interrupt for task 7
pub type Tcie7R = crate::BitReader;
///Field `TCIE7` writer - enable task complete interrupt for task 7
pub type Tcie7W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIE8` reader - enable task complete interrupt for task 8
pub type Tcie8R = crate::BitReader;
///Field `TCIE8` writer - enable task complete interrupt for task 8
pub type Tcie8W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEIE` reader - enable transfer error flag
pub type TeieR = crate::BitReader;
///Field `TEIE` writer - enable transfer error flag
pub type TeieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - enable task complete interrupt for task 1
    #[inline(always)]
    pub fn tcie1(&self) -> Tcie1R {
        Tcie1R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - enable task complete interrupt for task 2
    #[inline(always)]
    pub fn tcie2(&self) -> Tcie2R {
        Tcie2R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - enable task complete interrupt for task 3
    #[inline(always)]
    pub fn tcie3(&self) -> Tcie3R {
        Tcie3R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - enable task complete interrupt for task 4
    #[inline(always)]
    pub fn tcie4(&self) -> Tcie4R {
        Tcie4R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - enable task complete interrupt for task 5
    #[inline(always)]
    pub fn tcie5(&self) -> Tcie5R {
        Tcie5R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - enable task complete interrupt for task 6
    #[inline(always)]
    pub fn tcie6(&self) -> Tcie6R {
        Tcie6R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - enable task complete interrupt for task 7
    #[inline(always)]
    pub fn tcie7(&self) -> Tcie7R {
        Tcie7R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - enable task complete interrupt for task 8
    #[inline(always)]
    pub fn tcie8(&self) -> Tcie8R {
        Tcie8R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 16 - enable transfer error flag
    #[inline(always)]
    pub fn teie(&self) -> TeieR {
        TeieR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("teie", &self.teie())
            .field("tcie8", &self.tcie8())
            .field("tcie7", &self.tcie7())
            .field("tcie6", &self.tcie6())
            .field("tcie5", &self.tcie5())
            .field("tcie4", &self.tcie4())
            .field("tcie3", &self.tcie3())
            .field("tcie2", &self.tcie2())
            .field("tcie1", &self.tcie1())
            .finish()
    }
}
impl W {
    ///Bit 0 - enable task complete interrupt for task 1
    #[inline(always)]
    pub fn tcie1(&mut self) -> Tcie1W<IERrs> {
        Tcie1W::new(self, 0)
    }
    ///Bit 1 - enable task complete interrupt for task 2
    #[inline(always)]
    pub fn tcie2(&mut self) -> Tcie2W<IERrs> {
        Tcie2W::new(self, 1)
    }
    ///Bit 2 - enable task complete interrupt for task 3
    #[inline(always)]
    pub fn tcie3(&mut self) -> Tcie3W<IERrs> {
        Tcie3W::new(self, 2)
    }
    ///Bit 3 - enable task complete interrupt for task 4
    #[inline(always)]
    pub fn tcie4(&mut self) -> Tcie4W<IERrs> {
        Tcie4W::new(self, 3)
    }
    ///Bit 4 - enable task complete interrupt for task 5
    #[inline(always)]
    pub fn tcie5(&mut self) -> Tcie5W<IERrs> {
        Tcie5W::new(self, 4)
    }
    ///Bit 5 - enable task complete interrupt for task 6
    #[inline(always)]
    pub fn tcie6(&mut self) -> Tcie6W<IERrs> {
        Tcie6W::new(self, 5)
    }
    ///Bit 6 - enable task complete interrupt for task 7
    #[inline(always)]
    pub fn tcie7(&mut self) -> Tcie7W<IERrs> {
        Tcie7W::new(self, 6)
    }
    ///Bit 7 - enable task complete interrupt for task 8
    #[inline(always)]
    pub fn tcie8(&mut self) -> Tcie8W<IERrs> {
        Tcie8W::new(self, 7)
    }
    ///Bit 16 - enable transfer error flag
    #[inline(always)]
    pub fn teie(&mut self) -> TeieW<IERrs> {
        TeieW::new(self, 16)
    }
}
///interrupt enable register
///
///You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
///`read()` method returns [`ier::R`](R) reader structure
impl crate::Readable for IERrs {}
///`write(|w| ..)` method takes [`ier::W`](W) writer structure
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IERrs {
    const RESET_VALUE: u32 = 0;
}
