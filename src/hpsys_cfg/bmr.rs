///Register `BMR` reader
pub type R = crate::R<BMRrs>;
///Register `BMR` writer
pub type W = crate::W<BMRrs>;
///Field `BOOT_MODE` reader - 0 - normal mode, 1 - download mode
pub type BootModeR = crate::BitReader;
///Field `BOOT_MODE` writer - 0 - normal mode, 1 - download mode
pub type BootModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    ///Bit 0 - 0 - normal mode, 1 - download mode
    #[inline(always)]
    pub fn boot_mode(&self) -> BootModeR {
        BootModeR::new((self.bits & 1) != 0)
    }
    ///Bits 1:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BMR")
            .field("rsvd", &self.rsvd())
            .field("boot_mode", &self.boot_mode())
            .finish()
    }
}
impl W {
    ///Bit 0 - 0 - normal mode, 1 - download mode
    #[inline(always)]
    pub fn boot_mode(&mut self) -> BootModeW<BMRrs> {
        BootModeW::new(self, 0)
    }
    ///Bits 1:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<BMRrs> {
        RsvdW::new(self, 1)
    }
}
///Boot Mode Register
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
///`reset()` method sets BMR to value 0
impl crate::Resettable for BMRrs {
    const RESET_VALUE: u32 = 0;
}
