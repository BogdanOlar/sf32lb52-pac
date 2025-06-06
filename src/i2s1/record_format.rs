///Register `RECORD_FORMAT` reader
pub type R = crate::R<RECORD_FORMATrs>;
///Register `RECORD_FORMAT` writer
pub type W = crate::W<RECORD_FORMATrs>;
///Field `DW` reader - 0: 8bit 1: 16bit RX fifo data format: Mono 8 bit (unsigned): RX FIFO_DIN\[31:0\]
///= {L3,L2,L1,L0}, each four samples need one FIFO write operation Stereo 8 bit (unsigned): RX_FIFO_DIN\[31:0\]
///= {R1,L1,R0,L0}, each tow samples need one FIFO write operation Mono 16 bit (Signed 2's complement): RX_FIFO_DIN\[31:0\]
///= {L1,L0}, each two samples need one FIFO write operation Stereo 16 bit (Signed 2's complement): RX_FIFO_DIN\[31:0\]
///= {R0,L0}, each sample need one FIFO write operation
pub type DwR = crate::BitReader;
///Field `DW` writer - 0: 8bit 1: 16bit RX fifo data format: Mono 8 bit (unsigned): RX FIFO_DIN\[31:0\]
///= {L3,L2,L1,L0}, each four samples need one FIFO write operation Stereo 8 bit (unsigned): RX_FIFO_DIN\[31:0\]
///= {R1,L1,R0,L0}, each tow samples need one FIFO write operation Mono 16 bit (Signed 2's complement): RX_FIFO_DIN\[31:0\]
///= {L1,L0}, each two samples need one FIFO write operation Stereo 16 bit (Signed 2's complement): RX_FIFO_DIN\[31:0\]
///= {R0,L0}, each sample need one FIFO write operation
pub type DwW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRACK` reader - 1: mono recording, 0: stereo recording
pub type TrackR = crate::BitReader;
///Field `TRACK` writer - 1: mono recording, 0: stereo recording
pub type TrackW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    ///Bit 0 - 0: 8bit 1: 16bit RX fifo data format: Mono 8 bit (unsigned): RX FIFO_DIN\[31:0\]
    ///= {L3,L2,L1,L0}, each four samples need one FIFO write operation Stereo 8 bit (unsigned): RX_FIFO_DIN\[31:0\]
    ///= {R1,L1,R0,L0}, each tow samples need one FIFO write operation Mono 16 bit (Signed 2's complement): RX_FIFO_DIN\[31:0\]
    ///= {L1,L0}, each two samples need one FIFO write operation Stereo 16 bit (Signed 2's complement): RX_FIFO_DIN\[31:0\]
    ///= {R0,L0}, each sample need one FIFO write operation
    #[inline(always)]
    pub fn dw(&self) -> DwR {
        DwR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - 1: mono recording, 0: stereo recording
    #[inline(always)]
    pub fn track(&self) -> TrackR {
        TrackR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RECORD_FORMAT")
            .field("rsvd", &self.rsvd())
            .field("track", &self.track())
            .field("dw", &self.dw())
            .finish()
    }
}
impl W {
    ///Bit 0 - 0: 8bit 1: 16bit RX fifo data format: Mono 8 bit (unsigned): RX FIFO_DIN\[31:0\]
    ///= {L3,L2,L1,L0}, each four samples need one FIFO write operation Stereo 8 bit (unsigned): RX_FIFO_DIN\[31:0\]
    ///= {R1,L1,R0,L0}, each tow samples need one FIFO write operation Mono 16 bit (Signed 2's complement): RX_FIFO_DIN\[31:0\]
    ///= {L1,L0}, each two samples need one FIFO write operation Stereo 16 bit (Signed 2's complement): RX_FIFO_DIN\[31:0\]
    ///= {R0,L0}, each sample need one FIFO write operation
    #[inline(always)]
    pub fn dw(&mut self) -> DwW<RECORD_FORMATrs> {
        DwW::new(self, 0)
    }
    ///Bit 1 - 1: mono recording, 0: stereo recording
    #[inline(always)]
    pub fn track(&mut self) -> TrackW<RECORD_FORMATrs> {
        TrackW::new(self, 1)
    }
    ///Bits 2:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<RECORD_FORMATrs> {
        RsvdW::new(self, 2)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`record_format::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`record_format::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RECORD_FORMATrs;
impl crate::RegisterSpec for RECORD_FORMATrs {
    type Ux = u32;
}
///`read()` method returns [`record_format::R`](R) reader structure
impl crate::Readable for RECORD_FORMATrs {}
///`write(|w| ..)` method takes [`record_format::W`](W) writer structure
impl crate::Writable for RECORD_FORMATrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RECORD_FORMAT to value 0
impl crate::Resettable for RECORD_FORMATrs {
    const RESET_VALUE: u32 = 0;
}
