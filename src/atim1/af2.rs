///Register `AF2` reader
pub type R = crate::R<AF2rs>;
///Register `AF2` writer
pub type W = crate::W<AF2rs>;
///Field `BK2INE` reader - BRK2 BKIN input enable This bit enables the BKIN2 input. BKIN2 input is 'Ored' with the other BRK2 sources. 0: BKIN2 input disabled 1: BKIN2 input enabled This bit cannot be modified as long as LOCK level 1 has been programmed.
pub type Bk2ineR = crate::BitReader;
///Field `BK2INE` writer - BRK2 BKIN input enable This bit enables the BKIN2 input. BKIN2 input is 'Ored' with the other BRK2 sources. 0: BKIN2 input disabled 1: BKIN2 input enabled This bit cannot be modified as long as LOCK level 1 has been programmed.
pub type Bk2ineW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BK2CMP1E` reader - BRK2 LPCOMP output1 enable This bit enables the LPCOMP output1 (if LPCOMP integrated) for the timer's BRK2 input. LPCOMP output1 is 'ORed' with the other BRK2 sources. 0: LPCOMP output1 disabled 1: LPCOMP output1 enabled This bit cannot be modified as long as LOCK level 1 has been programmed.
pub type Bk2cmp1eR = crate::BitReader;
///Field `BK2CMP1E` writer - BRK2 LPCOMP output1 enable This bit enables the LPCOMP output1 (if LPCOMP integrated) for the timer's BRK2 input. LPCOMP output1 is 'ORed' with the other BRK2 sources. 0: LPCOMP output1 disabled 1: LPCOMP output1 enabled This bit cannot be modified as long as LOCK level 1 has been programmed.
pub type Bk2cmp1eW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BK2CMP2E` reader - BRK2 LPCOMP output2 enable This bit enables the LPCOMP output2 (if LPCOMP integrated) for the timer's BRK2 input. LPCOMP output2 is 'ORed' with the other BRK2 sources. 0: LPCOMP output2 disabled 1: LPCOMP output2 enabled This bit cannot be modified as long as LOCK level 1 has been programmed.
pub type Bk2cmp2eR = crate::BitReader;
///Field `BK2CMP2E` writer - BRK2 LPCOMP output2 enable This bit enables the LPCOMP output2 (if LPCOMP integrated) for the timer's BRK2 input. LPCOMP output2 is 'ORed' with the other BRK2 sources. 0: LPCOMP output2 disabled 1: LPCOMP output2 enabled This bit cannot be modified as long as LOCK level 1 has been programmed.
pub type Bk2cmp2eW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BK2INP` reader - BRK2 BKIN2 input polarity This bit selects the BKIN2 input sensitivity. It must be programmed together with the BK2P polarity bit. 0: BKIN2 input is active low 1: BKIN2 input is active high This bit cannot be modified as long as LOCK level 1 has been programmed.
pub type Bk2inpR = crate::BitReader;
///Field `BK2INP` writer - BRK2 BKIN2 input polarity This bit selects the BKIN2 input sensitivity. It must be programmed together with the BK2P polarity bit. 0: BKIN2 input is active low 1: BKIN2 input is active high This bit cannot be modified as long as LOCK level 1 has been programmed.
pub type Bk2inpW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BK2CMP1P` reader - BRK2 LPCOMP output1 polarity This bit selects the LPCOMP output1 sensitivity (if LPCOMP integrated). It must be programmed together with the BK2P polarity bit. 0: LPCOMP output1 is active high 1: LPCOMP output1 is active low This bit cannot be modified as long as LOCK level 1 has been programmed.
pub type Bk2cmp1pR = crate::BitReader;
///Field `BK2CMP1P` writer - BRK2 LPCOMP output1 polarity This bit selects the LPCOMP output1 sensitivity (if LPCOMP integrated). It must be programmed together with the BK2P polarity bit. 0: LPCOMP output1 is active high 1: LPCOMP output1 is active low This bit cannot be modified as long as LOCK level 1 has been programmed.
pub type Bk2cmp1pW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BK2CMP2P` reader - BRK2 LPCOMP output2 polarity This bit selects the LPCOMP output2 sensitivity (if LPCOMP integrated). It must be programmed together with the BK2P polarity bit. 0: LPCOMP output2 is active high 1: LPCOMP output2 is active low This bit cannot be modified as long as LOCK level 1 has been programmed.
pub type Bk2cmp2pR = crate::BitReader;
///Field `BK2CMP2P` writer - BRK2 LPCOMP output2 polarity This bit selects the LPCOMP output2 sensitivity (if LPCOMP integrated). It must be programmed together with the BK2P polarity bit. 0: LPCOMP output2 is active high 1: LPCOMP output2 is active low This bit cannot be modified as long as LOCK level 1 has been programmed.
pub type Bk2cmp2pW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - BRK2 BKIN input enable This bit enables the BKIN2 input. BKIN2 input is 'Ored' with the other BRK2 sources. 0: BKIN2 input disabled 1: BKIN2 input enabled This bit cannot be modified as long as LOCK level 1 has been programmed.
    #[inline(always)]
    pub fn bk2ine(&self) -> Bk2ineR {
        Bk2ineR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - BRK2 LPCOMP output1 enable This bit enables the LPCOMP output1 (if LPCOMP integrated) for the timer's BRK2 input. LPCOMP output1 is 'ORed' with the other BRK2 sources. 0: LPCOMP output1 disabled 1: LPCOMP output1 enabled This bit cannot be modified as long as LOCK level 1 has been programmed.
    #[inline(always)]
    pub fn bk2cmp1e(&self) -> Bk2cmp1eR {
        Bk2cmp1eR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - BRK2 LPCOMP output2 enable This bit enables the LPCOMP output2 (if LPCOMP integrated) for the timer's BRK2 input. LPCOMP output2 is 'ORed' with the other BRK2 sources. 0: LPCOMP output2 disabled 1: LPCOMP output2 enabled This bit cannot be modified as long as LOCK level 1 has been programmed.
    #[inline(always)]
    pub fn bk2cmp2e(&self) -> Bk2cmp2eR {
        Bk2cmp2eR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 9 - BRK2 BKIN2 input polarity This bit selects the BKIN2 input sensitivity. It must be programmed together with the BK2P polarity bit. 0: BKIN2 input is active low 1: BKIN2 input is active high This bit cannot be modified as long as LOCK level 1 has been programmed.
    #[inline(always)]
    pub fn bk2inp(&self) -> Bk2inpR {
        Bk2inpR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - BRK2 LPCOMP output1 polarity This bit selects the LPCOMP output1 sensitivity (if LPCOMP integrated). It must be programmed together with the BK2P polarity bit. 0: LPCOMP output1 is active high 1: LPCOMP output1 is active low This bit cannot be modified as long as LOCK level 1 has been programmed.
    #[inline(always)]
    pub fn bk2cmp1p(&self) -> Bk2cmp1pR {
        Bk2cmp1pR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - BRK2 LPCOMP output2 polarity This bit selects the LPCOMP output2 sensitivity (if LPCOMP integrated). It must be programmed together with the BK2P polarity bit. 0: LPCOMP output2 is active high 1: LPCOMP output2 is active low This bit cannot be modified as long as LOCK level 1 has been programmed.
    #[inline(always)]
    pub fn bk2cmp2p(&self) -> Bk2cmp2pR {
        Bk2cmp2pR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AF2")
            .field("bk2cmp2p", &self.bk2cmp2p())
            .field("bk2cmp1p", &self.bk2cmp1p())
            .field("bk2inp", &self.bk2inp())
            .field("bk2cmp2e", &self.bk2cmp2e())
            .field("bk2cmp1e", &self.bk2cmp1e())
            .field("bk2ine", &self.bk2ine())
            .finish()
    }
}
impl W {
    ///Bit 0 - BRK2 BKIN input enable This bit enables the BKIN2 input. BKIN2 input is 'Ored' with the other BRK2 sources. 0: BKIN2 input disabled 1: BKIN2 input enabled This bit cannot be modified as long as LOCK level 1 has been programmed.
    #[inline(always)]
    pub fn bk2ine(&mut self) -> Bk2ineW<AF2rs> {
        Bk2ineW::new(self, 0)
    }
    ///Bit 1 - BRK2 LPCOMP output1 enable This bit enables the LPCOMP output1 (if LPCOMP integrated) for the timer's BRK2 input. LPCOMP output1 is 'ORed' with the other BRK2 sources. 0: LPCOMP output1 disabled 1: LPCOMP output1 enabled This bit cannot be modified as long as LOCK level 1 has been programmed.
    #[inline(always)]
    pub fn bk2cmp1e(&mut self) -> Bk2cmp1eW<AF2rs> {
        Bk2cmp1eW::new(self, 1)
    }
    ///Bit 2 - BRK2 LPCOMP output2 enable This bit enables the LPCOMP output2 (if LPCOMP integrated) for the timer's BRK2 input. LPCOMP output2 is 'ORed' with the other BRK2 sources. 0: LPCOMP output2 disabled 1: LPCOMP output2 enabled This bit cannot be modified as long as LOCK level 1 has been programmed.
    #[inline(always)]
    pub fn bk2cmp2e(&mut self) -> Bk2cmp2eW<AF2rs> {
        Bk2cmp2eW::new(self, 2)
    }
    ///Bit 9 - BRK2 BKIN2 input polarity This bit selects the BKIN2 input sensitivity. It must be programmed together with the BK2P polarity bit. 0: BKIN2 input is active low 1: BKIN2 input is active high This bit cannot be modified as long as LOCK level 1 has been programmed.
    #[inline(always)]
    pub fn bk2inp(&mut self) -> Bk2inpW<AF2rs> {
        Bk2inpW::new(self, 9)
    }
    ///Bit 10 - BRK2 LPCOMP output1 polarity This bit selects the LPCOMP output1 sensitivity (if LPCOMP integrated). It must be programmed together with the BK2P polarity bit. 0: LPCOMP output1 is active high 1: LPCOMP output1 is active low This bit cannot be modified as long as LOCK level 1 has been programmed.
    #[inline(always)]
    pub fn bk2cmp1p(&mut self) -> Bk2cmp1pW<AF2rs> {
        Bk2cmp1pW::new(self, 10)
    }
    ///Bit 11 - BRK2 LPCOMP output2 polarity This bit selects the LPCOMP output2 sensitivity (if LPCOMP integrated). It must be programmed together with the BK2P polarity bit. 0: LPCOMP output2 is active high 1: LPCOMP output2 is active low This bit cannot be modified as long as LOCK level 1 has been programmed.
    #[inline(always)]
    pub fn bk2cmp2p(&mut self) -> Bk2cmp2pW<AF2rs> {
        Bk2cmp2pW::new(self, 11)
    }
}
///Alternate function option register 2
///
///You can [`read`](crate::Reg::read) this register and get [`af2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct AF2rs;
impl crate::RegisterSpec for AF2rs {
    type Ux = u32;
}
///`read()` method returns [`af2::R`](R) reader structure
impl crate::Readable for AF2rs {}
///`write(|w| ..)` method takes [`af2::W`](W) writer structure
impl crate::Writable for AF2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AF2 to value 0x01
impl crate::Resettable for AF2rs {
    const RESET_VALUE: u32 = 0x01;
}
