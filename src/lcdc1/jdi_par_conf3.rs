///Register `JDI_PAR_CONF3` reader
pub type R = crate::R<JDI_PAR_CONF3rs>;
///Register `JDI_PAR_CONF3` writer
pub type W = crate::W<JDI_PAR_CONF3rs>;
///Field `END_COL` reader - jdi parallel interface end column, column number start from 0
pub type EndColR = crate::FieldReader<u16>;
///Field `END_COL` writer - jdi parallel interface end column, column number start from 0
pub type EndColW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `ST_COL` reader - jdi parallel interface start column, column number start from 0
pub type StColR = crate::FieldReader<u16>;
///Field `ST_COL` writer - jdi parallel interface start column, column number start from 0
pub type StColW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - jdi parallel interface end column, column number start from 0
    #[inline(always)]
    pub fn end_col(&self) -> EndColR {
        EndColR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - jdi parallel interface start column, column number start from 0
    #[inline(always)]
    pub fn st_col(&self) -> StColR {
        StColR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JDI_PAR_CONF3")
            .field("st_col", &self.st_col())
            .field("end_col", &self.end_col())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - jdi parallel interface end column, column number start from 0
    #[inline(always)]
    pub fn end_col(&mut self) -> EndColW<JDI_PAR_CONF3rs> {
        EndColW::new(self, 0)
    }
    ///Bits 16:31 - jdi parallel interface start column, column number start from 0
    #[inline(always)]
    pub fn st_col(&mut self) -> StColW<JDI_PAR_CONF3rs> {
        StColW::new(self, 16)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`jdi_par_conf3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdi_par_conf3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct JDI_PAR_CONF3rs;
impl crate::RegisterSpec for JDI_PAR_CONF3rs {
    type Ux = u32;
}
///`read()` method returns [`jdi_par_conf3::R`](R) reader structure
impl crate::Readable for JDI_PAR_CONF3rs {}
///`write(|w| ..)` method takes [`jdi_par_conf3::W`](W) writer structure
impl crate::Writable for JDI_PAR_CONF3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets JDI_PAR_CONF3 to value 0
impl crate::Resettable for JDI_PAR_CONF3rs {
    const RESET_VALUE: u32 = 0;
}
