///Register `GPIO31_0` reader
pub type R = crate::R<GPIO31_0rs>;
///Register `GPIO31_0` writer
pub type W = crate::W<GPIO31_0rs>;
///Field `SELA` reader - select trigger A of GPIO 31~0 0: select GPIO 0 1: select GPIO 1 ...... 31: select GPIO 31
pub type SelaR = crate::FieldReader;
///Field `SELA` writer - select trigger A of GPIO 31~0 0: select GPIO 0 1: select GPIO 1 ...... 31: select GPIO 31
pub type SelaW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SELB` reader - select trigger B of GPIO 31~0
pub type SelbR = crate::FieldReader;
///Field `SELB` writer - select trigger B of GPIO 31~0
pub type SelbW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SELC` reader - select trigger C of GPIO 31~0
pub type SelcR = crate::FieldReader;
///Field `SELC` writer - select trigger C of GPIO 31~0
pub type SelcW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SELD` reader - select trigger D of GPIO 31~0
pub type SeldR = crate::FieldReader;
///Field `SELD` writer - select trigger D of GPIO 31~0
pub type SeldW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - select trigger A of GPIO 31~0 0: select GPIO 0 1: select GPIO 1 ...... 31: select GPIO 31
    #[inline(always)]
    pub fn sela(&self) -> SelaR {
        SelaR::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:12 - select trigger B of GPIO 31~0
    #[inline(always)]
    pub fn selb(&self) -> SelbR {
        SelbR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 16:20 - select trigger C of GPIO 31~0
    #[inline(always)]
    pub fn selc(&self) -> SelcR {
        SelcR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 24:28 - select trigger D of GPIO 31~0
    #[inline(always)]
    pub fn seld(&self) -> SeldR {
        SeldR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO31_0")
            .field("seld", &self.seld())
            .field("selc", &self.selc())
            .field("selb", &self.selb())
            .field("sela", &self.sela())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - select trigger A of GPIO 31~0 0: select GPIO 0 1: select GPIO 1 ...... 31: select GPIO 31
    #[inline(always)]
    pub fn sela(&mut self) -> SelaW<GPIO31_0rs> {
        SelaW::new(self, 0)
    }
    ///Bits 8:12 - select trigger B of GPIO 31~0
    #[inline(always)]
    pub fn selb(&mut self) -> SelbW<GPIO31_0rs> {
        SelbW::new(self, 8)
    }
    ///Bits 16:20 - select trigger C of GPIO 31~0
    #[inline(always)]
    pub fn selc(&mut self) -> SelcW<GPIO31_0rs> {
        SelcW::new(self, 16)
    }
    ///Bits 24:28 - select trigger D of GPIO 31~0
    #[inline(always)]
    pub fn seld(&mut self) -> SeldW<GPIO31_0rs> {
        SeldW::new(self, 24)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`gpio31_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio31_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct GPIO31_0rs;
impl crate::RegisterSpec for GPIO31_0rs {
    type Ux = u32;
}
///`read()` method returns [`gpio31_0::R`](R) reader structure
impl crate::Readable for GPIO31_0rs {}
///`write(|w| ..)` method takes [`gpio31_0::W`](W) writer structure
impl crate::Writable for GPIO31_0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GPIO31_0 to value 0
impl crate::Resettable for GPIO31_0rs {
    const RESET_VALUE: u32 = 0;
}
