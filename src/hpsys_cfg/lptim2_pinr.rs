///Register `LPTIM2_PINR` reader
pub type R = crate::R<LPTIM2_PINRrs>;
///Register `LPTIM2_PINR` writer
pub type W = crate::W<LPTIM2_PINRrs>;
///Field `IN_PIN` reader - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
pub type InPinR = crate::FieldReader;
///Field `IN_PIN` writer - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
pub type InPinW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `OUT_PIN` reader - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
pub type OutPinR = crate::FieldReader;
///Field `OUT_PIN` writer - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
pub type OutPinW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `ETR_PIN` reader - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
pub type EtrPinR = crate::FieldReader;
///Field `ETR_PIN` writer - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
pub type EtrPinW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:5 - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
    #[inline(always)]
    pub fn in_pin(&self) -> InPinR {
        InPinR::new((self.bits & 0x3f) as u8)
    }
    ///Bits 8:13 - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
    #[inline(always)]
    pub fn out_pin(&self) -> OutPinR {
        OutPinR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 16:21 - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
    #[inline(always)]
    pub fn etr_pin(&self) -> EtrPinR {
        EtrPinR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTIM2_PINR")
            .field("etr_pin", &self.etr_pin())
            .field("out_pin", &self.out_pin())
            .field("in_pin", &self.in_pin())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
    #[inline(always)]
    pub fn in_pin(&mut self) -> InPinW<LPTIM2_PINRrs> {
        InPinW::new(self, 0)
    }
    ///Bits 8:13 - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
    #[inline(always)]
    pub fn out_pin(&mut self) -> OutPinW<LPTIM2_PINRrs> {
        OutPinW::new(self, 8)
    }
    ///Bits 16:21 - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
    #[inline(always)]
    pub fn etr_pin(&mut self) -> EtrPinW<LPTIM2_PINRrs> {
        EtrPinW::new(self, 16)
    }
}
///LPTIM2 Pin Register
///
///You can [`read`](crate::Reg::read) this register and get [`lptim2_pinr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim2_pinr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct LPTIM2_PINRrs;
impl crate::RegisterSpec for LPTIM2_PINRrs {
    type Ux = u32;
}
///`read()` method returns [`lptim2_pinr::R`](R) reader structure
impl crate::Readable for LPTIM2_PINRrs {}
///`write(|w| ..)` method takes [`lptim2_pinr::W`](W) writer structure
impl crate::Writable for LPTIM2_PINRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LPTIM2_PINR to value 0
impl crate::Resettable for LPTIM2_PINRrs {
    const RESET_VALUE: u32 = 0;
}
