///Register `DEBUG` reader
pub type R = crate::R<DEBUGrs>;
///Register `DEBUG` writer
pub type W = crate::W<DEBUGrs>;
///Field `DEBUG_OUT_SEL` reader -
pub type DebugOutSelR = crate::FieldReader;
///Field `DEBUG_OUT_SEL` writer -
pub type DebugOutSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DEBUG_INT_SEL` reader -
pub type DebugIntSelR = crate::FieldReader;
///Field `DEBUG_INT_SEL` writer -
pub type DebugIntSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DEBUG_INT_DATA` reader - 4'h0: RSVD 4'h1: OL0 debug info 4'h2: OL1 debug info 4'h3: OL2 debug info 4'h4: VL debug info1 4'h5: VL debug info2 4'h6: ROI debug out 4'h7: mem intfa debug out 4'h8: mem intfb debug out 4'h9: ahb ctrl debug out 4'ha: ROI XX 4'hb: ROI YY 4'hc: EPIC_EZIP debug out others: RSVD
pub type DebugIntDataR = crate::FieldReader<u16>;
///Field `DEBUG_INT_DATA` writer - 4'h0: RSVD 4'h1: OL0 debug info 4'h2: OL1 debug info 4'h3: OL2 debug info 4'h4: VL debug info1 4'h5: VL debug info2 4'h6: ROI debug out 4'h7: mem intfa debug out 4'h8: mem intfb debug out 4'h9: ahb ctrl debug out 4'ha: ROI XX 4'hb: ROI YY 4'hc: EPIC_EZIP debug out others: RSVD
pub type DebugIntDataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:3
    #[inline(always)]
    pub fn debug_out_sel(&self) -> DebugOutSelR {
        DebugOutSelR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7
    #[inline(always)]
    pub fn debug_int_sel(&self) -> DebugIntSelR {
        DebugIntSelR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 16:31 - 4'h0: RSVD 4'h1: OL0 debug info 4'h2: OL1 debug info 4'h3: OL2 debug info 4'h4: VL debug info1 4'h5: VL debug info2 4'h6: ROI debug out 4'h7: mem intfa debug out 4'h8: mem intfb debug out 4'h9: ahb ctrl debug out 4'ha: ROI XX 4'hb: ROI YY 4'hc: EPIC_EZIP debug out others: RSVD
    #[inline(always)]
    pub fn debug_int_data(&self) -> DebugIntDataR {
        DebugIntDataR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBUG")
            .field("debug_int_data", &self.debug_int_data())
            .field("debug_int_sel", &self.debug_int_sel())
            .field("debug_out_sel", &self.debug_out_sel())
            .finish()
    }
}
impl W {
    ///Bits 0:3
    #[inline(always)]
    pub fn debug_out_sel(&mut self) -> DebugOutSelW<DEBUGrs> {
        DebugOutSelW::new(self, 0)
    }
    ///Bits 4:7
    #[inline(always)]
    pub fn debug_int_sel(&mut self) -> DebugIntSelW<DEBUGrs> {
        DebugIntSelW::new(self, 4)
    }
    ///Bits 16:31 - 4'h0: RSVD 4'h1: OL0 debug info 4'h2: OL1 debug info 4'h3: OL2 debug info 4'h4: VL debug info1 4'h5: VL debug info2 4'h6: ROI debug out 4'h7: mem intfa debug out 4'h8: mem intfb debug out 4'h9: ahb ctrl debug out 4'ha: ROI XX 4'hb: ROI YY 4'hc: EPIC_EZIP debug out others: RSVD
    #[inline(always)]
    pub fn debug_int_data(&mut self) -> DebugIntDataW<DEBUGrs> {
        DebugIntDataW::new(self, 16)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`debug::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DEBUGrs;
impl crate::RegisterSpec for DEBUGrs {
    type Ux = u32;
}
///`read()` method returns [`debug::R`](R) reader structure
impl crate::Readable for DEBUGrs {}
///`write(|w| ..)` method takes [`debug::W`](W) writer structure
impl crate::Writable for DEBUGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DEBUG to value 0
impl crate::Resettable for DEBUGrs {
    const RESET_VALUE: u32 = 0;
}
