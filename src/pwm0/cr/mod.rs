#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR {
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
#[doc = "Possible values of the field `PWMSEL2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMSEL2R {
    #[doc = "Single edge controlled mode is selected."]
    SINGLE_EDGE,
    #[doc = "Double edge controlled mode is selected."]
    DOUBLE_EDGE,
}
impl PWMSEL2R {
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
            PWMSEL2R::SINGLE_EDGE => false,
            PWMSEL2R::DOUBLE_EDGE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMSEL2R {
        match value {
            false => PWMSEL2R::SINGLE_EDGE,
            true => PWMSEL2R::DOUBLE_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_EDGE`"]
    #[inline]
    pub fn is_single_edge(&self) -> bool {
        *self == PWMSEL2R::SINGLE_EDGE
    }
    #[doc = "Checks if the value of the field is `DOUBLE_EDGE`"]
    #[inline]
    pub fn is_double_edge(&self) -> bool {
        *self == PWMSEL2R::DOUBLE_EDGE
    }
}
#[doc = "Possible values of the field `PWMSEL3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMSEL3R {
    #[doc = "Single edge controlled mode is selected."]
    SINGLE_EDGE,
    #[doc = "Double edge controlled mode is selected."]
    DOUBLE_EDGE,
}
impl PWMSEL3R {
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
            PWMSEL3R::SINGLE_EDGE => false,
            PWMSEL3R::DOUBLE_EDGE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMSEL3R {
        match value {
            false => PWMSEL3R::SINGLE_EDGE,
            true => PWMSEL3R::DOUBLE_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_EDGE`"]
    #[inline]
    pub fn is_single_edge(&self) -> bool {
        *self == PWMSEL3R::SINGLE_EDGE
    }
    #[doc = "Checks if the value of the field is `DOUBLE_EDGE`"]
    #[inline]
    pub fn is_double_edge(&self) -> bool {
        *self == PWMSEL3R::DOUBLE_EDGE
    }
}
#[doc = "Possible values of the field `PWMSEL4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMSEL4R {
    #[doc = "Single edge controlled mode is selected."]
    SINGLE_EDGE,
    #[doc = "Double edge controlled mode is selected."]
    DOUBLE_EDGE,
}
impl PWMSEL4R {
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
            PWMSEL4R::SINGLE_EDGE => false,
            PWMSEL4R::DOUBLE_EDGE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMSEL4R {
        match value {
            false => PWMSEL4R::SINGLE_EDGE,
            true => PWMSEL4R::DOUBLE_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_EDGE`"]
    #[inline]
    pub fn is_single_edge(&self) -> bool {
        *self == PWMSEL4R::SINGLE_EDGE
    }
    #[doc = "Checks if the value of the field is `DOUBLE_EDGE`"]
    #[inline]
    pub fn is_double_edge(&self) -> bool {
        *self == PWMSEL4R::DOUBLE_EDGE
    }
}
#[doc = "Possible values of the field `PWMSEL5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMSEL5R {
    #[doc = "Single edge controlled mode is selected."]
    SINGLE_EDGE,
    #[doc = "Double edge controlled mode is selected."]
    DOUBLE_EDGE,
}
impl PWMSEL5R {
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
            PWMSEL5R::SINGLE_EDGE => false,
            PWMSEL5R::DOUBLE_EDGE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMSEL5R {
        match value {
            false => PWMSEL5R::SINGLE_EDGE,
            true => PWMSEL5R::DOUBLE_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_EDGE`"]
    #[inline]
    pub fn is_single_edge(&self) -> bool {
        *self == PWMSEL5R::SINGLE_EDGE
    }
    #[doc = "Checks if the value of the field is `DOUBLE_EDGE`"]
    #[inline]
    pub fn is_double_edge(&self) -> bool {
        *self == PWMSEL5R::DOUBLE_EDGE
    }
}
#[doc = "Possible values of the field `PWMSEL6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMSEL6R {
    #[doc = "Single edge controlled mode is selected."]
    SINGLE_EDGE,
    #[doc = "Double edge controlled mode is selected."]
    DOUBLE_EDGE,
}
impl PWMSEL6R {
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
            PWMSEL6R::SINGLE_EDGE => false,
            PWMSEL6R::DOUBLE_EDGE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMSEL6R {
        match value {
            false => PWMSEL6R::SINGLE_EDGE,
            true => PWMSEL6R::DOUBLE_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_EDGE`"]
    #[inline]
    pub fn is_single_edge(&self) -> bool {
        *self == PWMSEL6R::SINGLE_EDGE
    }
    #[doc = "Checks if the value of the field is `DOUBLE_EDGE`"]
    #[inline]
    pub fn is_double_edge(&self) -> bool {
        *self == PWMSEL6R::DOUBLE_EDGE
    }
}
#[doc = "Possible values of the field `PWMENA1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMENA1R {
    #[doc = "The PWM output is disabled."]
    DISABLED,
    #[doc = "The PWM output is enabled."]
    ENABLED,
}
impl PWMENA1R {
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
            PWMENA1R::DISABLED => false,
            PWMENA1R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMENA1R {
        match value {
            false => PWMENA1R::DISABLED,
            true => PWMENA1R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PWMENA1R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PWMENA1R::ENABLED
    }
}
#[doc = "Possible values of the field `PWMENA2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMENA2R {
    #[doc = "The PWM output is disabled."]
    DISABLED,
    #[doc = "The PWM output is enabled."]
    ENABLED,
}
impl PWMENA2R {
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
            PWMENA2R::DISABLED => false,
            PWMENA2R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMENA2R {
        match value {
            false => PWMENA2R::DISABLED,
            true => PWMENA2R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PWMENA2R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PWMENA2R::ENABLED
    }
}
#[doc = "Possible values of the field `PWMENA3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMENA3R {
    #[doc = "The PWM output is disabled."]
    DISABLED,
    #[doc = "The PWM output is enabled."]
    ENABLED,
}
impl PWMENA3R {
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
            PWMENA3R::DISABLED => false,
            PWMENA3R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMENA3R {
        match value {
            false => PWMENA3R::DISABLED,
            true => PWMENA3R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PWMENA3R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PWMENA3R::ENABLED
    }
}
#[doc = "Possible values of the field `PWMENA4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMENA4R {
    #[doc = "The PWM output is disabled."]
    DISABLED,
    #[doc = "The PWM output is enabled."]
    ENABLED,
}
impl PWMENA4R {
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
            PWMENA4R::DISABLED => false,
            PWMENA4R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMENA4R {
        match value {
            false => PWMENA4R::DISABLED,
            true => PWMENA4R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PWMENA4R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PWMENA4R::ENABLED
    }
}
#[doc = "Possible values of the field `PWMENA5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMENA5R {
    #[doc = "The PWM output is disabled."]
    DISABLED,
    #[doc = "The PWM output is enabled."]
    ENABLED,
}
impl PWMENA5R {
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
            PWMENA5R::DISABLED => false,
            PWMENA5R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMENA5R {
        match value {
            false => PWMENA5R::DISABLED,
            true => PWMENA5R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PWMENA5R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PWMENA5R::ENABLED
    }
}
#[doc = "Possible values of the field `PWMENA6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMENA6R {
    #[doc = "The PWM output is disabled."]
    DISABLED,
    #[doc = "The PWM output is enabled."]
    ENABLED,
}
impl PWMENA6R {
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
            PWMENA6R::DISABLED => false,
            PWMENA6R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMENA6R {
        match value {
            false => PWMENA6R::DISABLED,
            true => PWMENA6R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PWMENA6R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PWMENA6R::ENABLED
    }
}
#[doc = "Values that can be written to the field `PWMSEL2`"]
pub enum PWMSEL2W {
    #[doc = "Single edge controlled mode is selected."]
    SINGLE_EDGE,
    #[doc = "Double edge controlled mode is selected."]
    DOUBLE_EDGE,
}
impl PWMSEL2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMSEL2W::SINGLE_EDGE => false,
            PWMSEL2W::DOUBLE_EDGE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMSEL2W<'a> {
    w: &'a mut W,
}
impl<'a> _PWMSEL2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMSEL2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Single edge controlled mode is selected."]
    #[inline]
    pub fn single_edge(self) -> &'a mut W {
        self.variant(PWMSEL2W::SINGLE_EDGE)
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline]
    pub fn double_edge(self) -> &'a mut W {
        self.variant(PWMSEL2W::DOUBLE_EDGE)
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
#[doc = "Values that can be written to the field `PWMSEL3`"]
pub enum PWMSEL3W {
    #[doc = "Single edge controlled mode is selected."]
    SINGLE_EDGE,
    #[doc = "Double edge controlled mode is selected."]
    DOUBLE_EDGE,
}
impl PWMSEL3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMSEL3W::SINGLE_EDGE => false,
            PWMSEL3W::DOUBLE_EDGE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMSEL3W<'a> {
    w: &'a mut W,
}
impl<'a> _PWMSEL3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMSEL3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Single edge controlled mode is selected."]
    #[inline]
    pub fn single_edge(self) -> &'a mut W {
        self.variant(PWMSEL3W::SINGLE_EDGE)
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline]
    pub fn double_edge(self) -> &'a mut W {
        self.variant(PWMSEL3W::DOUBLE_EDGE)
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
#[doc = "Values that can be written to the field `PWMSEL4`"]
pub enum PWMSEL4W {
    #[doc = "Single edge controlled mode is selected."]
    SINGLE_EDGE,
    #[doc = "Double edge controlled mode is selected."]
    DOUBLE_EDGE,
}
impl PWMSEL4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMSEL4W::SINGLE_EDGE => false,
            PWMSEL4W::DOUBLE_EDGE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMSEL4W<'a> {
    w: &'a mut W,
}
impl<'a> _PWMSEL4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMSEL4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Single edge controlled mode is selected."]
    #[inline]
    pub fn single_edge(self) -> &'a mut W {
        self.variant(PWMSEL4W::SINGLE_EDGE)
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline]
    pub fn double_edge(self) -> &'a mut W {
        self.variant(PWMSEL4W::DOUBLE_EDGE)
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
#[doc = "Values that can be written to the field `PWMSEL5`"]
pub enum PWMSEL5W {
    #[doc = "Single edge controlled mode is selected."]
    SINGLE_EDGE,
    #[doc = "Double edge controlled mode is selected."]
    DOUBLE_EDGE,
}
impl PWMSEL5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMSEL5W::SINGLE_EDGE => false,
            PWMSEL5W::DOUBLE_EDGE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMSEL5W<'a> {
    w: &'a mut W,
}
impl<'a> _PWMSEL5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMSEL5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Single edge controlled mode is selected."]
    #[inline]
    pub fn single_edge(self) -> &'a mut W {
        self.variant(PWMSEL5W::SINGLE_EDGE)
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline]
    pub fn double_edge(self) -> &'a mut W {
        self.variant(PWMSEL5W::DOUBLE_EDGE)
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
#[doc = "Values that can be written to the field `PWMSEL6`"]
pub enum PWMSEL6W {
    #[doc = "Single edge controlled mode is selected."]
    SINGLE_EDGE,
    #[doc = "Double edge controlled mode is selected."]
    DOUBLE_EDGE,
}
impl PWMSEL6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMSEL6W::SINGLE_EDGE => false,
            PWMSEL6W::DOUBLE_EDGE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMSEL6W<'a> {
    w: &'a mut W,
}
impl<'a> _PWMSEL6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMSEL6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Single edge controlled mode is selected."]
    #[inline]
    pub fn single_edge(self) -> &'a mut W {
        self.variant(PWMSEL6W::SINGLE_EDGE)
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline]
    pub fn double_edge(self) -> &'a mut W {
        self.variant(PWMSEL6W::DOUBLE_EDGE)
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
#[doc = "Values that can be written to the field `PWMENA1`"]
pub enum PWMENA1W {
    #[doc = "The PWM output is disabled."]
    DISABLED,
    #[doc = "The PWM output is enabled."]
    ENABLED,
}
impl PWMENA1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMENA1W::DISABLED => false,
            PWMENA1W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMENA1W<'a> {
    w: &'a mut W,
}
impl<'a> _PWMENA1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMENA1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The PWM output is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMENA1W::DISABLED)
    }
    #[doc = "The PWM output is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PWMENA1W::ENABLED)
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
#[doc = "Values that can be written to the field `PWMENA2`"]
pub enum PWMENA2W {
    #[doc = "The PWM output is disabled."]
    DISABLED,
    #[doc = "The PWM output is enabled."]
    ENABLED,
}
impl PWMENA2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMENA2W::DISABLED => false,
            PWMENA2W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMENA2W<'a> {
    w: &'a mut W,
}
impl<'a> _PWMENA2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMENA2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The PWM output is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMENA2W::DISABLED)
    }
    #[doc = "The PWM output is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PWMENA2W::ENABLED)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PWMENA3`"]
pub enum PWMENA3W {
    #[doc = "The PWM output is disabled."]
    DISABLED,
    #[doc = "The PWM output is enabled."]
    ENABLED,
}
impl PWMENA3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMENA3W::DISABLED => false,
            PWMENA3W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMENA3W<'a> {
    w: &'a mut W,
}
impl<'a> _PWMENA3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMENA3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The PWM output is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMENA3W::DISABLED)
    }
    #[doc = "The PWM output is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PWMENA3W::ENABLED)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PWMENA4`"]
pub enum PWMENA4W {
    #[doc = "The PWM output is disabled."]
    DISABLED,
    #[doc = "The PWM output is enabled."]
    ENABLED,
}
impl PWMENA4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMENA4W::DISABLED => false,
            PWMENA4W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMENA4W<'a> {
    w: &'a mut W,
}
impl<'a> _PWMENA4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMENA4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The PWM output is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMENA4W::DISABLED)
    }
    #[doc = "The PWM output is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PWMENA4W::ENABLED)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PWMENA5`"]
pub enum PWMENA5W {
    #[doc = "The PWM output is disabled."]
    DISABLED,
    #[doc = "The PWM output is enabled."]
    ENABLED,
}
impl PWMENA5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMENA5W::DISABLED => false,
            PWMENA5W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMENA5W<'a> {
    w: &'a mut W,
}
impl<'a> _PWMENA5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMENA5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The PWM output is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMENA5W::DISABLED)
    }
    #[doc = "The PWM output is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PWMENA5W::ENABLED)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PWMENA6`"]
pub enum PWMENA6W {
    #[doc = "The PWM output is disabled."]
    DISABLED,
    #[doc = "The PWM output is enabled."]
    ENABLED,
}
impl PWMENA6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMENA6W::DISABLED => false,
            PWMENA6W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMENA6W<'a> {
    w: &'a mut W,
}
impl<'a> _PWMENA6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMENA6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The PWM output is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMENA6W::DISABLED)
    }
    #[doc = "The PWM output is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PWMENA6W::ENABLED)
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
        const OFFSET: u8 = 14;
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
    #[doc = "Bit 2 - PWM[2] output single/double edge mode control."]
    #[inline]
    pub fn pwmsel2(&self) -> PWMSEL2R {
        PWMSEL2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - PWM[3] output edge control."]
    #[inline]
    pub fn pwmsel3(&self) -> PWMSEL3R {
        PWMSEL3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - PWM[4] output edge control."]
    #[inline]
    pub fn pwmsel4(&self) -> PWMSEL4R {
        PWMSEL4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - PWM[5] output edge control."]
    #[inline]
    pub fn pwmsel5(&self) -> PWMSEL5R {
        PWMSEL5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - PWM[6] output edge control."]
    #[inline]
    pub fn pwmsel6(&self) -> PWMSEL6R {
        PWMSEL6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - PWM[1] output enable control."]
    #[inline]
    pub fn pwmena1(&self) -> PWMENA1R {
        PWMENA1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - PWM[2] output enable control."]
    #[inline]
    pub fn pwmena2(&self) -> PWMENA2R {
        PWMENA2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - PWM[3] output enable control."]
    #[inline]
    pub fn pwmena3(&self) -> PWMENA3R {
        PWMENA3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - PWM[4] output enable control."]
    #[inline]
    pub fn pwmena4(&self) -> PWMENA4R {
        PWMENA4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - PWM[5] output enable control."]
    #[inline]
    pub fn pwmena5(&self) -> PWMENA5R {
        PWMENA5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - PWM[6] output enable control. See PWMENA1 for details."]
    #[inline]
    pub fn pwmena6(&self) -> PWMENA6R {
        PWMENA6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
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
    #[doc = "Bit 2 - PWM[2] output single/double edge mode control."]
    #[inline]
    pub fn pwmsel2(&mut self) -> _PWMSEL2W {
        _PWMSEL2W { w: self }
    }
    #[doc = "Bit 3 - PWM[3] output edge control."]
    #[inline]
    pub fn pwmsel3(&mut self) -> _PWMSEL3W {
        _PWMSEL3W { w: self }
    }
    #[doc = "Bit 4 - PWM[4] output edge control."]
    #[inline]
    pub fn pwmsel4(&mut self) -> _PWMSEL4W {
        _PWMSEL4W { w: self }
    }
    #[doc = "Bit 5 - PWM[5] output edge control."]
    #[inline]
    pub fn pwmsel5(&mut self) -> _PWMSEL5W {
        _PWMSEL5W { w: self }
    }
    #[doc = "Bit 6 - PWM[6] output edge control."]
    #[inline]
    pub fn pwmsel6(&mut self) -> _PWMSEL6W {
        _PWMSEL6W { w: self }
    }
    #[doc = "Bit 9 - PWM[1] output enable control."]
    #[inline]
    pub fn pwmena1(&mut self) -> _PWMENA1W {
        _PWMENA1W { w: self }
    }
    #[doc = "Bit 10 - PWM[2] output enable control."]
    #[inline]
    pub fn pwmena2(&mut self) -> _PWMENA2W {
        _PWMENA2W { w: self }
    }
    #[doc = "Bit 11 - PWM[3] output enable control."]
    #[inline]
    pub fn pwmena3(&mut self) -> _PWMENA3W {
        _PWMENA3W { w: self }
    }
    #[doc = "Bit 12 - PWM[4] output enable control."]
    #[inline]
    pub fn pwmena4(&mut self) -> _PWMENA4W {
        _PWMENA4W { w: self }
    }
    #[doc = "Bit 13 - PWM[5] output enable control."]
    #[inline]
    pub fn pwmena5(&mut self) -> _PWMENA5W {
        _PWMENA5W { w: self }
    }
    #[doc = "Bit 14 - PWM[6] output enable control. See PWMENA1 for details."]
    #[inline]
    pub fn pwmena6(&mut self) -> _PWMENA6W {
        _PWMENA6W { w: self }
    }
}
