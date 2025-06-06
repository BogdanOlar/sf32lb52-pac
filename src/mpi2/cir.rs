///Register `CIR` reader
pub type R = crate::R<CIRrs>;
///Register `CIR` writer
pub type W = crate::W<CIRrs>;
///Field `INTERVAL1` reader - The interval between CMD1 itself. The unit is in MCLK cycles
pub type Interval1R = crate::FieldReader<u16>;
///Field `INTERVAL1` writer - The interval between CMD1 itself. The unit is in MCLK cycles
pub type Interval1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `INTERVAL2` reader - The interval between CMD1 and CMD2 (or between CMD2 itself) if CMD2E is enabled. The unit is in MCLK cycles
pub type Interval2R = crate::FieldReader<u16>;
///Field `INTERVAL2` writer - The interval between CMD1 and CMD2 (or between CMD2 itself) if CMD2E is enabled. The unit is in MCLK cycles
pub type Interval2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - The interval between CMD1 itself. The unit is in MCLK cycles
    #[inline(always)]
    pub fn interval1(&self) -> Interval1R {
        Interval1R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - The interval between CMD1 and CMD2 (or between CMD2 itself) if CMD2E is enabled. The unit is in MCLK cycles
    #[inline(always)]
    pub fn interval2(&self) -> Interval2R {
        Interval2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIR")
            .field("interval2", &self.interval2())
            .field("interval1", &self.interval1())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - The interval between CMD1 itself. The unit is in MCLK cycles
    #[inline(always)]
    pub fn interval1(&mut self) -> Interval1W<CIRrs> {
        Interval1W::new(self, 0)
    }
    ///Bits 16:31 - The interval between CMD1 and CMD2 (or between CMD2 itself) if CMD2E is enabled. The unit is in MCLK cycles
    #[inline(always)]
    pub fn interval2(&mut self) -> Interval2W<CIRrs> {
        Interval2W::new(self, 16)
    }
}
///Command Interval Register
///
///You can [`read`](crate::Reg::read) this register and get [`cir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CIRrs;
impl crate::RegisterSpec for CIRrs {
    type Ux = u32;
}
///`read()` method returns [`cir::R`](R) reader structure
impl crate::Readable for CIRrs {}
///`write(|w| ..)` method takes [`cir::W`](W) writer structure
impl crate::Writable for CIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CIR to value 0
impl crate::Resettable for CIRrs {
    const RESET_VALUE: u32 = 0;
}
