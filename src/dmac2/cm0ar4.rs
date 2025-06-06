///Register `CM0AR4` reader
pub type R = crate::R<CM0AR4rs>;
///Register `CM0AR4` writer
pub type W = crate::W<CM0AR4rs>;
///Field `MA` reader - peripheral address It contains the base address of the memory from/to which the data will be read/written. In memory-to-memory mode, this register identifies the memory source address if DIR = 1 and the memory destination address if DIR = 0. In peripheral-to-peripheral mode, this register identifies the peripheral source address DIR = 1 and the peripheral destination address if DIR = 0.
pub type MaR = crate::FieldReader<u32>;
///Field `MA` writer - peripheral address It contains the base address of the memory from/to which the data will be read/written. In memory-to-memory mode, this register identifies the memory source address if DIR = 1 and the memory destination address if DIR = 0. In peripheral-to-peripheral mode, this register identifies the peripheral source address DIR = 1 and the peripheral destination address if DIR = 0.
pub type MaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - peripheral address It contains the base address of the memory from/to which the data will be read/written. In memory-to-memory mode, this register identifies the memory source address if DIR = 1 and the memory destination address if DIR = 0. In peripheral-to-peripheral mode, this register identifies the peripheral source address DIR = 1 and the peripheral destination address if DIR = 0.
    #[inline(always)]
    pub fn ma(&self) -> MaR {
        MaR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CM0AR4").field("ma", &self.ma()).finish()
    }
}
impl W {
    ///Bits 0:31 - peripheral address It contains the base address of the memory from/to which the data will be read/written. In memory-to-memory mode, this register identifies the memory source address if DIR = 1 and the memory destination address if DIR = 0. In peripheral-to-peripheral mode, this register identifies the peripheral source address DIR = 1 and the peripheral destination address if DIR = 0.
    #[inline(always)]
    pub fn ma(&mut self) -> MaW<CM0AR4rs> {
        MaW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`cm0ar4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0ar4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CM0AR4rs;
impl crate::RegisterSpec for CM0AR4rs {
    type Ux = u32;
}
///`read()` method returns [`cm0ar4::R`](R) reader structure
impl crate::Readable for CM0AR4rs {}
///`write(|w| ..)` method takes [`cm0ar4::W`](W) writer structure
impl crate::Writable for CM0AR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CM0AR4 to value 0
impl crate::Resettable for CM0AR4rs {
    const RESET_VALUE: u32 = 0;
}
