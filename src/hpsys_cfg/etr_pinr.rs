///Register `ETR_PINR` reader
pub type R = crate::R<ETR_PINRrs>;
///Register `ETR_PINR` writer
pub type W = crate::W<ETR_PINRrs>;
///Field `ETR1_PIN` reader - Connect GPTIM1_ETR to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
pub type Etr1PinR = crate::FieldReader;
///Field `ETR1_PIN` writer - Connect GPTIM1_ETR to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
pub type Etr1PinW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ETR2_PIN` reader - Connect GPTIM2_ETR to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
pub type Etr2PinR = crate::FieldReader;
///Field `ETR2_PIN` writer - Connect GPTIM2_ETR to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
pub type Etr2PinW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    ///Bits 0:5 - Connect GPTIM1_ETR to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
    #[inline(always)]
    pub fn etr1_pin(&self) -> Etr1PinR {
        Etr1PinR::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:7
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:13 - Connect GPTIM2_ETR to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
    #[inline(always)]
    pub fn etr2_pin(&self) -> Etr2PinR {
        Etr2PinR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 14:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETR_PINR")
            .field("rsvd", &self.rsvd())
            .field("etr2_pin", &self.etr2_pin())
            .field("rsvd2", &self.rsvd2())
            .field("etr1_pin", &self.etr1_pin())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Connect GPTIM1_ETR to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
    #[inline(always)]
    pub fn etr1_pin(&mut self) -> Etr1PinW<ETR_PINRrs> {
        Etr1PinW::new(self, 0)
    }
    ///Bits 6:7
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<ETR_PINRrs> {
        Rsvd2W::new(self, 6)
    }
    ///Bits 8:13 - Connect GPTIM2_ETR to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
    #[inline(always)]
    pub fn etr2_pin(&mut self) -> Etr2PinW<ETR_PINRrs> {
        Etr2PinW::new(self, 8)
    }
    ///Bits 14:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<ETR_PINRrs> {
        RsvdW::new(self, 14)
    }
}
///GPTIM ETR Pin Register
///
///You can [`read`](crate::Reg::read) this register and get [`etr_pinr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etr_pinr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ETR_PINRrs;
impl crate::RegisterSpec for ETR_PINRrs {
    type Ux = u32;
}
///`read()` method returns [`etr_pinr::R`](R) reader structure
impl crate::Readable for ETR_PINRrs {}
///`write(|w| ..)` method takes [`etr_pinr::W`](W) writer structure
impl crate::Writable for ETR_PINRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ETR_PINR to value 0
impl crate::Resettable for ETR_PINRrs {
    const RESET_VALUE: u32 = 0;
}
