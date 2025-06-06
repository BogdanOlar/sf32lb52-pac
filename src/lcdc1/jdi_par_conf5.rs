///Register `JDI_PAR_CONF5` reader
pub type R = crate::R<JDI_PAR_CONF5rs>;
///Register `JDI_PAR_CONF5` writer
pub type W = crate::W<JDI_PAR_CONF5rs>;
///Field `VST_WIDTH` reader - jdi parallel interface VST width, VST width = lcd_ck_cycle * VST_WIDTH
pub type VstWidthR = crate::FieldReader<u16>;
///Field `VST_WIDTH` writer - jdi parallel interface VST width, VST width = lcd_ck_cycle * VST_WIDTH
pub type VstWidthW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `VCK_WIDTH` reader - jdi parallel interface VCK width, VCK width = lcd_ck_cycle * VCK_WIDTH
pub type VckWidthR = crate::FieldReader<u16>;
///Field `VCK_WIDTH` writer - jdi parallel interface VCK width, VCK width = lcd_ck_cycle * VCK_WIDTH
pub type VckWidthW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - jdi parallel interface VST width, VST width = lcd_ck_cycle * VST_WIDTH
    #[inline(always)]
    pub fn vst_width(&self) -> VstWidthR {
        VstWidthR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - jdi parallel interface VCK width, VCK width = lcd_ck_cycle * VCK_WIDTH
    #[inline(always)]
    pub fn vck_width(&self) -> VckWidthR {
        VckWidthR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JDI_PAR_CONF5")
            .field("vck_width", &self.vck_width())
            .field("vst_width", &self.vst_width())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - jdi parallel interface VST width, VST width = lcd_ck_cycle * VST_WIDTH
    #[inline(always)]
    pub fn vst_width(&mut self) -> VstWidthW<JDI_PAR_CONF5rs> {
        VstWidthW::new(self, 0)
    }
    ///Bits 16:31 - jdi parallel interface VCK width, VCK width = lcd_ck_cycle * VCK_WIDTH
    #[inline(always)]
    pub fn vck_width(&mut self) -> VckWidthW<JDI_PAR_CONF5rs> {
        VckWidthW::new(self, 16)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`jdi_par_conf5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdi_par_conf5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct JDI_PAR_CONF5rs;
impl crate::RegisterSpec for JDI_PAR_CONF5rs {
    type Ux = u32;
}
///`read()` method returns [`jdi_par_conf5::R`](R) reader structure
impl crate::Readable for JDI_PAR_CONF5rs {}
///`write(|w| ..)` method takes [`jdi_par_conf5::W`](W) writer structure
impl crate::Writable for JDI_PAR_CONF5rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets JDI_PAR_CONF5 to value 0
impl crate::Resettable for JDI_PAR_CONF5rs {
    const RESET_VALUE: u32 = 0;
}
