///Register `IPHR1` reader
pub type R = crate::R<IPHR1rs>;
///Register `IPHR1` writer
pub type W = crate::W<IPHR1rs>;
///Field `IPH` reader - rising edge in edge mode, or high level in level mode of corresponding GPIO\[44:32\]
pub type IphR = crate::FieldReader<u16>;
///Field `IPH` writer - rising edge in edge mode, or high level in level mode of corresponding GPIO\[44:32\]
pub type IphW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    ///Bits 0:12 - rising edge in edge mode, or high level in level mode of corresponding GPIO\[44:32\]
    #[inline(always)]
    pub fn iph(&self) -> IphR {
        IphR::new((self.bits & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPHR1").field("iph", &self.iph()).finish()
    }
}
impl W {
    ///Bits 0:12 - rising edge in edge mode, or high level in level mode of corresponding GPIO\[44:32\]
    #[inline(always)]
    pub fn iph(&mut self) -> IphW<IPHR1rs> {
        IphW::new(self, 0)
    }
}
///Interrupt Polarity High Register
///
///You can [`read`](crate::Reg::read) this register and get [`iphr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iphr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IPHR1rs;
impl crate::RegisterSpec for IPHR1rs {
    type Ux = u32;
}
///`read()` method returns [`iphr1::R`](R) reader structure
impl crate::Readable for IPHR1rs {}
///`write(|w| ..)` method takes [`iphr1::W`](W) writer structure
impl crate::Writable for IPHR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IPHR1 to value 0
impl crate::Resettable for IPHR1rs {
    const RESET_VALUE: u32 = 0;
}
