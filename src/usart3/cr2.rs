///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `RSVD19` reader -
pub type Rsvd19R = crate::FieldReader;
///Field `RSVD19` writer -
pub type Rsvd19W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RSVD18` reader -
pub type Rsvd18R = crate::BitReader;
///Field `RSVD18` writer -
pub type Rsvd18W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD17` reader -
pub type Rsvd17R = crate::BitReader;
///Field `RSVD17` writer -
pub type Rsvd17W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD16` reader -
pub type Rsvd16R = crate::BitReader;
///Field `RSVD16` writer -
pub type Rsvd16W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD15` reader -
pub type Rsvd15R = crate::BitReader;
///Field `RSVD15` writer -
pub type Rsvd15W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD14` reader -
pub type Rsvd14R = crate::BitReader;
///Field `RSVD14` writer -
pub type Rsvd14W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD13` reader -
pub type Rsvd13R = crate::BitReader;
///Field `RSVD13` writer -
pub type Rsvd13W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD12` reader -
pub type Rsvd12R = crate::BitReader;
///Field `RSVD12` writer -
pub type Rsvd12W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD11` reader -
pub type Rsvd11R = crate::BitReader;
///Field `RSVD11` writer -
pub type Rsvd11W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOP` reader - Stop bits 0/1: 1 stop bit 2/3: 2 stop bits
pub type StopR = crate::FieldReader;
///Field `STOP` writer - Stop bits 0/1: 1 stop bit 2/3: 2 stop bits
pub type StopW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RSVD10` reader -
pub type Rsvd10R = crate::BitReader;
///Field `RSVD10` writer -
pub type Rsvd10W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD9` reader -
pub type Rsvd9R = crate::BitReader;
///Field `RSVD9` writer -
pub type Rsvd9W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD8` reader -
pub type Rsvd8R = crate::BitReader;
///Field `RSVD8` writer -
pub type Rsvd8W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD7` reader -
pub type Rsvd7R = crate::BitReader;
///Field `RSVD7` writer -
pub type Rsvd7W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD6` reader -
pub type Rsvd6R = crate::BitReader;
///Field `RSVD6` writer -
pub type Rsvd6W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD5` reader -
pub type Rsvd5R = crate::BitReader;
///Field `RSVD5` writer -
pub type Rsvd5W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD4` reader -
pub type Rsvd4R = crate::BitReader;
///Field `RSVD4` writer -
pub type Rsvd4W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD3` reader -
pub type Rsvd3R = crate::FieldReader;
///Field `RSVD3` writer -
pub type Rsvd3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::BitReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:3
    #[inline(always)]
    pub fn rsvd19(&self) -> Rsvd19R {
        Rsvd19R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4
    #[inline(always)]
    pub fn rsvd18(&self) -> Rsvd18R {
        Rsvd18R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5
    #[inline(always)]
    pub fn rsvd17(&self) -> Rsvd17R {
        Rsvd17R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6
    #[inline(always)]
    pub fn rsvd16(&self) -> Rsvd16R {
        Rsvd16R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7
    #[inline(always)]
    pub fn rsvd15(&self) -> Rsvd15R {
        Rsvd15R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8
    #[inline(always)]
    pub fn rsvd14(&self) -> Rsvd14R {
        Rsvd14R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9
    #[inline(always)]
    pub fn rsvd13(&self) -> Rsvd13R {
        Rsvd13R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10
    #[inline(always)]
    pub fn rsvd12(&self) -> Rsvd12R {
        Rsvd12R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11
    #[inline(always)]
    pub fn rsvd11(&self) -> Rsvd11R {
        Rsvd11R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - Stop bits 0/1: 1 stop bit 2/3: 2 stop bits
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14
    #[inline(always)]
    pub fn rsvd10(&self) -> Rsvd10R {
        Rsvd10R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15
    #[inline(always)]
    pub fn rsvd9(&self) -> Rsvd9R {
        Rsvd9R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16
    #[inline(always)]
    pub fn rsvd8(&self) -> Rsvd8R {
        Rsvd8R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17
    #[inline(always)]
    pub fn rsvd7(&self) -> Rsvd7R {
        Rsvd7R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18
    #[inline(always)]
    pub fn rsvd6(&self) -> Rsvd6R {
        Rsvd6R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19
    #[inline(always)]
    pub fn rsvd5(&self) -> Rsvd5R {
        Rsvd5R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20
    #[inline(always)]
    pub fn rsvd4(&self) -> Rsvd4R {
        Rsvd4R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22
    #[inline(always)]
    pub fn rsvd3(&self) -> Rsvd3R {
        Rsvd3R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bit 23
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("rsvd", &self.rsvd())
            .field("rsvd2", &self.rsvd2())
            .field("rsvd3", &self.rsvd3())
            .field("rsvd4", &self.rsvd4())
            .field("rsvd5", &self.rsvd5())
            .field("rsvd6", &self.rsvd6())
            .field("rsvd7", &self.rsvd7())
            .field("rsvd8", &self.rsvd8())
            .field("rsvd9", &self.rsvd9())
            .field("rsvd10", &self.rsvd10())
            .field("stop", &self.stop())
            .field("rsvd11", &self.rsvd11())
            .field("rsvd12", &self.rsvd12())
            .field("rsvd13", &self.rsvd13())
            .field("rsvd14", &self.rsvd14())
            .field("rsvd15", &self.rsvd15())
            .field("rsvd16", &self.rsvd16())
            .field("rsvd17", &self.rsvd17())
            .field("rsvd18", &self.rsvd18())
            .field("rsvd19", &self.rsvd19())
            .finish()
    }
}
impl W {
    ///Bits 0:3
    #[inline(always)]
    pub fn rsvd19(&mut self) -> Rsvd19W<CR2rs> {
        Rsvd19W::new(self, 0)
    }
    ///Bit 4
    #[inline(always)]
    pub fn rsvd18(&mut self) -> Rsvd18W<CR2rs> {
        Rsvd18W::new(self, 4)
    }
    ///Bit 5
    #[inline(always)]
    pub fn rsvd17(&mut self) -> Rsvd17W<CR2rs> {
        Rsvd17W::new(self, 5)
    }
    ///Bit 6
    #[inline(always)]
    pub fn rsvd16(&mut self) -> Rsvd16W<CR2rs> {
        Rsvd16W::new(self, 6)
    }
    ///Bit 7
    #[inline(always)]
    pub fn rsvd15(&mut self) -> Rsvd15W<CR2rs> {
        Rsvd15W::new(self, 7)
    }
    ///Bit 8
    #[inline(always)]
    pub fn rsvd14(&mut self) -> Rsvd14W<CR2rs> {
        Rsvd14W::new(self, 8)
    }
    ///Bit 9
    #[inline(always)]
    pub fn rsvd13(&mut self) -> Rsvd13W<CR2rs> {
        Rsvd13W::new(self, 9)
    }
    ///Bit 10
    #[inline(always)]
    pub fn rsvd12(&mut self) -> Rsvd12W<CR2rs> {
        Rsvd12W::new(self, 10)
    }
    ///Bit 11
    #[inline(always)]
    pub fn rsvd11(&mut self) -> Rsvd11W<CR2rs> {
        Rsvd11W::new(self, 11)
    }
    ///Bits 12:13 - Stop bits 0/1: 1 stop bit 2/3: 2 stop bits
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<CR2rs> {
        StopW::new(self, 12)
    }
    ///Bit 14
    #[inline(always)]
    pub fn rsvd10(&mut self) -> Rsvd10W<CR2rs> {
        Rsvd10W::new(self, 14)
    }
    ///Bit 15
    #[inline(always)]
    pub fn rsvd9(&mut self) -> Rsvd9W<CR2rs> {
        Rsvd9W::new(self, 15)
    }
    ///Bit 16
    #[inline(always)]
    pub fn rsvd8(&mut self) -> Rsvd8W<CR2rs> {
        Rsvd8W::new(self, 16)
    }
    ///Bit 17
    #[inline(always)]
    pub fn rsvd7(&mut self) -> Rsvd7W<CR2rs> {
        Rsvd7W::new(self, 17)
    }
    ///Bit 18
    #[inline(always)]
    pub fn rsvd6(&mut self) -> Rsvd6W<CR2rs> {
        Rsvd6W::new(self, 18)
    }
    ///Bit 19
    #[inline(always)]
    pub fn rsvd5(&mut self) -> Rsvd5W<CR2rs> {
        Rsvd5W::new(self, 19)
    }
    ///Bit 20
    #[inline(always)]
    pub fn rsvd4(&mut self) -> Rsvd4W<CR2rs> {
        Rsvd4W::new(self, 20)
    }
    ///Bits 21:22
    #[inline(always)]
    pub fn rsvd3(&mut self) -> Rsvd3W<CR2rs> {
        Rsvd3W::new(self, 21)
    }
    ///Bit 23
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<CR2rs> {
        Rsvd2W::new(self, 23)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<CR2rs> {
        RsvdW::new(self, 24)
    }
}
///Control Register 2
///
///You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0;
}
