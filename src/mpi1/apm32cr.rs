///Register `APM32CR` reader
pub type R = crate::R<APM32CRrs>;
///Register `APM32CR` writer
pub type W = crate::W<APM32CRrs>;
///Field `TCPHR` reader - For special use by AP 32Mb PSRAM.Reserved-Do not modify
pub type TcphrR = crate::FieldReader;
///Field `TCPHR` writer - For special use by AP 32Mb PSRAM.Reserved-Do not modify
pub type TcphrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TCPHW` reader - For special use by AP 32Mb PSRAM.Reserved-Do not modify
pub type TcphwR = crate::FieldReader;
///Field `TCPHW` writer - For special use by AP 32Mb PSRAM.Reserved-Do not modify
pub type TcphwW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - For special use by AP 32Mb PSRAM.Reserved-Do not modify
    #[inline(always)]
    pub fn tcphr(&self) -> TcphrR {
        TcphrR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - For special use by AP 32Mb PSRAM.Reserved-Do not modify
    #[inline(always)]
    pub fn tcphw(&self) -> TcphwR {
        TcphwR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APM32CR")
            .field("tcphw", &self.tcphw())
            .field("tcphr", &self.tcphr())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - For special use by AP 32Mb PSRAM.Reserved-Do not modify
    #[inline(always)]
    pub fn tcphr(&mut self) -> TcphrW<APM32CRrs> {
        TcphrW::new(self, 0)
    }
    ///Bits 4:7 - For special use by AP 32Mb PSRAM.Reserved-Do not modify
    #[inline(always)]
    pub fn tcphw(&mut self) -> TcphwW<APM32CRrs> {
        TcphwW::new(self, 4)
    }
}
///APM32 Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`apm32cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apm32cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct APM32CRrs;
impl crate::RegisterSpec for APM32CRrs {
    type Ux = u32;
}
///`read()` method returns [`apm32cr::R`](R) reader structure
impl crate::Readable for APM32CRrs {}
///`write(|w| ..)` method takes [`apm32cr::W`](W) writer structure
impl crate::Writable for APM32CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets APM32CR to value 0
impl crate::Resettable for APM32CRrs {
    const RESET_VALUE: u32 = 0;
}
