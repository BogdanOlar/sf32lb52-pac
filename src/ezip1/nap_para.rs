///Register `NAP_PARA` reader
pub type R = crate::R<NAP_PARArs>;
///Register `NAP_PARA` writer
pub type W = crate::W<NAP_PARArs>;
///Field `NAP_TIM` reader - ezip decoder release bus time 0000: not nap 0001: 16 cycle 0010: 32 cycle 0100: 64 cycle 1000: 128 cycle other: not nap
pub type NapTimR = crate::FieldReader;
///Field `NAP_TIM` writer - ezip decoder release bus time 0000: not nap 0001: 16 cycle 0010: 32 cycle 0100: 64 cycle 1000: 128 cycle other: not nap
pub type NapTimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `BURST_NUM` reader - ezip decoder burst number 0000: 16 0001: 32 0010: 64 0100: 128 1000: 256 other: 16
pub type BurstNumR = crate::FieldReader;
///Field `BURST_NUM` writer - ezip decoder burst number 0000: 16 0001: 32 0010: 64 0100: 128 1000: 256 other: 16
pub type BurstNumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:3 - ezip decoder release bus time 0000: not nap 0001: 16 cycle 0010: 32 cycle 0100: 64 cycle 1000: 128 cycle other: not nap
    #[inline(always)]
    pub fn nap_tim(&self) -> NapTimR {
        NapTimR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - ezip decoder burst number 0000: 16 0001: 32 0010: 64 0100: 128 1000: 256 other: 16
    #[inline(always)]
    pub fn burst_num(&self) -> BurstNumR {
        BurstNumR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NAP_PARA")
            .field("rsvd", &self.rsvd())
            .field("burst_num", &self.burst_num())
            .field("nap_tim", &self.nap_tim())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - ezip decoder release bus time 0000: not nap 0001: 16 cycle 0010: 32 cycle 0100: 64 cycle 1000: 128 cycle other: not nap
    #[inline(always)]
    pub fn nap_tim(&mut self) -> NapTimW<NAP_PARArs> {
        NapTimW::new(self, 0)
    }
    ///Bits 4:7 - ezip decoder burst number 0000: 16 0001: 32 0010: 64 0100: 128 1000: 256 other: 16
    #[inline(always)]
    pub fn burst_num(&mut self) -> BurstNumW<NAP_PARArs> {
        BurstNumW::new(self, 4)
    }
    ///Bits 8:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<NAP_PARArs> {
        RsvdW::new(self, 8)
    }
}
///ezip decoder release bus parameter
///
///You can [`read`](crate::Reg::read) this register and get [`nap_para::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nap_para::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct NAP_PARArs;
impl crate::RegisterSpec for NAP_PARArs {
    type Ux = u32;
}
///`read()` method returns [`nap_para::R`](R) reader structure
impl crate::Readable for NAP_PARArs {}
///`write(|w| ..)` method takes [`nap_para::W`](W) writer structure
impl crate::Writable for NAP_PARArs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets NAP_PARA to value 0
impl crate::Resettable for NAP_PARArs {
    const RESET_VALUE: u32 = 0;
}
