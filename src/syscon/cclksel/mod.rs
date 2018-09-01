#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCLKSEL {
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
pub struct CCLKDIVR {
    bits: u8,
}
impl CCLKDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CCLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCLKSELR {
    #[doc = "Sysclk is used as the input to the CPU clock divider."]
    SYSCLK,
    #[doc = "The output of the Main PLL is used as the input to the CPU clock divider."]
    MAIN_PLL,
}
impl CCLKSELR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CCLKSELR::SYSCLK => false,
            CCLKSELR::MAIN_PLL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCLKSELR {
        match value {
            false => CCLKSELR::SYSCLK,
            true => CCLKSELR::MAIN_PLL,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline]
    pub fn is_sysclk(&self) -> bool {
        *self == CCLKSELR::SYSCLK
    }
    #[doc = "Checks if the value of the field is `MAIN_PLL`"]
    #[inline]
    pub fn is_main_pll(&self) -> bool {
        *self == CCLKSELR::MAIN_PLL
    }
}
#[doc = r" Proxy"]
pub struct _CCLKDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _CCLKDIVW<'a> {
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
#[doc = "Values that can be written to the field `CCLKSEL`"]
pub enum CCLKSELW {
    #[doc = "Sysclk is used as the input to the CPU clock divider."]
    SYSCLK,
    #[doc = "The output of the Main PLL is used as the input to the CPU clock divider."]
    MAIN_PLL,
}
impl CCLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCLKSELW::SYSCLK => false,
            CCLKSELW::MAIN_PLL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CCLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCLKSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Sysclk is used as the input to the CPU clock divider."]
    #[inline]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(CCLKSELW::SYSCLK)
    }
    #[doc = "The output of the Main PLL is used as the input to the CPU clock divider."]
    #[inline]
    pub fn main_pll(self) -> &'a mut W {
        self.variant(CCLKSELW::MAIN_PLL)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
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
    #[doc = "Bits 0:4 - Selects the divide value for creating the CPU clock (CCLK) from the selected clock source. 0 = The divider is turned off., no clock will be provided to the CPU. This setting should typically not be used, the CPU will be halted and a reset will be required to restore operation. 1 = The input clock is divided by 1 to produce the CPU clock. 2 = The input clock is divided by 2 to produce the CPU clock. 3 = The input clock is divided by 3 to produce the CPU clock. ... 31 = The input clock is divided by 31 to produce the CPU clock."]
    #[inline]
    pub fn cclkdiv(&self) -> CCLKDIVR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CCLKDIVR { bits }
    }
    #[doc = "Bit 8 - Selects the input clock for the CPU clock divider."]
    #[inline]
    pub fn cclksel(&self) -> CCLKSELR {
        CCLKSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bits 0:4 - Selects the divide value for creating the CPU clock (CCLK) from the selected clock source. 0 = The divider is turned off., no clock will be provided to the CPU. This setting should typically not be used, the CPU will be halted and a reset will be required to restore operation. 1 = The input clock is divided by 1 to produce the CPU clock. 2 = The input clock is divided by 2 to produce the CPU clock. 3 = The input clock is divided by 3 to produce the CPU clock. ... 31 = The input clock is divided by 31 to produce the CPU clock."]
    #[inline]
    pub fn cclkdiv(&mut self) -> _CCLKDIVW {
        _CCLKDIVW { w: self }
    }
    #[doc = "Bit 8 - Selects the input clock for the CPU clock divider."]
    #[inline]
    pub fn cclksel(&mut self) -> _CCLKSELW {
        _CCLKSELW { w: self }
    }
}
