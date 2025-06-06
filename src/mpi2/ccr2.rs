///Register `CCR2` reader
pub type R = crate::R<CCR2rs>;
///Register `CCR2` writer
pub type W = crate::W<CCR2rs>;
///Field `IMODE` reader - This register specifies the format of CMD2 sequence. Refer to CCR1 description
pub type ImodeR = crate::FieldReader;
///Field `IMODE` writer - This register specifies the format of CMD2 sequence. Refer to CCR1 description
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
///Field `FMODE` reader -
pub type FmodeR = crate::BitReader;
///Field `FMODE` writer -
pub type FmodeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - This register specifies the format of CMD2 sequence. Refer to CCR1 description
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
    ///Bit 21
    #[inline(always)]
    pub fn fmode(&self) -> FmodeR {
        FmodeR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR2")
            .field("fmode", &self.fmode())
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
    ///Bits 0:2 - This register specifies the format of CMD2 sequence. Refer to CCR1 description
    #[inline(always)]
    pub fn imode(&mut self) -> ImodeW<CCR2rs> {
        ImodeW::new(self, 0)
    }
    ///Bits 3:5
    #[inline(always)]
    pub fn admode(&mut self) -> AdmodeW<CCR2rs> {
        AdmodeW::new(self, 3)
    }
    ///Bits 6:7
    #[inline(always)]
    pub fn adsize(&mut self) -> AdsizeW<CCR2rs> {
        AdsizeW::new(self, 6)
    }
    ///Bits 8:10
    #[inline(always)]
    pub fn abmode(&mut self) -> AbmodeW<CCR2rs> {
        AbmodeW::new(self, 8)
    }
    ///Bits 11:12
    #[inline(always)]
    pub fn absize(&mut self) -> AbsizeW<CCR2rs> {
        AbsizeW::new(self, 11)
    }
    ///Bits 13:17
    #[inline(always)]
    pub fn dcyc(&mut self) -> DcycW<CCR2rs> {
        DcycW::new(self, 13)
    }
    ///Bits 18:20
    #[inline(always)]
    pub fn dmode(&mut self) -> DmodeW<CCR2rs> {
        DmodeW::new(self, 18)
    }
    ///Bit 21
    #[inline(always)]
    pub fn fmode(&mut self) -> FmodeW<CCR2rs> {
        FmodeW::new(self, 21)
    }
}
///Communication Configuration Register
///
///You can [`read`](crate::Reg::read) this register and get [`ccr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CCR2rs;
impl crate::RegisterSpec for CCR2rs {
    type Ux = u32;
}
///`read()` method returns [`ccr2::R`](R) reader structure
impl crate::Readable for CCR2rs {}
///`write(|w| ..)` method takes [`ccr2::W`](W) writer structure
impl crate::Writable for CCR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CCR2 to value 0
impl crate::Resettable for CCR2rs {
    const RESET_VALUE: u32 = 0;
}
