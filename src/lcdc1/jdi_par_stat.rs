///Register `JDI_PAR_STAT` reader
pub type R = crate::R<JDI_PAR_STATrs>;
///Register `JDI_PAR_STAT` writer
pub type W = crate::W<JDI_PAR_STATrs>;
///Field `HPOS` reader - jdi parallel horizontal position
pub type HposR = crate::FieldReader<u16>;
///Field `HPOS` writer - jdi parallel horizontal position
pub type HposW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `VPOS` reader - jdi parallel vertical position
pub type VposR = crate::FieldReader<u16>;
///Field `VPOS` writer - jdi parallel vertical position
pub type VposW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - jdi parallel horizontal position
    #[inline(always)]
    pub fn hpos(&self) -> HposR {
        HposR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - jdi parallel vertical position
    #[inline(always)]
    pub fn vpos(&self) -> VposR {
        VposR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JDI_PAR_STAT")
            .field("vpos", &self.vpos())
            .field("hpos", &self.hpos())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - jdi parallel horizontal position
    #[inline(always)]
    pub fn hpos(&mut self) -> HposW<JDI_PAR_STATrs> {
        HposW::new(self, 0)
    }
    ///Bits 16:31 - jdi parallel vertical position
    #[inline(always)]
    pub fn vpos(&mut self) -> VposW<JDI_PAR_STATrs> {
        VposW::new(self, 16)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`jdi_par_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdi_par_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct JDI_PAR_STATrs;
impl crate::RegisterSpec for JDI_PAR_STATrs {
    type Ux = u32;
}
///`read()` method returns [`jdi_par_stat::R`](R) reader structure
impl crate::Readable for JDI_PAR_STATrs {}
///`write(|w| ..)` method takes [`jdi_par_stat::W`](W) writer structure
impl crate::Writable for JDI_PAR_STATrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets JDI_PAR_STAT to value 0
impl crate::Resettable for JDI_PAR_STATrs {
    const RESET_VALUE: u32 = 0;
}
