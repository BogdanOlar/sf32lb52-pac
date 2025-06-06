///Register `BMR` reader
pub type R = crate::R<BMRrs>;
///Register `BMR` writer
pub type W = crate::W<BMRrs>;
///Field `SDA` reader - value of the SDA pin.
pub type SdaR = crate::BitReader;
///Field `SDA` writer - value of the SDA pin.
pub type SdaW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCL` reader - value of the SCL pin. Software can check bus level when the I2C bus is hung and the I2C unit must be reset.
pub type SclR = crate::BitReader;
///Field `SCL` writer - value of the SCL pin. Software can check bus level when the I2C bus is hung and the I2C unit must be reset.
pub type SclW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - value of the SDA pin.
    #[inline(always)]
    pub fn sda(&self) -> SdaR {
        SdaR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - value of the SCL pin. Software can check bus level when the I2C bus is hung and the I2C unit must be reset.
    #[inline(always)]
    pub fn scl(&self) -> SclR {
        SclR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BMR")
            .field("scl", &self.scl())
            .field("sda", &self.sda())
            .finish()
    }
}
impl W {
    ///Bit 0 - value of the SDA pin.
    #[inline(always)]
    pub fn sda(&mut self) -> SdaW<BMRrs> {
        SdaW::new(self, 0)
    }
    ///Bit 1 - value of the SCL pin. Software can check bus level when the I2C bus is hung and the I2C unit must be reset.
    #[inline(always)]
    pub fn scl(&mut self) -> SclW<BMRrs> {
        SclW::new(self, 1)
    }
}
///Bus Monitor Register
///
///You can [`read`](crate::Reg::read) this register and get [`bmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct BMRrs;
impl crate::RegisterSpec for BMRrs {
    type Ux = u32;
}
///`read()` method returns [`bmr::R`](R) reader structure
impl crate::Readable for BMRrs {}
///`write(|w| ..)` method takes [`bmr::W`](W) writer structure
impl crate::Writable for BMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BMR to value 0x03
impl crate::Resettable for BMRrs {
    const RESET_VALUE: u32 = 0x03;
}
