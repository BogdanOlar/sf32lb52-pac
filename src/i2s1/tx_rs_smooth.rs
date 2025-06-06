///Register `TX_RS_SMOOTH` reader
pub type R = crate::R<TX_RS_SMOOTHrs>;
///Register `TX_RS_SMOOTH` writer
pub type W = crate::W<TX_RS_SMOOTHrs>;
///Field `EN` reader - 0: Disable TX re-sample smooth filter 1: Enable TX re-sample smooth filter This function is not implemented.
pub type EnR = crate::BitReader;
///Field `EN` writer - 0: Disable TX re-sample smooth filter 1: Enable TX re-sample smooth filter This function is not implemented.
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    ///Bit 0 - 0: Disable TX re-sample smooth filter 1: Enable TX re-sample smooth filter This function is not implemented.
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    ///Bits 1:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_RS_SMOOTH")
            .field("rsvd", &self.rsvd())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    ///Bit 0 - 0: Disable TX re-sample smooth filter 1: Enable TX re-sample smooth filter This function is not implemented.
    #[inline(always)]
    pub fn en(&mut self) -> EnW<TX_RS_SMOOTHrs> {
        EnW::new(self, 0)
    }
    ///Bits 1:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<TX_RS_SMOOTHrs> {
        RsvdW::new(self, 1)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`tx_rs_smooth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_rs_smooth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TX_RS_SMOOTHrs;
impl crate::RegisterSpec for TX_RS_SMOOTHrs {
    type Ux = u32;
}
///`read()` method returns [`tx_rs_smooth::R`](R) reader structure
impl crate::Readable for TX_RS_SMOOTHrs {}
///`write(|w| ..)` method takes [`tx_rs_smooth::W`](W) writer structure
impl crate::Writable for TX_RS_SMOOTHrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TX_RS_SMOOTH to value 0
impl crate::Resettable for TX_RS_SMOOTHrs {
    const RESET_VALUE: u32 = 0;
}
