///Register `LAYER0_DECOMP_CFG0` reader
pub type R = crate::R<LAYER0_DECOMP_CFG0rs>;
///Register `LAYER0_DECOMP_CFG0` writer
pub type W = crate::W<LAYER0_DECOMP_CFG0rs>;
///Field `EXTRA_HIGH` reader - extra bit for high quality bit
pub type ExtraHighR = crate::FieldReader;
///Field `EXTRA_HIGH` writer - extra bit for high quality bit
pub type ExtraHighW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `EXTRA_THRESHOLD` reader - the threshold to distinguish high/low quality block
pub type ExtraThresholdR = crate::FieldReader;
///Field `EXTRA_THRESHOLD` writer - the threshold to distinguish high/low quality block
pub type ExtraThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `USE_LOSSLESS_QIDX` reader - condition to increase qidx
pub type UseLosslessQidxR = crate::FieldReader;
///Field `USE_LOSSLESS_QIDX` writer - condition to increase qidx
pub type UseLosslessQidxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `LOSSLESS_QIDX1` reader - up level for adjusted qidx value for low quality block
pub type LosslessQidx1R = crate::FieldReader;
///Field `LOSSLESS_QIDX1` writer - up level for adjusted qidx value for low quality block
pub type LosslessQidx1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `LOSSLESS_QIDX2` reader - condition to decrease qidx
pub type LosslessQidx2R = crate::FieldReader;
///Field `LOSSLESS_QIDX2` writer - condition to decrease qidx
pub type LosslessQidx2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CFG0_RESERVED` reader -
pub type Cfg0ReservedR = crate::FieldReader<u16>;
///Field `CFG0_RESERVED` writer -
pub type Cfg0ReservedW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:3 - extra bit for high quality bit
    #[inline(always)]
    pub fn extra_high(&self) -> ExtraHighR {
        ExtraHighR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - the threshold to distinguish high/low quality block
    #[inline(always)]
    pub fn extra_threshold(&self) -> ExtraThresholdR {
        ExtraThresholdR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - condition to increase qidx
    #[inline(always)]
    pub fn use_lossless_qidx(&self) -> UseLosslessQidxR {
        UseLosslessQidxR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - up level for adjusted qidx value for low quality block
    #[inline(always)]
    pub fn lossless_qidx1(&self) -> LosslessQidx1R {
        LosslessQidx1R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - condition to decrease qidx
    #[inline(always)]
    pub fn lossless_qidx2(&self) -> LosslessQidx2R {
        LosslessQidx2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:31
    #[inline(always)]
    pub fn cfg0_reserved(&self) -> Cfg0ReservedR {
        Cfg0ReservedR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LAYER0_DECOMP_CFG0")
            .field("cfg0_reserved", &self.cfg0_reserved())
            .field("lossless_qidx2", &self.lossless_qidx2())
            .field("lossless_qidx1", &self.lossless_qidx1())
            .field("use_lossless_qidx", &self.use_lossless_qidx())
            .field("extra_threshold", &self.extra_threshold())
            .field("extra_high", &self.extra_high())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - extra bit for high quality bit
    #[inline(always)]
    pub fn extra_high(&mut self) -> ExtraHighW<LAYER0_DECOMP_CFG0rs> {
        ExtraHighW::new(self, 0)
    }
    ///Bits 4:7 - the threshold to distinguish high/low quality block
    #[inline(always)]
    pub fn extra_threshold(&mut self) -> ExtraThresholdW<LAYER0_DECOMP_CFG0rs> {
        ExtraThresholdW::new(self, 4)
    }
    ///Bits 8:11 - condition to increase qidx
    #[inline(always)]
    pub fn use_lossless_qidx(&mut self) -> UseLosslessQidxW<LAYER0_DECOMP_CFG0rs> {
        UseLosslessQidxW::new(self, 8)
    }
    ///Bits 12:15 - up level for adjusted qidx value for low quality block
    #[inline(always)]
    pub fn lossless_qidx1(&mut self) -> LosslessQidx1W<LAYER0_DECOMP_CFG0rs> {
        LosslessQidx1W::new(self, 12)
    }
    ///Bits 16:19 - condition to decrease qidx
    #[inline(always)]
    pub fn lossless_qidx2(&mut self) -> LosslessQidx2W<LAYER0_DECOMP_CFG0rs> {
        LosslessQidx2W::new(self, 16)
    }
    ///Bits 20:31
    #[inline(always)]
    pub fn cfg0_reserved(&mut self) -> Cfg0ReservedW<LAYER0_DECOMP_CFG0rs> {
        Cfg0ReservedW::new(self, 20)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`layer0_decomp_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer0_decomp_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct LAYER0_DECOMP_CFG0rs;
impl crate::RegisterSpec for LAYER0_DECOMP_CFG0rs {
    type Ux = u32;
}
///`read()` method returns [`layer0_decomp_cfg0::R`](R) reader structure
impl crate::Readable for LAYER0_DECOMP_CFG0rs {}
///`write(|w| ..)` method takes [`layer0_decomp_cfg0::W`](W) writer structure
impl crate::Writable for LAYER0_DECOMP_CFG0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LAYER0_DECOMP_CFG0 to value 0x0105_5982
impl crate::Resettable for LAYER0_DECOMP_CFG0rs {
    const RESET_VALUE: u32 = 0x0105_5982;
}
