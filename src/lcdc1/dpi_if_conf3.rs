///Register `DPI_IF_CONF3` reader
pub type R = crate::R<DPI_IF_CONF3rs>;
///Register `DPI_IF_CONF3` writer
pub type W = crate::W<DPI_IF_CONF3rs>;
///Field `VFP` reader - vertical front porch
pub type VfpR = crate::FieldReader<u16>;
///Field `VFP` writer - vertical front porch
pub type VfpW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `HFP` reader - horizontal front porch
pub type HfpR = crate::FieldReader<u16>;
///Field `HFP` writer - horizontal front porch
pub type HfpW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:10 - vertical front porch
    #[inline(always)]
    pub fn vfp(&self) -> VfpR {
        VfpR::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 11:15
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    ///Bits 16:26 - horizontal front porch
    #[inline(always)]
    pub fn hfp(&self) -> HfpR {
        HfpR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    ///Bits 27:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPI_IF_CONF3")
            .field("rsvd", &self.rsvd())
            .field("hfp", &self.hfp())
            .field("rsvd2", &self.rsvd2())
            .field("vfp", &self.vfp())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - vertical front porch
    #[inline(always)]
    pub fn vfp(&mut self) -> VfpW<DPI_IF_CONF3rs> {
        VfpW::new(self, 0)
    }
    ///Bits 11:15
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<DPI_IF_CONF3rs> {
        Rsvd2W::new(self, 11)
    }
    ///Bits 16:26 - horizontal front porch
    #[inline(always)]
    pub fn hfp(&mut self) -> HfpW<DPI_IF_CONF3rs> {
        HfpW::new(self, 16)
    }
    ///Bits 27:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<DPI_IF_CONF3rs> {
        RsvdW::new(self, 27)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`dpi_if_conf3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi_if_conf3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DPI_IF_CONF3rs;
impl crate::RegisterSpec for DPI_IF_CONF3rs {
    type Ux = u32;
}
///`read()` method returns [`dpi_if_conf3::R`](R) reader structure
impl crate::Readable for DPI_IF_CONF3rs {}
///`write(|w| ..)` method takes [`dpi_if_conf3::W`](W) writer structure
impl crate::Writable for DPI_IF_CONF3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DPI_IF_CONF3 to value 0
impl crate::Resettable for DPI_IF_CONF3rs {
    const RESET_VALUE: u32 = 0;
}
