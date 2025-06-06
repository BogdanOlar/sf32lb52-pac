///Register `JDI_PAR_CONF9` reader
pub type R = crate::R<JDI_PAR_CONF9rs>;
///Register `JDI_PAR_CONF9` writer
pub type W = crate::W<JDI_PAR_CONF9rs>;
///Field `ENB_END_LINE` reader - jdi parallel interface enb end line, line number start from 0
pub type EnbEndLineR = crate::FieldReader<u16>;
///Field `ENB_END_LINE` writer - jdi parallel interface enb end line, line number start from 0
pub type EnbEndLineW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `ENB_ST_LINE` reader - jdi parallel interface enb start line, line number start from 0
pub type EnbStLineR = crate::FieldReader<u16>;
///Field `ENB_ST_LINE` writer - jdi parallel interface enb start line, line number start from 0
pub type EnbStLineW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - jdi parallel interface enb end line, line number start from 0
    #[inline(always)]
    pub fn enb_end_line(&self) -> EnbEndLineR {
        EnbEndLineR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - jdi parallel interface enb start line, line number start from 0
    #[inline(always)]
    pub fn enb_st_line(&self) -> EnbStLineR {
        EnbStLineR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JDI_PAR_CONF9")
            .field("enb_st_line", &self.enb_st_line())
            .field("enb_end_line", &self.enb_end_line())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - jdi parallel interface enb end line, line number start from 0
    #[inline(always)]
    pub fn enb_end_line(&mut self) -> EnbEndLineW<JDI_PAR_CONF9rs> {
        EnbEndLineW::new(self, 0)
    }
    ///Bits 16:31 - jdi parallel interface enb start line, line number start from 0
    #[inline(always)]
    pub fn enb_st_line(&mut self) -> EnbStLineW<JDI_PAR_CONF9rs> {
        EnbStLineW::new(self, 16)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`jdi_par_conf9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdi_par_conf9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct JDI_PAR_CONF9rs;
impl crate::RegisterSpec for JDI_PAR_CONF9rs {
    type Ux = u32;
}
///`read()` method returns [`jdi_par_conf9::R`](R) reader structure
impl crate::Readable for JDI_PAR_CONF9rs {}
///`write(|w| ..)` method takes [`jdi_par_conf9::W`](W) writer structure
impl crate::Writable for JDI_PAR_CONF9rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets JDI_PAR_CONF9 to value 0
impl crate::Resettable for JDI_PAR_CONF9rs {
    const RESET_VALUE: u32 = 0;
}
