///Register `ATIM1_PINR1` reader
pub type R = crate::R<ATIM1_PINR1rs>;
///Register `ATIM1_PINR1` writer
pub type W = crate::W<ATIM1_PINR1rs>;
///Field `CH1_PIN` reader - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
pub type Ch1PinR = crate::FieldReader;
///Field `CH1_PIN` writer - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
pub type Ch1PinW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `CH2_PIN` reader - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
pub type Ch2PinR = crate::FieldReader;
///Field `CH2_PIN` writer - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
pub type Ch2PinW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `CH3_PIN` reader - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
pub type Ch3PinR = crate::FieldReader;
///Field `CH3_PIN` writer - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
pub type Ch3PinW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `CH4_PIN` reader - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
pub type Ch4PinR = crate::FieldReader;
///Field `CH4_PIN` writer - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
pub type Ch4PinW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:5 - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
    #[inline(always)]
    pub fn ch1_pin(&self) -> Ch1PinR {
        Ch1PinR::new((self.bits & 0x3f) as u8)
    }
    ///Bits 8:13 - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
    #[inline(always)]
    pub fn ch2_pin(&self) -> Ch2PinR {
        Ch2PinR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 16:21 - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
    #[inline(always)]
    pub fn ch3_pin(&self) -> Ch3PinR {
        Ch3PinR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bits 24:29 - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
    #[inline(always)]
    pub fn ch4_pin(&self) -> Ch4PinR {
        Ch4PinR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ATIM1_PINR1")
            .field("ch4_pin", &self.ch4_pin())
            .field("ch3_pin", &self.ch3_pin())
            .field("ch2_pin", &self.ch2_pin())
            .field("ch1_pin", &self.ch1_pin())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
    #[inline(always)]
    pub fn ch1_pin(&mut self) -> Ch1PinW<ATIM1_PINR1rs> {
        Ch1PinW::new(self, 0)
    }
    ///Bits 8:13 - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
    #[inline(always)]
    pub fn ch2_pin(&mut self) -> Ch2PinW<ATIM1_PINR1rs> {
        Ch2PinW::new(self, 8)
    }
    ///Bits 16:21 - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
    #[inline(always)]
    pub fn ch3_pin(&mut self) -> Ch3PinW<ATIM1_PINR1rs> {
        Ch3PinW::new(self, 16)
    }
    ///Bits 24:29 - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
    #[inline(always)]
    pub fn ch4_pin(&mut self) -> Ch4PinW<ATIM1_PINR1rs> {
        Ch4PinW::new(self, 24)
    }
}
///ATIM1 Pin Register 1
///
///You can [`read`](crate::Reg::read) this register and get [`atim1_pinr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atim1_pinr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ATIM1_PINR1rs;
impl crate::RegisterSpec for ATIM1_PINR1rs {
    type Ux = u32;
}
///`read()` method returns [`atim1_pinr1::R`](R) reader structure
impl crate::Readable for ATIM1_PINR1rs {}
///`write(|w| ..)` method takes [`atim1_pinr1::W`](W) writer structure
impl crate::Writable for ATIM1_PINR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ATIM1_PINR1 to value 0
impl crate::Resettable for ATIM1_PINR1rs {
    const RESET_VALUE: u32 = 0;
}
