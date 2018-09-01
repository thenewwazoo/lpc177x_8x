#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::P0_12 {
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
    P0_12,
    #[doc = "Port Power enable signal for USB port                                             2."]
    USB_PPWR2,
    #[doc = "Master In Slave Out for SSP1."]
    SSP1_MISO,
    #[doc = "A/D converter 0, input 6. When configured as an                                             ADC input, the digital function of the pin must be                                             disabled."]
    ADC0_IN_6,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FUNCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FUNCR::P0_12 => 0,
            FUNCR::USB_PPWR2 => 1,
            FUNCR::SSP1_MISO => 2,
            FUNCR::ADC0_IN_6 => 3,
            FUNCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FUNCR {
        match value {
            0 => FUNCR::P0_12,
            1 => FUNCR::USB_PPWR2,
            2 => FUNCR::SSP1_MISO,
            3 => FUNCR::ADC0_IN_6,
            i => FUNCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `P0_12`"]
    #[inline]
    pub fn is_p0_12(&self) -> bool {
        *self == FUNCR::P0_12
    }
    #[doc = "Checks if the value of the field is `USB_PPWR2`"]
    #[inline]
    pub fn is_usb_ppwr2(&self) -> bool {
        *self == FUNCR::USB_PPWR2
    }
    #[doc = "Checks if the value of the field is `SSP1_MISO`"]
    #[inline]
    pub fn is_ssp1_miso(&self) -> bool {
        *self == FUNCR::SSP1_MISO
    }
    #[doc = "Checks if the value of the field is `ADC0_IN_6`"]
    #[inline]
    pub fn is_adc0_in_6(&self) -> bool {
        *self == FUNCR::ADC0_IN_6
    }
}
#[doc = "Values that can be written to the field `FUNC`"]
pub enum FUNCW {
    #[doc = "General purpose digital input/output\n                                        pin."]
    P0_12,
    #[doc = "Port Power enable signal for USB port                                             2."]
    USB_PPWR2,
    #[doc = "Master In Slave Out for SSP1."]
    SSP1_MISO,
    #[doc = "A/D converter 0, input 6. When configured as an                                             ADC input, the digital function of the pin must be                                             disabled."]
    ADC0_IN_6,
}
impl FUNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FUNCW::P0_12 => 0,
            FUNCW::USB_PPWR2 => 1,
            FUNCW::SSP1_MISO => 2,
            FUNCW::ADC0_IN_6 => 3,
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
    pub fn p0_12(self) -> &'a mut W {
        self.variant(FUNCW::P0_12)
    }
    #[doc = "Port Power enable signal for USB port 2."]
    #[inline]
    pub fn usb_ppwr2(self) -> &'a mut W {
        self.variant(FUNCW::USB_PPWR2)
    }
    #[doc = "Master In Slave Out for SSP1."]
    #[inline]
    pub fn ssp1_miso(self) -> &'a mut W {
        self.variant(FUNCW::SSP1_MISO)
    }
    #[doc = "A/D converter 0, input 6. When configured as an ADC input, the digital function of the pin must be disabled."]
    #[inline]
    pub fn adc0_in_6(self) -> &'a mut W {
        self.variant(FUNCW::ADC0_IN_6)
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
    #[doc = "Bits 0:2 - Selects pin function for pin P0[12]"]
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
    #[doc = "Bits 0:2 - Selects pin function for pin P0[12]"]
    #[inline]
    pub fn func(&mut self) -> _FUNCW {
        _FUNCW { w: self }
    }
}