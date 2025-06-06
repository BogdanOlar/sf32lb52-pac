///Register `JDI_PAR_CONF4` reader
pub type R = crate::R<JDI_PAR_CONF4rs>;
///Register `JDI_PAR_CONF4` writer
pub type W = crate::W<JDI_PAR_CONF4rs>;
///Field `HST_WIDTH` reader - jdi parallel interface HST width, HST width = lcd_ck_cycle * HST_WIDTH
pub type HstWidthR = crate::FieldReader<u16>;
///Field `HST_WIDTH` writer - jdi parallel interface HST width, HST width = lcd_ck_cycle * HST_WIDTH
pub type HstWidthW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `HCK_WIDTH` reader - jdi parallel interface HCK width, HSK width = lcd_ck_cycle * HCK_WIDTH
pub type HckWidthR = crate::FieldReader<u16>;
///Field `HCK_WIDTH` writer - jdi parallel interface HCK width, HSK width = lcd_ck_cycle * HCK_WIDTH
pub type HckWidthW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - jdi parallel interface HST width, HST width = lcd_ck_cycle * HST_WIDTH
    #[inline(always)]
    pub fn hst_width(&self) -> HstWidthR {
        HstWidthR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - jdi parallel interface HCK width, HSK width = lcd_ck_cycle * HCK_WIDTH
    #[inline(always)]
    pub fn hck_width(&self) -> HckWidthR {
        HckWidthR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JDI_PAR_CONF4")
            .field("hck_width", &self.hck_width())
            .field("hst_width", &self.hst_width())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - jdi parallel interface HST width, HST width = lcd_ck_cycle * HST_WIDTH
    #[inline(always)]
    pub fn hst_width(&mut self) -> HstWidthW<JDI_PAR_CONF4rs> {
        HstWidthW::new(self, 0)
    }
    ///Bits 16:31 - jdi parallel interface HCK width, HSK width = lcd_ck_cycle * HCK_WIDTH
    #[inline(always)]
    pub fn hck_width(&mut self) -> HckWidthW<JDI_PAR_CONF4rs> {
        HckWidthW::new(self, 16)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`jdi_par_conf4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdi_par_conf4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct JDI_PAR_CONF4rs;
impl crate::RegisterSpec for JDI_PAR_CONF4rs {
    type Ux = u32;
}
///`read()` method returns [`jdi_par_conf4::R`](R) reader structure
impl crate::Readable for JDI_PAR_CONF4rs {}
///`write(|w| ..)` method takes [`jdi_par_conf4::W`](W) writer structure
impl crate::Writable for JDI_PAR_CONF4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets JDI_PAR_CONF4 to value 0
impl crate::Resettable for JDI_PAR_CONF4rs {
    const RESET_VALUE: u32 = 0;
}
