///Register `IPHR0` reader
pub type R = crate::R<IPHR0rs>;
///Register `IPHR0` writer
pub type W = crate::W<IPHR0rs>;
///Field `IPH` reader - rising edge in edge mode, or high level in level mode of corresponding GPIO\[31:0\]
pub type IphR = crate::FieldReader<u32>;
///Field `IPH` writer - rising edge in edge mode, or high level in level mode of corresponding GPIO\[31:0\]
pub type IphW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - rising edge in edge mode, or high level in level mode of corresponding GPIO\[31:0\]
    #[inline(always)]
    pub fn iph(&self) -> IphR {
        IphR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPHR0").field("iph", &self.iph()).finish()
    }
}
impl W {
    ///Bits 0:31 - rising edge in edge mode, or high level in level mode of corresponding GPIO\[31:0\]
    #[inline(always)]
    pub fn iph(&mut self) -> IphW<IPHR0rs> {
        IphW::new(self, 0)
    }
}
///Interrupt Polarity High Register
///
///You can [`read`](crate::Reg::read) this register and get [`iphr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iphr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IPHR0rs;
impl crate::RegisterSpec for IPHR0rs {
    type Ux = u32;
}
///`read()` method returns [`iphr0::R`](R) reader structure
impl crate::Readable for IPHR0rs {}
///`write(|w| ..)` method takes [`iphr0::W`](W) writer structure
impl crate::Writable for IPHR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IPHR0 to value 0
impl crate::Resettable for IPHR0rs {
    const RESET_VALUE: u32 = 0;
}
