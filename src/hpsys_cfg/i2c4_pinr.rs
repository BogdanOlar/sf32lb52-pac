///Register `I2C4_PINR` reader
pub type R = crate::R<I2C4_PINRrs>;
///Register `I2C4_PINR` writer
pub type W = crate::W<I2C4_PINRrs>;
///Field `SCL_PIN` reader - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
pub type SclPinR = crate::FieldReader;
///Field `SCL_PIN` writer - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
pub type SclPinW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `SDA_PIN` reader - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
pub type SdaPinR = crate::FieldReader;
///Field `SDA_PIN` writer - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
pub type SdaPinW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:5 - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
    #[inline(always)]
    pub fn scl_pin(&self) -> SclPinR {
        SclPinR::new((self.bits & 0x3f) as u8)
    }
    ///Bits 8:13 - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
    #[inline(always)]
    pub fn sda_pin(&self) -> SdaPinR {
        SdaPinR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C4_PINR")
            .field("sda_pin", &self.sda_pin())
            .field("scl_pin", &self.scl_pin())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
    #[inline(always)]
    pub fn scl_pin(&mut self) -> SclPinW<I2C4_PINRrs> {
        SclPinW::new(self, 0)
    }
    ///Bits 8:13 - Connect function pin to selected IO(PA). 0 to 44 for PA00 to PA44. Other values for floating.
    #[inline(always)]
    pub fn sda_pin(&mut self) -> SdaPinW<I2C4_PINRrs> {
        SdaPinW::new(self, 8)
    }
}
///I2C4 Pin Register
///
///You can [`read`](crate::Reg::read) this register and get [`i2c4_pinr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c4_pinr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct I2C4_PINRrs;
impl crate::RegisterSpec for I2C4_PINRrs {
    type Ux = u32;
}
///`read()` method returns [`i2c4_pinr::R`](R) reader structure
impl crate::Readable for I2C4_PINRrs {}
///`write(|w| ..)` method takes [`i2c4_pinr::W`](W) writer structure
impl crate::Writable for I2C4_PINRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets I2C4_PINR to value 0
impl crate::Resettable for I2C4_PINRrs {
    const RESET_VALUE: u32 = 0;
}
