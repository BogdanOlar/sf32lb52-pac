///Register `AON_BG` reader
pub type R = crate::R<AON_BGrs>;
///Register `AON_BG` writer
pub type W = crate::W<AON_BGrs>;
///Field `BUF_VOS_TRIM` reader -
pub type BufVosTrimR = crate::FieldReader;
///Field `BUF_VOS_TRIM` writer -
pub type BufVosTrimW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `BUF_VOS_STEP` reader -
pub type BufVosStepR = crate::FieldReader;
///Field `BUF_VOS_STEP` writer -
pub type BufVosStepW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BUF_VOS_POLAR` reader -
pub type BufVosPolarR = crate::BitReader;
///Field `BUF_VOS_POLAR` writer -
pub type BufVosPolarW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2
    #[inline(always)]
    pub fn buf_vos_trim(&self) -> BufVosTrimR {
        BufVosTrimR::new((self.bits & 7) as u8)
    }
    ///Bits 3:4
    #[inline(always)]
    pub fn buf_vos_step(&self) -> BufVosStepR {
        BufVosStepR::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 5
    #[inline(always)]
    pub fn buf_vos_polar(&self) -> BufVosPolarR {
        BufVosPolarR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AON_BG")
            .field("buf_vos_polar", &self.buf_vos_polar())
            .field("buf_vos_step", &self.buf_vos_step())
            .field("buf_vos_trim", &self.buf_vos_trim())
            .finish()
    }
}
impl W {
    ///Bits 0:2
    #[inline(always)]
    pub fn buf_vos_trim(&mut self) -> BufVosTrimW<AON_BGrs> {
        BufVosTrimW::new(self, 0)
    }
    ///Bits 3:4
    #[inline(always)]
    pub fn buf_vos_step(&mut self) -> BufVosStepW<AON_BGrs> {
        BufVosStepW::new(self, 3)
    }
    ///Bit 5
    #[inline(always)]
    pub fn buf_vos_polar(&mut self) -> BufVosPolarW<AON_BGrs> {
        BufVosPolarW::new(self, 5)
    }
}
///AON Bandgap Register
///
///You can [`read`](crate::Reg::read) this register and get [`aon_bg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aon_bg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct AON_BGrs;
impl crate::RegisterSpec for AON_BGrs {
    type Ux = u32;
}
///`read()` method returns [`aon_bg::R`](R) reader structure
impl crate::Readable for AON_BGrs {}
///`write(|w| ..)` method takes [`aon_bg::W`](W) writer structure
impl crate::Writable for AON_BGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AON_BG to value 0
impl crate::Resettable for AON_BGrs {
    const RESET_VALUE: u32 = 0;
}
