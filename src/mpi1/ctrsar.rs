///Register `CTRSAR` reader
pub type R = crate::R<CTRSARrs>;
///Register `CTRSAR` writer
pub type W = crate::W<CTRSARrs>;
///Field `SA` reader - Starting address of the AES decryption area. Since the lowest 10 bits are zero, the address is always 1KB aligned. Together with CTREAR, the total area is \[CTRSAR, CTREAR) For example, CTRSAR = 32'h0, CTREAR = 32'h200000, then the on-the-fly decryption area is 0x0 - 0x1FFFFF
pub type SaR = crate::FieldReader<u32>;
///Field `SA` writer - Starting address of the AES decryption area. Since the lowest 10 bits are zero, the address is always 1KB aligned. Together with CTREAR, the total area is \[CTRSAR, CTREAR) For example, CTRSAR = 32'h0, CTREAR = 32'h200000, then the on-the-fly decryption area is 0x0 - 0x1FFFFF
pub type SaW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    ///Bits 10:31 - Starting address of the AES decryption area. Since the lowest 10 bits are zero, the address is always 1KB aligned. Together with CTREAR, the total area is \[CTRSAR, CTREAR) For example, CTRSAR = 32'h0, CTREAR = 32'h200000, then the on-the-fly decryption area is 0x0 - 0x1FFFFF
    #[inline(always)]
    pub fn sa(&self) -> SaR {
        SaR::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRSAR").field("sa", &self.sa()).finish()
    }
}
impl W {
    ///Bits 10:31 - Starting address of the AES decryption area. Since the lowest 10 bits are zero, the address is always 1KB aligned. Together with CTREAR, the total area is \[CTRSAR, CTREAR) For example, CTRSAR = 32'h0, CTREAR = 32'h200000, then the on-the-fly decryption area is 0x0 - 0x1FFFFF
    #[inline(always)]
    pub fn sa(&mut self) -> SaW<CTRSARrs> {
        SaW::new(self, 10)
    }
}
///CTR Starting Address Register
///
///You can [`read`](crate::Reg::read) this register and get [`ctrsar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrsar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CTRSARrs;
impl crate::RegisterSpec for CTRSARrs {
    type Ux = u32;
}
///`read()` method returns [`ctrsar::R`](R) reader structure
impl crate::Readable for CTRSARrs {}
///`write(|w| ..)` method takes [`ctrsar::W`](W) writer structure
impl crate::Writable for CTRSARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CTRSAR to value 0
impl crate::Resettable for CTRSARrs {
    const RESET_VALUE: u32 = 0;
}
