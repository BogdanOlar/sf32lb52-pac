///Register `GPIO63_32` reader
pub type R = crate::R<GPIO63_32rs>;
///Register `GPIO63_32` writer
pub type W = crate::W<GPIO63_32rs>;
///Field `SELA` reader - select trigger A of GPIO 63~32 0: select GPIO 32 1: select GPIO 33 ...... 31: select GPIO 63
pub type SelaR = crate::FieldReader;
///Field `SELA` writer - select trigger A of GPIO 63~32 0: select GPIO 32 1: select GPIO 33 ...... 31: select GPIO 63
pub type SelaW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SELB` reader - select trigger B of GPIO 63~32
pub type SelbR = crate::FieldReader;
///Field `SELB` writer - select trigger B of GPIO 63~32
pub type SelbW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SELC` reader - select trigger C of GPIO 63~32
pub type SelcR = crate::FieldReader;
///Field `SELC` writer - select trigger C of GPIO 63~32
pub type SelcW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SELD` reader - select trigger D of GPIO 63~32
pub type SeldR = crate::FieldReader;
///Field `SELD` writer - select trigger D of GPIO 63~32
pub type SeldW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - select trigger A of GPIO 63~32 0: select GPIO 32 1: select GPIO 33 ...... 31: select GPIO 63
    #[inline(always)]
    pub fn sela(&self) -> SelaR {
        SelaR::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:12 - select trigger B of GPIO 63~32
    #[inline(always)]
    pub fn selb(&self) -> SelbR {
        SelbR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 16:20 - select trigger C of GPIO 63~32
    #[inline(always)]
    pub fn selc(&self) -> SelcR {
        SelcR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 24:28 - select trigger D of GPIO 63~32
    #[inline(always)]
    pub fn seld(&self) -> SeldR {
        SeldR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO63_32")
            .field("seld", &self.seld())
            .field("selc", &self.selc())
            .field("selb", &self.selb())
            .field("sela", &self.sela())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - select trigger A of GPIO 63~32 0: select GPIO 32 1: select GPIO 33 ...... 31: select GPIO 63
    #[inline(always)]
    pub fn sela(&mut self) -> SelaW<GPIO63_32rs> {
        SelaW::new(self, 0)
    }
    ///Bits 8:12 - select trigger B of GPIO 63~32
    #[inline(always)]
    pub fn selb(&mut self) -> SelbW<GPIO63_32rs> {
        SelbW::new(self, 8)
    }
    ///Bits 16:20 - select trigger C of GPIO 63~32
    #[inline(always)]
    pub fn selc(&mut self) -> SelcW<GPIO63_32rs> {
        SelcW::new(self, 16)
    }
    ///Bits 24:28 - select trigger D of GPIO 63~32
    #[inline(always)]
    pub fn seld(&mut self) -> SeldW<GPIO63_32rs> {
        SeldW::new(self, 24)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`gpio63_32::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio63_32::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct GPIO63_32rs;
impl crate::RegisterSpec for GPIO63_32rs {
    type Ux = u32;
}
///`read()` method returns [`gpio63_32::R`](R) reader structure
impl crate::Readable for GPIO63_32rs {}
///`write(|w| ..)` method takes [`gpio63_32::W`](W) writer structure
impl crate::Writable for GPIO63_32rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GPIO63_32 to value 0
impl crate::Resettable for GPIO63_32rs {
    const RESET_VALUE: u32 = 0;
}
