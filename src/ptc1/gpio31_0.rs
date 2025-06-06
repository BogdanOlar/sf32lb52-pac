///Register `GPIO31_0` reader
pub type R = crate::R<GPIO31_0rs>;
///Register `GPIO31_0` writer
pub type W = crate::W<GPIO31_0rs>;
///Field `SELA` reader - select trigger A of GPIO 31~0 0: select GPIO 0 1: select GPIO 1 ...... 31: select GPIO 31
pub type SelaR = crate::FieldReader;
///Field `SELA` writer - select trigger A of GPIO 31~0 0: select GPIO 0 1: select GPIO 1 ...... 31: select GPIO 31
pub type SelaW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `RSVD4` reader -
pub type Rsvd4R = crate::FieldReader;
///Field `RSVD4` writer -
pub type Rsvd4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SELB` reader - select trigger B of GPIO 31~0
pub type SelbR = crate::FieldReader;
///Field `SELB` writer - select trigger B of GPIO 31~0
pub type SelbW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `RSVD3` reader -
pub type Rsvd3R = crate::FieldReader;
///Field `RSVD3` writer -
pub type Rsvd3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SELC` reader - select trigger C of GPIO 31~0
pub type SelcR = crate::FieldReader;
///Field `SELC` writer - select trigger C of GPIO 31~0
pub type SelcW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SELD` reader - select trigger D of GPIO 31~0
pub type SeldR = crate::FieldReader;
///Field `SELD` writer - select trigger D of GPIO 31~0
pub type SeldW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:4 - select trigger A of GPIO 31~0 0: select GPIO 0 1: select GPIO 1 ...... 31: select GPIO 31
    #[inline(always)]
    pub fn sela(&self) -> SelaR {
        SelaR::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:7
    #[inline(always)]
    pub fn rsvd4(&self) -> Rsvd4R {
        Rsvd4R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bits 8:12 - select trigger B of GPIO 31~0
    #[inline(always)]
    pub fn selb(&self) -> SelbR {
        SelbR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 13:15
    #[inline(always)]
    pub fn rsvd3(&self) -> Rsvd3R {
        Rsvd3R::new(((self.bits >> 13) & 7) as u8)
    }
    ///Bits 16:20 - select trigger C of GPIO 31~0
    #[inline(always)]
    pub fn selc(&self) -> SelcR {
        SelcR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 21:23
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 21) & 7) as u8)
    }
    ///Bits 24:28 - select trigger D of GPIO 31~0
    #[inline(always)]
    pub fn seld(&self) -> SeldR {
        SeldR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    ///Bits 29:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO31_0")
            .field("rsvd", &self.rsvd())
            .field("seld", &self.seld())
            .field("rsvd2", &self.rsvd2())
            .field("selc", &self.selc())
            .field("rsvd3", &self.rsvd3())
            .field("selb", &self.selb())
            .field("rsvd4", &self.rsvd4())
            .field("sela", &self.sela())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - select trigger A of GPIO 31~0 0: select GPIO 0 1: select GPIO 1 ...... 31: select GPIO 31
    #[inline(always)]
    pub fn sela(&mut self) -> SelaW<GPIO31_0rs> {
        SelaW::new(self, 0)
    }
    ///Bits 5:7
    #[inline(always)]
    pub fn rsvd4(&mut self) -> Rsvd4W<GPIO31_0rs> {
        Rsvd4W::new(self, 5)
    }
    ///Bits 8:12 - select trigger B of GPIO 31~0
    #[inline(always)]
    pub fn selb(&mut self) -> SelbW<GPIO31_0rs> {
        SelbW::new(self, 8)
    }
    ///Bits 13:15
    #[inline(always)]
    pub fn rsvd3(&mut self) -> Rsvd3W<GPIO31_0rs> {
        Rsvd3W::new(self, 13)
    }
    ///Bits 16:20 - select trigger C of GPIO 31~0
    #[inline(always)]
    pub fn selc(&mut self) -> SelcW<GPIO31_0rs> {
        SelcW::new(self, 16)
    }
    ///Bits 21:23
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<GPIO31_0rs> {
        Rsvd2W::new(self, 21)
    }
    ///Bits 24:28 - select trigger D of GPIO 31~0
    #[inline(always)]
    pub fn seld(&mut self) -> SeldW<GPIO31_0rs> {
        SeldW::new(self, 24)
    }
    ///Bits 29:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<GPIO31_0rs> {
        RsvdW::new(self, 29)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`gpio31_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio31_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct GPIO31_0rs;
impl crate::RegisterSpec for GPIO31_0rs {
    type Ux = u32;
}
///`read()` method returns [`gpio31_0::R`](R) reader structure
impl crate::Readable for GPIO31_0rs {}
///`write(|w| ..)` method takes [`gpio31_0::W`](W) writer structure
impl crate::Writable for GPIO31_0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GPIO31_0 to value 0
impl crate::Resettable for GPIO31_0rs {
    const RESET_VALUE: u32 = 0;
}
