///Register `JDI_PAR_CONF10` reader
pub type R = crate::R<JDI_PAR_CONF10rs>;
///Register `JDI_PAR_CONF10` writer
pub type W = crate::W<JDI_PAR_CONF10rs>;
///Field `HC_END_LINE` reader - jdi parallel interface horizontal control end line, line number start from 0
pub type HcEndLineR = crate::FieldReader<u16>;
///Field `HC_END_LINE` writer - jdi parallel interface horizontal control end line, line number start from 0
pub type HcEndLineW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `HC_ST_LINE` reader - jdi parallel interface horizontal control start line, line number start from 0
pub type HcStLineR = crate::FieldReader<u16>;
///Field `HC_ST_LINE` writer - jdi parallel interface horizontal control start line, line number start from 0
pub type HcStLineW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - jdi parallel interface horizontal control end line, line number start from 0
    #[inline(always)]
    pub fn hc_end_line(&self) -> HcEndLineR {
        HcEndLineR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - jdi parallel interface horizontal control start line, line number start from 0
    #[inline(always)]
    pub fn hc_st_line(&self) -> HcStLineR {
        HcStLineR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JDI_PAR_CONF10")
            .field("hc_st_line", &self.hc_st_line())
            .field("hc_end_line", &self.hc_end_line())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - jdi parallel interface horizontal control end line, line number start from 0
    #[inline(always)]
    pub fn hc_end_line(&mut self) -> HcEndLineW<JDI_PAR_CONF10rs> {
        HcEndLineW::new(self, 0)
    }
    ///Bits 16:31 - jdi parallel interface horizontal control start line, line number start from 0
    #[inline(always)]
    pub fn hc_st_line(&mut self) -> HcStLineW<JDI_PAR_CONF10rs> {
        HcStLineW::new(self, 16)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`jdi_par_conf10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdi_par_conf10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct JDI_PAR_CONF10rs;
impl crate::RegisterSpec for JDI_PAR_CONF10rs {
    type Ux = u32;
}
///`read()` method returns [`jdi_par_conf10::R`](R) reader structure
impl crate::Readable for JDI_PAR_CONF10rs {}
///`write(|w| ..)` method takes [`jdi_par_conf10::W`](W) writer structure
impl crate::Writable for JDI_PAR_CONF10rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets JDI_PAR_CONF10 to value 0
impl crate::Resettable for JDI_PAR_CONF10rs {
    const RESET_VALUE: u32 = 0;
}
