///Register `DLR` reader
pub type R = crate::R<DLRrs>;
///Register `DLR` writer
pub type W = crate::W<DLRrs>;
///Field `DATA_LEN` reader - Data length value. The number of data bytes is data_len+1. The number of data bytes should be a multiple of data block size. 0 is 1 byte. 0x1ff is 512 bytes. Max is 63.5KB.
pub type DataLenR = crate::FieldReader<u16>;
///Field `DATA_LEN` writer - Data length value. The number of data bytes is data_len+1. The number of data bytes should be a multiple of data block size. 0 is 1 byte. 0x1ff is 512 bytes. Max is 63.5KB.
pub type DataLenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `BLOCK_TRAN_NUM` reader - The number of blocks which have been transferred successfully 1 = 1 block transferred It is cleared when start transfer data bit is set.
pub type BlockTranNumR = crate::FieldReader<u16>;
///Field `BLOCK_TRAN_NUM` writer - The number of blocks which have been transferred successfully 1 = 1 block transferred It is cleared when start transfer data bit is set.
pub type BlockTranNumW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Data length value. The number of data bytes is data_len+1. The number of data bytes should be a multiple of data block size. 0 is 1 byte. 0x1ff is 512 bytes. Max is 63.5KB.
    #[inline(always)]
    pub fn data_len(&self) -> DataLenR {
        DataLenR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - The number of blocks which have been transferred successfully 1 = 1 block transferred It is cleared when start transfer data bit is set.
    #[inline(always)]
    pub fn block_tran_num(&self) -> BlockTranNumR {
        BlockTranNumR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLR")
            .field("block_tran_num", &self.block_tran_num())
            .field("data_len", &self.data_len())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Data length value. The number of data bytes is data_len+1. The number of data bytes should be a multiple of data block size. 0 is 1 byte. 0x1ff is 512 bytes. Max is 63.5KB.
    #[inline(always)]
    pub fn data_len(&mut self) -> DataLenW<DLRrs> {
        DataLenW::new(self, 0)
    }
    ///Bits 16:31 - The number of blocks which have been transferred successfully 1 = 1 block transferred It is cleared when start transfer data bit is set.
    #[inline(always)]
    pub fn block_tran_num(&mut self) -> BlockTranNumW<DLRrs> {
        BlockTranNumW::new(self, 16)
    }
}
///data length register
///
///You can [`read`](crate::Reg::read) this register and get [`dlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DLRrs;
impl crate::RegisterSpec for DLRrs {
    type Ux = u32;
}
///`read()` method returns [`dlr::R`](R) reader structure
impl crate::Readable for DLRrs {}
///`write(|w| ..)` method takes [`dlr::W`](W) writer structure
impl crate::Writable for DLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DLR to value 0
impl crate::Resettable for DLRrs {
    const RESET_VALUE: u32 = 0;
}
