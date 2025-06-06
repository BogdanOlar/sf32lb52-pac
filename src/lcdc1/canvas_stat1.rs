///Register `CANVAS_STAT1` reader
pub type R = crate::R<CANVAS_STAT1rs>;
///Register `CANVAS_STAT1` writer
pub type W = crate::W<CANVAS_STAT1rs>;
///Field `FIFO_CNT` reader - pre calc fifo count
pub type FifoCntR = crate::FieldReader;
///Field `FIFO_CNT` writer - pre calc fifo count
pub type FifoCntW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `POSTC_STAT` reader - postc_status
pub type PostcStatR = crate::FieldReader;
///Field `POSTC_STAT` writer - postc_status
pub type PostcStatW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PREC_STAT` reader - prec status
pub type PrecStatR = crate::FieldReader;
///Field `PREC_STAT` writer - prec status
pub type PrecStatW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `FETCH_STAT` reader - fetch status
pub type FetchStatR = crate::FieldReader;
///Field `FETCH_STAT` writer - fetch status
pub type FetchStatW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - pre calc fifo count
    #[inline(always)]
    pub fn fifo_cnt(&self) -> FifoCntR {
        FifoCntR::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - postc_status
    #[inline(always)]
    pub fn postc_stat(&self) -> PostcStatR {
        PostcStatR::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:8 - prec status
    #[inline(always)]
    pub fn prec_stat(&self) -> PrecStatR {
        PrecStatR::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 9:11 - fetch status
    #[inline(always)]
    pub fn fetch_stat(&self) -> FetchStatR {
        FetchStatR::new(((self.bits >> 9) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CANVAS_STAT1")
            .field("fetch_stat", &self.fetch_stat())
            .field("prec_stat", &self.prec_stat())
            .field("postc_stat", &self.postc_stat())
            .field("fifo_cnt", &self.fifo_cnt())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - pre calc fifo count
    #[inline(always)]
    pub fn fifo_cnt(&mut self) -> FifoCntW<CANVAS_STAT1rs> {
        FifoCntW::new(self, 0)
    }
    ///Bits 3:5 - postc_status
    #[inline(always)]
    pub fn postc_stat(&mut self) -> PostcStatW<CANVAS_STAT1rs> {
        PostcStatW::new(self, 3)
    }
    ///Bits 6:8 - prec status
    #[inline(always)]
    pub fn prec_stat(&mut self) -> PrecStatW<CANVAS_STAT1rs> {
        PrecStatW::new(self, 6)
    }
    ///Bits 9:11 - fetch status
    #[inline(always)]
    pub fn fetch_stat(&mut self) -> FetchStatW<CANVAS_STAT1rs> {
        FetchStatW::new(self, 9)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`canvas_stat1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`canvas_stat1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CANVAS_STAT1rs;
impl crate::RegisterSpec for CANVAS_STAT1rs {
    type Ux = u32;
}
///`read()` method returns [`canvas_stat1::R`](R) reader structure
impl crate::Readable for CANVAS_STAT1rs {}
///`write(|w| ..)` method takes [`canvas_stat1::W`](W) writer structure
impl crate::Writable for CANVAS_STAT1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CANVAS_STAT1 to value 0
impl crate::Resettable for CANVAS_STAT1rs {
    const RESET_VALUE: u32 = 0;
}
