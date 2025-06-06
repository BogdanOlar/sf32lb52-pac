///Register `TIMR` reader
pub type R = crate::R<TIMRrs>;
///Register `TIMR` writer
pub type W = crate::W<TIMRrs>;
///Field `TIMEOUT` reader - After the transaction is complete, CS remains low for multiple cycles of MCLK as specified by this register. For example if TIMEOUT=n, CS remains active for n cycles, during which if a new transaction occurs and the address is consecutive, the memory access can be resumed w/o sending the command and address again.
pub type TimeoutR = crate::FieldReader<u16>;
///Field `TIMEOUT` writer - After the transaction is complete, CS remains low for multiple cycles of MCLK as specified by this register. For example if TIMEOUT=n, CS remains active for n cycles, during which if a new transaction occurs and the address is consecutive, the memory access can be resumed w/o sending the command and address again.
pub type TimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - After the transaction is complete, CS remains low for multiple cycles of MCLK as specified by this register. For example if TIMEOUT=n, CS remains active for n cycles, during which if a new transaction occurs and the address is consecutive, the memory access can be resumed w/o sending the command and address again.
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMR")
            .field("timeout", &self.timeout())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - After the transaction is complete, CS remains low for multiple cycles of MCLK as specified by this register. For example if TIMEOUT=n, CS remains active for n cycles, during which if a new transaction occurs and the address is consecutive, the memory access can be resumed w/o sending the command and address again.
    #[inline(always)]
    pub fn timeout(&mut self) -> TimeoutW<TIMRrs> {
        TimeoutW::new(self, 0)
    }
}
///Timer Register
///
///You can [`read`](crate::Reg::read) this register and get [`timr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TIMRrs;
impl crate::RegisterSpec for TIMRrs {
    type Ux = u32;
}
///`read()` method returns [`timr::R`](R) reader structure
impl crate::Readable for TIMRrs {}
///`write(|w| ..)` method takes [`timr::W`](W) writer structure
impl crate::Writable for TIMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TIMR to value 0
impl crate::Resettable for TIMRrs {
    const RESET_VALUE: u32 = 0;
}
