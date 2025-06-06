///Register `PWRKEY_CNT` reader
pub type R = crate::R<PWRKEY_CNTrs>;
///Register `PWRKEY_CNT` writer
pub type W = crate::W<PWRKEY_CNTrs>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::FieldReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RST_CNT` reader - press high for RST_CNT*16 CLK_WDT cycles to reset the whole chip
pub type RstCntR = crate::FieldReader<u16>;
///Field `RST_CNT` writer - press high for RST_CNT*16 CLK_WDT cycles to reset the whole chip
pub type RstCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u16>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:3
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:19 - press high for RST_CNT*16 CLK_WDT cycles to reset the whole chip
    #[inline(always)]
    pub fn rst_cnt(&self) -> RstCntR {
        RstCntR::new(((self.bits >> 4) & 0xffff) as u16)
    }
    ///Bits 20:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWRKEY_CNT")
            .field("rsvd", &self.rsvd())
            .field("rst_cnt", &self.rst_cnt())
            .field("rsvd2", &self.rsvd2())
            .finish()
    }
}
impl W {
    ///Bits 0:3
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<PWRKEY_CNTrs> {
        Rsvd2W::new(self, 0)
    }
    ///Bits 4:19 - press high for RST_CNT*16 CLK_WDT cycles to reset the whole chip
    #[inline(always)]
    pub fn rst_cnt(&mut self) -> RstCntW<PWRKEY_CNTrs> {
        RstCntW::new(self, 4)
    }
    ///Bits 20:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<PWRKEY_CNTrs> {
        RsvdW::new(self, 20)
    }
}
///PowerKey Count Register
///
///You can [`read`](crate::Reg::read) this register and get [`pwrkey_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrkey_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PWRKEY_CNTrs;
impl crate::RegisterSpec for PWRKEY_CNTrs {
    type Ux = u32;
}
///`read()` method returns [`pwrkey_cnt::R`](R) reader structure
impl crate::Readable for PWRKEY_CNTrs {}
///`write(|w| ..)` method takes [`pwrkey_cnt::W`](W) writer structure
impl crate::Writable for PWRKEY_CNTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PWRKEY_CNT to value 0
impl crate::Resettable for PWRKEY_CNTrs {
    const RESET_VALUE: u32 = 0;
}
