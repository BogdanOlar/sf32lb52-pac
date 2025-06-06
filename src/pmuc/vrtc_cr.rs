///Register `VRTC_CR` reader
pub type R = crate::R<VRTC_CRrs>;
///Register `VRTC_CR` writer
pub type W = crate::W<VRTC_CRrs>;
///Field `VRTC_VBIT` reader -
pub type VrtcVbitR = crate::FieldReader;
///Field `VRTC_VBIT` writer -
pub type VrtcVbitW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `VRTC_TRIM` reader -
pub type VrtcTrimR = crate::FieldReader;
///Field `VRTC_TRIM` writer -
pub type VrtcTrimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `BOR_EN` reader - Brownout Reset Enable
pub type BorEnR = crate::BitReader;
///Field `BOR_EN` writer - Brownout Reset Enable
pub type BorEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOR_VT_TRIM` reader -
pub type BorVtTrimR = crate::FieldReader;
///Field `BOR_VT_TRIM` writer -
pub type BorVtTrimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3
    #[inline(always)]
    pub fn vrtc_vbit(&self) -> VrtcVbitR {
        VrtcVbitR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7
    #[inline(always)]
    pub fn vrtc_trim(&self) -> VrtcTrimR {
        VrtcTrimR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 8 - Brownout Reset Enable
    #[inline(always)]
    pub fn bor_en(&self) -> BorEnR {
        BorEnR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:12
    #[inline(always)]
    pub fn bor_vt_trim(&self) -> BorVtTrimR {
        BorVtTrimR::new(((self.bits >> 9) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VRTC_CR")
            .field("bor_vt_trim", &self.bor_vt_trim())
            .field("bor_en", &self.bor_en())
            .field("vrtc_trim", &self.vrtc_trim())
            .field("vrtc_vbit", &self.vrtc_vbit())
            .finish()
    }
}
impl W {
    ///Bits 0:3
    #[inline(always)]
    pub fn vrtc_vbit(&mut self) -> VrtcVbitW<VRTC_CRrs> {
        VrtcVbitW::new(self, 0)
    }
    ///Bits 4:7
    #[inline(always)]
    pub fn vrtc_trim(&mut self) -> VrtcTrimW<VRTC_CRrs> {
        VrtcTrimW::new(self, 4)
    }
    ///Bit 8 - Brownout Reset Enable
    #[inline(always)]
    pub fn bor_en(&mut self) -> BorEnW<VRTC_CRrs> {
        BorEnW::new(self, 8)
    }
    ///Bits 9:12
    #[inline(always)]
    pub fn bor_vt_trim(&mut self) -> BorVtTrimW<VRTC_CRrs> {
        BorVtTrimW::new(self, 9)
    }
}
///VRTC Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`vrtc_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vrtc_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct VRTC_CRrs;
impl crate::RegisterSpec for VRTC_CRrs {
    type Ux = u32;
}
///`read()` method returns [`vrtc_cr::R`](R) reader structure
impl crate::Readable for VRTC_CRrs {}
///`write(|w| ..)` method takes [`vrtc_cr::W`](W) writer structure
impl crate::Writable for VRTC_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets VRTC_CR to value 0
impl crate::Resettable for VRTC_CRrs {
    const RESET_VALUE: u32 = 0;
}
