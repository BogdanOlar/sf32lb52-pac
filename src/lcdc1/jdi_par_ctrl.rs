///Register `JDI_PAR_CTRL` reader
pub type R = crate::R<JDI_PAR_CTRLrs>;
///Register `JDI_PAR_CTRL` writer
pub type W = crate::W<JDI_PAR_CTRLrs>;
///Field `ENABLE` reader - jdi parallel interface enable
pub type EnableR = crate::BitReader;
///Field `ENABLE` writer - jdi parallel interface enable
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XRST` reader - jdi parallel interface XRST
pub type XrstR = crate::BitReader;
///Field `XRST` writer - jdi parallel interface XRST
pub type XrstW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENBPOL` reader - jdi parallel enb polarity
pub type EnbpolR = crate::BitReader;
///Field `ENBPOL` writer - jdi parallel enb polarity
pub type EnbpolW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HCKPOL` reader - jdi parallel hck polarity
pub type HckpolR = crate::BitReader;
///Field `HCKPOL` writer - jdi parallel hck polarity
pub type HckpolW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSTPOL` reader - jdi parallel hst polarity
pub type HstpolR = crate::BitReader;
///Field `HSTPOL` writer - jdi parallel hst polarity
pub type HstpolW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VCKPOL` reader - jdi parallel vck polarity
pub type VckpolR = crate::BitReader;
///Field `VCKPOL` writer - jdi parallel vck polarity
pub type VckpolW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VSTPOL` reader - jdi parallel vst polarity
pub type VstpolR = crate::BitReader;
///Field `VSTPOL` writer - jdi parallel vst polarity
pub type VstpolW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INT_LINE_NUM` reader - jdi parallel interface interrupt line number, line number start from 0.
pub type IntLineNumR = crate::FieldReader<u16>;
///Field `INT_LINE_NUM` writer - jdi parallel interface interrupt line number, line number start from 0.
pub type IntLineNumW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bit 0 - jdi parallel interface enable
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    ///Bit 4 - jdi parallel interface XRST
    #[inline(always)]
    pub fn xrst(&self) -> XrstR {
        XrstR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - jdi parallel enb polarity
    #[inline(always)]
    pub fn enbpol(&self) -> EnbpolR {
        EnbpolR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - jdi parallel hck polarity
    #[inline(always)]
    pub fn hckpol(&self) -> HckpolR {
        HckpolR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - jdi parallel hst polarity
    #[inline(always)]
    pub fn hstpol(&self) -> HstpolR {
        HstpolR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - jdi parallel vck polarity
    #[inline(always)]
    pub fn vckpol(&self) -> VckpolR {
        VckpolR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - jdi parallel vst polarity
    #[inline(always)]
    pub fn vstpol(&self) -> VstpolR {
        VstpolR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 16:31 - jdi parallel interface interrupt line number, line number start from 0.
    #[inline(always)]
    pub fn int_line_num(&self) -> IntLineNumR {
        IntLineNumR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JDI_PAR_CTRL")
            .field("int_line_num", &self.int_line_num())
            .field("vstpol", &self.vstpol())
            .field("vckpol", &self.vckpol())
            .field("hstpol", &self.hstpol())
            .field("hckpol", &self.hckpol())
            .field("enbpol", &self.enbpol())
            .field("xrst", &self.xrst())
            .field("enable", &self.enable())
            .finish()
    }
}
impl W {
    ///Bit 0 - jdi parallel interface enable
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<JDI_PAR_CTRLrs> {
        EnableW::new(self, 0)
    }
    ///Bit 4 - jdi parallel interface XRST
    #[inline(always)]
    pub fn xrst(&mut self) -> XrstW<JDI_PAR_CTRLrs> {
        XrstW::new(self, 4)
    }
    ///Bit 5 - jdi parallel enb polarity
    #[inline(always)]
    pub fn enbpol(&mut self) -> EnbpolW<JDI_PAR_CTRLrs> {
        EnbpolW::new(self, 5)
    }
    ///Bit 6 - jdi parallel hck polarity
    #[inline(always)]
    pub fn hckpol(&mut self) -> HckpolW<JDI_PAR_CTRLrs> {
        HckpolW::new(self, 6)
    }
    ///Bit 7 - jdi parallel hst polarity
    #[inline(always)]
    pub fn hstpol(&mut self) -> HstpolW<JDI_PAR_CTRLrs> {
        HstpolW::new(self, 7)
    }
    ///Bit 8 - jdi parallel vck polarity
    #[inline(always)]
    pub fn vckpol(&mut self) -> VckpolW<JDI_PAR_CTRLrs> {
        VckpolW::new(self, 8)
    }
    ///Bit 9 - jdi parallel vst polarity
    #[inline(always)]
    pub fn vstpol(&mut self) -> VstpolW<JDI_PAR_CTRLrs> {
        VstpolW::new(self, 9)
    }
    ///Bits 16:31 - jdi parallel interface interrupt line number, line number start from 0.
    #[inline(always)]
    pub fn int_line_num(&mut self) -> IntLineNumW<JDI_PAR_CTRLrs> {
        IntLineNumW::new(self, 16)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`jdi_par_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jdi_par_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct JDI_PAR_CTRLrs;
impl crate::RegisterSpec for JDI_PAR_CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`jdi_par_ctrl::R`](R) reader structure
impl crate::Readable for JDI_PAR_CTRLrs {}
///`write(|w| ..)` method takes [`jdi_par_ctrl::W`](W) writer structure
impl crate::Writable for JDI_PAR_CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets JDI_PAR_CTRL to value 0
impl crate::Resettable for JDI_PAR_CTRLrs {
    const RESET_VALUE: u32 = 0;
}
