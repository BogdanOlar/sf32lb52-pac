///Register `DPI_IF_CONF4` reader
pub type R = crate::R<DPI_IF_CONF4rs>;
///Register `DPI_IF_CONF4` writer
pub type W = crate::W<DPI_IF_CONF4rs>;
///Field `VAH` reader - vertical active height
pub type VahR = crate::FieldReader<u16>;
///Field `VAH` writer - vertical active height
pub type VahW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `HAW` reader - horizontal active width
pub type HawR = crate::FieldReader<u16>;
///Field `HAW` writer - horizontal active width
pub type HawW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    ///Bits 0:10 - vertical active height
    #[inline(always)]
    pub fn vah(&self) -> VahR {
        VahR::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - horizontal active width
    #[inline(always)]
    pub fn haw(&self) -> HawR {
        HawR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPI_IF_CONF4")
            .field("haw", &self.haw())
            .field("vah", &self.vah())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - vertical active height
    #[inline(always)]
    pub fn vah(&mut self) -> VahW<DPI_IF_CONF4rs> {
        VahW::new(self, 0)
    }
    ///Bits 16:26 - horizontal active width
    #[inline(always)]
    pub fn haw(&mut self) -> HawW<DPI_IF_CONF4rs> {
        HawW::new(self, 16)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`dpi_if_conf4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi_if_conf4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DPI_IF_CONF4rs;
impl crate::RegisterSpec for DPI_IF_CONF4rs {
    type Ux = u32;
}
///`read()` method returns [`dpi_if_conf4::R`](R) reader structure
impl crate::Readable for DPI_IF_CONF4rs {}
///`write(|w| ..)` method takes [`dpi_if_conf4::W`](W) writer structure
impl crate::Writable for DPI_IF_CONF4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DPI_IF_CONF4 to value 0
impl crate::Resettable for DPI_IF_CONF4rs {
    const RESET_VALUE: u32 = 0;
}
