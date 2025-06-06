///Register `TE_CONF2` reader
pub type R = crate::R<TE_CONF2rs>;
///Register `TE_CONF2` writer
pub type W = crate::W<TE_CONF2rs>;
///Field `DLY_CNT` reader - TE delay counter
pub type DlyCntR = crate::FieldReader<u32>;
///Field `DLY_CNT` writer - TE delay counter
pub type DlyCntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - TE delay counter
    #[inline(always)]
    pub fn dly_cnt(&self) -> DlyCntR {
        DlyCntR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TE_CONF2")
            .field("dly_cnt", &self.dly_cnt())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - TE delay counter
    #[inline(always)]
    pub fn dly_cnt(&mut self) -> DlyCntW<TE_CONF2rs> {
        DlyCntW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`te_conf2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`te_conf2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TE_CONF2rs;
impl crate::RegisterSpec for TE_CONF2rs {
    type Ux = u32;
}
///`read()` method returns [`te_conf2::R`](R) reader structure
impl crate::Readable for TE_CONF2rs {}
///`write(|w| ..)` method takes [`te_conf2::W`](W) writer structure
impl crate::Writable for TE_CONF2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TE_CONF2 to value 0
impl crate::Resettable for TE_CONF2rs {
    const RESET_VALUE: u32 = 0;
}
