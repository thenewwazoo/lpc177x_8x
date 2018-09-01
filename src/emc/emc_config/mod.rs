#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EMC_CONFIG {
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
#[doc = "Possible values of the field `EM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMR {
    #[doc = "Little-endian\n                                        mode (POR reset value)."]
    LITTLEENDIAN,
    #[doc = "Big-endian\n                                        mode."]
    BIGENDIAN,
}
impl EMR {
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
            EMR::LITTLEENDIAN => false,
            EMR::BIGENDIAN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EMR {
        match value {
            false => EMR::LITTLEENDIAN,
            true => EMR::BIGENDIAN,
        }
    }
    #[doc = "Checks if the value of the field is `LITTLEENDIAN`"]
    #[inline]
    pub fn is_littleendian(&self) -> bool {
        *self == EMR::LITTLEENDIAN
    }
    #[doc = "Checks if the value of the field is `BIGENDIAN`"]
    #[inline]
    pub fn is_bigendian(&self) -> bool {
        *self == EMR::BIGENDIAN
    }
}
#[doc = "Possible values of the field `CLKR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKRR {
    #[doc = "1:1(POR reset value)"]
    PORRESET,
    #[doc = "1:2 (this option is not available on the LPC178x/177x)"]
    DONOTUSE,
}
impl CLKRR {
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
            CLKRR::PORRESET => false,
            CLKRR::DONOTUSE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLKRR {
        match value {
            false => CLKRR::PORRESET,
            true => CLKRR::DONOTUSE,
        }
    }
    #[doc = "Checks if the value of the field is `PORRESET`"]
    #[inline]
    pub fn is_porreset(&self) -> bool {
        *self == CLKRR::PORRESET
    }
    #[doc = "Checks if the value of the field is `DONOTUSE`"]
    #[inline]
    pub fn is_donotuse(&self) -> bool {
        *self == CLKRR::DONOTUSE
    }
}
#[doc = "Values that can be written to the field `EM`"]
pub enum EMW {
    #[doc = "Little-endian\n                                        mode (POR reset value)."]
    LITTLEENDIAN,
    #[doc = "Big-endian\n                                        mode."]
    BIGENDIAN,
}
impl EMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EMW::LITTLEENDIAN => false,
            EMW::BIGENDIAN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EMW<'a> {
    w: &'a mut W,
}
impl<'a> _EMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Little-endian mode (POR reset value)."]
    #[inline]
    pub fn littleendian(self) -> &'a mut W {
        self.variant(EMW::LITTLEENDIAN)
    }
    #[doc = "Big-endian mode."]
    #[inline]
    pub fn bigendian(self) -> &'a mut W {
        self.variant(EMW::BIGENDIAN)
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
#[doc = "Values that can be written to the field `CLKR`"]
pub enum CLKRW {
    #[doc = "1:1(POR reset value)"]
    PORRESET,
    #[doc = "1:2 (this option is not available on the LPC178x/177x)"]
    DONOTUSE,
}
impl CLKRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLKRW::PORRESET => false,
            CLKRW::DONOTUSE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKRW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "1:1(POR reset value)"]
    #[inline]
    pub fn porreset(self) -> &'a mut W {
        self.variant(CLKRW::PORRESET)
    }
    #[doc = "1:2 (this option is not available on the LPC178x/177x)"]
    #[inline]
    pub fn donotuse(self) -> &'a mut W {
        self.variant(CLKRW::DONOTUSE)
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
    #[doc = "Bit 0 - Endian mode. On power-on reset, the value of the endian bit is 0. All data must be flushed in the EMC before switching between little-endian and big-endian modes."]
    #[inline]
    pub fn em(&self) -> EMR {
        EMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - CCLK: CLKOUT ratio. This bit must contain 0 for proper operation of the EMC."]
    #[inline]
    pub fn clkr(&self) -> CLKRR {
        CLKRR::_from({
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
    #[doc = "Bit 0 - Endian mode. On power-on reset, the value of the endian bit is 0. All data must be flushed in the EMC before switching between little-endian and big-endian modes."]
    #[inline]
    pub fn em(&mut self) -> _EMW {
        _EMW { w: self }
    }
    #[doc = "Bit 8 - CCLK: CLKOUT ratio. This bit must contain 0 for proper operation of the EMC."]
    #[inline]
    pub fn clkr(&mut self) -> _CLKRW {
        _CLKRW { w: self }
    }
}
