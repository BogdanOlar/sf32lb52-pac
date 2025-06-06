///Register `JDI_PAR_CONF7` reader
pub type R = crate::R<JDI_PAR_CONF7rs>;
///Register `JDI_PAR_CONF7` writer
pub type W = crate::W<JDI_PAR_CONF7rs>;
///Field `HCK_DLY` reader - jdi parallel interface HST to HCK delay
pub type HckDlyR = crate::FieldReader<u16>;
///Field `HCK_DLY` writer - jdi parallel interface HST to HCK delay
pub type HckDlyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `DP_MODE` reader - double pixel mode. Some jdi parallel screens use large pixel+small pixel structure. Set this bit to 1 to support this structure.
pub type DpModeR = crate::BitReader;
///Field `DP_MODE` writer - double pixel mode. Some jdi parallel screens use large pixel+small pixel structure. Set this bit to 1 to support this structure.
pub type DpModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    ///Bits 0:15 - jdi parallel interface HST to HCK delay
    #[inline(always)]
    pub fn hck_dly(&self) -> HckDlyR {
        HckDlyR::new((self.bits & 0xffff) as u16)
    }
    ///Bit 16 - double pixel mode. Some jdi parallel screens use large pixel+small pixel structure. Set this bit to 1 to support this structure.
    #[inline(always)]
    pub fn dp_mode(&self) -> DpModeR {
        DpModeR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JDI_PAR_CONF7")
            .field("rsvd", &self.rsvd())
            .field("dp_mode", &self.dp_mode())
            .field("hck_dly", &self.hck_dly())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - jdi parallel interface HST to HCK delay
    #[inline(always)]
    pub fn hck_dly(&mut self) -> HckDlyW<JDI_PAR_CONF7rs> {
        HckDlyW::new(self, 0)
    }
    ///Bit 16 - double pixel mode. Some jdi parallel screens use large pixel+small pixel structure. Set this bit to 1 to support this structure.
    #[inline(always)]
    pub fn dp_mode(&mut self) -> DpModeW<JDI_PAR_CONF7rs> {
        DpModeW::new(self, 16)
    }
    ///Bits 17:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<JDI_PAR_CONF7rs> {
        RsvdW::new(self, 17)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`jdi_par_conf7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdi_par_conf7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct JDI_PAR_CONF7rs;
impl crate::RegisterSpec for JDI_PAR_CONF7rs {
    type Ux = u32;
}
///`read()` method returns [`jdi_par_conf7::R`](R) reader structure
impl crate::Readable for JDI_PAR_CONF7rs {}
///`write(|w| ..)` method takes [`jdi_par_conf7::W`](W) writer structure
impl crate::Writable for JDI_PAR_CONF7rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets JDI_PAR_CONF7 to value 0
impl crate::Resettable for JDI_PAR_CONF7rs {
    const RESET_VALUE: u32 = 0;
}
