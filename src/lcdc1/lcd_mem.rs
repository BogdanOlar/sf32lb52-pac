///Register `LCD_MEM` reader
pub type R = crate::R<LCD_MEMrs>;
///Register `LCD_MEM` writer
pub type W = crate::W<LCD_MEMrs>;
///Field `ADDR` reader - address for AHB LCD/AHB RAM
pub type AddrR = crate::FieldReader<u32>;
///Field `ADDR` writer - address for AHB LCD/AHB RAM
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - address for AHB LCD/AHB RAM
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCD_MEM")
            .field("addr", &self.addr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - address for AHB LCD/AHB RAM
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<LCD_MEMrs> {
        AddrW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`lcd_mem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_mem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct LCD_MEMrs;
impl crate::RegisterSpec for LCD_MEMrs {
    type Ux = u32;
}
///`read()` method returns [`lcd_mem::R`](R) reader structure
impl crate::Readable for LCD_MEMrs {}
///`write(|w| ..)` method takes [`lcd_mem::W`](W) writer structure
impl crate::Writable for LCD_MEMrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LCD_MEM to value 0
impl crate::Resettable for LCD_MEMrs {
    const RESET_VALUE: u32 = 0;
}
