///Register `VL_ROT_M_CFG2` reader
pub type R = crate::R<VL_ROT_M_CFG2rs>;
///Register `VL_ROT_M_CFG2` writer
pub type W = crate::W<VL_ROT_M_CFG2rs>;
///Field `M_PIVOT_X` reader - manual mode pivot x, signed value, -1023~1023, -1024 is not supported
pub type MPivotXR = crate::FieldReader<u16>;
///Field `M_PIVOT_X` writer - manual mode pivot x, signed value, -1023~1023, -1024 is not supported
pub type MPivotXW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `M_PIVOT_Y` reader - manual mode pivot y, signed value, -1023~1023, -1024 is not supported
pub type MPivotYR = crate::FieldReader<u16>;
///Field `M_PIVOT_Y` writer - manual mode pivot y, signed value, -1023~1023, -1024 is not supported
pub type MPivotYW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    ///Bits 0:10 - manual mode pivot x, signed value, -1023~1023, -1024 is not supported
    #[inline(always)]
    pub fn m_pivot_x(&self) -> MPivotXR {
        MPivotXR::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - manual mode pivot y, signed value, -1023~1023, -1024 is not supported
    #[inline(always)]
    pub fn m_pivot_y(&self) -> MPivotYR {
        MPivotYR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VL_ROT_M_CFG2")
            .field("m_pivot_y", &self.m_pivot_y())
            .field("m_pivot_x", &self.m_pivot_x())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - manual mode pivot x, signed value, -1023~1023, -1024 is not supported
    #[inline(always)]
    pub fn m_pivot_x(&mut self) -> MPivotXW<VL_ROT_M_CFG2rs> {
        MPivotXW::new(self, 0)
    }
    ///Bits 16:26 - manual mode pivot y, signed value, -1023~1023, -1024 is not supported
    #[inline(always)]
    pub fn m_pivot_y(&mut self) -> MPivotYW<VL_ROT_M_CFG2rs> {
        MPivotYW::new(self, 16)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`vl_rot_m_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vl_rot_m_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct VL_ROT_M_CFG2rs;
impl crate::RegisterSpec for VL_ROT_M_CFG2rs {
    type Ux = u32;
}
///`read()` method returns [`vl_rot_m_cfg2::R`](R) reader structure
impl crate::Readable for VL_ROT_M_CFG2rs {}
///`write(|w| ..)` method takes [`vl_rot_m_cfg2::W`](W) writer structure
impl crate::Writable for VL_ROT_M_CFG2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets VL_ROT_M_CFG2 to value 0
impl crate::Resettable for VL_ROT_M_CFG2rs {
    const RESET_VALUE: u32 = 0;
}
