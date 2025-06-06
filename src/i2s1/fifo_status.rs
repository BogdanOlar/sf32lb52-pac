///Register `FIFO_STATUS` reader
pub type R = crate::R<FIFO_STATUSrs>;
///Register `FIFO_STATUS` writer
pub type W = crate::W<FIFO_STATUSrs>;
///Field `FIFO_STATUS_OUT` reader - FIFO Status output: Bit \[7:0\]
///= {tx_full,tx_empty,tx_almost_full,tx_almost_empty,rx_full,rx_empty,rx_almost_full,rx_almost_empty}
pub type FifoStatusOutR = crate::FieldReader;
///Field `FIFO_STATUS_OUT` writer - FIFO Status output: Bit \[7:0\]
///= {tx_full,tx_empty,tx_almost_full,tx_almost_empty,rx_full,rx_empty,rx_almost_full,rx_almost_empty}
pub type FifoStatusOutW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:7 - FIFO Status output: Bit \[7:0\]
    ///= {tx_full,tx_empty,tx_almost_full,tx_almost_empty,rx_full,rx_empty,rx_almost_full,rx_almost_empty}
    #[inline(always)]
    pub fn fifo_status_out(&self) -> FifoStatusOutR {
        FifoStatusOutR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFO_STATUS")
            .field("rsvd", &self.rsvd())
            .field("fifo_status_out", &self.fifo_status_out())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - FIFO Status output: Bit \[7:0\]
    ///= {tx_full,tx_empty,tx_almost_full,tx_almost_empty,rx_full,rx_empty,rx_almost_full,rx_almost_empty}
    #[inline(always)]
    pub fn fifo_status_out(&mut self) -> FifoStatusOutW<FIFO_STATUSrs> {
        FifoStatusOutW::new(self, 0)
    }
    ///Bits 8:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<FIFO_STATUSrs> {
        RsvdW::new(self, 8)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`fifo_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct FIFO_STATUSrs;
impl crate::RegisterSpec for FIFO_STATUSrs {
    type Ux = u32;
}
///`read()` method returns [`fifo_status::R`](R) reader structure
impl crate::Readable for FIFO_STATUSrs {}
///`write(|w| ..)` method takes [`fifo_status::W`](W) writer structure
impl crate::Writable for FIFO_STATUSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FIFO_STATUS to value 0
impl crate::Resettable for FIFO_STATUSrs {
    const RESET_VALUE: u32 = 0;
}
