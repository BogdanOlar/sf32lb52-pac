///Register `LAYER0_DECOMP` reader
pub type R = crate::R<LAYER0_DECOMPrs>;
///Register `LAYER0_DECOMP` writer
pub type W = crate::W<LAYER0_DECOMPrs>;
///Field `ENABLE` reader - decompression enable
pub type EnableR = crate::BitReader;
///Field `ENABLE` writer - decompression enable
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TARGET_WORDS` reader - size of a single channel data before decompression. Unit is half word. Each line has 3 channels. So for each line, the compressed data size is target_words * 3 * 2 bytes.
pub type TargetWordsR = crate::FieldReader<u16>;
///Field `TARGET_WORDS` writer - size of a single channel data before decompression. Unit is half word. Each line has 3 channels. So for each line, the compressed data size is target_words * 3 * 2 bytes.
pub type TargetWordsW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `COL_SIZE` reader - number of colums in a line of original image, max column size is 1024
pub type ColSizeR = crate::FieldReader<u16>;
///Field `COL_SIZE` writer - number of colums in a line of original image, max column size is 1024
pub type ColSizeW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    ///Bit 0 - decompression enable
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    ///Bits 1:12 - size of a single channel data before decompression. Unit is half word. Each line has 3 channels. So for each line, the compressed data size is target_words * 3 * 2 bytes.
    #[inline(always)]
    pub fn target_words(&self) -> TargetWordsR {
        TargetWordsR::new(((self.bits >> 1) & 0x0fff) as u16)
    }
    ///Bits 13:23 - number of colums in a line of original image, max column size is 1024
    #[inline(always)]
    pub fn col_size(&self) -> ColSizeR {
        ColSizeR::new(((self.bits >> 13) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LAYER0_DECOMP")
            .field("col_size", &self.col_size())
            .field("target_words", &self.target_words())
            .field("enable", &self.enable())
            .finish()
    }
}
impl W {
    ///Bit 0 - decompression enable
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<LAYER0_DECOMPrs> {
        EnableW::new(self, 0)
    }
    ///Bits 1:12 - size of a single channel data before decompression. Unit is half word. Each line has 3 channels. So for each line, the compressed data size is target_words * 3 * 2 bytes.
    #[inline(always)]
    pub fn target_words(&mut self) -> TargetWordsW<LAYER0_DECOMPrs> {
        TargetWordsW::new(self, 1)
    }
    ///Bits 13:23 - number of colums in a line of original image, max column size is 1024
    #[inline(always)]
    pub fn col_size(&mut self) -> ColSizeW<LAYER0_DECOMPrs> {
        ColSizeW::new(self, 13)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`layer0_decomp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`layer0_decomp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct LAYER0_DECOMPrs;
impl crate::RegisterSpec for LAYER0_DECOMPrs {
    type Ux = u32;
}
///`read()` method returns [`layer0_decomp::R`](R) reader structure
impl crate::Readable for LAYER0_DECOMPrs {}
///`write(|w| ..)` method takes [`layer0_decomp::W`](W) writer structure
impl crate::Writable for LAYER0_DECOMPrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LAYER0_DECOMP to value 0
impl crate::Resettable for LAYER0_DECOMPrs {
    const RESET_VALUE: u32 = 0;
}
