///Register `JDI_PAR_CONF1` reader
pub type R = crate::R<JDI_PAR_CONF1rs>;
///Register `JDI_PAR_CONF1` writer
pub type W = crate::W<JDI_PAR_CONF1rs>;
///Field `MAX_COL` reader - jdi parallel interface max column, column number start from 0
pub type MaxColR = crate::FieldReader<u16>;
///Field `MAX_COL` writer - jdi parallel interface max column, column number start from 0
pub type MaxColW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `MAX_LINE` reader - jdi parallel interface max line, line number start from 0
pub type MaxLineR = crate::FieldReader<u16>;
///Field `MAX_LINE` writer - jdi parallel interface max line, line number start from 0
pub type MaxLineW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - jdi parallel interface max column, column number start from 0
    #[inline(always)]
    pub fn max_col(&self) -> MaxColR {
        MaxColR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - jdi parallel interface max line, line number start from 0
    #[inline(always)]
    pub fn max_line(&self) -> MaxLineR {
        MaxLineR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JDI_PAR_CONF1")
            .field("max_line", &self.max_line())
            .field("max_col", &self.max_col())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - jdi parallel interface max column, column number start from 0
    #[inline(always)]
    pub fn max_col(&mut self) -> MaxColW<JDI_PAR_CONF1rs> {
        MaxColW::new(self, 0)
    }
    ///Bits 16:31 - jdi parallel interface max line, line number start from 0
    #[inline(always)]
    pub fn max_line(&mut self) -> MaxLineW<JDI_PAR_CONF1rs> {
        MaxLineW::new(self, 16)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`jdi_par_conf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdi_par_conf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct JDI_PAR_CONF1rs;
impl crate::RegisterSpec for JDI_PAR_CONF1rs {
    type Ux = u32;
}
///`read()` method returns [`jdi_par_conf1::R`](R) reader structure
impl crate::Readable for JDI_PAR_CONF1rs {}
///`write(|w| ..)` method takes [`jdi_par_conf1::W`](W) writer structure
impl crate::Writable for JDI_PAR_CONF1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets JDI_PAR_CONF1 to value 0
impl crate::Resettable for JDI_PAR_CONF1rs {
    const RESET_VALUE: u32 = 0;
}
