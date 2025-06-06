///Register `CDCR` reader
pub type R = crate::R<CDCRrs>;
///Register `CDCR` writer
pub type W = crate::W<CDCRrs>;
///Field `CLK_CONFIG` reader - 1: the sd clock is 50% duty cycle 0: the high level of the sd clock is 1 hclk cycle
pub type ClkConfigR = crate::BitReader;
///Field `CLK_CONFIG` writer - 1: the sd clock is 50% duty cycle 0: the high level of the sd clock is 1 hclk cycle
pub type ClkConfigW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    ///Bit 0 - 1: the sd clock is 50% duty cycle 0: the high level of the sd clock is 1 hclk cycle
    #[inline(always)]
    pub fn clk_config(&self) -> ClkConfigR {
        ClkConfigR::new((self.bits & 1) != 0)
    }
    ///Bits 1:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CDCR")
            .field("rsvd", &self.rsvd())
            .field("clk_config", &self.clk_config())
            .finish()
    }
}
impl W {
    ///Bit 0 - 1: the sd clock is 50% duty cycle 0: the high level of the sd clock is 1 hclk cycle
    #[inline(always)]
    pub fn clk_config(&mut self) -> ClkConfigW<CDCRrs> {
        ClkConfigW::new(self, 0)
    }
    ///Bits 1:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<CDCRrs> {
        RsvdW::new(self, 1)
    }
}
///clock duty cycle register
///
///You can [`read`](crate::Reg::read) this register and get [`cdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CDCRrs;
impl crate::RegisterSpec for CDCRrs {
    type Ux = u32;
}
///`read()` method returns [`cdcr::R`](R) reader structure
impl crate::Readable for CDCRrs {}
///`write(|w| ..)` method takes [`cdcr::W`](W) writer structure
impl crate::Writable for CDCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CDCR to value 0x01
impl crate::Resettable for CDCRrs {
    const RESET_VALUE: u32 = 0x01;
}
