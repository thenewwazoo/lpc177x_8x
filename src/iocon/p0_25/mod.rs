#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::P0_25 {
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
    P0_25,
    #[doc = "A/D converter 0, input 2. When configured as an                                             ADC input, the digital function of the pin must be                                             disabled."]
    ADC0_IN_2,
    #[doc = "Receive data. It is driven by the transmitter and                                             read by the receiver. Corresponds to the signal SD in                                             the                                                 I2S-bus                                                 specification."]
    I2S_RX_SDA,
    #[doc = "Transmitter output for UART3."]
    U3_TXD,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FUNCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FUNCR::P0_25 => 0,
            FUNCR::ADC0_IN_2 => 1,
            FUNCR::I2S_RX_SDA => 2,
            FUNCR::U3_TXD => 3,
            FUNCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FUNCR {
        match value {
            0 => FUNCR::P0_25,
            1 => FUNCR::ADC0_IN_2,
            2 => FUNCR::I2S_RX_SDA,
            3 => FUNCR::U3_TXD,
            i => FUNCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `P0_25`"]
    #[inline]
    pub fn is_p0_25(&self) -> bool {
        *self == FUNCR::P0_25
    }
    #[doc = "Checks if the value of the field is `ADC0_IN_2`"]
    #[inline]
    pub fn is_adc0_in_2(&self) -> bool {
        *self == FUNCR::ADC0_IN_2
    }
    #[doc = "Checks if the value of the field is `I2S_RX_SDA`"]
    #[inline]
    pub fn is_i2s_rx_sda(&self) -> bool {
        *self == FUNCR::I2S_RX_SDA
    }
    #[doc = "Checks if the value of the field is `U3_TXD`"]
    #[inline]
    pub fn is_u3_txd(&self) -> bool {
        *self == FUNCR::U3_TXD
    }
}
#[doc = "Values that can be written to the field `FUNC`"]
pub enum FUNCW {
    #[doc = "General purpose digital input/output\n                                        pin."]
    P0_25,
    #[doc = "A/D converter 0, input 2. When configured as an                                             ADC input, the digital function of the pin must be                                             disabled."]
    ADC0_IN_2,
    #[doc = "Receive data. It is driven by the transmitter and                                             read by the receiver. Corresponds to the signal SD in                                             the                                                 I2S-bus                                                 specification."]
    I2S_RX_SDA,
    #[doc = "Transmitter output for UART3."]
    U3_TXD,
}
impl FUNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FUNCW::P0_25 => 0,
            FUNCW::ADC0_IN_2 => 1,
            FUNCW::I2S_RX_SDA => 2,
            FUNCW::U3_TXD => 3,
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
    pub fn p0_25(self) -> &'a mut W {
        self.variant(FUNCW::P0_25)
    }
    #[doc = "A/D converter 0, input 2. When configured as an ADC input, the digital function of the pin must be disabled."]
    #[inline]
    pub fn adc0_in_2(self) -> &'a mut W {
        self.variant(FUNCW::ADC0_IN_2)
    }
    #[doc = "Receive data. It is driven by the transmitter and read by the receiver. Corresponds to the signal SD in the I2S-bus specification."]
    #[inline]
    pub fn i2s_rx_sda(self) -> &'a mut W {
        self.variant(FUNCW::I2S_RX_SDA)
    }
    #[doc = "Transmitter output for UART3."]
    #[inline]
    pub fn u3_txd(self) -> &'a mut W {
        self.variant(FUNCW::U3_TXD)
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
    #[doc = "Bits 0:2 - Selects pin function for pin P0[25]"]
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
    #[doc = "Bits 0:2 - Selects pin function for pin P0[25]"]
    #[inline]
    pub fn func(&mut self) -> _FUNCW {
        _FUNCW { w: self }
    }
}
