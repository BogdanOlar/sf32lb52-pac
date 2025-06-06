///Register `START_POINT` reader
pub type R = crate::R<START_POINTrs>;
///Register `START_POINT` writer
pub type W = crate::W<START_POINTrs>;
///Field `START_ROW` reader - ezip start row,count from 0
pub type StartRowR = crate::FieldReader<u16>;
///Field `START_ROW` writer - ezip start row,count from 0
pub type StartRowW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `START_COL` reader - ezip start col,count from 0
pub type StartColR = crate::FieldReader<u16>;
///Field `START_COL` writer - ezip start col,count from 0
pub type StartColW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - ezip start row,count from 0
    #[inline(always)]
    pub fn start_row(&self) -> StartRowR {
        StartRowR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - ezip start col,count from 0
    #[inline(always)]
    pub fn start_col(&self) -> StartColR {
        StartColR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("START_POINT")
            .field("start_col", &self.start_col())
            .field("start_row", &self.start_row())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - ezip start row,count from 0
    #[inline(always)]
    pub fn start_row(&mut self) -> StartRowW<START_POINTrs> {
        StartRowW::new(self, 0)
    }
    ///Bits 16:31 - ezip start col,count from 0
    #[inline(always)]
    pub fn start_col(&mut self) -> StartColW<START_POINTrs> {
        StartColW::new(self, 16)
    }
}
///ezip decoder start point
///
///You can [`read`](crate::Reg::read) this register and get [`start_point::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`start_point::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct START_POINTrs;
impl crate::RegisterSpec for START_POINTrs {
    type Ux = u32;
}
///`read()` method returns [`start_point::R`](R) reader structure
impl crate::Readable for START_POINTrs {}
///`write(|w| ..)` method takes [`start_point::W`](W) writer structure
impl crate::Writable for START_POINTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets START_POINT to value 0
impl crate::Resettable for START_POINTrs {
    const RESET_VALUE: u32 = 0;
}
