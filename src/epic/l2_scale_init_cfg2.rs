///Register `L2_SCALE_INIT_CFG2` reader
pub type R = crate::R<L2_SCALE_INIT_CFG2rs>;
///Register `L2_SCALE_INIT_CFG2` writer
pub type W = crate::W<L2_SCALE_INIT_CFG2rs>;
///Field `Y_VAL` reader - y-axis scale initial value, 10.16 format
pub type YValR = crate::FieldReader<u32>;
///Field `Y_VAL` writer - y-axis scale initial value, 10.16 format
pub type YValW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:25 - y-axis scale initial value, 10.16 format
    #[inline(always)]
    pub fn y_val(&self) -> YValR {
        YValR::new(self.bits & 0x03ff_ffff)
    }
    ///Bits 26:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_SCALE_INIT_CFG2")
            .field("rsvd", &self.rsvd())
            .field("y_val", &self.y_val())
            .finish()
    }
}
impl W {
    ///Bits 0:25 - y-axis scale initial value, 10.16 format
    #[inline(always)]
    pub fn y_val(&mut self) -> YValW<L2_SCALE_INIT_CFG2rs> {
        YValW::new(self, 0)
    }
    ///Bits 26:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<L2_SCALE_INIT_CFG2rs> {
        RsvdW::new(self, 26)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`l2_scale_init_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_scale_init_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct L2_SCALE_INIT_CFG2rs;
impl crate::RegisterSpec for L2_SCALE_INIT_CFG2rs {
    type Ux = u32;
}
///`read()` method returns [`l2_scale_init_cfg2::R`](R) reader structure
impl crate::Readable for L2_SCALE_INIT_CFG2rs {}
///`write(|w| ..)` method takes [`l2_scale_init_cfg2::W`](W) writer structure
impl crate::Writable for L2_SCALE_INIT_CFG2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets L2_SCALE_INIT_CFG2 to value 0
impl crate::Resettable for L2_SCALE_INIT_CFG2rs {
    const RESET_VALUE: u32 = 0;
}
