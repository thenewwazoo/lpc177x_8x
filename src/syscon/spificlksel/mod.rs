#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SPIFICLKSEL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct SPIFIDIVR {
    bits: u8,
}
impl SPIFIDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SPIFISEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIFISELR {
    #[doc = "Sysclk is used as the input to the SPIFI clock divider."]
    SE_SYSCLK,
    #[doc = "The output of the Main PLL is used as the input to the SPIFI clock divider."]
    MAIN_PLL,
    #[doc = "The output of the Alt PLL is used as the input to the SPIFI clock divider."]
    ALT_PLL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SPIFISELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SPIFISELR::SE_SYSCLK => 0,
            SPIFISELR::MAIN_PLL => 1,
            SPIFISELR::ALT_PLL => 2,
            SPIFISELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SPIFISELR {
        match value {
            0 => SPIFISELR::SE_SYSCLK,
            1 => SPIFISELR::MAIN_PLL,
            2 => SPIFISELR::ALT_PLL,
            i => SPIFISELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SE_SYSCLK`"]
    #[inline]
    pub fn is_se_sysclk(&self) -> bool {
        *self == SPIFISELR::SE_SYSCLK
    }
    #[doc = "Checks if the value of the field is `MAIN_PLL`"]
    #[inline]
    pub fn is_main_pll(&self) -> bool {
        *self == SPIFISELR::MAIN_PLL
    }
    #[doc = "Checks if the value of the field is `ALT_PLL`"]
    #[inline]
    pub fn is_alt_pll(&self) -> bool {
        *self == SPIFISELR::ALT_PLL
    }
}
#[doc = r" Proxy"]
pub struct _SPIFIDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _SPIFIDIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPIFISEL`"]
pub enum SPIFISELW {
    #[doc = "Sysclk is used as the input to the SPIFI clock divider."]
    SE_SYSCLK,
    #[doc = "The output of the Main PLL is used as the input to the SPIFI clock divider."]
    MAIN_PLL,
    #[doc = "The output of the Alt PLL is used as the input to the SPIFI clock divider."]
    ALT_PLL,
}
impl SPIFISELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SPIFISELW::SE_SYSCLK => 0,
            SPIFISELW::MAIN_PLL => 1,
            SPIFISELW::ALT_PLL => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPIFISELW<'a> {
    w: &'a mut W,
}
impl<'a> _SPIFISELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPIFISELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Sysclk is used as the input to the SPIFI clock divider."]
    #[inline]
    pub fn se_sysclk(self) -> &'a mut W {
        self.variant(SPIFISELW::SE_SYSCLK)
    }
    #[doc = "The output of the Main PLL is used as the input to the SPIFI clock divider."]
    #[inline]
    pub fn main_pll(self) -> &'a mut W {
        self.variant(SPIFISELW::MAIN_PLL)
    }
    #[doc = "The output of the Alt PLL is used as the input to the SPIFI clock divider."]
    #[inline]
    pub fn alt_pll(self) -> &'a mut W {
        self.variant(SPIFISELW::ALT_PLL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:4 - Selects the divide value for creating the SPIFI clock from the selected clock source. 0 = The divider is turned off., no clock will be provided to the SPIFI. 1 = The input clock is divided by 1 to produce the SPIFI clock. 2 = The input clock is divided by 2 to produce the SPIFI clock. 3 = The input clock is divided by 3 to produce the SPIFI clock. ... 31 = The input clock is divided by 31 to produce the SPIFI clock."]
    #[inline]
    pub fn spifidiv(&self) -> SPIFIDIVR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SPIFIDIVR { bits }
    }
    #[doc = "Bits 8:9 - Selects the input clock for the USB clock divider."]
    #[inline]
    pub fn spifisel(&self) -> SPIFISELR {
        SPIFISELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - Selects the divide value for creating the SPIFI clock from the selected clock source. 0 = The divider is turned off., no clock will be provided to the SPIFI. 1 = The input clock is divided by 1 to produce the SPIFI clock. 2 = The input clock is divided by 2 to produce the SPIFI clock. 3 = The input clock is divided by 3 to produce the SPIFI clock. ... 31 = The input clock is divided by 31 to produce the SPIFI clock."]
    #[inline]
    pub fn spifidiv(&mut self) -> _SPIFIDIVW {
        _SPIFIDIVW { w: self }
    }
    #[doc = "Bits 8:9 - Selects the input clock for the USB clock divider."]
    #[inline]
    pub fn spifisel(&mut self) -> _SPIFISELW {
        _SPIFISELW { w: self }
    }
}
