#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLKSRCSEL {
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
#[doc = "Possible values of the field `CLKSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSRCR {
    #[doc = "Selects the Internal RC oscillator as the sysclk and PLL0 clock source (default)."]
    INTERNAL_RC_OSC,
    #[doc = "Selects the main oscillator as the sysclk and PLL0 clock source."]
    MAIN_OSC,
}
impl CLKSRCR {
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
            CLKSRCR::INTERNAL_RC_OSC => false,
            CLKSRCR::MAIN_OSC => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLKSRCR {
        match value {
            false => CLKSRCR::INTERNAL_RC_OSC,
            true => CLKSRCR::MAIN_OSC,
        }
    }
    #[doc = "Checks if the value of the field is `INTERNAL_RC_OSC`"]
    #[inline]
    pub fn is_internal_rc_osc(&self) -> bool {
        *self == CLKSRCR::INTERNAL_RC_OSC
    }
    #[doc = "Checks if the value of the field is `MAIN_OSC`"]
    #[inline]
    pub fn is_main_osc(&self) -> bool {
        *self == CLKSRCR::MAIN_OSC
    }
}
#[doc = "Values that can be written to the field `CLKSRC`"]
pub enum CLKSRCW {
    #[doc = "Selects the Internal RC oscillator as the sysclk and PLL0 clock source (default)."]
    INTERNAL_RC_OSC,
    #[doc = "Selects the main oscillator as the sysclk and PLL0 clock source."]
    MAIN_OSC,
}
impl CLKSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLKSRCW::INTERNAL_RC_OSC => false,
            CLKSRCW::MAIN_OSC => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKSRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selects the Internal RC oscillator as the sysclk and PLL0 clock source (default)."]
    #[inline]
    pub fn internal_rc_osc(self) -> &'a mut W {
        self.variant(CLKSRCW::INTERNAL_RC_OSC)
    }
    #[doc = "Selects the main oscillator as the sysclk and PLL0 clock source."]
    #[inline]
    pub fn main_osc(self) -> &'a mut W {
        self.variant(CLKSRCW::MAIN_OSC)
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
        const OFFSET: u8 = 0;
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
    #[doc = "Bit 0 - Selects the clock source for sysclk and PLL0 as follows:"]
    #[inline]
    pub fn clksrc(&self) -> CLKSRCR {
        CLKSRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
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
    #[doc = "Bit 0 - Selects the clock source for sysclk and PLL0 as follows:"]
    #[inline]
    pub fn clksrc(&mut self) -> _CLKSRCW {
        _CLKSRCW { w: self }
    }
}
