///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Register `ISR` writer
pub type W = crate::W<ISRrs>;
///Field `GIF` reader - GIF, global interrupt flag
pub type GifR = crate::BitReader;
///Field `GIF` writer - GIF, global interrupt flag
pub type GifW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIF` reader - TCIF, transfer complete flag
pub type TcifR = crate::BitReader;
///Field `TCIF` writer - TCIF, transfer complete flag
pub type TcifW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HTIF` reader - HTIF, half transfer flag
pub type HtifR = crate::BitReader;
///Field `HTIF` writer - HTIF, half transfer flag
pub type HtifW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEIF` reader - TEIF, transfer error flag
pub type TeifR = crate::BitReader;
///Field `TEIF` writer - TEIF, transfer error flag
pub type TeifW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - GIF, global interrupt flag
    #[inline(always)]
    pub fn gif(&self) -> GifR {
        GifR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TCIF, transfer complete flag
    #[inline(always)]
    pub fn tcif(&self) -> TcifR {
        TcifR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - HTIF, half transfer flag
    #[inline(always)]
    pub fn htif(&self) -> HtifR {
        HtifR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TEIF, transfer error flag
    #[inline(always)]
    pub fn teif(&self) -> TeifR {
        TeifR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("teif", &self.teif())
            .field("htif", &self.htif())
            .field("tcif", &self.tcif())
            .field("gif", &self.gif())
            .finish()
    }
}
impl W {
    ///Bit 0 - GIF, global interrupt flag
    #[inline(always)]
    pub fn gif(&mut self) -> GifW<ISRrs> {
        GifW::new(self, 0)
    }
    ///Bit 1 - TCIF, transfer complete flag
    #[inline(always)]
    pub fn tcif(&mut self) -> TcifW<ISRrs> {
        TcifW::new(self, 1)
    }
    ///Bit 2 - HTIF, half transfer flag
    #[inline(always)]
    pub fn htif(&mut self) -> HtifW<ISRrs> {
        HtifW::new(self, 2)
    }
    ///Bit 3 - TEIF, transfer error flag
    #[inline(always)]
    pub fn teif(&mut self) -> TeifW<ISRrs> {
        TeifW::new(self, 3)
    }
}
///interrupt status register
///
///You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`write(|w| ..)` method takes [`isr::W`](W) writer structure
impl crate::Writable for ISRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0;
}
