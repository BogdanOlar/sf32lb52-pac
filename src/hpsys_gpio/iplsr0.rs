///Register `IPLSR0` reader
pub type R = crate::R<IPLSR0rs>;
///Register `IPLSR0` writer
pub type W = crate::W<IPLSR0rs>;
///Field `IPLS` reader - set 1 for falling edge in edge mode, or low level in level mode of corresponding GPIO\[31:0\]
pub type IplsR = crate::FieldReader<u32>;
///Field `IPLS` writer - set 1 for falling edge in edge mode, or low level in level mode of corresponding GPIO\[31:0\]
pub type IplsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - set 1 for falling edge in edge mode, or low level in level mode of corresponding GPIO\[31:0\]
    #[inline(always)]
    pub fn ipls(&self) -> IplsR {
        IplsR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPLSR0")
            .field("ipls", &self.ipls())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - set 1 for falling edge in edge mode, or low level in level mode of corresponding GPIO\[31:0\]
    #[inline(always)]
    pub fn ipls(&mut self) -> IplsW<IPLSR0rs> {
        IplsW::new(self, 0)
    }
}
///Interrupt Polarity Low Set Register
///
///You can [`read`](crate::Reg::read) this register and get [`iplsr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iplsr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IPLSR0rs;
impl crate::RegisterSpec for IPLSR0rs {
    type Ux = u32;
}
///`read()` method returns [`iplsr0::R`](R) reader structure
impl crate::Readable for IPLSR0rs {}
///`write(|w| ..)` method takes [`iplsr0::W`](W) writer structure
impl crate::Writable for IPLSR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IPLSR0 to value 0
impl crate::Resettable for IPLSR0rs {
    const RESET_VALUE: u32 = 0;
}
