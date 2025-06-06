///Register `SAR` reader
pub type R = crate::R<SARrs>;
///Register `SAR` writer
pub type W = crate::W<SARrs>;
///Field `ADDR` reader - The seven-bit address to which the I2C responds when in slave-receive mode
pub type AddrR = crate::FieldReader;
///Field `ADDR` writer - The seven-bit address to which the I2C responds when in slave-receive mode
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6 - The seven-bit address to which the I2C responds when in slave-receive mode
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR").field("addr", &self.addr()).finish()
    }
}
impl W {
    ///Bits 0:6 - The seven-bit address to which the I2C responds when in slave-receive mode
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<SARrs> {
        AddrW::new(self, 0)
    }
}
///Slave Address Register
///
///You can [`read`](crate::Reg::read) this register and get [`sar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SARrs;
impl crate::RegisterSpec for SARrs {
    type Ux = u32;
}
///`read()` method returns [`sar::R`](R) reader structure
impl crate::Readable for SARrs {}
///`write(|w| ..)` method takes [`sar::W`](W) writer structure
impl crate::Writable for SARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SAR to value 0x47
impl crate::Resettable for SARrs {
    const RESET_VALUE: u32 = 0x47;
}
