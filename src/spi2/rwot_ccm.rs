///Register `RWOT_CCM` reader
pub type R = crate::R<RWOT_CCMrs>;
///Register `RWOT_CCM` writer
pub type W = crate::W<RWOT_CCMrs>;
///Field `RWOTCCM` reader - It's just total SPI_CLK Cycles. The value of this register defines the total number of SPI_CLK cycles when SPI controller works in master and RWOT mode. When the rwot_counter matches this value, SPI controller returns to IDLE state and does not output SPI_CLK anymore.
pub type RwotccmR = crate::FieldReader<u32>;
///Field `RWOTCCM` writer - It's just total SPI_CLK Cycles. The value of this register defines the total number of SPI_CLK cycles when SPI controller works in master and RWOT mode. When the rwot_counter matches this value, SPI controller returns to IDLE state and does not output SPI_CLK anymore.
pub type RwotccmW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - It's just total SPI_CLK Cycles. The value of this register defines the total number of SPI_CLK cycles when SPI controller works in master and RWOT mode. When the rwot_counter matches this value, SPI controller returns to IDLE state and does not output SPI_CLK anymore.
    #[inline(always)]
    pub fn rwotccm(&self) -> RwotccmR {
        RwotccmR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RWOT_CCM")
            .field("rwotccm", &self.rwotccm())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - It's just total SPI_CLK Cycles. The value of this register defines the total number of SPI_CLK cycles when SPI controller works in master and RWOT mode. When the rwot_counter matches this value, SPI controller returns to IDLE state and does not output SPI_CLK anymore.
    #[inline(always)]
    pub fn rwotccm(&mut self) -> RwotccmW<RWOT_CCMrs> {
        RwotccmW::new(self, 0)
    }
}
///RWOT Counter Cycles Match Register
///
///You can [`read`](crate::Reg::read) this register and get [`rwot_ccm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rwot_ccm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RWOT_CCMrs;
impl crate::RegisterSpec for RWOT_CCMrs {
    type Ux = u32;
}
///`read()` method returns [`rwot_ccm::R`](R) reader structure
impl crate::Readable for RWOT_CCMrs {}
///`write(|w| ..)` method takes [`rwot_ccm::W`](W) writer structure
impl crate::Writable for RWOT_CCMrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RWOT_CCM to value 0
impl crate::Resettable for RWOT_CCMrs {
    const RESET_VALUE: u32 = 0;
}
