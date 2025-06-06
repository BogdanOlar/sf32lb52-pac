///Register `BT_PCM_DW` reader
pub type R = crate::R<BT_PCM_DWrs>;
///Register `BT_PCM_DW` writer
pub type W = crate::W<BT_PCM_DWrs>;
///Field `DW` reader - BT PCM master data width (>= 8), common value: 8, 13,14, 16
pub type DwR = crate::FieldReader;
///Field `DW` writer - BT PCM master data width (>= 8), common value: 8, 13,14, 16
pub type DwW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - BT PCM master data width (>= 8), common value: 8, 13,14, 16
    #[inline(always)]
    pub fn dw(&self) -> DwR {
        DwR::new((self.bits & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BT_PCM_DW").field("dw", &self.dw()).finish()
    }
}
impl W {
    ///Bits 0:4 - BT PCM master data width (>= 8), common value: 8, 13,14, 16
    #[inline(always)]
    pub fn dw(&mut self) -> DwW<BT_PCM_DWrs> {
        DwW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`bt_pcm_dw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bt_pcm_dw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct BT_PCM_DWrs;
impl crate::RegisterSpec for BT_PCM_DWrs {
    type Ux = u32;
}
///`read()` method returns [`bt_pcm_dw::R`](R) reader structure
impl crate::Readable for BT_PCM_DWrs {}
///`write(|w| ..)` method takes [`bt_pcm_dw::W`](W) writer structure
impl crate::Writable for BT_PCM_DWrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BT_PCM_DW to value 0x10
impl crate::Resettable for BT_PCM_DWrs {
    const RESET_VALUE: u32 = 0x10;
}
