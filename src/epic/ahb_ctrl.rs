///Register `AHB_CTRL` reader
pub type R = crate::R<AHB_CTRLrs>;
///Register `AHB_CTRL` writer
pub type W = crate::W<AHB_CTRLrs>;
///Field `DESTINATION` reader - The Data can be sent to two destinations: 2'b0: AHB RAM 2'b1: AHB LCD
pub type DestinationR = crate::BitReader;
///Field `DESTINATION` writer - The Data can be sent to two destinations: 2'b0: AHB RAM 2'b1: AHB LCD
pub type DestinationW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `O_FORMAT` reader - AHB output format: 2'h0: RGB565 2'h1: RGB888 2'h2: ARGB8888 2'h3: ARGB8565
pub type OFormatR = crate::FieldReader;
///Field `O_FORMAT` writer - AHB output format: 2'h0: RGB565 2'h1: RGB888 2'h2: ARGB8888 2'h3: ARGB8565
pub type OFormatW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    ///Bit 0 - The Data can be sent to two destinations: 2'b0: AHB RAM 2'b1: AHB LCD
    #[inline(always)]
    pub fn destination(&self) -> DestinationR {
        DestinationR::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - AHB output format: 2'h0: RGB565 2'h1: RGB888 2'h2: ARGB8888 2'h3: ARGB8565
    #[inline(always)]
    pub fn o_format(&self) -> OFormatR {
        OFormatR::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 3:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_CTRL")
            .field("rsvd", &self.rsvd())
            .field("o_format", &self.o_format())
            .field("destination", &self.destination())
            .finish()
    }
}
impl W {
    ///Bit 0 - The Data can be sent to two destinations: 2'b0: AHB RAM 2'b1: AHB LCD
    #[inline(always)]
    pub fn destination(&mut self) -> DestinationW<AHB_CTRLrs> {
        DestinationW::new(self, 0)
    }
    ///Bits 1:2 - AHB output format: 2'h0: RGB565 2'h1: RGB888 2'h2: ARGB8888 2'h3: ARGB8565
    #[inline(always)]
    pub fn o_format(&mut self) -> OFormatW<AHB_CTRLrs> {
        OFormatW::new(self, 1)
    }
    ///Bits 3:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<AHB_CTRLrs> {
        RsvdW::new(self, 3)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`ahb_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct AHB_CTRLrs;
impl crate::RegisterSpec for AHB_CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`ahb_ctrl::R`](R) reader structure
impl crate::Readable for AHB_CTRLrs {}
///`write(|w| ..)` method takes [`ahb_ctrl::W`](W) writer structure
impl crate::Writable for AHB_CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AHB_CTRL to value 0
impl crate::Resettable for AHB_CTRLrs {
    const RESET_VALUE: u32 = 0;
}
