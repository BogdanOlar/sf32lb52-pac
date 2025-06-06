///Register `VL_SCALE_INIT_CFG2` reader
pub type R = crate::R<VL_SCALE_INIT_CFG2rs>;
///Register `VL_SCALE_INIT_CFG2` writer
pub type W = crate::W<VL_SCALE_INIT_CFG2rs>;
///Field `Y_VAL` reader - y-axis scale initial value, 10.16 format
pub type YValR = crate::FieldReader<u32>;
///Field `Y_VAL` writer - y-axis scale initial value, 10.16 format
pub type YValW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    ///Bits 0:25 - y-axis scale initial value, 10.16 format
    #[inline(always)]
    pub fn y_val(&self) -> YValR {
        YValR::new(self.bits & 0x03ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VL_SCALE_INIT_CFG2")
            .field("y_val", &self.y_val())
            .finish()
    }
}
impl W {
    ///Bits 0:25 - y-axis scale initial value, 10.16 format
    #[inline(always)]
    pub fn y_val(&mut self) -> YValW<VL_SCALE_INIT_CFG2rs> {
        YValW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`vl_scale_init_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vl_scale_init_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct VL_SCALE_INIT_CFG2rs;
impl crate::RegisterSpec for VL_SCALE_INIT_CFG2rs {
    type Ux = u32;
}
///`read()` method returns [`vl_scale_init_cfg2::R`](R) reader structure
impl crate::Readable for VL_SCALE_INIT_CFG2rs {}
///`write(|w| ..)` method takes [`vl_scale_init_cfg2::W`](W) writer structure
impl crate::Writable for VL_SCALE_INIT_CFG2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets VL_SCALE_INIT_CFG2 to value 0
impl crate::Resettable for VL_SCALE_INIT_CFG2rs {
    const RESET_VALUE: u32 = 0;
}
