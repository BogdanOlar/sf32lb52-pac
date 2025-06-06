///Register `ECR2` reader
pub type R = crate::R<ECR2rs>;
///Register `ECR2` writer
pub type W = crate::W<ECR2rs>;
///Field `GPIO1` reader - write 1 to clear module enable, write 0 has no effect
pub type Gpio1R = crate::BitReader;
///Field `GPIO1` writer - write 1 to clear module enable, write 0 has no effect
pub type Gpio1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPI1` reader - write 1 to clear module enable, write 0 has no effect
pub type Mpi1R = crate::BitReader;
///Field `MPI1` writer - write 1 to clear module enable, write 0 has no effect
pub type Mpi1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MPI2` reader - write 1 to clear module enable, write 0 has no effect
pub type Mpi2R = crate::BitReader;
///Field `MPI2` writer - write 1 to clear module enable, write 0 has no effect
pub type Mpi2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD8` reader -
pub type Rsvd8R = crate::BitReader;
///Field `RSVD8` writer -
pub type Rsvd8W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDMMC1` reader - write 1 to clear module enable, write 0 has no effect
pub type Sdmmc1R = crate::BitReader;
///Field `SDMMC1` writer - write 1 to clear module enable, write 0 has no effect
pub type Sdmmc1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD7` reader -
pub type Rsvd7R = crate::BitReader;
///Field `RSVD7` writer -
pub type Rsvd7W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBC` reader - write 1 to clear module enable, write 0 has no effect
pub type UsbcR = crate::BitReader;
///Field `USBC` writer - write 1 to clear module enable, write 0 has no effect
pub type UsbcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD6` reader -
pub type Rsvd6R = crate::BitReader;
///Field `RSVD6` writer -
pub type Rsvd6W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C3` reader - write 1 to clear module enable, write 0 has no effect
pub type I2c3R = crate::BitReader;
///Field `I2C3` writer - write 1 to clear module enable, write 0 has no effect
pub type I2c3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ATIM1` reader - write 1 to clear module enable, write 0 has no effect
pub type Atim1R = crate::BitReader;
///Field `ATIM1` writer - write 1 to clear module enable, write 0 has no effect
pub type Atim1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD5` reader -
pub type Rsvd5R = crate::FieldReader;
///Field `RSVD5` writer -
pub type Rsvd5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `USART3` reader - write 1 to clear module enable, write 0 has no effect
pub type Usart3R = crate::BitReader;
///Field `USART3` writer - write 1 to clear module enable, write 0 has no effect
pub type Usart3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD4` reader -
pub type Rsvd4R = crate::FieldReader;
///Field `RSVD4` writer -
pub type Rsvd4W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `AUDCODEC` reader - write 1 to clear module enable, write 0 has no effect
pub type AudcodecR = crate::BitReader;
///Field `AUDCODEC` writer - write 1 to clear module enable, write 0 has no effect
pub type AudcodecW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AUDPRC` reader - write 1 to clear module enable, write 0 has no effect
pub type AudprcR = crate::BitReader;
///Field `AUDPRC` writer - write 1 to clear module enable, write 0 has no effect
pub type AudprcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD3` reader -
pub type Rsvd3R = crate::BitReader;
///Field `RSVD3` writer -
pub type Rsvd3W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPADC` reader - write 1 to clear module enable, write 0 has no effect
pub type GpadcR = crate::BitReader;
///Field `GPADC` writer - write 1 to clear module enable, write 0 has no effect
pub type GpadcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSEN` reader - write 1 to clear module enable, write 0 has no effect
pub type TsenR = crate::BitReader;
///Field `TSEN` writer - write 1 to clear module enable, write 0 has no effect
pub type TsenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::BitReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C4` reader - write 1 to clear module enable, write 0 has no effect
pub type I2c4R = crate::BitReader;
///Field `I2C4` writer - write 1 to clear module enable, write 0 has no effect
pub type I2c4W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bit 0 - write 1 to clear module enable, write 0 has no effect
    #[inline(always)]
    pub fn gpio1(&self) -> Gpio1R {
        Gpio1R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - write 1 to clear module enable, write 0 has no effect
    #[inline(always)]
    pub fn mpi1(&self) -> Mpi1R {
        Mpi1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - write 1 to clear module enable, write 0 has no effect
    #[inline(always)]
    pub fn mpi2(&self) -> Mpi2R {
        Mpi2R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3
    #[inline(always)]
    pub fn rsvd8(&self) -> Rsvd8R {
        Rsvd8R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - write 1 to clear module enable, write 0 has no effect
    #[inline(always)]
    pub fn sdmmc1(&self) -> Sdmmc1R {
        Sdmmc1R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5
    #[inline(always)]
    pub fn rsvd7(&self) -> Rsvd7R {
        Rsvd7R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - write 1 to clear module enable, write 0 has no effect
    #[inline(always)]
    pub fn usbc(&self) -> UsbcR {
        UsbcR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7
    #[inline(always)]
    pub fn rsvd6(&self) -> Rsvd6R {
        Rsvd6R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - write 1 to clear module enable, write 0 has no effect
    #[inline(always)]
    pub fn i2c3(&self) -> I2c3R {
        I2c3R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - write 1 to clear module enable, write 0 has no effect
    #[inline(always)]
    pub fn atim1(&self) -> Atim1R {
        Atim1R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:11
    #[inline(always)]
    pub fn rsvd5(&self) -> Rsvd5R {
        Rsvd5R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 12 - write 1 to clear module enable, write 0 has no effect
    #[inline(always)]
    pub fn usart3(&self) -> Usart3R {
        Usart3R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:18
    #[inline(always)]
    pub fn rsvd4(&self) -> Rsvd4R {
        Rsvd4R::new(((self.bits >> 13) & 0x3f) as u8)
    }
    ///Bit 19 - write 1 to clear module enable, write 0 has no effect
    #[inline(always)]
    pub fn audcodec(&self) -> AudcodecR {
        AudcodecR::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - write 1 to clear module enable, write 0 has no effect
    #[inline(always)]
    pub fn audprc(&self) -> AudprcR {
        AudprcR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21
    #[inline(always)]
    pub fn rsvd3(&self) -> Rsvd3R {
        Rsvd3R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - write 1 to clear module enable, write 0 has no effect
    #[inline(always)]
    pub fn gpadc(&self) -> GpadcR {
        GpadcR::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - write 1 to clear module enable, write 0 has no effect
    #[inline(always)]
    pub fn tsen(&self) -> TsenR {
        TsenR::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - write 1 to clear module enable, write 0 has no effect
    #[inline(always)]
    pub fn i2c4(&self) -> I2c4R {
        I2c4R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bits 26:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECR2")
            .field("rsvd", &self.rsvd())
            .field("i2c4", &self.i2c4())
            .field("rsvd2", &self.rsvd2())
            .field("tsen", &self.tsen())
            .field("gpadc", &self.gpadc())
            .field("rsvd3", &self.rsvd3())
            .field("audprc", &self.audprc())
            .field("audcodec", &self.audcodec())
            .field("rsvd4", &self.rsvd4())
            .field("usart3", &self.usart3())
            .field("rsvd5", &self.rsvd5())
            .field("atim1", &self.atim1())
            .field("i2c3", &self.i2c3())
            .field("rsvd6", &self.rsvd6())
            .field("usbc", &self.usbc())
            .field("rsvd7", &self.rsvd7())
            .field("sdmmc1", &self.sdmmc1())
            .field("rsvd8", &self.rsvd8())
            .field("mpi2", &self.mpi2())
            .field("mpi1", &self.mpi1())
            .field("gpio1", &self.gpio1())
            .finish()
    }
}
impl W {
    ///Bit 0 - write 1 to clear module enable, write 0 has no effect
    #[inline(always)]
    pub fn gpio1(&mut self) -> Gpio1W<ECR2rs> {
        Gpio1W::new(self, 0)
    }
    ///Bit 1 - write 1 to clear module enable, write 0 has no effect
    #[inline(always)]
    pub fn mpi1(&mut self) -> Mpi1W<ECR2rs> {
        Mpi1W::new(self, 1)
    }
    ///Bit 2 - write 1 to clear module enable, write 0 has no effect
    #[inline(always)]
    pub fn mpi2(&mut self) -> Mpi2W<ECR2rs> {
        Mpi2W::new(self, 2)
    }
    ///Bit 3
    #[inline(always)]
    pub fn rsvd8(&mut self) -> Rsvd8W<ECR2rs> {
        Rsvd8W::new(self, 3)
    }
    ///Bit 4 - write 1 to clear module enable, write 0 has no effect
    #[inline(always)]
    pub fn sdmmc1(&mut self) -> Sdmmc1W<ECR2rs> {
        Sdmmc1W::new(self, 4)
    }
    ///Bit 5
    #[inline(always)]
    pub fn rsvd7(&mut self) -> Rsvd7W<ECR2rs> {
        Rsvd7W::new(self, 5)
    }
    ///Bit 6 - write 1 to clear module enable, write 0 has no effect
    #[inline(always)]
    pub fn usbc(&mut self) -> UsbcW<ECR2rs> {
        UsbcW::new(self, 6)
    }
    ///Bit 7
    #[inline(always)]
    pub fn rsvd6(&mut self) -> Rsvd6W<ECR2rs> {
        Rsvd6W::new(self, 7)
    }
    ///Bit 8 - write 1 to clear module enable, write 0 has no effect
    #[inline(always)]
    pub fn i2c3(&mut self) -> I2c3W<ECR2rs> {
        I2c3W::new(self, 8)
    }
    ///Bit 9 - write 1 to clear module enable, write 0 has no effect
    #[inline(always)]
    pub fn atim1(&mut self) -> Atim1W<ECR2rs> {
        Atim1W::new(self, 9)
    }
    ///Bits 10:11
    #[inline(always)]
    pub fn rsvd5(&mut self) -> Rsvd5W<ECR2rs> {
        Rsvd5W::new(self, 10)
    }
    ///Bit 12 - write 1 to clear module enable, write 0 has no effect
    #[inline(always)]
    pub fn usart3(&mut self) -> Usart3W<ECR2rs> {
        Usart3W::new(self, 12)
    }
    ///Bits 13:18
    #[inline(always)]
    pub fn rsvd4(&mut self) -> Rsvd4W<ECR2rs> {
        Rsvd4W::new(self, 13)
    }
    ///Bit 19 - write 1 to clear module enable, write 0 has no effect
    #[inline(always)]
    pub fn audcodec(&mut self) -> AudcodecW<ECR2rs> {
        AudcodecW::new(self, 19)
    }
    ///Bit 20 - write 1 to clear module enable, write 0 has no effect
    #[inline(always)]
    pub fn audprc(&mut self) -> AudprcW<ECR2rs> {
        AudprcW::new(self, 20)
    }
    ///Bit 21
    #[inline(always)]
    pub fn rsvd3(&mut self) -> Rsvd3W<ECR2rs> {
        Rsvd3W::new(self, 21)
    }
    ///Bit 22 - write 1 to clear module enable, write 0 has no effect
    #[inline(always)]
    pub fn gpadc(&mut self) -> GpadcW<ECR2rs> {
        GpadcW::new(self, 22)
    }
    ///Bit 23 - write 1 to clear module enable, write 0 has no effect
    #[inline(always)]
    pub fn tsen(&mut self) -> TsenW<ECR2rs> {
        TsenW::new(self, 23)
    }
    ///Bit 24
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<ECR2rs> {
        Rsvd2W::new(self, 24)
    }
    ///Bit 25 - write 1 to clear module enable, write 0 has no effect
    #[inline(always)]
    pub fn i2c4(&mut self) -> I2c4W<ECR2rs> {
        I2c4W::new(self, 25)
    }
    ///Bits 26:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<ECR2rs> {
        RsvdW::new(self, 26)
    }
}
///Enable Clear Register 2
///
///You can [`read`](crate::Reg::read) this register and get [`ecr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ECR2rs;
impl crate::RegisterSpec for ECR2rs {
    type Ux = u32;
}
///`read()` method returns [`ecr2::R`](R) reader structure
impl crate::Readable for ECR2rs {}
///`write(|w| ..)` method takes [`ecr2::W`](W) writer structure
impl crate::Writable for ECR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ECR2 to value 0
impl crate::Resettable for ECR2rs {
    const RESET_VALUE: u32 = 0;
}
