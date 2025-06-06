///Register `CPAR1` reader
pub type R = crate::R<CPAR1rs>;
///Register `CPAR1` writer
pub type W = crate::W<CPAR1rs>;
///Field `PA` reader - peripheral address It contains the base address of the peripheral data register from/to which the data will be read/written. In memory-to-memory mode, this register identifies the memory destination address if DIR = 1 and the memory source address if DIR = 0. In peripheral-to-peripheral mode, this register identifies the peripheral destination address DIR = 1 and the peripheral source address if DIR = 0.
pub type PaR = crate::FieldReader<u32>;
///Field `PA` writer - peripheral address It contains the base address of the peripheral data register from/to which the data will be read/written. In memory-to-memory mode, this register identifies the memory destination address if DIR = 1 and the memory source address if DIR = 0. In peripheral-to-peripheral mode, this register identifies the peripheral destination address DIR = 1 and the peripheral source address if DIR = 0.
pub type PaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - peripheral address It contains the base address of the peripheral data register from/to which the data will be read/written. In memory-to-memory mode, this register identifies the memory destination address if DIR = 1 and the memory source address if DIR = 0. In peripheral-to-peripheral mode, this register identifies the peripheral destination address DIR = 1 and the peripheral source address if DIR = 0.
    #[inline(always)]
    pub fn pa(&self) -> PaR {
        PaR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPAR1").field("pa", &self.pa()).finish()
    }
}
impl W {
    ///Bits 0:31 - peripheral address It contains the base address of the peripheral data register from/to which the data will be read/written. In memory-to-memory mode, this register identifies the memory destination address if DIR = 1 and the memory source address if DIR = 0. In peripheral-to-peripheral mode, this register identifies the peripheral destination address DIR = 1 and the peripheral source address if DIR = 0.
    #[inline(always)]
    pub fn pa(&mut self) -> PaW<CPAR1rs> {
        PaW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`cpar1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CPAR1rs;
impl crate::RegisterSpec for CPAR1rs {
    type Ux = u32;
}
///`read()` method returns [`cpar1::R`](R) reader structure
impl crate::Readable for CPAR1rs {}
///`write(|w| ..)` method takes [`cpar1::W`](W) writer structure
impl crate::Writable for CPAR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CPAR1 to value 0
impl crate::Resettable for CPAR1rs {
    const RESET_VALUE: u32 = 0;
}
