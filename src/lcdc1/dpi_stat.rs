///Register `DPI_STAT` reader
pub type R = crate::R<DPI_STATrs>;
///Register `DPI_STAT` writer
pub type W = crate::W<DPI_STATrs>;
///Field `HPOS` reader - dpi horizontal position
pub type HposR = crate::FieldReader<u16>;
///Field `HPOS` writer - dpi horizontal position
pub type HposW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `HSTAT` reader - horizontal status 0: idle 1: prep 2: hsync 3: hbp 4: hact 5: hfp 6: wait
pub type HstatR = crate::FieldReader;
///Field `HSTAT` writer - horizontal status 0: idle 1: prep 2: hsync 3: hbp 4: hact 5: hfp 6: wait
pub type HstatW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `VPOS` reader - dpi vertical position
pub type VposR = crate::FieldReader<u16>;
///Field `VPOS` writer - dpi vertical position
pub type VposW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:10 - dpi horizontal position
    #[inline(always)]
    pub fn hpos(&self) -> HposR {
        HposR::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 11:13 - horizontal status 0: idle 1: prep 2: hsync 3: hbp 4: hact 5: hfp 6: wait
    #[inline(always)]
    pub fn hstat(&self) -> HstatR {
        HstatR::new(((self.bits >> 11) & 7) as u8)
    }
    ///Bits 14:15
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:31 - dpi vertical position
    #[inline(always)]
    pub fn vpos(&self) -> VposR {
        VposR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPI_STAT")
            .field("vpos", &self.vpos())
            .field("rsvd", &self.rsvd())
            .field("hstat", &self.hstat())
            .field("hpos", &self.hpos())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - dpi horizontal position
    #[inline(always)]
    pub fn hpos(&mut self) -> HposW<DPI_STATrs> {
        HposW::new(self, 0)
    }
    ///Bits 11:13 - horizontal status 0: idle 1: prep 2: hsync 3: hbp 4: hact 5: hfp 6: wait
    #[inline(always)]
    pub fn hstat(&mut self) -> HstatW<DPI_STATrs> {
        HstatW::new(self, 11)
    }
    ///Bits 14:15
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<DPI_STATrs> {
        RsvdW::new(self, 14)
    }
    ///Bits 16:31 - dpi vertical position
    #[inline(always)]
    pub fn vpos(&mut self) -> VposW<DPI_STATrs> {
        VposW::new(self, 16)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`dpi_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DPI_STATrs;
impl crate::RegisterSpec for DPI_STATrs {
    type Ux = u32;
}
///`read()` method returns [`dpi_stat::R`](R) reader structure
impl crate::Readable for DPI_STATrs {}
///`write(|w| ..)` method takes [`dpi_stat::W`](W) writer structure
impl crate::Writable for DPI_STATrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DPI_STAT to value 0
impl crate::Resettable for DPI_STATrs {
    const RESET_VALUE: u32 = 0;
}
