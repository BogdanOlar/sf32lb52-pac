///Register `JDI_PAR_CONF2` reader
pub type R = crate::R<JDI_PAR_CONF2rs>;
///Register `JDI_PAR_CONF2` writer
pub type W = crate::W<JDI_PAR_CONF2rs>;
///Field `END_LINE` reader - jdi parallel interface end line, line number start from 0
pub type EndLineR = crate::FieldReader<u16>;
///Field `END_LINE` writer - jdi parallel interface end line, line number start from 0
pub type EndLineW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `ST_LINE` reader - jdi parallel interface start line, line number start from 0
pub type StLineR = crate::FieldReader<u16>;
///Field `ST_LINE` writer - jdi parallel interface start line, line number start from 0
pub type StLineW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - jdi parallel interface end line, line number start from 0
    #[inline(always)]
    pub fn end_line(&self) -> EndLineR {
        EndLineR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - jdi parallel interface start line, line number start from 0
    #[inline(always)]
    pub fn st_line(&self) -> StLineR {
        StLineR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JDI_PAR_CONF2")
            .field("st_line", &self.st_line())
            .field("end_line", &self.end_line())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - jdi parallel interface end line, line number start from 0
    #[inline(always)]
    pub fn end_line(&mut self) -> EndLineW<JDI_PAR_CONF2rs> {
        EndLineW::new(self, 0)
    }
    ///Bits 16:31 - jdi parallel interface start line, line number start from 0
    #[inline(always)]
    pub fn st_line(&mut self) -> StLineW<JDI_PAR_CONF2rs> {
        StLineW::new(self, 16)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`jdi_par_conf2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdi_par_conf2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct JDI_PAR_CONF2rs;
impl crate::RegisterSpec for JDI_PAR_CONF2rs {
    type Ux = u32;
}
///`read()` method returns [`jdi_par_conf2::R`](R) reader structure
impl crate::Readable for JDI_PAR_CONF2rs {}
///`write(|w| ..)` method takes [`jdi_par_conf2::W`](W) writer structure
impl crate::Writable for JDI_PAR_CONF2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets JDI_PAR_CONF2 to value 0
impl crate::Resettable for JDI_PAR_CONF2rs {
    const RESET_VALUE: u32 = 0;
}
