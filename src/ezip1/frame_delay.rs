///Register `FRAME_DELAY` reader
pub type R = crate::R<FRAME_DELAYrs>;
///Register `FRAME_DELAY` writer
pub type W = crate::W<FRAME_DELAYrs>;
///Field `DELAY_DEN` reader - AEZIP frame delay fraction denominator
pub type DelayDenR = crate::FieldReader<u16>;
///Field `DELAY_DEN` writer - AEZIP frame delay fraction denominator
pub type DelayDenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `DELAY_NUM` reader - AEZIP frame delay fraction numerator
pub type DelayNumR = crate::FieldReader<u16>;
///Field `DELAY_NUM` writer - AEZIP frame delay fraction numerator
pub type DelayNumW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - AEZIP frame delay fraction denominator
    #[inline(always)]
    pub fn delay_den(&self) -> DelayDenR {
        DelayDenR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - AEZIP frame delay fraction numerator
    #[inline(always)]
    pub fn delay_num(&self) -> DelayNumR {
        DelayNumR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRAME_DELAY")
            .field("delay_num", &self.delay_num())
            .field("delay_den", &self.delay_den())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - AEZIP frame delay fraction denominator
    #[inline(always)]
    pub fn delay_den(&mut self) -> DelayDenW<FRAME_DELAYrs> {
        DelayDenW::new(self, 0)
    }
    ///Bits 16:31 - AEZIP frame delay fraction numerator
    #[inline(always)]
    pub fn delay_num(&mut self) -> DelayNumW<FRAME_DELAYrs> {
        DelayNumW::new(self, 16)
    }
}
///Aezip frame delay
///
///You can [`read`](crate::Reg::read) this register and get [`frame_delay::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame_delay::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct FRAME_DELAYrs;
impl crate::RegisterSpec for FRAME_DELAYrs {
    type Ux = u32;
}
///`read()` method returns [`frame_delay::R`](R) reader structure
impl crate::Readable for FRAME_DELAYrs {}
///`write(|w| ..)` method takes [`frame_delay::W`](W) writer structure
impl crate::Writable for FRAME_DELAYrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FRAME_DELAY to value 0
impl crate::Resettable for FRAME_DELAYrs {
    const RESET_VALUE: u32 = 0;
}
