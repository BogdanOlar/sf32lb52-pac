///Register `ANACR` reader
pub type R = crate::R<ANACRrs>;
///Register `ANACR` writer
pub type W = crate::W<ANACRrs>;
///Field `PB_ISO` reader - Set 1 to force IO(PB) into retention mode
pub type PbIsoR = crate::BitReader;
///Field `PB_ISO` writer - Set 1 to force IO(PB) into retention mode
pub type PbIsoW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VLP_ISO` reader - Set 1 to force off all LPSYS related analog modules
pub type VlpIsoR = crate::BitReader;
///Field `VLP_ISO` writer - Set 1 to force off all LPSYS related analog modules
pub type VlpIsoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Set 1 to force IO(PB) into retention mode
    #[inline(always)]
    pub fn pb_iso(&self) -> PbIsoR {
        PbIsoR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Set 1 to force off all LPSYS related analog modules
    #[inline(always)]
    pub fn vlp_iso(&self) -> VlpIsoR {
        VlpIsoR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANACR")
            .field("vlp_iso", &self.vlp_iso())
            .field("pb_iso", &self.pb_iso())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set 1 to force IO(PB) into retention mode
    #[inline(always)]
    pub fn pb_iso(&mut self) -> PbIsoW<ANACRrs> {
        PbIsoW::new(self, 0)
    }
    ///Bit 1 - Set 1 to force off all LPSYS related analog modules
    #[inline(always)]
    pub fn vlp_iso(&mut self) -> VlpIsoW<ANACRrs> {
        VlpIsoW::new(self, 1)
    }
}
///Analog Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`anacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ANACRrs;
impl crate::RegisterSpec for ANACRrs {
    type Ux = u32;
}
///`read()` method returns [`anacr::R`](R) reader structure
impl crate::Readable for ANACRrs {}
///`write(|w| ..)` method takes [`anacr::W`](W) writer structure
impl crate::Writable for ANACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ANACR to value 0
impl crate::Resettable for ANACRrs {
    const RESET_VALUE: u32 = 0;
}
