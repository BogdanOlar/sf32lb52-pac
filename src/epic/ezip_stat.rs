///Register `EZIP_STAT` reader
pub type R = crate::R<EZIP_STATrs>;
///Register `EZIP_STAT` writer
pub type W = crate::W<EZIP_STATrs>;
///Field `LINE_CNT0` reader - ezip engine 0 line count
pub type LineCnt0R = crate::FieldReader<u16>;
///Field `LINE_CNT0` writer - ezip engine 0 line count
pub type LineCnt0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `BUF_CNT0` reader - ezip engine 0 buffer count
pub type BufCnt0R = crate::FieldReader;
///Field `BUF_CNT0` writer - ezip engine 0 buffer count
pub type BufCnt0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RUN_STAT0` reader - ezip engine 0 status
pub type RunStat0R = crate::FieldReader;
///Field `RUN_STAT0` writer - ezip engine 0 status
pub type RunStat0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RSVD2` reader -
pub type Rsvd2R = crate::BitReader;
///Field `RSVD2` writer -
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LINE_CNT1` reader - ezip engine 1 line count
pub type LineCnt1R = crate::FieldReader<u16>;
///Field `LINE_CNT1` writer - ezip engine 1 line count
pub type LineCnt1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `BUF_CNT1` reader - ezip engine 1 buffer count
pub type BufCnt1R = crate::FieldReader;
///Field `BUF_CNT1` writer - ezip engine 1 buffer count
pub type BufCnt1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RUN_STAT1` reader - ezip engine 1 status
pub type RunStat1R = crate::FieldReader;
///Field `RUN_STAT1` writer - ezip engine 1 status
pub type RunStat1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RSVD` reader -
pub type RsvdR = crate::BitReader;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:9 - ezip engine 0 line count
    #[inline(always)]
    pub fn line_cnt0(&self) -> LineCnt0R {
        LineCnt0R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 10:11 - ezip engine 0 buffer count
    #[inline(always)]
    pub fn buf_cnt0(&self) -> BufCnt0R {
        BufCnt0R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:14 - ezip engine 0 status
    #[inline(always)]
    pub fn run_stat0(&self) -> RunStat0R {
        RunStat0R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 15
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:25 - ezip engine 1 line count
    #[inline(always)]
    pub fn line_cnt1(&self) -> LineCnt1R {
        LineCnt1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    ///Bits 26:27 - ezip engine 1 buffer count
    #[inline(always)]
    pub fn buf_cnt1(&self) -> BufCnt1R {
        BufCnt1R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:30 - ezip engine 1 status
    #[inline(always)]
    pub fn run_stat1(&self) -> RunStat1R {
        RunStat1R::new(((self.bits >> 28) & 7) as u8)
    }
    ///Bit 31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EZIP_STAT")
            .field("rsvd", &self.rsvd())
            .field("run_stat1", &self.run_stat1())
            .field("buf_cnt1", &self.buf_cnt1())
            .field("line_cnt1", &self.line_cnt1())
            .field("rsvd2", &self.rsvd2())
            .field("run_stat0", &self.run_stat0())
            .field("buf_cnt0", &self.buf_cnt0())
            .field("line_cnt0", &self.line_cnt0())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - ezip engine 0 line count
    #[inline(always)]
    pub fn line_cnt0(&mut self) -> LineCnt0W<EZIP_STATrs> {
        LineCnt0W::new(self, 0)
    }
    ///Bits 10:11 - ezip engine 0 buffer count
    #[inline(always)]
    pub fn buf_cnt0(&mut self) -> BufCnt0W<EZIP_STATrs> {
        BufCnt0W::new(self, 10)
    }
    ///Bits 12:14 - ezip engine 0 status
    #[inline(always)]
    pub fn run_stat0(&mut self) -> RunStat0W<EZIP_STATrs> {
        RunStat0W::new(self, 12)
    }
    ///Bit 15
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<EZIP_STATrs> {
        Rsvd2W::new(self, 15)
    }
    ///Bits 16:25 - ezip engine 1 line count
    #[inline(always)]
    pub fn line_cnt1(&mut self) -> LineCnt1W<EZIP_STATrs> {
        LineCnt1W::new(self, 16)
    }
    ///Bits 26:27 - ezip engine 1 buffer count
    #[inline(always)]
    pub fn buf_cnt1(&mut self) -> BufCnt1W<EZIP_STATrs> {
        BufCnt1W::new(self, 26)
    }
    ///Bits 28:30 - ezip engine 1 status
    #[inline(always)]
    pub fn run_stat1(&mut self) -> RunStat1W<EZIP_STATrs> {
        RunStat1W::new(self, 28)
    }
    ///Bit 31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<EZIP_STATrs> {
        RsvdW::new(self, 31)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`ezip_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ezip_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct EZIP_STATrs;
impl crate::RegisterSpec for EZIP_STATrs {
    type Ux = u32;
}
///`read()` method returns [`ezip_stat::R`](R) reader structure
impl crate::Readable for EZIP_STATrs {}
///`write(|w| ..)` method takes [`ezip_stat::W`](W) writer structure
impl crate::Writable for EZIP_STATrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EZIP_STAT to value 0
impl crate::Resettable for EZIP_STATrs {
    const RESET_VALUE: u32 = 0;
}
