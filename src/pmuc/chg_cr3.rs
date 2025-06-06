///Register `CHG_CR3` reader
pub type R = crate::R<CHG_CR3rs>;
///Register `CHG_CR3` writer
pub type W = crate::W<CHG_CR3rs>;
///Field `DLY1` reader -
pub type Dly1R = crate::FieldReader;
///Field `DLY1` writer -
pub type Dly1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `DLY2` reader -
pub type Dly2R = crate::FieldReader;
///Field `DLY2` writer -
pub type Dly2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `FORCE_RST` reader - When charger plugged out, this bit will auto reset
pub type ForceRstR = crate::BitReader;
///Field `FORCE_RST` writer - When charger plugged out, this bit will auto reset
pub type ForceRstW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FORCE_CTRL` reader - When charger plugged out, this bit will auto reset
pub type ForceCtrlR = crate::BitReader;
///Field `FORCE_CTRL` writer - When charger plugged out, this bit will auto reset
pub type ForceCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:5
    #[inline(always)]
    pub fn dly1(&self) -> Dly1R {
        Dly1R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:10
    #[inline(always)]
    pub fn dly2(&self) -> Dly2R {
        Dly2R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bit 30 - When charger plugged out, this bit will auto reset
    #[inline(always)]
    pub fn force_rst(&self) -> ForceRstR {
        ForceRstR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - When charger plugged out, this bit will auto reset
    #[inline(always)]
    pub fn force_ctrl(&self) -> ForceCtrlR {
        ForceCtrlR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHG_CR3")
            .field("force_ctrl", &self.force_ctrl())
            .field("force_rst", &self.force_rst())
            .field("dly2", &self.dly2())
            .field("dly1", &self.dly1())
            .finish()
    }
}
impl W {
    ///Bits 0:5
    #[inline(always)]
    pub fn dly1(&mut self) -> Dly1W<CHG_CR3rs> {
        Dly1W::new(self, 0)
    }
    ///Bits 6:10
    #[inline(always)]
    pub fn dly2(&mut self) -> Dly2W<CHG_CR3rs> {
        Dly2W::new(self, 6)
    }
    ///Bit 30 - When charger plugged out, this bit will auto reset
    #[inline(always)]
    pub fn force_rst(&mut self) -> ForceRstW<CHG_CR3rs> {
        ForceRstW::new(self, 30)
    }
    ///Bit 31 - When charger plugged out, this bit will auto reset
    #[inline(always)]
    pub fn force_ctrl(&mut self) -> ForceCtrlW<CHG_CR3rs> {
        ForceCtrlW::new(self, 31)
    }
}
///Charger Control Register 3
///
///You can [`read`](crate::Reg::read) this register and get [`chg_cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chg_cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CHG_CR3rs;
impl crate::RegisterSpec for CHG_CR3rs {
    type Ux = u32;
}
///`read()` method returns [`chg_cr3::R`](R) reader structure
impl crate::Readable for CHG_CR3rs {}
///`write(|w| ..)` method takes [`chg_cr3::W`](W) writer structure
impl crate::Writable for CHG_CR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CHG_CR3 to value 0
impl crate::Resettable for CHG_CR3rs {
    const RESET_VALUE: u32 = 0;
}
