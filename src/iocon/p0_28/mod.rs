#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::P0_28 {
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
    P0_28,
    #[doc = "I2C0 clock                                             input/output (this pin uses a specialized I2C                                             pad."]
    I2C0_SCL,
    #[doc = "I2C serial clock for communication with an                                             external USB transceiver."]
    USB_SCL1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FUNCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FUNCR::P0_28 => 0,
            FUNCR::I2C0_SCL => 1,
            FUNCR::USB_SCL1 => 2,
            FUNCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FUNCR {
        match value {
            0 => FUNCR::P0_28,
            1 => FUNCR::I2C0_SCL,
            2 => FUNCR::USB_SCL1,
            i => FUNCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `P0_28`"]
    #[inline]
    pub fn is_p0_28(&self) -> bool {
        *self == FUNCR::P0_28
    }
    #[doc = "Checks if the value of the field is `I2C0_SCL`"]
    #[inline]
    pub fn is_i2c0_scl(&self) -> bool {
        *self == FUNCR::I2C0_SCL
    }
    #[doc = "Checks if the value of the field is `USB_SCL1`"]
    #[inline]
    pub fn is_usb_scl1(&self) -> bool {
        *self == FUNCR::USB_SCL1
    }
}
#[doc = "Possible values of the field `INV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVR {
    #[doc = "Input not inverted (HIGH on pin reads as 1, LOW on pin\n                                        reads as 0)."]
    INPUT_NOT_INVERTED,
    #[doc = "Input inverted (HIGH on pin reads as 0, LOW on pin reads as\n                                        1)."]
    INPUT_INVERTED_HIGH,
}
impl INVR {
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
            INVR::INPUT_NOT_INVERTED => false,
            INVR::INPUT_INVERTED_HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INVR {
        match value {
            false => INVR::INPUT_NOT_INVERTED,
            true => INVR::INPUT_INVERTED_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT_NOT_INVERTED`"]
    #[inline]
    pub fn is_input_not_inverted(&self) -> bool {
        *self == INVR::INPUT_NOT_INVERTED
    }
    #[doc = "Checks if the value of the field is `INPUT_INVERTED_HIGH`"]
    #[inline]
    pub fn is_input_inverted_high(&self) -> bool {
        *self == INVR::INPUT_INVERTED_HIGH
    }
}
#[doc = "Possible values of the field `HS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSR {
    #[doc = "I2C 50ns glitch filter and slew rate control\n                                        enabled."]
    ENABLED,
    #[doc = "I2C 50ns glitch filter and slew rate control\n                                        disabled."]
    DISABLED,
}
impl HSR {
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
            HSR::ENABLED => false,
            HSR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HSR {
        match value {
            false => HSR::ENABLED,
            true => HSR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == HSR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == HSR::DISABLED
    }
}
#[doc = "Possible values of the field `HIDRIVE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIDRIVER {
    #[doc = "Output drive sink is 4 mA. This is sufficient for standard\n                                        and fast mode I2C."]
    LOWDRIVE,
    #[doc = "Output drive sink is 20 mA. This is needed for Fast Mode\n                                        Plus I2C. Refer to the appropriate specific device data sheet for\n                                        details."]
    HIGHDRIVE,
}
impl HIDRIVER {
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
            HIDRIVER::LOWDRIVE => false,
            HIDRIVER::HIGHDRIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HIDRIVER {
        match value {
            false => HIDRIVER::LOWDRIVE,
            true => HIDRIVER::HIGHDRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOWDRIVE`"]
    #[inline]
    pub fn is_lowdrive(&self) -> bool {
        *self == HIDRIVER::LOWDRIVE
    }
    #[doc = "Checks if the value of the field is `HIGHDRIVE`"]
    #[inline]
    pub fn is_highdrive(&self) -> bool {
        *self == HIDRIVER::HIGHDRIVE
    }
}
#[doc = "Values that can be written to the field `FUNC`"]
pub enum FUNCW {
    #[doc = "General purpose digital input/output\n                                        pin."]
    P0_28,
    #[doc = "I2C0 clock                                             input/output (this pin uses a specialized I2C                                             pad."]
    I2C0_SCL,
    #[doc = "I2C serial clock for communication with an                                             external USB transceiver."]
    USB_SCL1,
}
impl FUNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FUNCW::P0_28 => 0,
            FUNCW::I2C0_SCL => 1,
            FUNCW::USB_SCL1 => 2,
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
    pub fn p0_28(self) -> &'a mut W {
        self.variant(FUNCW::P0_28)
    }
    #[doc = "I2C0 clock input/output (this pin uses a specialized I2C pad."]
    #[inline]
    pub fn i2c0_scl(self) -> &'a mut W {
        self.variant(FUNCW::I2C0_SCL)
    }
    #[doc = "I2C serial clock for communication with an external USB transceiver."]
    #[inline]
    pub fn usb_scl1(self) -> &'a mut W {
        self.variant(FUNCW::USB_SCL1)
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
#[doc = "Values that can be written to the field `INV`"]
pub enum INVW {
    #[doc = "Input not inverted (HIGH on pin reads as 1, LOW on pin\n                                        reads as 0)."]
    INPUT_NOT_INVERTED,
    #[doc = "Input inverted (HIGH on pin reads as 0, LOW on pin reads as\n                                        1)."]
    INPUT_INVERTED_HIGH,
}
impl INVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INVW::INPUT_NOT_INVERTED => false,
            INVW::INPUT_INVERTED_HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INVW<'a> {
    w: &'a mut W,
}
impl<'a> _INVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input not inverted (HIGH on pin reads as 1, LOW on pin reads as 0)."]
    #[inline]
    pub fn input_not_inverted(self) -> &'a mut W {
        self.variant(INVW::INPUT_NOT_INVERTED)
    }
    #[doc = "Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    #[inline]
    pub fn input_inverted_high(self) -> &'a mut W {
        self.variant(INVW::INPUT_INVERTED_HIGH)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HS`"]
pub enum HSW {
    #[doc = "I2C 50ns glitch filter and slew rate control\n                                        enabled."]
    ENABLED,
    #[doc = "I2C 50ns glitch filter and slew rate control\n                                        disabled."]
    DISABLED,
}
impl HSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HSW::ENABLED => false,
            HSW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HSW<'a> {
    w: &'a mut W,
}
impl<'a> _HSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "I2C 50ns glitch filter and slew rate control enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HSW::ENABLED)
    }
    #[doc = "I2C 50ns glitch filter and slew rate control disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HSW::DISABLED)
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
#[doc = "Values that can be written to the field `HIDRIVE`"]
pub enum HIDRIVEW {
    #[doc = "Output drive sink is 4 mA. This is sufficient for standard\n                                        and fast mode I2C."]
    LOWDRIVE,
    #[doc = "Output drive sink is 20 mA. This is needed for Fast Mode\n                                        Plus I2C. Refer to the appropriate specific device data sheet for\n                                        details."]
    HIGHDRIVE,
}
impl HIDRIVEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HIDRIVEW::LOWDRIVE => false,
            HIDRIVEW::HIGHDRIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HIDRIVEW<'a> {
    w: &'a mut W,
}
impl<'a> _HIDRIVEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HIDRIVEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Output drive sink is 4 mA. This is sufficient for standard and fast mode I2C."]
    #[inline]
    pub fn lowdrive(self) -> &'a mut W {
        self.variant(HIDRIVEW::LOWDRIVE)
    }
    #[doc = "Output drive sink is 20 mA. This is needed for Fast Mode Plus I2C. Refer to the appropriate specific device data sheet for details."]
    #[inline]
    pub fn highdrive(self) -> &'a mut W {
        self.variant(HIDRIVEW::HIGHDRIVE)
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
        const OFFSET: u8 = 9;
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
    #[doc = "Bits 0:2 - Selects pin function for pin P0[28]"]
    #[inline]
    pub fn func(&self) -> FUNCR {
        FUNCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - Invert input"]
    #[inline]
    pub fn inv(&self) -> INVR {
        INVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation."]
    #[inline]
    pub fn hs(&self) -> HSR {
        HSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Controls sink current capability of the pin, only for P5[2] and P5[3]."]
    #[inline]
    pub fn hidrive(&self) -> HIDRIVER {
        HIDRIVER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 128 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Selects pin function for pin P0[28]"]
    #[inline]
    pub fn func(&mut self) -> _FUNCW {
        _FUNCW { w: self }
    }
    #[doc = "Bit 6 - Invert input"]
    #[inline]
    pub fn inv(&mut self) -> _INVW {
        _INVW { w: self }
    }
    #[doc = "Bit 8 - Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation."]
    #[inline]
    pub fn hs(&mut self) -> _HSW {
        _HSW { w: self }
    }
    #[doc = "Bit 9 - Controls sink current capability of the pin, only for P5[2] and P5[3]."]
    #[inline]
    pub fn hidrive(&mut self) -> _HIDRIVEW {
        _HIDRIVEW { w: self }
    }
}
