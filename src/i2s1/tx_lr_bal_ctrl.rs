///Register `TX_LR_BAL_CTRL` reader
pub type R = crate::R<TX_LR_BAL_CTRLrs>;
///Register `TX_LR_BAL_CTRL` writer
pub type W = crate::W<TX_LR_BAL_CTRLrs>;
///Field `BAL_VOL` reader - Balance volume control: 0000: Reserved, 0001: -1.5dB, 0010: -3.0dB, 0011: -4.5dB, 0100: -6.0dB, 0101: -7.5dB, 0110: -9.0dB, 0111: -10.5dB, 1000: -12dB, 1001: -13.5dB, 1010: -15dB, 1011: -16.5dB, 1100: -18dB, 1101: -19.5dB, 1110: -21dB, 1111: mute Note: 1) bit\[5:0\]
///= 101111 for left mute 2) bit\[5:0\]
///= 011111 for right mute 3) bit\[5:4\]
///= 00 or 11, bit\[3:0\]
///is don't care 4) +1.5db = 20log(1+1/4-1/16+1/1024) 5) -1.5dB = 20log(1-1/8-1/32-1/512-1/2048)
pub type BalVolR = crate::FieldReader;
///Field `BAL_VOL` writer - Balance volume control: 0000: Reserved, 0001: -1.5dB, 0010: -3.0dB, 0011: -4.5dB, 0100: -6.0dB, 0101: -7.5dB, 0110: -9.0dB, 0111: -10.5dB, 1000: -12dB, 1001: -13.5dB, 1010: -15dB, 1011: -16.5dB, 1100: -18dB, 1101: -19.5dB, 1110: -21dB, 1111: mute Note: 1) bit\[5:0\]
///= 101111 for left mute 2) bit\[5:0\]
///= 011111 for right mute 3) bit\[5:4\]
///= 00 or 11, bit\[3:0\]
///is don't care 4) +1.5db = 20log(1+1/4-1/16+1/1024) 5) -1.5dB = 20log(1-1/8-1/32-1/512-1/2048)
pub type BalVolW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `EN` reader - LR balance enable: 00: both left and right in full volume 10: left channel balance volume adjustment enable 01: right channel balance volume adjustment enable 11: reserved, still kepp left and right in full volume
pub type EnR = crate::FieldReader;
///Field `EN` writer - LR balance enable: 00: both left and right in full volume 10: left channel balance volume adjustment enable 01: right channel balance volume adjustment enable 11: reserved, still kepp left and right in full volume
pub type EnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:3 - Balance volume control: 0000: Reserved, 0001: -1.5dB, 0010: -3.0dB, 0011: -4.5dB, 0100: -6.0dB, 0101: -7.5dB, 0110: -9.0dB, 0111: -10.5dB, 1000: -12dB, 1001: -13.5dB, 1010: -15dB, 1011: -16.5dB, 1100: -18dB, 1101: -19.5dB, 1110: -21dB, 1111: mute Note: 1) bit\[5:0\]
    ///= 101111 for left mute 2) bit\[5:0\]
    ///= 011111 for right mute 3) bit\[5:4\]
    ///= 00 or 11, bit\[3:0\]
    ///is don't care 4) +1.5db = 20log(1+1/4-1/16+1/1024) 5) -1.5dB = 20log(1-1/8-1/32-1/512-1/2048)
    #[inline(always)]
    pub fn bal_vol(&self) -> BalVolR {
        BalVolR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:5 - LR balance enable: 00: both left and right in full volume 10: left channel balance volume adjustment enable 01: right channel balance volume adjustment enable 11: reserved, still kepp left and right in full volume
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_LR_BAL_CTRL")
            .field("en", &self.en())
            .field("bal_vol", &self.bal_vol())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Balance volume control: 0000: Reserved, 0001: -1.5dB, 0010: -3.0dB, 0011: -4.5dB, 0100: -6.0dB, 0101: -7.5dB, 0110: -9.0dB, 0111: -10.5dB, 1000: -12dB, 1001: -13.5dB, 1010: -15dB, 1011: -16.5dB, 1100: -18dB, 1101: -19.5dB, 1110: -21dB, 1111: mute Note: 1) bit\[5:0\]
    ///= 101111 for left mute 2) bit\[5:0\]
    ///= 011111 for right mute 3) bit\[5:4\]
    ///= 00 or 11, bit\[3:0\]
    ///is don't care 4) +1.5db = 20log(1+1/4-1/16+1/1024) 5) -1.5dB = 20log(1-1/8-1/32-1/512-1/2048)
    #[inline(always)]
    pub fn bal_vol(&mut self) -> BalVolW<TX_LR_BAL_CTRLrs> {
        BalVolW::new(self, 0)
    }
    ///Bits 4:5 - LR balance enable: 00: both left and right in full volume 10: left channel balance volume adjustment enable 01: right channel balance volume adjustment enable 11: reserved, still kepp left and right in full volume
    #[inline(always)]
    pub fn en(&mut self) -> EnW<TX_LR_BAL_CTRLrs> {
        EnW::new(self, 4)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`tx_lr_bal_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_lr_bal_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TX_LR_BAL_CTRLrs;
impl crate::RegisterSpec for TX_LR_BAL_CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`tx_lr_bal_ctrl::R`](R) reader structure
impl crate::Readable for TX_LR_BAL_CTRLrs {}
///`write(|w| ..)` method takes [`tx_lr_bal_ctrl::W`](W) writer structure
impl crate::Writable for TX_LR_BAL_CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TX_LR_BAL_CTRL to value 0
impl crate::Resettable for TX_LR_BAL_CTRLrs {
    const RESET_VALUE: u32 = 0;
}
