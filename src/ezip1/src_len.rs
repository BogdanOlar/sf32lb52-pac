///Register `SRC_LEN` reader
pub type R = crate::R<SRC_LENrs>;
///Register `SRC_LEN` writer
pub type W = crate::W<SRC_LENrs>;
///Field `SRC_LEN` reader - source data byte length only in source data fifo mode
pub type SrcLenR = crate::FieldReader<u32>;
///Field `SRC_LEN` writer - source data byte length only in source data fifo mode
pub type SrcLenW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - source data byte length only in source data fifo mode
    #[inline(always)]
    pub fn src_len(&self) -> SrcLenR {
        SrcLenR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRC_LEN")
            .field("src_len", &self.src_len())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - source data byte length only in source data fifo mode
    #[inline(always)]
    pub fn src_len(&mut self) -> SrcLenW<SRC_LENrs> {
        SrcLenW::new(self, 0)
    }
}
///ezip source data length
///
///You can [`read`](crate::Reg::read) this register and get [`src_len::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src_len::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SRC_LENrs;
impl crate::RegisterSpec for SRC_LENrs {
    type Ux = u32;
}
///`read()` method returns [`src_len::R`](R) reader structure
impl crate::Readable for SRC_LENrs {}
///`write(|w| ..)` method takes [`src_len::W`](W) writer structure
impl crate::Writable for SRC_LENrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SRC_LEN to value 0
impl crate::Resettable for SRC_LENrs {
    const RESET_VALUE: u32 = 0;
}
