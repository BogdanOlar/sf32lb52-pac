///Register `ENR2` reader
pub type R = crate::R<ENR2rs>;
///Register `ENR2` writer
pub type W = crate::W<ENR2rs>;
///Field `GPIO1` reader - write 1 to set module enable, write 0 to disable module
pub type Gpio1R = crate::BitReader;
///Field `GPIO1` writer - write 1 to set module enable, write 0 to disable module
pub type Gpio1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPI1` reader - write 1 to set module enable, write 0 to disable module
pub type Mpi1R = crate::BitReader;
///Field `MPI1` writer - write 1 to set module enable, write 0 to disable module
pub type Mpi1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPI2` reader - write 1 to set module enable, write 0 to disable module
pub type Mpi2R = crate::BitReader;
///Field `MPI2` writer - write 1 to set module enable, write 0 to disable module
pub type Mpi2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC1` reader - write 1 to set module enable, write 0 to disable module
pub type Sdmmc1R = crate::BitReader;
///Field `SDMMC1` writer - write 1 to set module enable, write 0 to disable module
pub type Sdmmc1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBC` reader - write 1 to set module enable, write 0 to disable module
pub type UsbcR = crate::BitReader;
///Field `USBC` writer - write 1 to set module enable, write 0 to disable module
pub type UsbcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C3` reader - write 1 to set module enable, write 0 to disable module
pub type I2c3R = crate::BitReader;
///Field `I2C3` writer - write 1 to set module enable, write 0 to disable module
pub type I2c3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ATIM1` reader - write 1 to set module enable, write 0 to disable module
pub type Atim1R = crate::BitReader;
///Field `ATIM1` writer - write 1 to set module enable, write 0 to disable module
pub type Atim1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART3` reader - write 1 to set module enable, write 0 to disable module
pub type Usart3R = crate::BitReader;
///Field `USART3` writer - write 1 to set module enable, write 0 to disable module
pub type Usart3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AUDCODEC` reader - write 1 to set module enable, write 0 to disable module
pub type AudcodecR = crate::BitReader;
///Field `AUDCODEC` writer - write 1 to set module enable, write 0 to disable module
pub type AudcodecW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AUDPRC` reader - write 1 to set module enable, write 0 to disable module
pub type AudprcR = crate::BitReader;
///Field `AUDPRC` writer - write 1 to set module enable, write 0 to disable module
pub type AudprcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPADC` reader - write 1 to set module enable, write 0 to disable module
pub type GpadcR = crate::BitReader;
///Field `GPADC` writer - write 1 to set module enable, write 0 to disable module
pub type GpadcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSEN` reader - write 1 to set module enable, write 0 to disable module
pub type TsenR = crate::BitReader;
///Field `TSEN` writer - write 1 to set module enable, write 0 to disable module
pub type TsenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C4` reader - write 1 to set module enable, write 0 to disable module
pub type I2c4R = crate::BitReader;
///Field `I2C4` writer - write 1 to set module enable, write 0 to disable module
pub type I2c4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - write 1 to set module enable, write 0 to disable module
    #[inline(always)]
    pub fn gpio1(&self) -> Gpio1R {
        Gpio1R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - write 1 to set module enable, write 0 to disable module
    #[inline(always)]
    pub fn mpi1(&self) -> Mpi1R {
        Mpi1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - write 1 to set module enable, write 0 to disable module
    #[inline(always)]
    pub fn mpi2(&self) -> Mpi2R {
        Mpi2R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - write 1 to set module enable, write 0 to disable module
    #[inline(always)]
    pub fn sdmmc1(&self) -> Sdmmc1R {
        Sdmmc1R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - write 1 to set module enable, write 0 to disable module
    #[inline(always)]
    pub fn usbc(&self) -> UsbcR {
        UsbcR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - write 1 to set module enable, write 0 to disable module
    #[inline(always)]
    pub fn i2c3(&self) -> I2c3R {
        I2c3R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - write 1 to set module enable, write 0 to disable module
    #[inline(always)]
    pub fn atim1(&self) -> Atim1R {
        Atim1R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - write 1 to set module enable, write 0 to disable module
    #[inline(always)]
    pub fn usart3(&self) -> Usart3R {
        Usart3R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 19 - write 1 to set module enable, write 0 to disable module
    #[inline(always)]
    pub fn audcodec(&self) -> AudcodecR {
        AudcodecR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - write 1 to set module enable, write 0 to disable module
    #[inline(always)]
    pub fn audprc(&self) -> AudprcR {
        AudprcR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - write 1 to set module enable, write 0 to disable module
    #[inline(always)]
    pub fn gpadc(&self) -> GpadcR {
        GpadcR::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - write 1 to set module enable, write 0 to disable module
    #[inline(always)]
    pub fn tsen(&self) -> TsenR {
        TsenR::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 25 - write 1 to set module enable, write 0 to disable module
    #[inline(always)]
    pub fn i2c4(&self) -> I2c4R {
        I2c4R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENR2")
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
    ///Bit 0 - write 1 to set module enable, write 0 to disable module
    #[inline(always)]
    pub fn gpio1(&mut self) -> Gpio1W<ENR2rs> {
        Gpio1W::new(self, 0)
    }
    ///Bit 1 - write 1 to set module enable, write 0 to disable module
    #[inline(always)]
    pub fn mpi1(&mut self) -> Mpi1W<ENR2rs> {
        Mpi1W::new(self, 1)
    }
    ///Bit 2 - write 1 to set module enable, write 0 to disable module
    #[inline(always)]
    pub fn mpi2(&mut self) -> Mpi2W<ENR2rs> {
        Mpi2W::new(self, 2)
    }
    ///Bit 4 - write 1 to set module enable, write 0 to disable module
    #[inline(always)]
    pub fn sdmmc1(&mut self) -> Sdmmc1W<ENR2rs> {
        Sdmmc1W::new(self, 4)
    }
    ///Bit 6 - write 1 to set module enable, write 0 to disable module
    #[inline(always)]
    pub fn usbc(&mut self) -> UsbcW<ENR2rs> {
        UsbcW::new(self, 6)
    }
    ///Bit 8 - write 1 to set module enable, write 0 to disable module
    #[inline(always)]
    pub fn i2c3(&mut self) -> I2c3W<ENR2rs> {
        I2c3W::new(self, 8)
    }
    ///Bit 9 - write 1 to set module enable, write 0 to disable module
    #[inline(always)]
    pub fn atim1(&mut self) -> Atim1W<ENR2rs> {
        Atim1W::new(self, 9)
    }
    ///Bit 12 - write 1 to set module enable, write 0 to disable module
    #[inline(always)]
    pub fn usart3(&mut self) -> Usart3W<ENR2rs> {
        Usart3W::new(self, 12)
    }
    ///Bit 19 - write 1 to set module enable, write 0 to disable module
    #[inline(always)]
    pub fn audcodec(&mut self) -> AudcodecW<ENR2rs> {
        AudcodecW::new(self, 19)
    }
    ///Bit 20 - write 1 to set module enable, write 0 to disable module
    #[inline(always)]
    pub fn audprc(&mut self) -> AudprcW<ENR2rs> {
        AudprcW::new(self, 20)
    }
    ///Bit 22 - write 1 to set module enable, write 0 to disable module
    #[inline(always)]
    pub fn gpadc(&mut self) -> GpadcW<ENR2rs> {
        GpadcW::new(self, 22)
    }
    ///Bit 23 - write 1 to set module enable, write 0 to disable module
    #[inline(always)]
    pub fn tsen(&mut self) -> TsenW<ENR2rs> {
        TsenW::new(self, 23)
    }
    ///Bit 25 - write 1 to set module enable, write 0 to disable module
    #[inline(always)]
    pub fn i2c4(&mut self) -> I2c4W<ENR2rs> {
        I2c4W::new(self, 25)
    }
}
///Enable Register 2
///
///You can [`read`](crate::Reg::read) this register and get [`enr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ENR2rs;
impl crate::RegisterSpec for ENR2rs {
    type Ux = u32;
}
///`read()` method returns [`enr2::R`](R) reader structure
impl crate::Readable for ENR2rs {}
///`write(|w| ..)` method takes [`enr2::W`](W) writer structure
impl crate::Writable for ENR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ENR2 to value 0
impl crate::Resettable for ENR2rs {
    const RESET_VALUE: u32 = 0;
}
