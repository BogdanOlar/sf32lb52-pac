///Register `LCD_O_WIDTH` reader
pub type R = crate::R<LCD_O_WIDTHrs>;
///Register `LCD_O_WIDTH` writer
pub type W = crate::W<LCD_O_WIDTHrs>;
///Field `OFFSET` reader - AHB RAM address offset for each line
pub type OffsetR = crate::FieldReader<u16>;
///Field `OFFSET` writer - AHB RAM address offset for each line
pub type OffsetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - AHB RAM address offset for each line
    #[inline(always)]
    pub fn offset(&self) -> OffsetR {
        OffsetR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCD_O_WIDTH")
            .field("offset", &self.offset())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - AHB RAM address offset for each line
    #[inline(always)]
    pub fn offset(&mut self) -> OffsetW<LCD_O_WIDTHrs> {
        OffsetW::new(self, 0)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`lcd_o_width::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_o_width::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct LCD_O_WIDTHrs;
impl crate::RegisterSpec for LCD_O_WIDTHrs {
    type Ux = u32;
}
///`read()` method returns [`lcd_o_width::R`](R) reader structure
impl crate::Readable for LCD_O_WIDTHrs {}
///`write(|w| ..)` method takes [`lcd_o_width::W`](W) writer structure
impl crate::Writable for LCD_O_WIDTHrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LCD_O_WIDTH to value 0
impl crate::Resettable for LCD_O_WIDTHrs {
    const RESET_VALUE: u32 = 0;
}
