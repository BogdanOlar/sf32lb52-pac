///Register `RECORD_DATA_SEL` reader
pub type R = crate::R<RECORD_DATA_SELrs>;
///Register `RECORD_DATA_SEL` writer
pub type W = crate::W<RECORD_DATA_SELrs>;
///Field `RS_DATA_SEL` reader - 0: I2S audio recording 1: BT recording
pub type RsDataSelR = crate::BitReader;
///Field `RS_DATA_SEL` writer - 0: I2S audio recording 1: BT recording
pub type RsDataSelW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    ///Bit 0 - 0: I2S audio recording 1: BT recording
    #[inline(always)]
    pub fn rs_data_sel(&self) -> RsDataSelR {
        RsDataSelR::new((self.bits & 1) != 0)
    }
    ///Bits 1:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RECORD_DATA_SEL")
            .field("rsvd", &self.rsvd())
            .field("rs_data_sel", &self.rs_data_sel())
            .finish()
    }
}
impl W {
    ///Bit 0 - 0: I2S audio recording 1: BT recording
    #[inline(always)]
    pub fn rs_data_sel(&mut self) -> RsDataSelW<RECORD_DATA_SELrs> {
        RsDataSelW::new(self, 0)
    }
    ///Bits 1:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<RECORD_DATA_SELrs> {
        RsvdW::new(self, 1)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`record_data_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`record_data_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RECORD_DATA_SELrs;
impl crate::RegisterSpec for RECORD_DATA_SELrs {
    type Ux = u32;
}
///`read()` method returns [`record_data_sel::R`](R) reader structure
impl crate::Readable for RECORD_DATA_SELrs {}
///`write(|w| ..)` method takes [`record_data_sel::W`](W) writer structure
impl crate::Writable for RECORD_DATA_SELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RECORD_DATA_SEL to value 0
impl crate::Resettable for RECORD_DATA_SELrs {
    const RESET_VALUE: u32 = 0;
}
