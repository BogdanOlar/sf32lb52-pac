///Register `TX_VOL_CTRL` reader
pub type R = crate::R<TX_VOL_CTRLrs>;
///Register `TX_VOL_CTRL` writer
pub type W = crate::W<TX_VOL_CTRLrs>;
///Field `VOL` reader - volume control: 0000: +6dB, 0001: +4.5dB, 0010: +3dB, 0011: +1.5dB, 0100: 0dB, 0101: -1.5dB, 0110: -3.0dB, 0111: -4.5dB, 1000: -6.0dB, 1001: -7.5dB, 1010: -9dB, 1011: -10.5dB, 1100: -12dB, 1101: -13.5dB, 1110: -15dB, 1111: mute Note: 1) +1.5db = 20log(1+1/4-1/16+1/1024) 2) -1.5dB = 20log(1-1/8-1/32-1/512-1/2048)
pub type VolR = crate::FieldReader;
///Field `VOL` writer - volume control: 0000: +6dB, 0001: +4.5dB, 0010: +3dB, 0011: +1.5dB, 0100: 0dB, 0101: -1.5dB, 0110: -3.0dB, 0111: -4.5dB, 1000: -6.0dB, 1001: -7.5dB, 1010: -9dB, 1011: -10.5dB, 1100: -12dB, 1101: -13.5dB, 1110: -15dB, 1111: mute Note: 1) +1.5db = 20log(1+1/4-1/16+1/1024) 2) -1.5dB = 20log(1-1/8-1/32-1/512-1/2048)
pub type VolW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    ///Bits 0:3 - volume control: 0000: +6dB, 0001: +4.5dB, 0010: +3dB, 0011: +1.5dB, 0100: 0dB, 0101: -1.5dB, 0110: -3.0dB, 0111: -4.5dB, 1000: -6.0dB, 1001: -7.5dB, 1010: -9dB, 1011: -10.5dB, 1100: -12dB, 1101: -13.5dB, 1110: -15dB, 1111: mute Note: 1) +1.5db = 20log(1+1/4-1/16+1/1024) 2) -1.5dB = 20log(1-1/8-1/32-1/512-1/2048)
    #[inline(always)]
    pub fn vol(&self) -> VolR {
        VolR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_VOL_CTRL")
            .field("rsvd", &self.rsvd())
            .field("vol", &self.vol())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - volume control: 0000: +6dB, 0001: +4.5dB, 0010: +3dB, 0011: +1.5dB, 0100: 0dB, 0101: -1.5dB, 0110: -3.0dB, 0111: -4.5dB, 1000: -6.0dB, 1001: -7.5dB, 1010: -9dB, 1011: -10.5dB, 1100: -12dB, 1101: -13.5dB, 1110: -15dB, 1111: mute Note: 1) +1.5db = 20log(1+1/4-1/16+1/1024) 2) -1.5dB = 20log(1-1/8-1/32-1/512-1/2048)
    #[inline(always)]
    pub fn vol(&mut self) -> VolW<TX_VOL_CTRLrs> {
        VolW::new(self, 0)
    }
    ///Bits 4:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<TX_VOL_CTRLrs> {
        RsvdW::new(self, 4)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`tx_vol_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_vol_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TX_VOL_CTRLrs;
impl crate::RegisterSpec for TX_VOL_CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`tx_vol_ctrl::R`](R) reader structure
impl crate::Readable for TX_VOL_CTRLrs {}
///`write(|w| ..)` method takes [`tx_vol_ctrl::W`](W) writer structure
impl crate::Writable for TX_VOL_CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TX_VOL_CTRL to value 0x0f
impl crate::Resettable for TX_VOL_CTRLrs {
    const RESET_VALUE: u32 = 0x0f;
}
