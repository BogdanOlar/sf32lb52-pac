///Register `RSTR2` reader
pub type R = crate::R<RSTR2rs>;
///Register `RSTR2` writer
pub type W = crate::W<RSTR2rs>;
///Field `GPIO1` reader - 0 - no reset; 1 - reset
pub type Gpio1R = crate::BitReader;
///Field `GPIO1` writer - 0 - no reset; 1 - reset
pub type Gpio1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPI1` reader - 0 - no reset; 1 - reset
pub type Mpi1R = crate::BitReader;
///Field `MPI1` writer - 0 - no reset; 1 - reset
pub type Mpi1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPI2` reader - 0 - no reset; 1 - reset
pub type Mpi2R = crate::BitReader;
///Field `MPI2` writer - 0 - no reset; 1 - reset
pub type Mpi2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC1` reader - 0 - no reset; 1 - reset
pub type Sdmmc1R = crate::BitReader;
///Field `SDMMC1` writer - 0 - no reset; 1 - reset
pub type Sdmmc1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBC` reader - 0 - no reset; 1 - reset
pub type UsbcR = crate::BitReader;
///Field `USBC` writer - 0 - no reset; 1 - reset
pub type UsbcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C3` reader - 0 - no reset; 1 - reset
pub type I2c3R = crate::BitReader;
///Field `I2C3` writer - 0 - no reset; 1 - reset
pub type I2c3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ATIM1` reader - 0 - no reset; 1 - reset
pub type Atim1R = crate::BitReader;
///Field `ATIM1` writer - 0 - no reset; 1 - reset
pub type Atim1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART3` reader - 0 - no reset; 1 - reset
pub type Usart3R = crate::BitReader;
///Field `USART3` writer - 0 - no reset; 1 - reset
pub type Usart3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AUDCODEC` reader - 0 - no reset; 1 - reset
pub type AudcodecR = crate::BitReader;
///Field `AUDCODEC` writer - 0 - no reset; 1 - reset
pub type AudcodecW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AUDPRC` reader - 0 - no reset; 1 - reset
pub type AudprcR = crate::BitReader;
///Field `AUDPRC` writer - 0 - no reset; 1 - reset
pub type AudprcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPADC` reader - 0 - no reset; 1 - reset
pub type GpadcR = crate::BitReader;
///Field `GPADC` writer - 0 - no reset; 1 - reset
pub type GpadcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSEN` reader - 0 - no reset; 1 - reset
pub type TsenR = crate::BitReader;
///Field `TSEN` writer - 0 - no reset; 1 - reset
pub type TsenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C4` reader - 0 - no reset; 1 - reset
pub type I2c4R = crate::BitReader;
///Field `I2C4` writer - 0 - no reset; 1 - reset
pub type I2c4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - 0 - no reset; 1 - reset
    #[inline(always)]
    pub fn gpio1(&self) -> Gpio1R {
        Gpio1R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - 0 - no reset; 1 - reset
    #[inline(always)]
    pub fn mpi1(&self) -> Mpi1R {
        Mpi1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - 0 - no reset; 1 - reset
    #[inline(always)]
    pub fn mpi2(&self) -> Mpi2R {
        Mpi2R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - 0 - no reset; 1 - reset
    #[inline(always)]
    pub fn sdmmc1(&self) -> Sdmmc1R {
        Sdmmc1R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - 0 - no reset; 1 - reset
    #[inline(always)]
    pub fn usbc(&self) -> UsbcR {
        UsbcR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - 0 - no reset; 1 - reset
    #[inline(always)]
    pub fn i2c3(&self) -> I2c3R {
        I2c3R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - 0 - no reset; 1 - reset
    #[inline(always)]
    pub fn atim1(&self) -> Atim1R {
        Atim1R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - 0 - no reset; 1 - reset
    #[inline(always)]
    pub fn usart3(&self) -> Usart3R {
        Usart3R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 19 - 0 - no reset; 1 - reset
    #[inline(always)]
    pub fn audcodec(&self) -> AudcodecR {
        AudcodecR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - 0 - no reset; 1 - reset
    #[inline(always)]
    pub fn audprc(&self) -> AudprcR {
        AudprcR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - 0 - no reset; 1 - reset
    #[inline(always)]
    pub fn gpadc(&self) -> GpadcR {
        GpadcR::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - 0 - no reset; 1 - reset
    #[inline(always)]
    pub fn tsen(&self) -> TsenR {
        TsenR::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 25 - 0 - no reset; 1 - reset
    #[inline(always)]
    pub fn i2c4(&self) -> I2c4R {
        I2c4R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSTR2")
            .field("i2c4", &self.i2c4())
            .field("tsen", &self.tsen())
            .field("gpadc", &self.gpadc())
            .field("audprc", &self.audprc())
            .field("audcodec", &self.audcodec())
            .field("usart3", &self.usart3())
            .field("atim1", &self.atim1())
            .field("i2c3", &self.i2c3())
            .field("usbc", &self.usbc())
            .field("sdmmc1", &self.sdmmc1())
            .field("mpi2", &self.mpi2())
            .field("mpi1", &self.mpi1())
            .field("gpio1", &self.gpio1())
            .finish()
    }
}
impl W {
    ///Bit 0 - 0 - no reset; 1 - reset
    #[inline(always)]
    pub fn gpio1(&mut self) -> Gpio1W<RSTR2rs> {
        Gpio1W::new(self, 0)
    }
    ///Bit 1 - 0 - no reset; 1 - reset
    #[inline(always)]
    pub fn mpi1(&mut self) -> Mpi1W<RSTR2rs> {
        Mpi1W::new(self, 1)
    }
    ///Bit 2 - 0 - no reset; 1 - reset
    #[inline(always)]
    pub fn mpi2(&mut self) -> Mpi2W<RSTR2rs> {
        Mpi2W::new(self, 2)
    }
    ///Bit 4 - 0 - no reset; 1 - reset
    #[inline(always)]
    pub fn sdmmc1(&mut self) -> Sdmmc1W<RSTR2rs> {
        Sdmmc1W::new(self, 4)
    }
    ///Bit 6 - 0 - no reset; 1 - reset
    #[inline(always)]
    pub fn usbc(&mut self) -> UsbcW<RSTR2rs> {
        UsbcW::new(self, 6)
    }
    ///Bit 8 - 0 - no reset; 1 - reset
    #[inline(always)]
    pub fn i2c3(&mut self) -> I2c3W<RSTR2rs> {
        I2c3W::new(self, 8)
    }
    ///Bit 9 - 0 - no reset; 1 - reset
    #[inline(always)]
    pub fn atim1(&mut self) -> Atim1W<RSTR2rs> {
        Atim1W::new(self, 9)
    }
    ///Bit 12 - 0 - no reset; 1 - reset
    #[inline(always)]
    pub fn usart3(&mut self) -> Usart3W<RSTR2rs> {
        Usart3W::new(self, 12)
    }
    ///Bit 19 - 0 - no reset; 1 - reset
    #[inline(always)]
    pub fn audcodec(&mut self) -> AudcodecW<RSTR2rs> {
        AudcodecW::new(self, 19)
    }
    ///Bit 20 - 0 - no reset; 1 - reset
    #[inline(always)]
    pub fn audprc(&mut self) -> AudprcW<RSTR2rs> {
        AudprcW::new(self, 20)
    }
    ///Bit 22 - 0 - no reset; 1 - reset
    #[inline(always)]
    pub fn gpadc(&mut self) -> GpadcW<RSTR2rs> {
        GpadcW::new(self, 22)
    }
    ///Bit 23 - 0 - no reset; 1 - reset
    #[inline(always)]
    pub fn tsen(&mut self) -> TsenW<RSTR2rs> {
        TsenW::new(self, 23)
    }
    ///Bit 25 - 0 - no reset; 1 - reset
    #[inline(always)]
    pub fn i2c4(&mut self) -> I2c4W<RSTR2rs> {
        I2c4W::new(self, 25)
    }
}
///Reset Register 2
///
///You can [`read`](crate::Reg::read) this register and get [`rstr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RSTR2rs;
impl crate::RegisterSpec for RSTR2rs {
    type Ux = u32;
}
///`read()` method returns [`rstr2::R`](R) reader structure
impl crate::Readable for RSTR2rs {}
///`write(|w| ..)` method takes [`rstr2::W`](W) writer structure
impl crate::Writable for RSTR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RSTR2 to value 0
impl crate::Resettable for RSTR2rs {
    const RESET_VALUE: u32 = 0;
}
