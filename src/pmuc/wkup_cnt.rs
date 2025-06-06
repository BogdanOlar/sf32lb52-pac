///Register `WKUP_CNT` reader
pub type R = crate::R<WKUP_CNTrs>;
///Register `WKUP_CNT` writer
pub type W = crate::W<WKUP_CNTrs>;
///Field `PIN0_CNT` reader -
pub type Pin0CntR = crate::FieldReader<u16>;
///Field `PIN0_CNT` writer -
pub type Pin0CntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `PIN1_CNT` reader -
pub type Pin1CntR = crate::FieldReader<u16>;
///Field `PIN1_CNT` writer -
pub type Pin1CntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15
    #[inline(always)]
    pub fn pin0_cnt(&self) -> Pin0CntR {
        Pin0CntR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31
    #[inline(always)]
    pub fn pin1_cnt(&self) -> Pin1CntR {
        Pin1CntR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WKUP_CNT")
            .field("pin1_cnt", &self.pin1_cnt())
            .field("pin0_cnt", &self.pin0_cnt())
            .finish()
    }
}
impl W {
    ///Bits 0:15
    #[inline(always)]
    pub fn pin0_cnt(&mut self) -> Pin0CntW<WKUP_CNTrs> {
        Pin0CntW::new(self, 0)
    }
    ///Bits 16:31
    #[inline(always)]
    pub fn pin1_cnt(&mut self) -> Pin1CntW<WKUP_CNTrs> {
        Pin1CntW::new(self, 16)
    }
}
///Wakeup Count Register
///
///You can [`read`](crate::Reg::read) this register and get [`wkup_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkup_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct WKUP_CNTrs;
impl crate::RegisterSpec for WKUP_CNTrs {
    type Ux = u32;
}
///`read()` method returns [`wkup_cnt::R`](R) reader structure
impl crate::Readable for WKUP_CNTrs {}
///`write(|w| ..)` method takes [`wkup_cnt::W`](W) writer structure
impl crate::Writable for WKUP_CNTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets WKUP_CNT to value 0
impl crate::Resettable for WKUP_CNTrs {
    const RESET_VALUE: u32 = 0;
}
