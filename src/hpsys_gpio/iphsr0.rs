///Register `IPHSR0` reader
pub type R = crate::R<IPHSR0rs>;
///Register `IPHSR0` writer
pub type W = crate::W<IPHSR0rs>;
///Field `IPHS` reader - set 1 for rising edge in edge mode, or high level in level mode of corresponding GPIO\[31:0\]
pub type IphsR = crate::FieldReader<u32>;
///Field `IPHS` writer - set 1 for rising edge in edge mode, or high level in level mode of corresponding GPIO\[31:0\]
pub type IphsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - set 1 for rising edge in edge mode, or high level in level mode of corresponding GPIO\[31:0\]
    #[inline(always)]
    pub fn iphs(&self) -> IphsR {
        IphsR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPHSR0")
            .field("iphs", &self.iphs())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - set 1 for rising edge in edge mode, or high level in level mode of corresponding GPIO\[31:0\]
    #[inline(always)]
    pub fn iphs(&mut self) -> IphsW<IPHSR0rs> {
        IphsW::new(self, 0)
    }
}
///Interrupt Polarity High Set Register
///
///You can [`read`](crate::Reg::read) this register and get [`iphsr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iphsr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IPHSR0rs;
impl crate::RegisterSpec for IPHSR0rs {
    type Ux = u32;
}
///`read()` method returns [`iphsr0::R`](R) reader structure
impl crate::Readable for IPHSR0rs {}
///`write(|w| ..)` method takes [`iphsr0::W`](W) writer structure
impl crate::Writable for IPHSR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IPHSR0 to value 0
impl crate::Resettable for IPHSR0rs {
    const RESET_VALUE: u32 = 0;
}
