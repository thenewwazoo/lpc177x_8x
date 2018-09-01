#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::P2_10 {
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
#[doc = "Possible values of the field `FUNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FUNCR {
    #[doc = "General purpose digital input/output pin. This\n                                        pin includes a 5 ns input glitch filter.A LOW on this pin while RESET is LOW forces the\n                                        on-chip boot loader to take over control of the part\n                                        after a reset and go into ISP mode. "]
    P2_10,
    #[doc = "External interrupt 0 input."]
    EINT0,
    #[doc = "Non-maskable interrupt input."]
    NMI,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FUNCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FUNCR::P2_10 => 0,
            FUNCR::EINT0 => 1,
            FUNCR::NMI => 2,
            FUNCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FUNCR {
        match value {
            0 => FUNCR::P2_10,
            1 => FUNCR::EINT0,
            2 => FUNCR::NMI,
            i => FUNCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `P2_10`"]
    #[inline]
    pub fn is_p2_10(&self) -> bool {
        *self == FUNCR::P2_10
    }
    #[doc = "Checks if the value of the field is `EINT0`"]
    #[inline]
    pub fn is_eint0(&self) -> bool {
        *self == FUNCR::EINT0
    }
    #[doc = "Checks if the value of the field is `NMI`"]
    #[inline]
    pub fn is_nmi(&self) -> bool {
        *self == FUNCR::NMI
    }
}
#[doc = "Values that can be written to the field `FUNC`"]
pub enum FUNCW {
    #[doc = "General purpose digital input/output pin. This\n                                        pin includes a 5 ns input glitch filter.A LOW on this pin while RESET is LOW forces the\n                                        on-chip boot loader to take over control of the part\n                                        after a reset and go into ISP mode. "]
    P2_10,
    #[doc = "External interrupt 0 input."]
    EINT0,
    #[doc = "Non-maskable interrupt input."]
    NMI,
}
impl FUNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FUNCW::P2_10 => 0,
            FUNCW::EINT0 => 1,
            FUNCW::NMI => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FUNCW<'a> {
    w: &'a mut W,
}
impl<'a> _FUNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FUNCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "General purpose digital input/output pin. This pin includes a 5 ns input glitch filter.A LOW on this pin while RESET is LOW forces the on-chip boot loader to take over control of the part after a reset and go into ISP mode."]
    #[inline]
    pub fn p2_10(self) -> &'a mut W {
        self.variant(FUNCW::P2_10)
    }
    #[doc = "External interrupt 0 input."]
    #[inline]
    pub fn eint0(self) -> &'a mut W {
        self.variant(FUNCW::EINT0)
    }
    #[doc = "Non-maskable interrupt input."]
    #[inline]
    pub fn nmi(self) -> &'a mut W {
        self.variant(FUNCW::NMI)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bits 0:2 - Selects pin function for pin P2[10]"]
    #[inline]
    pub fn func(&self) -> FUNCR {
        FUNCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 48 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Selects pin function for pin P2[10]"]
    #[inline]
    pub fn func(&mut self) -> _FUNCW {
        _FUNCW { w: self }
    }
}
