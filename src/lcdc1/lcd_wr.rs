///Register `LCD_WR` reader
pub type R = crate::R<LCD_WRrs>;
///Register `LCD_WR` writer
pub type W = crate::W<LCD_WRrs>;
///Field `DATA` reader - LCD write data
pub type DataR = crate::FieldReader<u32>;
///Field `DATA` writer - LCD write data
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - LCD write data
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCD_WR")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - LCD write data
    #[inline(always)]
    pub fn data(&mut self) -> DataW<LCD_WRrs> {
        DataW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`lcd_wr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct LCD_WRrs;
impl crate::RegisterSpec for LCD_WRrs {
    type Ux = u32;
}
///`read()` method returns [`lcd_wr::R`](R) reader structure
impl crate::Readable for LCD_WRrs {}
///`write(|w| ..)` method takes [`lcd_wr::W`](W) writer structure
impl crate::Writable for LCD_WRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LCD_WR to value 0
impl crate::Resettable for LCD_WRrs {
    const RESET_VALUE: u32 = 0;
}
