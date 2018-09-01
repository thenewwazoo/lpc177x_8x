#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::P0_23 {
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
    #[doc = "General purpose digital input/output\n                                        pin."]
    P0_23,
    #[doc = "A/D converter 0, input 0. When configured as an                                             ADC input, the digital function of the pin must be                                             disabled."]
    ADC0_IN_0,
    #[doc = "Receive Clock. It is driven by the master and                                             received by the slave. Corresponds to the signal SCK in                                             the                                                 I2S-bus                                                 specification."]
    I2S_RX_SCK,
    #[doc = "Capture input for Timer 3, channel 0."]
    T3_CAP0,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FUNCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FUNCR::P0_23 => 0,
            FUNCR::ADC0_IN_0 => 1,
            FUNCR::I2S_RX_SCK => 2,
            FUNCR::T3_CAP0 => 3,
            FUNCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FUNCR {
        match value {
            0 => FUNCR::P0_23,
            1 => FUNCR::ADC0_IN_0,
            2 => FUNCR::I2S_RX_SCK,
            3 => FUNCR::T3_CAP0,
            i => FUNCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `P0_23`"]
    #[inline]
    pub fn is_p0_23(&self) -> bool {
        *self == FUNCR::P0_23
    }
    #[doc = "Checks if the value of the field is `ADC0_IN_0`"]
    #[inline]
    pub fn is_adc0_in_0(&self) -> bool {
        *self == FUNCR::ADC0_IN_0
    }
    #[doc = "Checks if the value of the field is `I2S_RX_SCK`"]
    #[inline]
    pub fn is_i2s_rx_sck(&self) -> bool {
        *self == FUNCR::I2S_RX_SCK
    }
    #[doc = "Checks if the value of the field is `T3_CAP0`"]
    #[inline]
    pub fn is_t3_cap0(&self) -> bool {
        *self == FUNCR::T3_CAP0
    }
}
#[doc = "Values that can be written to the field `FUNC`"]
pub enum FUNCW {
    #[doc = "General purpose digital input/output\n                                        pin."]
    P0_23,
    #[doc = "A/D converter 0, input 0. When configured as an                                             ADC input, the digital function of the pin must be                                             disabled."]
    ADC0_IN_0,
    #[doc = "Receive Clock. It is driven by the master and                                             received by the slave. Corresponds to the signal SCK in                                             the                                                 I2S-bus                                                 specification."]
    I2S_RX_SCK,
    #[doc = "Capture input for Timer 3, channel 0."]
    T3_CAP0,
}
impl FUNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FUNCW::P0_23 => 0,
            FUNCW::ADC0_IN_0 => 1,
            FUNCW::I2S_RX_SCK => 2,
            FUNCW::T3_CAP0 => 3,
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
    #[doc = "General purpose digital input/output pin."]
    #[inline]
    pub fn p0_23(self) -> &'a mut W {
        self.variant(FUNCW::P0_23)
    }
    #[doc = "A/D converter 0, input 0. When configured as an ADC input, the digital function of the pin must be disabled."]
    #[inline]
    pub fn adc0_in_0(self) -> &'a mut W {
        self.variant(FUNCW::ADC0_IN_0)
    }
    #[doc = "Receive Clock. It is driven by the master and received by the slave. Corresponds to the signal SCK in the I2S-bus specification."]
    #[inline]
    pub fn i2s_rx_sck(self) -> &'a mut W {
        self.variant(FUNCW::I2S_RX_SCK)
    }
    #[doc = "Capture input for Timer 3, channel 0."]
    #[inline]
    pub fn t3_cap0(self) -> &'a mut W {
        self.variant(FUNCW::T3_CAP0)
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
    #[doc = "Bits 0:2 - Selects pin function for pin P0[23]"]
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
        W { bits: 432 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Selects pin function for pin P0[23]"]
    #[inline]
    pub fn func(&mut self) -> _FUNCW {
        _FUNCW { w: self }
    }
}
