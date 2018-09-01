#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USBCLKSEL {
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
#[doc = "Possible values of the field `USBDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBDIVR {
    #[doc = "The divider is turned off, no clock will be provided to the USB subsystem."]
    DIVIDER_OFF,
    #[doc = "PLL0 output is divided by 4. PLL0 output must be 192 MHz."]
    PLL0_DIV_4,
    #[doc = "PLL0 output is divided by 6. PLL0 output must be 288 MHz."]
    PLL0_DIV_6,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl USBDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            USBDIVR::DIVIDER_OFF => 0,
            USBDIVR::PLL0_DIV_4 => 4,
            USBDIVR::PLL0_DIV_6 => 6,
            USBDIVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> USBDIVR {
        match value {
            0 => USBDIVR::DIVIDER_OFF,
            4 => USBDIVR::PLL0_DIV_4,
            6 => USBDIVR::PLL0_DIV_6,
            i => USBDIVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIVIDER_OFF`"]
    #[inline]
    pub fn is_divider_off(&self) -> bool {
        *self == USBDIVR::DIVIDER_OFF
    }
    #[doc = "Checks if the value of the field is `PLL0_DIV_4`"]
    #[inline]
    pub fn is_pll0_div_4(&self) -> bool {
        *self == USBDIVR::PLL0_DIV_4
    }
    #[doc = "Checks if the value of the field is `PLL0_DIV_6`"]
    #[inline]
    pub fn is_pll0_div_6(&self) -> bool {
        *self == USBDIVR::PLL0_DIV_6
    }
}
#[doc = "Possible values of the field `USBSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBSELR {
    #[doc = "Sysclk is used as the input to the USB clock divider. When this clock is selected, the USB can be accessed by software but cannot perform USB functions."]
    SYSCLK,
    #[doc = "The output of the Main PLL is used as the input to the USB clock divider."]
    MAIN_PLL,
    #[doc = "The output of the Alt PLL is used as the input to the USB clock divider."]
    ALT_PLL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl USBSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            USBSELR::SYSCLK => 0,
            USBSELR::MAIN_PLL => 1,
            USBSELR::ALT_PLL => 2,
            USBSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> USBSELR {
        match value {
            0 => USBSELR::SYSCLK,
            1 => USBSELR::MAIN_PLL,
            2 => USBSELR::ALT_PLL,
            i => USBSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline]
    pub fn is_sysclk(&self) -> bool {
        *self == USBSELR::SYSCLK
    }
    #[doc = "Checks if the value of the field is `MAIN_PLL`"]
    #[inline]
    pub fn is_main_pll(&self) -> bool {
        *self == USBSELR::MAIN_PLL
    }
    #[doc = "Checks if the value of the field is `ALT_PLL`"]
    #[inline]
    pub fn is_alt_pll(&self) -> bool {
        *self == USBSELR::ALT_PLL
    }
}
#[doc = "Values that can be written to the field `USBDIV`"]
pub enum USBDIVW {
    #[doc = "The divider is turned off, no clock will be provided to the USB subsystem."]
    DIVIDER_OFF,
    #[doc = "PLL0 output is divided by 4. PLL0 output must be 192 MHz."]
    PLL0_DIV_4,
    #[doc = "PLL0 output is divided by 6. PLL0 output must be 288 MHz."]
    PLL0_DIV_6,
}
impl USBDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            USBDIVW::DIVIDER_OFF => 0,
            USBDIVW::PLL0_DIV_4 => 4,
            USBDIVW::PLL0_DIV_6 => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _USBDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBDIVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The divider is turned off, no clock will be provided to the USB subsystem."]
    #[inline]
    pub fn divider_off(self) -> &'a mut W {
        self.variant(USBDIVW::DIVIDER_OFF)
    }
    #[doc = "PLL0 output is divided by 4. PLL0 output must be 192 MHz."]
    #[inline]
    pub fn pll0_div_4(self) -> &'a mut W {
        self.variant(USBDIVW::PLL0_DIV_4)
    }
    #[doc = "PLL0 output is divided by 6. PLL0 output must be 288 MHz."]
    #[inline]
    pub fn pll0_div_6(self) -> &'a mut W {
        self.variant(USBDIVW::PLL0_DIV_6)
    }
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
#[doc = "Values that can be written to the field `USBSEL`"]
pub enum USBSELW {
    #[doc = "Sysclk is used as the input to the USB clock divider. When this clock is selected, the USB can be accessed by software but cannot perform USB functions."]
    SYSCLK,
    #[doc = "The output of the Main PLL is used as the input to the USB clock divider."]
    MAIN_PLL,
    #[doc = "The output of the Alt PLL is used as the input to the USB clock divider."]
    ALT_PLL,
}
impl USBSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            USBSELW::SYSCLK => 0,
            USBSELW::MAIN_PLL => 1,
            USBSELW::ALT_PLL => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBSELW<'a> {
    w: &'a mut W,
}
impl<'a> _USBSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Sysclk is used as the input to the USB clock divider. When this clock is selected, the USB can be accessed by software but cannot perform USB functions."]
    #[inline]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(USBSELW::SYSCLK)
    }
    #[doc = "The output of the Main PLL is used as the input to the USB clock divider."]
    #[inline]
    pub fn main_pll(self) -> &'a mut W {
        self.variant(USBSELW::MAIN_PLL)
    }
    #[doc = "The output of the Alt PLL is used as the input to the USB clock divider."]
    #[inline]
    pub fn alt_pll(self) -> &'a mut W {
        self.variant(USBSELW::ALT_PLL)
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
    #[doc = "Bits 0:4 - Selects the divide value for creating the USB clock from the selected PLL output. Only the values shown below can produce even number multiples of 48 MHz from the PLL. Warning: Improper setting of this value will result in incorrect operation of the USB interface. Only the main oscillator in conjunction with either PLL0 or PLL1 can provide a clock that meets USB accuracy and jitter specifications. Other values cannot produce the 48 MHz clock required for USB operation."]
    #[inline]
    pub fn usbdiv(&self) -> USBDIVR {
        USBDIVR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Selects the input clock for the USB clock divider."]
    #[inline]
    pub fn usbsel(&self) -> USBSELR {
        USBSELR::_from({
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
    #[doc = "Bits 0:4 - Selects the divide value for creating the USB clock from the selected PLL output. Only the values shown below can produce even number multiples of 48 MHz from the PLL. Warning: Improper setting of this value will result in incorrect operation of the USB interface. Only the main oscillator in conjunction with either PLL0 or PLL1 can provide a clock that meets USB accuracy and jitter specifications. Other values cannot produce the 48 MHz clock required for USB operation."]
    #[inline]
    pub fn usbdiv(&mut self) -> _USBDIVW {
        _USBDIVW { w: self }
    }
    #[doc = "Bits 8:9 - Selects the input clock for the USB clock divider."]
    #[inline]
    pub fn usbsel(&mut self) -> _USBSELW {
        _USBSELW { w: self }
    }
}
