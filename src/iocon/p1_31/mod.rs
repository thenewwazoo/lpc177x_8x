#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::P1_31 {
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
    P1_31,
    #[doc = "Over-Current status for USB port 2."]
    USB_OVRCR2,
    #[doc = "Serial Clock for SSP1."]
    SSP1_SCK,
    #[doc = "A/D converter 0, input 5. When configured as an                                             ADC input, the digital function of the pin must be                                             disabled."]
    ADC0_IN_5,
    #[doc = "I2C0 clock                                             input/output (this pin does not use a specialized I2C                                             pad."]
    I2C0_SCL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FUNCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FUNCR::P1_31 => 0,
            FUNCR::USB_OVRCR2 => 1,
            FUNCR::SSP1_SCK => 2,
            FUNCR::ADC0_IN_5 => 3,
            FUNCR::I2C0_SCL => 4,
            FUNCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FUNCR {
        match value {
            0 => FUNCR::P1_31,
            1 => FUNCR::USB_OVRCR2,
            2 => FUNCR::SSP1_SCK,
            3 => FUNCR::ADC0_IN_5,
            4 => FUNCR::I2C0_SCL,
            i => FUNCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `P1_31`"]
    #[inline]
    pub fn is_p1_31(&self) -> bool {
        *self == FUNCR::P1_31
    }
    #[doc = "Checks if the value of the field is `USB_OVRCR2`"]
    #[inline]
    pub fn is_usb_ovrcr2(&self) -> bool {
        *self == FUNCR::USB_OVRCR2
    }
    #[doc = "Checks if the value of the field is `SSP1_SCK`"]
    #[inline]
    pub fn is_ssp1_sck(&self) -> bool {
        *self == FUNCR::SSP1_SCK
    }
    #[doc = "Checks if the value of the field is `ADC0_IN_5`"]
    #[inline]
    pub fn is_adc0_in_5(&self) -> bool {
        *self == FUNCR::ADC0_IN_5
    }
    #[doc = "Checks if the value of the field is `I2C0_SCL`"]
    #[inline]
    pub fn is_i2c0_scl(&self) -> bool {
        *self == FUNCR::I2C0_SCL
    }
}
#[doc = "Values that can be written to the field `FUNC`"]
pub enum FUNCW {
    #[doc = "General purpose digital input/output\n                                        pin."]
    P1_31,
    #[doc = "Over-Current status for USB port 2."]
    USB_OVRCR2,
    #[doc = "Serial Clock for SSP1."]
    SSP1_SCK,
    #[doc = "A/D converter 0, input 5. When configured as an                                             ADC input, the digital function of the pin must be                                             disabled."]
    ADC0_IN_5,
    #[doc = "I2C0 clock                                             input/output (this pin does not use a specialized I2C                                             pad."]
    I2C0_SCL,
}
impl FUNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FUNCW::P1_31 => 0,
            FUNCW::USB_OVRCR2 => 1,
            FUNCW::SSP1_SCK => 2,
            FUNCW::ADC0_IN_5 => 3,
            FUNCW::I2C0_SCL => 4,
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
    pub fn p1_31(self) -> &'a mut W {
        self.variant(FUNCW::P1_31)
    }
    #[doc = "Over-Current status for USB port 2."]
    #[inline]
    pub fn usb_ovrcr2(self) -> &'a mut W {
        self.variant(FUNCW::USB_OVRCR2)
    }
    #[doc = "Serial Clock for SSP1."]
    #[inline]
    pub fn ssp1_sck(self) -> &'a mut W {
        self.variant(FUNCW::SSP1_SCK)
    }
    #[doc = "A/D converter 0, input 5. When configured as an ADC input, the digital function of the pin must be disabled."]
    #[inline]
    pub fn adc0_in_5(self) -> &'a mut W {
        self.variant(FUNCW::ADC0_IN_5)
    }
    #[doc = "I2C0 clock input/output (this pin does not use a specialized I2C pad."]
    #[inline]
    pub fn i2c0_scl(self) -> &'a mut W {
        self.variant(FUNCW::I2C0_SCL)
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
    #[doc = "Bits 0:2 - Selects pin function for pin P1[31]"]
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
    #[doc = "Bits 0:2 - Selects pin function for pin P1[31]"]
    #[inline]
    pub fn func(&mut self) -> _FUNCW {
        _FUNCW { w: self }
    }
}
