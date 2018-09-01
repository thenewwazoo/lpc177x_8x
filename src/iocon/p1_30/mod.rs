#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::P1_30 {
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
    P1_30,
    #[doc = "Power Status for USB port 2."]
    USB_PWRD2,
    #[doc = "Monitors the presence of USB bus                                             power.This signal must be HIGH for USB reset to                                             occur."]
    USB_VBUS,
    #[doc = "A/D converter 0, input 4. When configured as an                                             ADC input, the digital function of the pin must be                                             disabled."]
    ADC0_IN_4,
    #[doc = "I2C0 data input/output                                             (this pin does not use a specialized I2C                                             pad."]
    I2C0_SDA,
    #[doc = "RS-485/EIA-485 output enable signal for                                             UART3."]
    U3_OE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FUNCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FUNCR::P1_30 => 0,
            FUNCR::USB_PWRD2 => 1,
            FUNCR::USB_VBUS => 2,
            FUNCR::ADC0_IN_4 => 3,
            FUNCR::I2C0_SDA => 4,
            FUNCR::U3_OE => 5,
            FUNCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FUNCR {
        match value {
            0 => FUNCR::P1_30,
            1 => FUNCR::USB_PWRD2,
            2 => FUNCR::USB_VBUS,
            3 => FUNCR::ADC0_IN_4,
            4 => FUNCR::I2C0_SDA,
            5 => FUNCR::U3_OE,
            i => FUNCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `P1_30`"]
    #[inline]
    pub fn is_p1_30(&self) -> bool {
        *self == FUNCR::P1_30
    }
    #[doc = "Checks if the value of the field is `USB_PWRD2`"]
    #[inline]
    pub fn is_usb_pwrd2(&self) -> bool {
        *self == FUNCR::USB_PWRD2
    }
    #[doc = "Checks if the value of the field is `USB_VBUS`"]
    #[inline]
    pub fn is_usb_vbus(&self) -> bool {
        *self == FUNCR::USB_VBUS
    }
    #[doc = "Checks if the value of the field is `ADC0_IN_4`"]
    #[inline]
    pub fn is_adc0_in_4(&self) -> bool {
        *self == FUNCR::ADC0_IN_4
    }
    #[doc = "Checks if the value of the field is `I2C0_SDA`"]
    #[inline]
    pub fn is_i2c0_sda(&self) -> bool {
        *self == FUNCR::I2C0_SDA
    }
    #[doc = "Checks if the value of the field is `U3_OE`"]
    #[inline]
    pub fn is_u3_oe(&self) -> bool {
        *self == FUNCR::U3_OE
    }
}
#[doc = "Values that can be written to the field `FUNC`"]
pub enum FUNCW {
    #[doc = "General purpose digital input/output\n                                        pin."]
    P1_30,
    #[doc = "Power Status for USB port 2."]
    USB_PWRD2,
    #[doc = "Monitors the presence of USB bus                                             power.This signal must be HIGH for USB reset to                                             occur."]
    USB_VBUS,
    #[doc = "A/D converter 0, input 4. When configured as an                                             ADC input, the digital function of the pin must be                                             disabled."]
    ADC0_IN_4,
    #[doc = "I2C0 data input/output                                             (this pin does not use a specialized I2C                                             pad."]
    I2C0_SDA,
    #[doc = "RS-485/EIA-485 output enable signal for                                             UART3."]
    U3_OE,
}
impl FUNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FUNCW::P1_30 => 0,
            FUNCW::USB_PWRD2 => 1,
            FUNCW::USB_VBUS => 2,
            FUNCW::ADC0_IN_4 => 3,
            FUNCW::I2C0_SDA => 4,
            FUNCW::U3_OE => 5,
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
    pub fn p1_30(self) -> &'a mut W {
        self.variant(FUNCW::P1_30)
    }
    #[doc = "Power Status for USB port 2."]
    #[inline]
    pub fn usb_pwrd2(self) -> &'a mut W {
        self.variant(FUNCW::USB_PWRD2)
    }
    #[doc = "Monitors the presence of USB bus power.This signal must be HIGH for USB reset to occur."]
    #[inline]
    pub fn usb_vbus(self) -> &'a mut W {
        self.variant(FUNCW::USB_VBUS)
    }
    #[doc = "A/D converter 0, input 4. When configured as an ADC input, the digital function of the pin must be disabled."]
    #[inline]
    pub fn adc0_in_4(self) -> &'a mut W {
        self.variant(FUNCW::ADC0_IN_4)
    }
    #[doc = "I2C0 data input/output (this pin does not use a specialized I2C pad."]
    #[inline]
    pub fn i2c0_sda(self) -> &'a mut W {
        self.variant(FUNCW::I2C0_SDA)
    }
    #[doc = "RS-485/EIA-485 output enable signal for UART3."]
    #[inline]
    pub fn u3_oe(self) -> &'a mut W {
        self.variant(FUNCW::U3_OE)
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
    #[doc = "Bits 0:2 - Selects pin function for pin P1[30]"]
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
    #[doc = "Bits 0:2 - Selects pin function for pin P1[30]"]
    #[inline]
    pub fn func(&mut self) -> _FUNCW {
        _FUNCW { w: self }
    }
}
