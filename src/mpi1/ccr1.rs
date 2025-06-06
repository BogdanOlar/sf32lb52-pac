///Register `CCR1` reader
pub type R = crate::R<CCR1rs>;
///Register `CCR1` writer
pub type W = crate::W<CCR1rs>;
///Field `IMODE` reader - Instruction mode 0: no instruction phase 1: single line 2: dual lines 3: quad lines 4/5/6 - reserved 7 - quad lines DDR
pub type ImodeR = crate::FieldReader;
///Field `IMODE` writer - Instruction mode 0: no instruction phase 1: single line 2: dual lines 3: quad lines 4/5/6 - reserved 7 - quad lines DDR
pub type ImodeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ADMODE` reader - Address mode 0: no address phase 1: single line 2: dual line 3: quad line 4/5/6: reserved 7: quad line DDR
pub type AdmodeR = crate::FieldReader;
///Field `ADMODE` writer - Address mode 0: no address phase 1: single line 2: dual line 3: quad line 4/5/6: reserved 7: quad line DDR
pub type AdmodeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ADSIZE` reader - Address size 0: one byte 1: two bytes 2: three bytes 3: four bytes
pub type AdsizeR = crate::FieldReader;
///Field `ADSIZE` writer - Address size 0: one byte 1: two bytes 2: three bytes 3: four bytes
pub type AdsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ABMODE` reader - Alternate byte mode 0: no alternate byte 1: single line 2: dual lines 3: quad lines 4/5/6: reserved 7: quad lines DDR
pub type AbmodeR = crate::FieldReader;
///Field `ABMODE` writer - Alternate byte mode 0: no alternate byte 1: single line 2: dual lines 3: quad lines 4/5/6: reserved 7: quad lines DDR
pub type AbmodeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ABSIZE` reader - Alternate byte size 0: one byte 1: two bytes 2: three bytes 3: four bytes
pub type AbsizeR = crate::FieldReader;
///Field `ABSIZE` writer - Alternate byte size 0: one byte 1: two bytes 2: three bytes 3: four bytes
pub type AbsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DCYC` reader - Number of dummy cycles 0: no dummy cycle 1: one dummy cycle 2: two dummy cycles
pub type DcycR = crate::FieldReader;
///Field `DCYC` writer - Number of dummy cycles 0: no dummy cycle 1: one dummy cycle 2: two dummy cycles
pub type DcycW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DMODE` reader - Data Mode 0: no data phase 1: single line 2: dual lines 3: quad lines 4/5/6: reserved 7: quad lines DDR
pub type DmodeR = crate::FieldReader;
///Field `DMODE` writer - Data Mode 0: no data phase 1: single line 2: dual lines 3: quad lines 4/5/6: reserved 7: quad lines DDR
pub type DmodeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `FMODE` reader - Function Mode 0: read mode 1: write mode
pub type FmodeR = crate::BitReader;
///Field `FMODE` writer - Function Mode 0: read mode 1: write mode
pub type FmodeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:2 - Instruction mode 0: no instruction phase 1: single line 2: dual lines 3: quad lines 4/5/6 - reserved 7 - quad lines DDR
    #[inline(always)]
    pub fn imode(&self) -> ImodeR {
        ImodeR::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - Address mode 0: no address phase 1: single line 2: dual line 3: quad line 4/5/6: reserved 7: quad line DDR
    #[inline(always)]
    pub fn admode(&self) -> AdmodeR {
        AdmodeR::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:7 - Address size 0: one byte 1: two bytes 2: three bytes 3: four bytes
    #[inline(always)]
    pub fn adsize(&self) -> AdsizeR {
        AdsizeR::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:10 - Alternate byte mode 0: no alternate byte 1: single line 2: dual lines 3: quad lines 4/5/6: reserved 7: quad lines DDR
    #[inline(always)]
    pub fn abmode(&self) -> AbmodeR {
        AbmodeR::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 11:12 - Alternate byte size 0: one byte 1: two bytes 2: three bytes 3: four bytes
    #[inline(always)]
    pub fn absize(&self) -> AbsizeR {
        AbsizeR::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bits 13:17 - Number of dummy cycles 0: no dummy cycle 1: one dummy cycle 2: two dummy cycles
    #[inline(always)]
    pub fn dcyc(&self) -> DcycR {
        DcycR::new(((self.bits >> 13) & 0x1f) as u8)
    }
    ///Bits 18:20 - Data Mode 0: no data phase 1: single line 2: dual lines 3: quad lines 4/5/6: reserved 7: quad lines DDR
    #[inline(always)]
    pub fn dmode(&self) -> DmodeR {
        DmodeR::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bit 21 - Function Mode 0: read mode 1: write mode
    #[inline(always)]
    pub fn fmode(&self) -> FmodeR {
        FmodeR::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 22:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR1")
            .field("rsvd", &self.rsvd())
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
    ///Bits 0:2 - Instruction mode 0: no instruction phase 1: single line 2: dual lines 3: quad lines 4/5/6 - reserved 7 - quad lines DDR
    #[inline(always)]
    pub fn imode(&mut self) -> ImodeW<CCR1rs> {
        ImodeW::new(self, 0)
    }
    ///Bits 3:5 - Address mode 0: no address phase 1: single line 2: dual line 3: quad line 4/5/6: reserved 7: quad line DDR
    #[inline(always)]
    pub fn admode(&mut self) -> AdmodeW<CCR1rs> {
        AdmodeW::new(self, 3)
    }
    ///Bits 6:7 - Address size 0: one byte 1: two bytes 2: three bytes 3: four bytes
    #[inline(always)]
    pub fn adsize(&mut self) -> AdsizeW<CCR1rs> {
        AdsizeW::new(self, 6)
    }
    ///Bits 8:10 - Alternate byte mode 0: no alternate byte 1: single line 2: dual lines 3: quad lines 4/5/6: reserved 7: quad lines DDR
    #[inline(always)]
    pub fn abmode(&mut self) -> AbmodeW<CCR1rs> {
        AbmodeW::new(self, 8)
    }
    ///Bits 11:12 - Alternate byte size 0: one byte 1: two bytes 2: three bytes 3: four bytes
    #[inline(always)]
    pub fn absize(&mut self) -> AbsizeW<CCR1rs> {
        AbsizeW::new(self, 11)
    }
    ///Bits 13:17 - Number of dummy cycles 0: no dummy cycle 1: one dummy cycle 2: two dummy cycles
    #[inline(always)]
    pub fn dcyc(&mut self) -> DcycW<CCR1rs> {
        DcycW::new(self, 13)
    }
    ///Bits 18:20 - Data Mode 0: no data phase 1: single line 2: dual lines 3: quad lines 4/5/6: reserved 7: quad lines DDR
    #[inline(always)]
    pub fn dmode(&mut self) -> DmodeW<CCR1rs> {
        DmodeW::new(self, 18)
    }
    ///Bit 21 - Function Mode 0: read mode 1: write mode
    #[inline(always)]
    pub fn fmode(&mut self) -> FmodeW<CCR1rs> {
        FmodeW::new(self, 21)
    }
    ///Bits 22:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<CCR1rs> {
        RsvdW::new(self, 22)
    }
}
///Communication Configuration Register
///
///You can [`read`](crate::Reg::read) this register and get [`ccr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CCR1rs;
impl crate::RegisterSpec for CCR1rs {
    type Ux = u32;
}
///`read()` method returns [`ccr1::R`](R) reader structure
impl crate::Readable for CCR1rs {}
///`write(|w| ..)` method takes [`ccr1::W`](W) writer structure
impl crate::Writable for CCR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CCR1 to value 0
impl crate::Resettable for CCR1rs {
    const RESET_VALUE: u32 = 0;
}
