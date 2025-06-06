///Register `BT_VOL_CTRL` reader
pub type R = crate::R<BT_VOL_CTRLrs>;
///Register `BT_VOL_CTRL` writer
pub type W = crate::W<BT_VOL_CTRLrs>;
///Field `VOL` reader - BT master volume
pub type VolR = crate::FieldReader;
///Field `VOL` writer - BT master volume
pub type VolW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `VOL_ADJ_EN` reader - BT volume adjust enable
pub type VolAdjEnR = crate::BitReader;
///Field `VOL_ADJ_EN` writer - BT volume adjust enable
pub type VolAdjEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSVD` reader -
pub type RsvdR = crate::FieldReader<u32>;
///Field `RSVD` writer -
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    ///Bits 0:2 - BT master volume
    #[inline(always)]
    pub fn vol(&self) -> VolR {
        VolR::new((self.bits & 7) as u8)
    }
    ///Bit 3 - BT volume adjust enable
    #[inline(always)]
    pub fn vol_adj_en(&self) -> VolAdjEnR {
        VolAdjEnR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:31
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BT_VOL_CTRL")
            .field("rsvd", &self.rsvd())
            .field("vol_adj_en", &self.vol_adj_en())
            .field("vol", &self.vol())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - BT master volume
    #[inline(always)]
    pub fn vol(&mut self) -> VolW<BT_VOL_CTRLrs> {
        VolW::new(self, 0)
    }
    ///Bit 3 - BT volume adjust enable
    #[inline(always)]
    pub fn vol_adj_en(&mut self) -> VolAdjEnW<BT_VOL_CTRLrs> {
        VolAdjEnW::new(self, 3)
    }
    ///Bits 4:31
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<BT_VOL_CTRLrs> {
        RsvdW::new(self, 4)
    }
}
///
///
///You can [`read`](crate::Reg::read) this register and get [`bt_vol_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bt_vol_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct BT_VOL_CTRLrs;
impl crate::RegisterSpec for BT_VOL_CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`bt_vol_ctrl::R`](R) reader structure
impl crate::Readable for BT_VOL_CTRLrs {}
///`write(|w| ..)` method takes [`bt_vol_ctrl::W`](W) writer structure
impl crate::Writable for BT_VOL_CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BT_VOL_CTRL to value 0
impl crate::Resettable for BT_VOL_CTRLrs {
    const RESET_VALUE: u32 = 0;
}
