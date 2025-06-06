///Register `JDI_PAR_CONF6` reader
pub type R = crate::R<JDI_PAR_CONF6rs>;
///Register `JDI_PAR_CONF6` writer
pub type W = crate::W<JDI_PAR_CONF6rs>;
///Field `HST_DLY` reader - jdi parallel interface VCK to HST delay, VCK2HST delay = lcd_ck_cycle * HST_DLY
pub type HstDlyR = crate::FieldReader<u16>;
///Field `HST_DLY` writer - jdi parallel interface VCK to HST delay, VCK2HST delay = lcd_ck_cycle * HST_DLY
pub type HstDlyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `VCK_DLY` reader - jdi parallel interface VST to VCK delay, VST2VCK delay = lcd_ck_cycle * VCK_DLY
pub type VckDlyR = crate::FieldReader<u16>;
///Field `VCK_DLY` writer - jdi parallel interface VST to VCK delay, VST2VCK delay = lcd_ck_cycle * VCK_DLY
pub type VckDlyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - jdi parallel interface VCK to HST delay, VCK2HST delay = lcd_ck_cycle * HST_DLY
    #[inline(always)]
    pub fn hst_dly(&self) -> HstDlyR {
        HstDlyR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - jdi parallel interface VST to VCK delay, VST2VCK delay = lcd_ck_cycle * VCK_DLY
    #[inline(always)]
    pub fn vck_dly(&self) -> VckDlyR {
        VckDlyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JDI_PAR_CONF6")
            .field("vck_dly", &self.vck_dly())
            .field("hst_dly", &self.hst_dly())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - jdi parallel interface VCK to HST delay, VCK2HST delay = lcd_ck_cycle * HST_DLY
    #[inline(always)]
    pub fn hst_dly(&mut self) -> HstDlyW<JDI_PAR_CONF6rs> {
        HstDlyW::new(self, 0)
    }
    ///Bits 16:31 - jdi parallel interface VST to VCK delay, VST2VCK delay = lcd_ck_cycle * VCK_DLY
    #[inline(always)]
    pub fn vck_dly(&mut self) -> VckDlyW<JDI_PAR_CONF6rs> {
        VckDlyW::new(self, 16)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`jdi_par_conf6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdi_par_conf6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct JDI_PAR_CONF6rs;
impl crate::RegisterSpec for JDI_PAR_CONF6rs {
    type Ux = u32;
}
///`read()` method returns [`jdi_par_conf6::R`](R) reader structure
impl crate::Readable for JDI_PAR_CONF6rs {}
///`write(|w| ..)` method takes [`jdi_par_conf6::W`](W) writer structure
impl crate::Writable for JDI_PAR_CONF6rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets JDI_PAR_CONF6 to value 0
impl crate::Resettable for JDI_PAR_CONF6rs {
    const RESET_VALUE: u32 = 0;
}
