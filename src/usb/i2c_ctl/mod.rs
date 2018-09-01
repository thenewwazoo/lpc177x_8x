#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::I2C_CTL {
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
#[doc = "Possible values of the field `TDIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDIER {
    #[doc = "Disable the TDI interrupt."]
    DISABLE,
    #[doc = "Enable the TDI interrupt."]
    ENABLE,
}
impl TDIER {
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
            TDIER::DISABLE => false,
            TDIER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TDIER {
        match value {
            false => TDIER::DISABLE,
            true => TDIER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == TDIER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == TDIER::ENABLE
    }
}
#[doc = "Possible values of the field `AFIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFIER {
    #[doc = "Disable the AFI."]
    DISABLE,
    #[doc = "Enable the AFI."]
    ENABLE,
}
impl AFIER {
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
            AFIER::DISABLE => false,
            AFIER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AFIER {
        match value {
            false => AFIER::DISABLE,
            true => AFIER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == AFIER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == AFIER::ENABLE
    }
}
#[doc = "Possible values of the field `NAIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NAIER {
    #[doc = "Disable the NAI."]
    DISABLE,
    #[doc = "Enable the NAI."]
    ENABLE,
}
impl NAIER {
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
            NAIER::DISABLE => false,
            NAIER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NAIER {
        match value {
            false => NAIER::DISABLE,
            true => NAIER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == NAIER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == NAIER::ENABLE
    }
}
#[doc = "Possible values of the field `DRMIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRMIER {
    #[doc = "Disable the DRMI interrupt."]
    DISABLE,
    #[doc = "Enable the DRMI interrupt."]
    ENABLE,
}
impl DRMIER {
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
            DRMIER::DISABLE => false,
            DRMIER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DRMIER {
        match value {
            false => DRMIER::DISABLE,
            true => DRMIER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == DRMIER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == DRMIER::ENABLE
    }
}
#[doc = "Possible values of the field `DRSIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRSIER {
    #[doc = "Disable the DRSI interrupt."]
    DISABLE,
    #[doc = "Enable the DRSI interrupt."]
    ENABLE,
}
impl DRSIER {
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
            DRSIER::DISABLE => false,
            DRSIER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DRSIER {
        match value {
            false => DRSIER::DISABLE,
            true => DRSIER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == DRSIER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == DRSIER::ENABLE
    }
}
#[doc = "Possible values of the field `REFIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFIER {
    #[doc = "Disable the RFFI."]
    DISABLE,
    #[doc = "Enable the RFFI."]
    ENABLE,
}
impl REFIER {
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
            REFIER::DISABLE => false,
            REFIER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REFIER {
        match value {
            false => REFIER::DISABLE,
            true => REFIER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == REFIER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == REFIER::ENABLE
    }
}
#[doc = "Possible values of the field `RFDAIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFDAIER {
    #[doc = "Disable the DAI."]
    DISABLE,
    #[doc = "Enable the DAI."]
    ENABLE,
}
impl RFDAIER {
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
            RFDAIER::DISABLE => false,
            RFDAIER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RFDAIER {
        match value {
            false => RFDAIER::DISABLE,
            true => RFDAIER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RFDAIER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RFDAIER::ENABLE
    }
}
#[doc = "Possible values of the field `TFFIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFFIER {
    #[doc = "Disable the TFFI."]
    DISABLE,
    #[doc = "Enable the TFFI."]
    ENABLE,
}
impl TFFIER {
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
            TFFIER::DISABLE => false,
            TFFIER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TFFIER {
        match value {
            false => TFFIER::DISABLE,
            true => TFFIER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == TFFIER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == TFFIER::ENABLE
    }
}
#[doc = "Possible values of the field `SRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRSTR {
    #[doc = "No reset."]
    NO_RESET,
    #[doc = "Reset the I2C to idle state. Self clearing."]
    RESET,
}
impl SRSTR {
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
            SRSTR::NO_RESET => false,
            SRSTR::RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRSTR {
        match value {
            false => SRSTR::NO_RESET,
            true => SRSTR::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NO_RESET`"]
    #[inline]
    pub fn is_no_reset(&self) -> bool {
        *self == SRSTR::NO_RESET
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == SRSTR::RESET
    }
}
#[doc = "Values that can be written to the field `TDIE`"]
pub enum TDIEW {
    #[doc = "Disable the TDI interrupt."]
    DISABLE,
    #[doc = "Enable the TDI interrupt."]
    ENABLE,
}
impl TDIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TDIEW::DISABLE => false,
            TDIEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TDIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TDIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TDIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the TDI interrupt."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(TDIEW::DISABLE)
    }
    #[doc = "Enable the TDI interrupt."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(TDIEW::ENABLE)
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
#[doc = "Values that can be written to the field `AFIE`"]
pub enum AFIEW {
    #[doc = "Disable the AFI."]
    DISABLE,
    #[doc = "Enable the AFI."]
    ENABLE,
}
impl AFIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AFIEW::DISABLE => false,
            AFIEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AFIEW<'a> {
    w: &'a mut W,
}
impl<'a> _AFIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the AFI."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(AFIEW::DISABLE)
    }
    #[doc = "Enable the AFI."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(AFIEW::ENABLE)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NAIE`"]
pub enum NAIEW {
    #[doc = "Disable the NAI."]
    DISABLE,
    #[doc = "Enable the NAI."]
    ENABLE,
}
impl NAIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NAIEW::DISABLE => false,
            NAIEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NAIEW<'a> {
    w: &'a mut W,
}
impl<'a> _NAIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NAIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the NAI."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(NAIEW::DISABLE)
    }
    #[doc = "Enable the NAI."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(NAIEW::ENABLE)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DRMIE`"]
pub enum DRMIEW {
    #[doc = "Disable the DRMI interrupt."]
    DISABLE,
    #[doc = "Enable the DRMI interrupt."]
    ENABLE,
}
impl DRMIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DRMIEW::DISABLE => false,
            DRMIEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DRMIEW<'a> {
    w: &'a mut W,
}
impl<'a> _DRMIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DRMIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the DRMI interrupt."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(DRMIEW::DISABLE)
    }
    #[doc = "Enable the DRMI interrupt."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(DRMIEW::ENABLE)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DRSIE`"]
pub enum DRSIEW {
    #[doc = "Disable the DRSI interrupt."]
    DISABLE,
    #[doc = "Enable the DRSI interrupt."]
    ENABLE,
}
impl DRSIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DRSIEW::DISABLE => false,
            DRSIEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DRSIEW<'a> {
    w: &'a mut W,
}
impl<'a> _DRSIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DRSIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the DRSI interrupt."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(DRSIEW::DISABLE)
    }
    #[doc = "Enable the DRSI interrupt."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(DRSIEW::ENABLE)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REFIE`"]
pub enum REFIEW {
    #[doc = "Disable the RFFI."]
    DISABLE,
    #[doc = "Enable the RFFI."]
    ENABLE,
}
impl REFIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REFIEW::DISABLE => false,
            REFIEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REFIEW<'a> {
    w: &'a mut W,
}
impl<'a> _REFIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REFIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the RFFI."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(REFIEW::DISABLE)
    }
    #[doc = "Enable the RFFI."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(REFIEW::ENABLE)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RFDAIE`"]
pub enum RFDAIEW {
    #[doc = "Disable the DAI."]
    DISABLE,
    #[doc = "Enable the DAI."]
    ENABLE,
}
impl RFDAIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RFDAIEW::DISABLE => false,
            RFDAIEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RFDAIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RFDAIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RFDAIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the DAI."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RFDAIEW::DISABLE)
    }
    #[doc = "Enable the DAI."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RFDAIEW::ENABLE)
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
#[doc = "Values that can be written to the field `TFFIE`"]
pub enum TFFIEW {
    #[doc = "Disable the TFFI."]
    DISABLE,
    #[doc = "Enable the TFFI."]
    ENABLE,
}
impl TFFIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TFFIEW::DISABLE => false,
            TFFIEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TFFIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TFFIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TFFIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the TFFI."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(TFFIEW::DISABLE)
    }
    #[doc = "Enable the TFFI."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(TFFIEW::ENABLE)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRST`"]
pub enum SRSTW {
    #[doc = "No reset."]
    NO_RESET,
    #[doc = "Reset the I2C to idle state. Self clearing."]
    RESET,
}
impl SRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRSTW::NO_RESET => false,
            SRSTW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No reset."]
    #[inline]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(SRSTW::NO_RESET)
    }
    #[doc = "Reset the I2C to idle state. Self clearing."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(SRSTW::RESET)
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
    #[doc = "Bit 0 - Transmit Done Interrupt Enable. This enables the TDI interrupt signalling that this I2C issued a STOP condition."]
    #[inline]
    pub fn tdie(&self) -> TDIER {
        TDIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Transmitter Arbitration Failure Interrupt Enable. This enables the AFI interrupt which is asserted during transmission when trying to set SDA high, but the bus is driven low by another device."]
    #[inline]
    pub fn afie(&self) -> AFIER {
        AFIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Transmitter No Acknowledge Interrupt Enable. This enables the NAI interrupt signalling that transmitted byte was not acknowledged."]
    #[inline]
    pub fn naie(&self) -> NAIER {
        NAIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Master Transmitter Data Request Interrupt Enable. This enables the DRMI interrupt which signals that the master transmitter has run out of data, has not issued a STOP, and is holding the SCL line low."]
    #[inline]
    pub fn drmie(&self) -> DRMIER {
        DRMIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Slave Transmitter Data Request Interrupt Enable. This enables the DRSI interrupt which signals that the slave transmitter has run out of data and the last byte was acknowledged, so the SCL line is being held low."]
    #[inline]
    pub fn drsie(&self) -> DRSIER {
        DRSIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Receive FIFO Full Interrupt Enable. This enables the Receive FIFO Full interrupt to indicate that the receive FIFO cannot accept any more data."]
    #[inline]
    pub fn refie(&self) -> REFIER {
        REFIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Receive Data Available Interrupt Enable. This enables the DAI interrupt to indicate that data is available in the receive FIFO (i.e. not empty)."]
    #[inline]
    pub fn rfdaie(&self) -> RFDAIER {
        RFDAIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Transmit FIFO Not Full Interrupt Enable. This enables the Transmit FIFO Not Full interrupt to indicate that the more data can be written to the transmit FIFO. Note that this is not full. It is intended help the CPU to write to the I2C block only when there is room in the FIFO and do this without polling the status register."]
    #[inline]
    pub fn tffie(&self) -> TFFIER {
        TFFIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Soft reset. This is only needed in unusual circumstances. If a device issues a start condition without issuing a stop condition. A system timer may be used to reset the I2C if the bus remains busy longer than the time-out period. On a soft reset, the Tx and Rx FIFOs are flushed, I2C_STS register is cleared, and all internal state machines are reset to appear idle. The I2C_CLKHI, I2C_CLKLO and I2C_CTL (except Soft Reset Bit) are NOT modified by a soft reset."]
    #[inline]
    pub fn srst(&self) -> SRSTR {
        SRSTR::_from({
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
    #[doc = "Bit 0 - Transmit Done Interrupt Enable. This enables the TDI interrupt signalling that this I2C issued a STOP condition."]
    #[inline]
    pub fn tdie(&mut self) -> _TDIEW {
        _TDIEW { w: self }
    }
    #[doc = "Bit 1 - Transmitter Arbitration Failure Interrupt Enable. This enables the AFI interrupt which is asserted during transmission when trying to set SDA high, but the bus is driven low by another device."]
    #[inline]
    pub fn afie(&mut self) -> _AFIEW {
        _AFIEW { w: self }
    }
    #[doc = "Bit 2 - Transmitter No Acknowledge Interrupt Enable. This enables the NAI interrupt signalling that transmitted byte was not acknowledged."]
    #[inline]
    pub fn naie(&mut self) -> _NAIEW {
        _NAIEW { w: self }
    }
    #[doc = "Bit 3 - Master Transmitter Data Request Interrupt Enable. This enables the DRMI interrupt which signals that the master transmitter has run out of data, has not issued a STOP, and is holding the SCL line low."]
    #[inline]
    pub fn drmie(&mut self) -> _DRMIEW {
        _DRMIEW { w: self }
    }
    #[doc = "Bit 4 - Slave Transmitter Data Request Interrupt Enable. This enables the DRSI interrupt which signals that the slave transmitter has run out of data and the last byte was acknowledged, so the SCL line is being held low."]
    #[inline]
    pub fn drsie(&mut self) -> _DRSIEW {
        _DRSIEW { w: self }
    }
    #[doc = "Bit 5 - Receive FIFO Full Interrupt Enable. This enables the Receive FIFO Full interrupt to indicate that the receive FIFO cannot accept any more data."]
    #[inline]
    pub fn refie(&mut self) -> _REFIEW {
        _REFIEW { w: self }
    }
    #[doc = "Bit 6 - Receive Data Available Interrupt Enable. This enables the DAI interrupt to indicate that data is available in the receive FIFO (i.e. not empty)."]
    #[inline]
    pub fn rfdaie(&mut self) -> _RFDAIEW {
        _RFDAIEW { w: self }
    }
    #[doc = "Bit 7 - Transmit FIFO Not Full Interrupt Enable. This enables the Transmit FIFO Not Full interrupt to indicate that the more data can be written to the transmit FIFO. Note that this is not full. It is intended help the CPU to write to the I2C block only when there is room in the FIFO and do this without polling the status register."]
    #[inline]
    pub fn tffie(&mut self) -> _TFFIEW {
        _TFFIEW { w: self }
    }
    #[doc = "Bit 8 - Soft reset. This is only needed in unusual circumstances. If a device issues a start condition without issuing a stop condition. A system timer may be used to reset the I2C if the bus remains busy longer than the time-out period. On a soft reset, the Tx and Rx FIFOs are flushed, I2C_STS register is cleared, and all internal state machines are reset to appear idle. The I2C_CLKHI, I2C_CLKLO and I2C_CTL (except Soft Reset Bit) are NOT modified by a soft reset."]
    #[inline]
    pub fn srst(&mut self) -> _SRSTW {
        _SRSTW { w: self }
    }
}
