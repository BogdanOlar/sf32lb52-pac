///Register `ATIM1_PINR3` reader
pub type R = crate::R<ATIM1_PINR3rs>;
///Register `ATIM1_PINR3` writer
pub type W = crate::W<ATIM1_PINR3rs>;
///Field `BK_PIN` reader - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
pub type BkPinR = crate::FieldReader;
///Field `BK_PIN` writer - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
pub type BkPinW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `RSVD3` reader -
pub type Rsvd3R = crate::FieldReader;
///Field `RSVD3` writer -
pub type Rsvd3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BK2_PIN` reader - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
pub type Bk2PinR = crate::FieldReader;
///Field `BK2_PIN` writer - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
pub type Bk2PinW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ETR_PIN` reader - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
pub type EtrPinR = crate::FieldReader;
///Field `ETR_PIN` writer - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
pub type EtrPinW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:5 - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
    #[inline(always)]
    pub fn bk_pin(&self) -> BkPinR {
        BkPinR::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:7
    #[inline(always)]
    pub fn rsvd3(&self) -> Rsvd3R {
        Rsvd3R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:13 - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
    #[inline(always)]
    pub fn bk2_pin(&self) -> Bk2PinR {
        Bk2PinR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 14:15
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:21 - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
    #[inline(always)]
    pub fn etr_pin(&self) -> EtrPinR {
        EtrPinR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bits 22:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ATIM1_PINR3")
            .field("rsvd", &self.rsvd())
            .field("etr_pin", &self.etr_pin())
            .field("rsvd2", &self.rsvd2())
            .field("bk2_pin", &self.bk2_pin())
            .field("rsvd3", &self.rsvd3())
            .field("bk_pin", &self.bk_pin())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
    #[inline(always)]
    pub fn bk_pin(&mut self) -> BkPinW<ATIM1_PINR3rs> {
        BkPinW::new(self, 0)
    }
    ///Bits 6:7
    #[inline(always)]
    pub fn rsvd3(&mut self) -> Rsvd3W<ATIM1_PINR3rs> {
        Rsvd3W::new(self, 6)
    }
    ///Bits 8:13 - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
    #[inline(always)]
    pub fn bk2_pin(&mut self) -> Bk2PinW<ATIM1_PINR3rs> {
        Bk2PinW::new(self, 8)
    }
    ///Bits 14:15
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<ATIM1_PINR3rs> {
        Rsvd2W::new(self, 14)
    }
    ///Bits 16:21 - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
    #[inline(always)]
    pub fn etr_pin(&mut self) -> EtrPinW<ATIM1_PINR3rs> {
        EtrPinW::new(self, 16)
    }
    ///Bits 22:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<ATIM1_PINR3rs> {
        RsvdW::new(self, 22)
    }
}
///ATIM1 Pin Register 3
///
///You can [`read`](crate::Reg::read) this register and get [`atim1_pinr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atim1_pinr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ATIM1_PINR3rs;
impl crate::RegisterSpec for ATIM1_PINR3rs {
    type Ux = u32;
}
///`read()` method returns [`atim1_pinr3::R`](R) reader structure
impl crate::Readable for ATIM1_PINR3rs {}
///`write(|w| ..)` method takes [`atim1_pinr3::W`](W) writer structure
impl crate::Writable for ATIM1_PINR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ATIM1_PINR3 to value 0
impl crate::Resettable for ATIM1_PINR3rs {
    const RESET_VALUE: u32 = 0;
}
