///Register `POL` reader
pub type R = crate::R<POLrs>;
///Register `POL` writer
pub type W = crate::W<POLrs>;
///Field `POL` reader - Programmable polynomial This register is used to write the coefficients of the polynomial to be used for CRC calculation. If the polynomial size is less than 32 bits, the least significant bits have to be used to program the correct value.
pub type PolR = crate::FieldReader<u32>;
///Field `POL` writer - Programmable polynomial This register is used to write the coefficients of the polynomial to be used for CRC calculation. If the polynomial size is less than 32 bits, the least significant bits have to be used to program the correct value.
pub type PolW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Programmable polynomial This register is used to write the coefficients of the polynomial to be used for CRC calculation. If the polynomial size is less than 32 bits, the least significant bits have to be used to program the correct value.
    #[inline(always)]
    pub fn pol(&self) -> PolR {
        PolR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POL").field("pol", &self.pol()).finish()
    }
}
impl W {
    ///Bits 0:31 - Programmable polynomial This register is used to write the coefficients of the polynomial to be used for CRC calculation. If the polynomial size is less than 32 bits, the least significant bits have to be used to program the correct value.
    #[inline(always)]
    pub fn pol(&mut self) -> PolW<POLrs> {
        PolW::new(self, 0)
    }
}
///CRC polynomial
///
///You can [`read`](crate::Reg::read) this register and get [`pol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct POLrs;
impl crate::RegisterSpec for POLrs {
    type Ux = u32;
}
///`read()` method returns [`pol::R`](R) reader structure
impl crate::Readable for POLrs {}
///`write(|w| ..)` method takes [`pol::W`](W) writer structure
impl crate::Writable for POLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets POL to value 0
impl crate::Resettable for POLrs {
    const RESET_VALUE: u32 = 0;
}
