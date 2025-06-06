///Register `L2_ROT_STAT` reader
pub type R = crate::R<L2_ROT_STATrs>;
///Register `L2_ROT_STAT` writer
pub type W = crate::W<L2_ROT_STATrs>;
///Field `ROT_MAX_LINE` reader - max line of rotated image
pub type RotMaxLineR = crate::FieldReader<u16>;
///Field `ROT_MAX_LINE` writer - max line of rotated image
pub type RotMaxLineW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `ROT_MAX_COL` reader - max column of rotated image
pub type RotMaxColR = crate::FieldReader<u16>;
///Field `ROT_MAX_COL` writer - max column of rotated image
pub type RotMaxColW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    ///Bits 0:10 - max line of rotated image
    #[inline(always)]
    pub fn rot_max_line(&self) -> RotMaxLineR {
        RotMaxLineR::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - max column of rotated image
    #[inline(always)]
    pub fn rot_max_col(&self) -> RotMaxColR {
        RotMaxColR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_ROT_STAT")
            .field("rot_max_col", &self.rot_max_col())
            .field("rot_max_line", &self.rot_max_line())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - max line of rotated image
    #[inline(always)]
    pub fn rot_max_line(&mut self) -> RotMaxLineW<L2_ROT_STATrs> {
        RotMaxLineW::new(self, 0)
    }
    ///Bits 16:26 - max column of rotated image
    #[inline(always)]
    pub fn rot_max_col(&mut self) -> RotMaxColW<L2_ROT_STATrs> {
        RotMaxColW::new(self, 16)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`l2_rot_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_rot_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct L2_ROT_STATrs;
impl crate::RegisterSpec for L2_ROT_STATrs {
    type Ux = u32;
}
///`read()` method returns [`l2_rot_stat::R`](R) reader structure
impl crate::Readable for L2_ROT_STATrs {}
///`write(|w| ..)` method takes [`l2_rot_stat::W`](W) writer structure
impl crate::Writable for L2_ROT_STATrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets L2_ROT_STAT to value 0
impl crate::Resettable for L2_ROT_STATrs {
    const RESET_VALUE: u32 = 0;
}
