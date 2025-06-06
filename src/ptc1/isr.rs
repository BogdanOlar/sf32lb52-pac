///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Register `ISR` writer
pub type W = crate::W<ISRrs>;
///Field `TCIF1` reader - task complete interrupt flag for task 1
pub type Tcif1R = crate::BitReader;
///Field `TCIF1` writer - task complete interrupt flag for task 1
pub type Tcif1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIF2` reader - task complete interrupt flag for task 2
pub type Tcif2R = crate::BitReader;
///Field `TCIF2` writer - task complete interrupt flag for task 2
pub type Tcif2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIF3` reader - task complete interrupt flag for task 3
pub type Tcif3R = crate::BitReader;
///Field `TCIF3` writer - task complete interrupt flag for task 3
pub type Tcif3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIF4` reader - task complete interrupt flag for task 4
pub type Tcif4R = crate::BitReader;
///Field `TCIF4` writer - task complete interrupt flag for task 4
pub type Tcif4W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIF5` reader - task complete interrupt flag for task 5
pub type Tcif5R = crate::BitReader;
///Field `TCIF5` writer - task complete interrupt flag for task 5
pub type Tcif5W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIF6` reader - task complete interrupt flag for task 6
pub type Tcif6R = crate::BitReader;
///Field `TCIF6` writer - task complete interrupt flag for task 6
pub type Tcif6W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIF7` reader - task complete interrupt flag for task 7
pub type Tcif7R = crate::BitReader;
///Field `TCIF7` writer - task complete interrupt flag for task 7
pub type Tcif7W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIF8` reader - task complete interrupt flag for task 8
pub type Tcif8R = crate::BitReader;
///Field `TCIF8` writer - task complete interrupt flag for task 8
pub type Tcif8W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TEIF1` reader - transfer error flag for task 1
pub type Teif1R = crate::BitReader;
///Field `TEIF1` writer - transfer error flag for task 1
pub type Teif1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEIF2` reader - transfer error flag for task 2
pub type Teif2R = crate::BitReader;
///Field `TEIF2` writer - transfer error flag for task 2
pub type Teif2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEIF3` reader - transfer error flag for task 3
pub type Teif3R = crate::BitReader;
///Field `TEIF3` writer - transfer error flag for task 3
pub type Teif3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEIF4` reader - transfer error flag for task 4
pub type Teif4R = crate::BitReader;
///Field `TEIF4` writer - transfer error flag for task 4
pub type Teif4W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEIF5` reader - transfer error flag for task 5
pub type Teif5R = crate::BitReader;
///Field `TEIF5` writer - transfer error flag for task 5
pub type Teif5W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEIF6` reader - transfer error flag for task 6
pub type Teif6R = crate::BitReader;
///Field `TEIF6` writer - transfer error flag for task 6
pub type Teif6W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEIF7` reader - transfer error flag for task 7
pub type Teif7R = crate::BitReader;
///Field `TEIF7` writer - transfer error flag for task 7
pub type Teif7W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEIF8` reader - transfer error flag for task 8
pub type Teif8R = crate::BitReader;
///Field `TEIF8` writer - transfer error flag for task 8
pub type Teif8W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0 - task complete interrupt flag for task 1
    #[inline(always)]
    pub fn tcif1(&self) -> Tcif1R {
        Tcif1R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - task complete interrupt flag for task 2
    #[inline(always)]
    pub fn tcif2(&self) -> Tcif2R {
        Tcif2R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - task complete interrupt flag for task 3
    #[inline(always)]
    pub fn tcif3(&self) -> Tcif3R {
        Tcif3R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - task complete interrupt flag for task 4
    #[inline(always)]
    pub fn tcif4(&self) -> Tcif4R {
        Tcif4R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - task complete interrupt flag for task 5
    #[inline(always)]
    pub fn tcif5(&self) -> Tcif5R {
        Tcif5R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - task complete interrupt flag for task 6
    #[inline(always)]
    pub fn tcif6(&self) -> Tcif6R {
        Tcif6R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - task complete interrupt flag for task 7
    #[inline(always)]
    pub fn tcif7(&self) -> Tcif7R {
        Tcif7R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - task complete interrupt flag for task 8
    #[inline(always)]
    pub fn tcif8(&self) -> Tcif8R {
        Tcif8R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:15
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 16 - transfer error flag for task 1
    #[inline(always)]
    pub fn teif1(&self) -> Teif1R {
        Teif1R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - transfer error flag for task 2
    #[inline(always)]
    pub fn teif2(&self) -> Teif2R {
        Teif2R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - transfer error flag for task 3
    #[inline(always)]
    pub fn teif3(&self) -> Teif3R {
        Teif3R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - transfer error flag for task 4
    #[inline(always)]
    pub fn teif4(&self) -> Teif4R {
        Teif4R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - transfer error flag for task 5
    #[inline(always)]
    pub fn teif5(&self) -> Teif5R {
        Teif5R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - transfer error flag for task 6
    #[inline(always)]
    pub fn teif6(&self) -> Teif6R {
        Teif6R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - transfer error flag for task 7
    #[inline(always)]
    pub fn teif7(&self) -> Teif7R {
        Teif7R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - transfer error flag for task 8
    #[inline(always)]
    pub fn teif8(&self) -> Teif8R {
        Teif8R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("rsvd", &self.rsvd())
            .field("teif8", &self.teif8())
            .field("teif7", &self.teif7())
            .field("teif6", &self.teif6())
            .field("teif5", &self.teif5())
            .field("teif4", &self.teif4())
            .field("teif3", &self.teif3())
            .field("teif2", &self.teif2())
            .field("teif1", &self.teif1())
            .field("rsvd2", &self.rsvd2())
            .field("tcif8", &self.tcif8())
            .field("tcif7", &self.tcif7())
            .field("tcif6", &self.tcif6())
            .field("tcif5", &self.tcif5())
            .field("tcif4", &self.tcif4())
            .field("tcif3", &self.tcif3())
            .field("tcif2", &self.tcif2())
            .field("tcif1", &self.tcif1())
            .finish()
    }
}
impl W {
    ///Bit 0 - task complete interrupt flag for task 1
    #[inline(always)]
    pub fn tcif1(&mut self) -> Tcif1W<ISRrs> {
        Tcif1W::new(self, 0)
    }
    ///Bit 1 - task complete interrupt flag for task 2
    #[inline(always)]
    pub fn tcif2(&mut self) -> Tcif2W<ISRrs> {
        Tcif2W::new(self, 1)
    }
    ///Bit 2 - task complete interrupt flag for task 3
    #[inline(always)]
    pub fn tcif3(&mut self) -> Tcif3W<ISRrs> {
        Tcif3W::new(self, 2)
    }
    ///Bit 3 - task complete interrupt flag for task 4
    #[inline(always)]
    pub fn tcif4(&mut self) -> Tcif4W<ISRrs> {
        Tcif4W::new(self, 3)
    }
    ///Bit 4 - task complete interrupt flag for task 5
    #[inline(always)]
    pub fn tcif5(&mut self) -> Tcif5W<ISRrs> {
        Tcif5W::new(self, 4)
    }
    ///Bit 5 - task complete interrupt flag for task 6
    #[inline(always)]
    pub fn tcif6(&mut self) -> Tcif6W<ISRrs> {
        Tcif6W::new(self, 5)
    }
    ///Bit 6 - task complete interrupt flag for task 7
    #[inline(always)]
    pub fn tcif7(&mut self) -> Tcif7W<ISRrs> {
        Tcif7W::new(self, 6)
    }
    ///Bit 7 - task complete interrupt flag for task 8
    #[inline(always)]
    pub fn tcif8(&mut self) -> Tcif8W<ISRrs> {
        Tcif8W::new(self, 7)
    }
    ///Bits 8:15
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<ISRrs> {
        Rsvd2W::new(self, 8)
    }
    ///Bit 16 - transfer error flag for task 1
    #[inline(always)]
    pub fn teif1(&mut self) -> Teif1W<ISRrs> {
        Teif1W::new(self, 16)
    }
    ///Bit 17 - transfer error flag for task 2
    #[inline(always)]
    pub fn teif2(&mut self) -> Teif2W<ISRrs> {
        Teif2W::new(self, 17)
    }
    ///Bit 18 - transfer error flag for task 3
    #[inline(always)]
    pub fn teif3(&mut self) -> Teif3W<ISRrs> {
        Teif3W::new(self, 18)
    }
    ///Bit 19 - transfer error flag for task 4
    #[inline(always)]
    pub fn teif4(&mut self) -> Teif4W<ISRrs> {
        Teif4W::new(self, 19)
    }
    ///Bit 20 - transfer error flag for task 5
    #[inline(always)]
    pub fn teif5(&mut self) -> Teif5W<ISRrs> {
        Teif5W::new(self, 20)
    }
    ///Bit 21 - transfer error flag for task 6
    #[inline(always)]
    pub fn teif6(&mut self) -> Teif6W<ISRrs> {
        Teif6W::new(self, 21)
    }
    ///Bit 22 - transfer error flag for task 7
    #[inline(always)]
    pub fn teif7(&mut self) -> Teif7W<ISRrs> {
        Teif7W::new(self, 22)
    }
    ///Bit 23 - transfer error flag for task 8
    #[inline(always)]
    pub fn teif8(&mut self) -> Teif8W<ISRrs> {
        Teif8W::new(self, 23)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<ISRrs> {
        RsvdW::new(self, 24)
    }
}
///interrupt status register
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
