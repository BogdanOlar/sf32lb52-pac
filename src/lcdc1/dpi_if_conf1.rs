///Register `DPI_IF_CONF1` reader
pub type R = crate::R<DPI_IF_CONF1rs>;
///Register `DPI_IF_CONF1` writer
pub type W = crate::W<DPI_IF_CONF1rs>;
///Field `VSH` reader - dpi vsync height
pub type VshR = crate::FieldReader<u16>;
///Field `VSH` writer - dpi vsync height
pub type VshW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `HSW` reader - dpi hsync width
pub type HswR = crate::FieldReader<u16>;
///Field `HSW` writer - dpi hsync width
pub type HswW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    ///Bits 0:10 - dpi vsync height
    #[inline(always)]
    pub fn vsh(&self) -> VshR {
        VshR::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - dpi hsync width
    #[inline(always)]
    pub fn hsw(&self) -> HswR {
        HswR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPI_IF_CONF1")
            .field("hsw", &self.hsw())
            .field("vsh", &self.vsh())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - dpi vsync height
    #[inline(always)]
    pub fn vsh(&mut self) -> VshW<DPI_IF_CONF1rs> {
        VshW::new(self, 0)
    }
    ///Bits 16:26 - dpi hsync width
    #[inline(always)]
    pub fn hsw(&mut self) -> HswW<DPI_IF_CONF1rs> {
        HswW::new(self, 16)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`dpi_if_conf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi_if_conf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DPI_IF_CONF1rs;
impl crate::RegisterSpec for DPI_IF_CONF1rs {
    type Ux = u32;
}
///`read()` method returns [`dpi_if_conf1::R`](R) reader structure
impl crate::Readable for DPI_IF_CONF1rs {}
///`write(|w| ..)` method takes [`dpi_if_conf1::W`](W) writer structure
impl crate::Writable for DPI_IF_CONF1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DPI_IF_CONF1 to value 0
impl crate::Resettable for DPI_IF_CONF1rs {
    const RESET_VALUE: u32 = 0;
}
