///Register `TX_PCM_FORMAT` reader
pub type R = crate::R<TX_PCM_FORMATrs>;
///Register `TX_PCM_FORMAT` writer
pub type W = crate::W<TX_PCM_FORMATrs>;
///Field `DW` reader - tx source pcm data width N(N>=8) common value is 8,13,14,16,18,20,22,24 This data width indicate the tx fifo output data width. When writing to tx fifo, please refer to following format: Mono 8 bit: fifo_data\[31:0\]
///= {L3,L2,L1,L0}, each word contains 4 samples, so four samples need read one word Stereo 8 bit: fifo_data\[31:0\]
///= { R1,L1,R0,L0 }, each word contains 2 samples, so two samples need read one word Mono 13/14/16 bit: fifo_data\[31:0\]
///= {L1,L0}, each word contains 2 samples, so two samples need read one word Stereo 13/14/16 bit: fifo_data\[31:0\]
///= {R0,L0}, each word contains 1 samples, so each sample need read one word Mono 18/20/22/24 bit: fifo_data\[31:0\]
///= L0, each word contains 1 samples, so each sample need read one word Stereo 18/20/22/24 bit: fifo_data\[31:0\]\[0\]
///= {L0}, fifo_data\[31:0\]\[1\]={R0}, each 2 words contain 1 samples, so each sample need read two word
pub type DwR = crate::FieldReader;
///Field `DW` writer - tx source pcm data width N(N>=8) common value is 8,13,14,16,18,20,22,24 This data width indicate the tx fifo output data width. When writing to tx fifo, please refer to following format: Mono 8 bit: fifo_data\[31:0\]
///= {L3,L2,L1,L0}, each word contains 4 samples, so four samples need read one word Stereo 8 bit: fifo_data\[31:0\]
///= { R1,L1,R0,L0 }, each word contains 2 samples, so two samples need read one word Mono 13/14/16 bit: fifo_data\[31:0\]
///= {L1,L0}, each word contains 2 samples, so two samples need read one word Stereo 13/14/16 bit: fifo_data\[31:0\]
///= {R0,L0}, each word contains 1 samples, so each sample need read one word Mono 18/20/22/24 bit: fifo_data\[31:0\]
///= L0, each word contains 1 samples, so each sample need read one word Stereo 18/20/22/24 bit: fifo_data\[31:0\]\[0\]
///= {L0}, fifo_data\[31:0\]\[1\]={R0}, each 2 words contain 1 samples, so each sample need read two word
pub type DwW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `TRACK_FLAG` reader - 0: stereo 1: mono
pub type TrackFlagR = crate::BitReader;
///Field `TRACK_FLAG` writer - 0: stereo 1: mono
pub type TrackFlagW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    ///Bits 0:4 - tx source pcm data width N(N>=8) common value is 8,13,14,16,18,20,22,24 This data width indicate the tx fifo output data width. When writing to tx fifo, please refer to following format: Mono 8 bit: fifo_data\[31:0\]
    ///= {L3,L2,L1,L0}, each word contains 4 samples, so four samples need read one word Stereo 8 bit: fifo_data\[31:0\]
    ///= { R1,L1,R0,L0 }, each word contains 2 samples, so two samples need read one word Mono 13/14/16 bit: fifo_data\[31:0\]
    ///= {L1,L0}, each word contains 2 samples, so two samples need read one word Stereo 13/14/16 bit: fifo_data\[31:0\]
    ///= {R0,L0}, each word contains 1 samples, so each sample need read one word Mono 18/20/22/24 bit: fifo_data\[31:0\]
    ///= L0, each word contains 1 samples, so each sample need read one word Stereo 18/20/22/24 bit: fifo_data\[31:0\]\[0\]
    ///= {L0}, fifo_data\[31:0\]\[1\]={R0}, each 2 words contain 1 samples, so each sample need read two word
    #[inline(always)]
    pub fn dw(&self) -> DwR {
        DwR::new((self.bits & 0x1f) as u8)
    }
    ///Bit 5 - 0: stereo 1: mono
    #[inline(always)]
    pub fn track_flag(&self) -> TrackFlagR {
        TrackFlagR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_PCM_FORMAT")
            .field("rsvd", &self.rsvd())
            .field("track_flag", &self.track_flag())
            .field("dw", &self.dw())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - tx source pcm data width N(N>=8) common value is 8,13,14,16,18,20,22,24 This data width indicate the tx fifo output data width. When writing to tx fifo, please refer to following format: Mono 8 bit: fifo_data\[31:0\]
    ///= {L3,L2,L1,L0}, each word contains 4 samples, so four samples need read one word Stereo 8 bit: fifo_data\[31:0\]
    ///= { R1,L1,R0,L0 }, each word contains 2 samples, so two samples need read one word Mono 13/14/16 bit: fifo_data\[31:0\]
    ///= {L1,L0}, each word contains 2 samples, so two samples need read one word Stereo 13/14/16 bit: fifo_data\[31:0\]
    ///= {R0,L0}, each word contains 1 samples, so each sample need read one word Mono 18/20/22/24 bit: fifo_data\[31:0\]
    ///= L0, each word contains 1 samples, so each sample need read one word Stereo 18/20/22/24 bit: fifo_data\[31:0\]\[0\]
    ///= {L0}, fifo_data\[31:0\]\[1\]={R0}, each 2 words contain 1 samples, so each sample need read two word
    #[inline(always)]
    pub fn dw(&mut self) -> DwW<TX_PCM_FORMATrs> {
        DwW::new(self, 0)
    }
    ///Bit 5 - 0: stereo 1: mono
    #[inline(always)]
    pub fn track_flag(&mut self) -> TrackFlagW<TX_PCM_FORMATrs> {
        TrackFlagW::new(self, 5)
    }
    ///Bits 6:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<TX_PCM_FORMATrs> {
        RsvdW::new(self, 6)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`tx_pcm_format::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_pcm_format::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TX_PCM_FORMATrs;
impl crate::RegisterSpec for TX_PCM_FORMATrs {
    type Ux = u32;
}
///`read()` method returns [`tx_pcm_format::R`](R) reader structure
impl crate::Readable for TX_PCM_FORMATrs {}
///`write(|w| ..)` method takes [`tx_pcm_format::W`](W) writer structure
impl crate::Writable for TX_PCM_FORMATrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TX_PCM_FORMAT to value 0x10
impl crate::Resettable for TX_PCM_FORMATrs {
    const RESET_VALUE: u32 = 0x10;
}
