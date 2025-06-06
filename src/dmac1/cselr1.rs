///Register `CSELR1` reader
pub type R = crate::R<CSELR1rs>;
///Register `CSELR1` writer
pub type W = crate::W<CSELR1rs>;
///Field `C1S` reader - DMA channel 1 selection
pub type C1sR = crate::FieldReader;
///Field `C1S` writer - DMA channel 1 selection
pub type C1sW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `C2S` reader - DMA channel 2 selection
pub type C2sR = crate::FieldReader;
///Field `C2S` writer - DMA channel 2 selection
pub type C2sW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `C3S` reader - DMA channel 3 selection
pub type C3sR = crate::FieldReader;
///Field `C3S` writer - DMA channel 3 selection
pub type C3sW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `C4S` reader - DMA channel 4 selection
pub type C4sR = crate::FieldReader;
///Field `C4S` writer - DMA channel 4 selection
pub type C4sW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:5 - DMA channel 1 selection
    #[inline(always)]
    pub fn c1s(&self) -> C1sR {
        C1sR::new((self.bits & 0x3f) as u8)
    }
    ///Bits 8:13 - DMA channel 2 selection
    #[inline(always)]
    pub fn c2s(&self) -> C2sR {
        C2sR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 16:21 - DMA channel 3 selection
    #[inline(always)]
    pub fn c3s(&self) -> C3sR {
        C3sR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bits 24:29 - DMA channel 4 selection
    #[inline(always)]
    pub fn c4s(&self) -> C4sR {
        C4sR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSELR1")
            .field("c4s", &self.c4s())
            .field("c3s", &self.c3s())
            .field("c2s", &self.c2s())
            .field("c1s", &self.c1s())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - DMA channel 1 selection
    #[inline(always)]
    pub fn c1s(&mut self) -> C1sW<CSELR1rs> {
        C1sW::new(self, 0)
    }
    ///Bits 8:13 - DMA channel 2 selection
    #[inline(always)]
    pub fn c2s(&mut self) -> C2sW<CSELR1rs> {
        C2sW::new(self, 8)
    }
    ///Bits 16:21 - DMA channel 3 selection
    #[inline(always)]
    pub fn c3s(&mut self) -> C3sW<CSELR1rs> {
        C3sW::new(self, 16)
    }
    ///Bits 24:29 - DMA channel 4 selection
    #[inline(always)]
    pub fn c4s(&mut self) -> C4sW<CSELR1rs> {
        C4sW::new(self, 24)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`cselr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cselr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CSELR1rs;
impl crate::RegisterSpec for CSELR1rs {
    type Ux = u32;
}
///`read()` method returns [`cselr1::R`](R) reader structure
impl crate::Readable for CSELR1rs {}
///`write(|w| ..)` method takes [`cselr1::W`](W) writer structure
impl crate::Writable for CSELR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CSELR1 to value 0
impl crate::Resettable for CSELR1rs {
    const RESET_VALUE: u32 = 0;
}
