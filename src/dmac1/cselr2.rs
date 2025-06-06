///Register `CSELR2` reader
pub type R = crate::R<CSELR2rs>;
///Register `CSELR2` writer
pub type W = crate::W<CSELR2rs>;
///Field `C5S` reader - DMA channel 5 selection
pub type C5sR = crate::FieldReader;
///Field `C5S` writer - DMA channel 5 selection
pub type C5sW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `C6S` reader - DMA channel 6 selection
pub type C6sR = crate::FieldReader;
///Field `C6S` writer - DMA channel 6 selection
pub type C6sW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `C7S` reader - DMA channel 7 selection
pub type C7sR = crate::FieldReader;
///Field `C7S` writer - DMA channel 7 selection
pub type C7sW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `C8S` reader - DMA channel 8 selection
pub type C8sR = crate::FieldReader;
///Field `C8S` writer - DMA channel 8 selection
pub type C8sW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:5 - DMA channel 5 selection
    #[inline(always)]
    pub fn c5s(&self) -> C5sR {
        C5sR::new((self.bits & 0x3f) as u8)
    }
    ///Bits 8:13 - DMA channel 6 selection
    #[inline(always)]
    pub fn c6s(&self) -> C6sR {
        C6sR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 16:21 - DMA channel 7 selection
    #[inline(always)]
    pub fn c7s(&self) -> C7sR {
        C7sR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bits 24:29 - DMA channel 8 selection
    #[inline(always)]
    pub fn c8s(&self) -> C8sR {
        C8sR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSELR2")
            .field("c8s", &self.c8s())
            .field("c7s", &self.c7s())
            .field("c6s", &self.c6s())
            .field("c5s", &self.c5s())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - DMA channel 5 selection
    #[inline(always)]
    pub fn c5s(&mut self) -> C5sW<CSELR2rs> {
        C5sW::new(self, 0)
    }
    ///Bits 8:13 - DMA channel 6 selection
    #[inline(always)]
    pub fn c6s(&mut self) -> C6sW<CSELR2rs> {
        C6sW::new(self, 8)
    }
    ///Bits 16:21 - DMA channel 7 selection
    #[inline(always)]
    pub fn c7s(&mut self) -> C7sW<CSELR2rs> {
        C7sW::new(self, 16)
    }
    ///Bits 24:29 - DMA channel 8 selection
    #[inline(always)]
    pub fn c8s(&mut self) -> C8sW<CSELR2rs> {
        C8sW::new(self, 24)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`cselr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cselr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CSELR2rs;
impl crate::RegisterSpec for CSELR2rs {
    type Ux = u32;
}
///`read()` method returns [`cselr2::R`](R) reader structure
impl crate::Readable for CSELR2rs {}
///`write(|w| ..)` method takes [`cselr2::W`](W) writer structure
impl crate::Writable for CSELR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CSELR2 to value 0
impl crate::Resettable for CSELR2rs {
    const RESET_VALUE: u32 = 0;
}
