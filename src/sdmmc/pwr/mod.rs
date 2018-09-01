#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PWR {
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
#[doc = "Possible values of the field `CTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTRLR {
    #[doc = "Power-off"]
    POWER_OFF,
    #[doc = "Power-up"]
    POWER_UP,
    #[doc = "Power-on"]
    POWER_ON,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CTRLR::POWER_OFF => 0,
            CTRLR::POWER_UP => 2,
            CTRLR::POWER_ON => 3,
            CTRLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CTRLR {
        match value {
            0 => CTRLR::POWER_OFF,
            2 => CTRLR::POWER_UP,
            3 => CTRLR::POWER_ON,
            i => CTRLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `POWER_OFF`"]
    #[inline]
    pub fn is_power_off(&self) -> bool {
        *self == CTRLR::POWER_OFF
    }
    #[doc = "Checks if the value of the field is `POWER_UP`"]
    #[inline]
    pub fn is_power_up(&self) -> bool {
        *self == CTRLR::POWER_UP
    }
    #[doc = "Checks if the value of the field is `POWER_ON`"]
    #[inline]
    pub fn is_power_on(&self) -> bool {
        *self == CTRLR::POWER_ON
    }
}
#[doc = r" Value of the field"]
pub struct OPENDRAINR {
    bits: bool,
}
impl OPENDRAINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct RODR {
    bits: bool,
}
impl RODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Values that can be written to the field `CTRL`"]
pub enum CTRLW {
    #[doc = "Power-off"]
    POWER_OFF,
    #[doc = "Power-up"]
    POWER_UP,
    #[doc = "Power-on"]
    POWER_ON,
}
impl CTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CTRLW::POWER_OFF => 0,
            CTRLW::POWER_UP => 2,
            CTRLW::POWER_ON => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTRLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Power-off"]
    #[inline]
    pub fn power_off(self) -> &'a mut W {
        self.variant(CTRLW::POWER_OFF)
    }
    #[doc = "Power-up"]
    #[inline]
    pub fn power_up(self) -> &'a mut W {
        self.variant(CTRLW::POWER_UP)
    }
    #[doc = "Power-on"]
    #[inline]
    pub fn power_on(self) -> &'a mut W {
        self.variant(CTRLW::POWER_ON)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OPENDRAINW<'a> {
    w: &'a mut W,
}
impl<'a> _OPENDRAINW<'a> {
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
#[doc = r" Proxy"]
pub struct _RODW<'a> {
    w: &'a mut W,
}
impl<'a> _RODW<'a> {
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
        const OFFSET: u8 = 7;
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
    #[doc = "Bits 0:1 - Power control"]
    #[inline]
    pub fn ctrl(&self) -> CTRLR {
        CTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - SD_CMD output control."]
    #[inline]
    pub fn opendrain(&self) -> OPENDRAINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OPENDRAINR { bits }
    }
    #[doc = "Bit 7 - Rod control."]
    #[inline]
    pub fn rod(&self) -> RODR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RODR { bits }
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
    #[doc = "Bits 0:1 - Power control"]
    #[inline]
    pub fn ctrl(&mut self) -> _CTRLW {
        _CTRLW { w: self }
    }
    #[doc = "Bit 6 - SD_CMD output control."]
    #[inline]
    pub fn opendrain(&mut self) -> _OPENDRAINW {
        _OPENDRAINW { w: self }
    }
    #[doc = "Bit 7 - Rod control."]
    #[inline]
    pub fn rod(&mut self) -> _RODW {
        _RODW { w: self }
    }
}
