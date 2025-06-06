///Register `HRCCR` reader
pub type R = crate::R<HRCCRrs>;
///Register `HRCCR` writer
pub type W = crate::W<HRCCRrs>;
///Field `IMODE` reader - This register specifies the format of AHB read command sequence. Refer to CCR1 description
pub type ImodeR = crate::FieldReader;
///Field `IMODE` writer - This register specifies the format of AHB read command sequence. Refer to CCR1 description
pub type ImodeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ADMODE` reader -
pub type AdmodeR = crate::FieldReader;
///Field `ADMODE` writer -
pub type AdmodeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ADSIZE` reader -
pub type AdsizeR = crate::FieldReader;
///Field `ADSIZE` writer -
pub type AdsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ABMODE` reader -
pub type AbmodeR = crate::FieldReader;
///Field `ABMODE` writer -
pub type AbmodeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ABSIZE` reader -
pub type AbsizeR = crate::FieldReader;
///Field `ABSIZE` writer -
pub type AbsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DCYC` reader -
pub type DcycR = crate::FieldReader;
///Field `DCYC` writer -
pub type DcycW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DMODE` reader -
pub type DmodeR = crate::FieldReader;
///Field `DMODE` writer -
pub type DmodeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    ///Bits 0:2 - This register specifies the format of AHB read command sequence. Refer to CCR1 description
    #[inline(always)]
    pub fn imode(&self) -> ImodeR {
        ImodeR::new((self.bits & 7) as u8)
    }
    ///Bits 3:5
    #[inline(always)]
    pub fn admode(&self) -> AdmodeR {
        AdmodeR::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:7
    #[inline(always)]
    pub fn adsize(&self) -> AdsizeR {
        AdsizeR::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:10
    #[inline(always)]
    pub fn abmode(&self) -> AbmodeR {
        AbmodeR::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 11:12
    #[inline(always)]
    pub fn absize(&self) -> AbsizeR {
        AbsizeR::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bits 13:17
    #[inline(always)]
    pub fn dcyc(&self) -> DcycR {
        DcycR::new(((self.bits >> 13) & 0x1f) as u8)
    }
    ///Bits 18:20
    #[inline(always)]
    pub fn dmode(&self) -> DmodeR {
        DmodeR::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bits 21:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HRCCR")
            .field("rsvd", &self.rsvd())
            .field("dmode", &self.dmode())
            .field("dcyc", &self.dcyc())
            .field("absize", &self.absize())
            .field("abmode", &self.abmode())
            .field("adsize", &self.adsize())
            .field("admode", &self.admode())
            .field("imode", &self.imode())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - This register specifies the format of AHB read command sequence. Refer to CCR1 description
    #[inline(always)]
    pub fn imode(&mut self) -> ImodeW<HRCCRrs> {
        ImodeW::new(self, 0)
    }
    ///Bits 3:5
    #[inline(always)]
    pub fn admode(&mut self) -> AdmodeW<HRCCRrs> {
        AdmodeW::new(self, 3)
    }
    ///Bits 6:7
    #[inline(always)]
    pub fn adsize(&mut self) -> AdsizeW<HRCCRrs> {
        AdsizeW::new(self, 6)
    }
    ///Bits 8:10
    #[inline(always)]
    pub fn abmode(&mut self) -> AbmodeW<HRCCRrs> {
        AbmodeW::new(self, 8)
    }
    ///Bits 11:12
    #[inline(always)]
    pub fn absize(&mut self) -> AbsizeW<HRCCRrs> {
        AbsizeW::new(self, 11)
    }
    ///Bits 13:17
    #[inline(always)]
    pub fn dcyc(&mut self) -> DcycW<HRCCRrs> {
        DcycW::new(self, 13)
    }
    ///Bits 18:20
    #[inline(always)]
    pub fn dmode(&mut self) -> DmodeW<HRCCRrs> {
        DmodeW::new(self, 18)
    }
    ///Bits 21:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<HRCCRrs> {
        RsvdW::new(self, 21)
    }
}
///AHB Read Communication Configuration Register
///
///You can [`read`](crate::Reg::read) this register and get [`hrccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hrccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct HRCCRrs;
impl crate::RegisterSpec for HRCCRrs {
    type Ux = u32;
}
///`read()` method returns [`hrccr::R`](R) reader structure
impl crate::Readable for HRCCRrs {}
///`write(|w| ..)` method takes [`hrccr::W`](W) writer structure
impl crate::Writable for HRCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HRCCR to value 0
impl crate::Resettable for HRCCRrs {
    const RESET_VALUE: u32 = 0;
}
