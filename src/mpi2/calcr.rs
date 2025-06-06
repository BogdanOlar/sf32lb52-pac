///Register `CALCR` reader
pub type R = crate::R<CALCRrs>;
///Register `CALCR` writer
pub type W = crate::W<CALCRrs>;
///Field `DELAY` reader - calibration delay result
pub type DelayR = crate::FieldReader;
///Field `DELAY` writer - calibration delay result
pub type DelayW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DONE` reader - calibration done flag
pub type DoneR = crate::BitReader;
///Field `DONE` writer - calibration done flag
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN` reader - calibration enable
pub type EnR = crate::BitReader;
///Field `EN` writer - calibration enable
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - calibration delay result
    #[inline(always)]
    pub fn delay(&self) -> DelayR {
        DelayR::new((self.bits & 0xff) as u8)
    }
    ///Bit 8 - calibration done flag
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 31 - calibration enable
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CALCR")
            .field("en", &self.en())
            .field("done", &self.done())
            .field("delay", &self.delay())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - calibration delay result
    #[inline(always)]
    pub fn delay(&mut self) -> DelayW<CALCRrs> {
        DelayW::new(self, 0)
    }
    ///Bit 8 - calibration done flag
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<CALCRrs> {
        DoneW::new(self, 8)
    }
    ///Bit 31 - calibration enable
    #[inline(always)]
    pub fn en(&mut self) -> EnW<CALCRrs> {
        EnW::new(self, 31)
    }
}
///Calibration Clock Register
///
///You can [`read`](crate::Reg::read) this register and get [`calcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CALCRrs;
impl crate::RegisterSpec for CALCRrs {
    type Ux = u32;
}
///`read()` method returns [`calcr::R`](R) reader structure
impl crate::Readable for CALCRrs {}
///`write(|w| ..)` method takes [`calcr::W`](W) writer structure
impl crate::Writable for CALCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CALCR to value 0
impl crate::Resettable for CALCRrs {
    const RESET_VALUE: u32 = 0;
}
