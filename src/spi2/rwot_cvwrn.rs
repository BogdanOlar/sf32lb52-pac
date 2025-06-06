///Register `RWOT_CVWRN` reader
pub type R = crate::R<RWOT_CVWRNrs>;
///Register `RWOT_CVWRN` writer
pub type W = crate::W<RWOT_CVWRNrs>;
///Field `RWOTCVWR` reader - RWOTCVWR This register prevents the risk of instability on rwot_counter value reading, it's only valid after SPI controller has been enabled Write 0 = No effect Write 1 = Capture value of rwot_counter Read: Returns the captured value of rwot_counter
pub type RwotcvwrR = crate::FieldReader<u32>;
///Field `RWOTCVWR` writer - RWOTCVWR This register prevents the risk of instability on rwot_counter value reading, it's only valid after SPI controller has been enabled Write 0 = No effect Write 1 = Capture value of rwot_counter Read: Returns the captured value of rwot_counter
pub type RwotcvwrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - RWOTCVWR This register prevents the risk of instability on rwot_counter value reading, it's only valid after SPI controller has been enabled Write 0 = No effect Write 1 = Capture value of rwot_counter Read: Returns the captured value of rwot_counter
    #[inline(always)]
    pub fn rwotcvwr(&self) -> RwotcvwrR {
        RwotcvwrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RWOT_CVWRN")
            .field("rwotcvwr", &self.rwotcvwr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - RWOTCVWR This register prevents the risk of instability on rwot_counter value reading, it's only valid after SPI controller has been enabled Write 0 = No effect Write 1 = Capture value of rwot_counter Read: Returns the captured value of rwot_counter
    #[inline(always)]
    pub fn rwotcvwr(&mut self) -> RwotcvwrW<RWOT_CVWRNrs> {
        RwotcvwrW::new(self, 0)
    }
}
///RWOT Counter Value Write for Red Request Register
///
///You can [`read`](crate::Reg::read) this register and get [`rwot_cvwrn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rwot_cvwrn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RWOT_CVWRNrs;
impl crate::RegisterSpec for RWOT_CVWRNrs {
    type Ux = u32;
}
///`read()` method returns [`rwot_cvwrn::R`](R) reader structure
impl crate::Readable for RWOT_CVWRNrs {}
///`write(|w| ..)` method takes [`rwot_cvwrn::W`](W) writer structure
impl crate::Writable for RWOT_CVWRNrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RWOT_CVWRN to value 0
impl crate::Resettable for RWOT_CVWRNrs {
    const RESET_VALUE: u32 = 0;
}
