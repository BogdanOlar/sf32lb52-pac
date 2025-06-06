///Register `CANVAS_STAT` reader
pub type R = crate::R<CANVAS_STATrs>;
///Register `CANVAS_STAT` writer
pub type W = crate::W<CANVAS_STATrs>;
///Field `X_COR` reader - canvas x cordinate
pub type XCorR = crate::FieldReader<u16>;
///Field `X_COR` writer - canvas x cordinate
pub type XCorW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `Y_COR` reader - canvas y cordinate
pub type YCorR = crate::FieldReader<u16>;
///Field `Y_COR` writer - canvas y cordinate
pub type YCorW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
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
    ///Bits 0:9 - canvas x cordinate
    #[inline(always)]
    pub fn x_cor(&self) -> XCorR {
        XCorR::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 10:19 - canvas y cordinate
    #[inline(always)]
    pub fn y_cor(&self) -> YCorR {
        YCorR::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    ///Bits 20:22 - pre calc fifo count
    #[inline(always)]
    pub fn fifo_cnt(&self) -> FifoCntR {
        FifoCntR::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 23:25 - postc_status
    #[inline(always)]
    pub fn postc_stat(&self) -> PostcStatR {
        PostcStatR::new(((self.bits >> 23) & 7) as u8)
    }
    ///Bits 26:28 - prec status
    #[inline(always)]
    pub fn prec_stat(&self) -> PrecStatR {
        PrecStatR::new(((self.bits >> 26) & 7) as u8)
    }
    ///Bits 29:31 - fetch status
    #[inline(always)]
    pub fn fetch_stat(&self) -> FetchStatR {
        FetchStatR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CANVAS_STAT")
            .field("fetch_stat", &self.fetch_stat())
            .field("prec_stat", &self.prec_stat())
            .field("postc_stat", &self.postc_stat())
            .field("fifo_cnt", &self.fifo_cnt())
            .field("y_cor", &self.y_cor())
            .field("x_cor", &self.x_cor())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - canvas x cordinate
    #[inline(always)]
    pub fn x_cor(&mut self) -> XCorW<CANVAS_STATrs> {
        XCorW::new(self, 0)
    }
    ///Bits 10:19 - canvas y cordinate
    #[inline(always)]
    pub fn y_cor(&mut self) -> YCorW<CANVAS_STATrs> {
        YCorW::new(self, 10)
    }
    ///Bits 20:22 - pre calc fifo count
    #[inline(always)]
    pub fn fifo_cnt(&mut self) -> FifoCntW<CANVAS_STATrs> {
        FifoCntW::new(self, 20)
    }
    ///Bits 23:25 - postc_status
    #[inline(always)]
    pub fn postc_stat(&mut self) -> PostcStatW<CANVAS_STATrs> {
        PostcStatW::new(self, 23)
    }
    ///Bits 26:28 - prec status
    #[inline(always)]
    pub fn prec_stat(&mut self) -> PrecStatW<CANVAS_STATrs> {
        PrecStatW::new(self, 26)
    }
    ///Bits 29:31 - fetch status
    #[inline(always)]
    pub fn fetch_stat(&mut self) -> FetchStatW<CANVAS_STATrs> {
        FetchStatW::new(self, 29)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`canvas_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`canvas_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CANVAS_STATrs;
impl crate::RegisterSpec for CANVAS_STATrs {
    type Ux = u32;
}
///`read()` method returns [`canvas_stat::R`](R) reader structure
impl crate::Readable for CANVAS_STATrs {}
///`write(|w| ..)` method takes [`canvas_stat::W`](W) writer structure
impl crate::Writable for CANVAS_STATrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CANVAS_STAT to value 0
impl crate::Resettable for CANVAS_STATrs {
    const RESET_VALUE: u32 = 0;
}
