#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCR {
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
#[doc = "Possible values of the field `PWMMR0I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR0IR {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Interrupt on PWMMR0: an interrupt is generated when PWMMR0 matches the value in the PWMTC."]
    INTERRUPT,
}
impl PWMMR0IR {
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
            PWMMR0IR::DISABLED => false,
            PWMMR0IR::INTERRUPT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMMR0IR {
        match value {
            false => PWMMR0IR::DISABLED,
            true => PWMMR0IR::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR0IR::DISABLED
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline]
    pub fn is_interrupt(&self) -> bool {
        *self == PWMMR0IR::INTERRUPT
    }
}
#[doc = "Possible values of the field `PWMMR0R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR0RR {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Reset on PWMMR0: the PWMTC will be reset if PWMMR0 matches it."]
    RESET,
}
impl PWMMR0RR {
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
            PWMMR0RR::DISABLED => false,
            PWMMR0RR::RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMMR0RR {
        match value {
            false => PWMMR0RR::DISABLED,
            true => PWMMR0RR::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR0RR::DISABLED
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == PWMMR0RR::RESET
    }
}
#[doc = "Possible values of the field `PWMMR0S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR0SR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Stop on PWMMR0: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    STOP,
}
impl PWMMR0SR {
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
            PWMMR0SR::DISABLED => false,
            PWMMR0SR::STOP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMMR0SR {
        match value {
            false => PWMMR0SR::DISABLED,
            true => PWMMR0SR::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR0SR::DISABLED
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline]
    pub fn is_stop(&self) -> bool {
        *self == PWMMR0SR::STOP
    }
}
#[doc = "Possible values of the field `PWMMR1I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR1IR {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Interrupt on PWMMR1: an interrupt is generated when PWMMR1 matches the value in the PWMTC."]
    INTERRUPT,
}
impl PWMMR1IR {
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
            PWMMR1IR::DISABLED => false,
            PWMMR1IR::INTERRUPT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMMR1IR {
        match value {
            false => PWMMR1IR::DISABLED,
            true => PWMMR1IR::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR1IR::DISABLED
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline]
    pub fn is_interrupt(&self) -> bool {
        *self == PWMMR1IR::INTERRUPT
    }
}
#[doc = "Possible values of the field `PWMMR1R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR1RR {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Reset on PWMMR1: the PWMTC will be reset if PWMMR1 matches it."]
    RESET,
}
impl PWMMR1RR {
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
            PWMMR1RR::DISABLED => false,
            PWMMR1RR::RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMMR1RR {
        match value {
            false => PWMMR1RR::DISABLED,
            true => PWMMR1RR::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR1RR::DISABLED
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == PWMMR1RR::RESET
    }
}
#[doc = "Possible values of the field `PWMMR1S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR1SR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Stop on PWMMR1: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR1 matches the PWMTC."]
    STOP,
}
impl PWMMR1SR {
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
            PWMMR1SR::DISABLED => false,
            PWMMR1SR::STOP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMMR1SR {
        match value {
            false => PWMMR1SR::DISABLED,
            true => PWMMR1SR::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR1SR::DISABLED
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline]
    pub fn is_stop(&self) -> bool {
        *self == PWMMR1SR::STOP
    }
}
#[doc = "Possible values of the field `PWMMR2I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR2IR {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Interrupt on PWMMR2: an interrupt is generated when PWMMR2 matches the value in the PWMTC."]
    INTERRUPT,
}
impl PWMMR2IR {
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
            PWMMR2IR::DISABLED => false,
            PWMMR2IR::INTERRUPT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMMR2IR {
        match value {
            false => PWMMR2IR::DISABLED,
            true => PWMMR2IR::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR2IR::DISABLED
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline]
    pub fn is_interrupt(&self) -> bool {
        *self == PWMMR2IR::INTERRUPT
    }
}
#[doc = "Possible values of the field `PWMMR2R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR2RR {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Reset on PWMMR2: the PWMTC will be reset if PWMMR2 matches it."]
    RESET,
}
impl PWMMR2RR {
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
            PWMMR2RR::DISABLED => false,
            PWMMR2RR::RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMMR2RR {
        match value {
            false => PWMMR2RR::DISABLED,
            true => PWMMR2RR::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR2RR::DISABLED
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == PWMMR2RR::RESET
    }
}
#[doc = "Possible values of the field `PWMMR2S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR2SR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Stop on PWMMR2: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    STOP,
}
impl PWMMR2SR {
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
            PWMMR2SR::DISABLED => false,
            PWMMR2SR::STOP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMMR2SR {
        match value {
            false => PWMMR2SR::DISABLED,
            true => PWMMR2SR::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR2SR::DISABLED
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline]
    pub fn is_stop(&self) -> bool {
        *self == PWMMR2SR::STOP
    }
}
#[doc = "Possible values of the field `PWMMR3I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR3IR {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Interrupt on PWMMR3: an interrupt is generated when PWMMR3 matches the value in the PWMTC."]
    INTERRUPT,
}
impl PWMMR3IR {
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
            PWMMR3IR::DISABLED => false,
            PWMMR3IR::INTERRUPT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMMR3IR {
        match value {
            false => PWMMR3IR::DISABLED,
            true => PWMMR3IR::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR3IR::DISABLED
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline]
    pub fn is_interrupt(&self) -> bool {
        *self == PWMMR3IR::INTERRUPT
    }
}
#[doc = "Possible values of the field `PWMMR3R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR3RR {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Reset on PWMMR3: the PWMTC will be reset if PWMMR3 matches it."]
    RESET,
}
impl PWMMR3RR {
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
            PWMMR3RR::DISABLED => false,
            PWMMR3RR::RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMMR3RR {
        match value {
            false => PWMMR3RR::DISABLED,
            true => PWMMR3RR::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR3RR::DISABLED
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == PWMMR3RR::RESET
    }
}
#[doc = "Possible values of the field `PWMMR3S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR3SR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Stop on PWMMR3: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    STOP,
}
impl PWMMR3SR {
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
            PWMMR3SR::DISABLED => false,
            PWMMR3SR::STOP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMMR3SR {
        match value {
            false => PWMMR3SR::DISABLED,
            true => PWMMR3SR::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR3SR::DISABLED
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline]
    pub fn is_stop(&self) -> bool {
        *self == PWMMR3SR::STOP
    }
}
#[doc = "Possible values of the field `PWMMR4I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR4IR {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Interrupt on PWMMR4: an interrupt is generated when PWMMR4 matches the value in the PWMTC."]
    INTERRUPT,
}
impl PWMMR4IR {
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
            PWMMR4IR::DISABLED => false,
            PWMMR4IR::INTERRUPT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMMR4IR {
        match value {
            false => PWMMR4IR::DISABLED,
            true => PWMMR4IR::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR4IR::DISABLED
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline]
    pub fn is_interrupt(&self) -> bool {
        *self == PWMMR4IR::INTERRUPT
    }
}
#[doc = "Possible values of the field `PWMMR4R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR4RR {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Reset on PWMMR4: the PWMTC will be reset if PWMMR4 matches it."]
    RESET,
}
impl PWMMR4RR {
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
            PWMMR4RR::DISABLED => false,
            PWMMR4RR::RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMMR4RR {
        match value {
            false => PWMMR4RR::DISABLED,
            true => PWMMR4RR::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR4RR::DISABLED
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == PWMMR4RR::RESET
    }
}
#[doc = "Possible values of the field `PWMMR4S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR4SR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Stop on PWMMR4: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR4 matches the PWMTC."]
    STOP,
}
impl PWMMR4SR {
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
            PWMMR4SR::DISABLED => false,
            PWMMR4SR::STOP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMMR4SR {
        match value {
            false => PWMMR4SR::DISABLED,
            true => PWMMR4SR::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR4SR::DISABLED
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline]
    pub fn is_stop(&self) -> bool {
        *self == PWMMR4SR::STOP
    }
}
#[doc = "Possible values of the field `PWMMR5I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR5IR {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Interrupt on PWMMR5: an interrupt is generated when PWMMR5 matches the value in the PWMTC."]
    INTERRUPT,
}
impl PWMMR5IR {
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
            PWMMR5IR::DISABLED => false,
            PWMMR5IR::INTERRUPT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMMR5IR {
        match value {
            false => PWMMR5IR::DISABLED,
            true => PWMMR5IR::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR5IR::DISABLED
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline]
    pub fn is_interrupt(&self) -> bool {
        *self == PWMMR5IR::INTERRUPT
    }
}
#[doc = "Possible values of the field `PWMMR5R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR5RR {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Reset on PWMMR5: the PWMTC will be reset if PWMMR5 matches it."]
    RESET,
}
impl PWMMR5RR {
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
            PWMMR5RR::DISABLED => false,
            PWMMR5RR::RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMMR5RR {
        match value {
            false => PWMMR5RR::DISABLED,
            true => PWMMR5RR::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR5RR::DISABLED
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == PWMMR5RR::RESET
    }
}
#[doc = "Possible values of the field `PWMMR5S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR5SR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Stop on PWMMR5: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR5 matches the PWMTC."]
    STOP,
}
impl PWMMR5SR {
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
            PWMMR5SR::DISABLED => false,
            PWMMR5SR::STOP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMMR5SR {
        match value {
            false => PWMMR5SR::DISABLED,
            true => PWMMR5SR::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR5SR::DISABLED
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline]
    pub fn is_stop(&self) -> bool {
        *self == PWMMR5SR::STOP
    }
}
#[doc = "Possible values of the field `PWMMR6I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR6IR {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Interrupt on PWMMR6: an interrupt is generated when PWMMR6 matches the value in the PWMTC."]
    INTERRUPT,
}
impl PWMMR6IR {
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
            PWMMR6IR::DISABLED => false,
            PWMMR6IR::INTERRUPT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMMR6IR {
        match value {
            false => PWMMR6IR::DISABLED,
            true => PWMMR6IR::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR6IR::DISABLED
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline]
    pub fn is_interrupt(&self) -> bool {
        *self == PWMMR6IR::INTERRUPT
    }
}
#[doc = "Possible values of the field `PWMMR6R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR6RR {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Reset on PWMMR6: the PWMTC will be reset if PWMMR6 matches it."]
    RESET,
}
impl PWMMR6RR {
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
            PWMMR6RR::DISABLED => false,
            PWMMR6RR::RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMMR6RR {
        match value {
            false => PWMMR6RR::DISABLED,
            true => PWMMR6RR::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR6RR::DISABLED
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == PWMMR6RR::RESET
    }
}
#[doc = "Possible values of the field `PWMMR6S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR6SR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Stop on PWMMR6: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR6 matches the PWMTC."]
    STOP,
}
impl PWMMR6SR {
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
            PWMMR6SR::DISABLED => false,
            PWMMR6SR::STOP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMMR6SR {
        match value {
            false => PWMMR6SR::DISABLED,
            true => PWMMR6SR::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR6SR::DISABLED
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline]
    pub fn is_stop(&self) -> bool {
        *self == PWMMR6SR::STOP
    }
}
#[doc = "Values that can be written to the field `PWMMR0I`"]
pub enum PWMMR0IW {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Interrupt on PWMMR0: an interrupt is generated when PWMMR0 matches the value in the PWMTC."]
    INTERRUPT,
}
impl PWMMR0IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR0IW::DISABLED => false,
            PWMMR0IW::INTERRUPT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMMR0IW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR0IW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMMR0IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMMR0IW::DISABLED)
    }
    #[doc = "Interrupt on PWMMR0: an interrupt is generated when PWMMR0 matches the value in the PWMTC."]
    #[inline]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(PWMMR0IW::INTERRUPT)
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
#[doc = "Values that can be written to the field `PWMMR0R`"]
pub enum PWMMR0RW {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Reset on PWMMR0: the PWMTC will be reset if PWMMR0 matches it."]
    RESET,
}
impl PWMMR0RW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR0RW::DISABLED => false,
            PWMMR0RW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMMR0RW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR0RW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMMR0RW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMMR0RW::DISABLED)
    }
    #[doc = "Reset on PWMMR0: the PWMTC will be reset if PWMMR0 matches it."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(PWMMR0RW::RESET)
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
#[doc = "Values that can be written to the field `PWMMR0S`"]
pub enum PWMMR0SW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Stop on PWMMR0: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    STOP,
}
impl PWMMR0SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR0SW::DISABLED => false,
            PWMMR0SW::STOP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMMR0SW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR0SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMMR0SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMMR0SW::DISABLED)
    }
    #[doc = "Stop on PWMMR0: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    #[inline]
    pub fn stop(self) -> &'a mut W {
        self.variant(PWMMR0SW::STOP)
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
#[doc = "Values that can be written to the field `PWMMR1I`"]
pub enum PWMMR1IW {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Interrupt on PWMMR1: an interrupt is generated when PWMMR1 matches the value in the PWMTC."]
    INTERRUPT,
}
impl PWMMR1IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR1IW::DISABLED => false,
            PWMMR1IW::INTERRUPT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMMR1IW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR1IW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMMR1IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMMR1IW::DISABLED)
    }
    #[doc = "Interrupt on PWMMR1: an interrupt is generated when PWMMR1 matches the value in the PWMTC."]
    #[inline]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(PWMMR1IW::INTERRUPT)
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
#[doc = "Values that can be written to the field `PWMMR1R`"]
pub enum PWMMR1RW {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Reset on PWMMR1: the PWMTC will be reset if PWMMR1 matches it."]
    RESET,
}
impl PWMMR1RW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR1RW::DISABLED => false,
            PWMMR1RW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMMR1RW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR1RW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMMR1RW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMMR1RW::DISABLED)
    }
    #[doc = "Reset on PWMMR1: the PWMTC will be reset if PWMMR1 matches it."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(PWMMR1RW::RESET)
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
#[doc = "Values that can be written to the field `PWMMR1S`"]
pub enum PWMMR1SW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Stop on PWMMR1: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR1 matches the PWMTC."]
    STOP,
}
impl PWMMR1SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR1SW::DISABLED => false,
            PWMMR1SW::STOP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMMR1SW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR1SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMMR1SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMMR1SW::DISABLED)
    }
    #[doc = "Stop on PWMMR1: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR1 matches the PWMTC."]
    #[inline]
    pub fn stop(self) -> &'a mut W {
        self.variant(PWMMR1SW::STOP)
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
#[doc = "Values that can be written to the field `PWMMR2I`"]
pub enum PWMMR2IW {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Interrupt on PWMMR2: an interrupt is generated when PWMMR2 matches the value in the PWMTC."]
    INTERRUPT,
}
impl PWMMR2IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR2IW::DISABLED => false,
            PWMMR2IW::INTERRUPT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMMR2IW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR2IW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMMR2IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMMR2IW::DISABLED)
    }
    #[doc = "Interrupt on PWMMR2: an interrupt is generated when PWMMR2 matches the value in the PWMTC."]
    #[inline]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(PWMMR2IW::INTERRUPT)
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
#[doc = "Values that can be written to the field `PWMMR2R`"]
pub enum PWMMR2RW {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Reset on PWMMR2: the PWMTC will be reset if PWMMR2 matches it."]
    RESET,
}
impl PWMMR2RW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR2RW::DISABLED => false,
            PWMMR2RW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMMR2RW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR2RW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMMR2RW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMMR2RW::DISABLED)
    }
    #[doc = "Reset on PWMMR2: the PWMTC will be reset if PWMMR2 matches it."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(PWMMR2RW::RESET)
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
#[doc = "Values that can be written to the field `PWMMR2S`"]
pub enum PWMMR2SW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Stop on PWMMR2: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    STOP,
}
impl PWMMR2SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR2SW::DISABLED => false,
            PWMMR2SW::STOP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMMR2SW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR2SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMMR2SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMMR2SW::DISABLED)
    }
    #[doc = "Stop on PWMMR2: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    #[inline]
    pub fn stop(self) -> &'a mut W {
        self.variant(PWMMR2SW::STOP)
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
#[doc = "Values that can be written to the field `PWMMR3I`"]
pub enum PWMMR3IW {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Interrupt on PWMMR3: an interrupt is generated when PWMMR3 matches the value in the PWMTC."]
    INTERRUPT,
}
impl PWMMR3IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR3IW::DISABLED => false,
            PWMMR3IW::INTERRUPT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMMR3IW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR3IW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMMR3IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMMR3IW::DISABLED)
    }
    #[doc = "Interrupt on PWMMR3: an interrupt is generated when PWMMR3 matches the value in the PWMTC."]
    #[inline]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(PWMMR3IW::INTERRUPT)
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
#[doc = "Values that can be written to the field `PWMMR3R`"]
pub enum PWMMR3RW {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Reset on PWMMR3: the PWMTC will be reset if PWMMR3 matches it."]
    RESET,
}
impl PWMMR3RW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR3RW::DISABLED => false,
            PWMMR3RW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMMR3RW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR3RW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMMR3RW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMMR3RW::DISABLED)
    }
    #[doc = "Reset on PWMMR3: the PWMTC will be reset if PWMMR3 matches it."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(PWMMR3RW::RESET)
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
#[doc = "Values that can be written to the field `PWMMR3S`"]
pub enum PWMMR3SW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Stop on PWMMR3: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    STOP,
}
impl PWMMR3SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR3SW::DISABLED => false,
            PWMMR3SW::STOP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMMR3SW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR3SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMMR3SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMMR3SW::DISABLED)
    }
    #[doc = "Stop on PWMMR3: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    #[inline]
    pub fn stop(self) -> &'a mut W {
        self.variant(PWMMR3SW::STOP)
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
#[doc = "Values that can be written to the field `PWMMR4I`"]
pub enum PWMMR4IW {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Interrupt on PWMMR4: an interrupt is generated when PWMMR4 matches the value in the PWMTC."]
    INTERRUPT,
}
impl PWMMR4IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR4IW::DISABLED => false,
            PWMMR4IW::INTERRUPT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMMR4IW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR4IW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMMR4IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMMR4IW::DISABLED)
    }
    #[doc = "Interrupt on PWMMR4: an interrupt is generated when PWMMR4 matches the value in the PWMTC."]
    #[inline]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(PWMMR4IW::INTERRUPT)
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
#[doc = "Values that can be written to the field `PWMMR4R`"]
pub enum PWMMR4RW {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Reset on PWMMR4: the PWMTC will be reset if PWMMR4 matches it."]
    RESET,
}
impl PWMMR4RW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR4RW::DISABLED => false,
            PWMMR4RW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMMR4RW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR4RW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMMR4RW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMMR4RW::DISABLED)
    }
    #[doc = "Reset on PWMMR4: the PWMTC will be reset if PWMMR4 matches it."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(PWMMR4RW::RESET)
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
#[doc = "Values that can be written to the field `PWMMR4S`"]
pub enum PWMMR4SW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Stop on PWMMR4: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR4 matches the PWMTC."]
    STOP,
}
impl PWMMR4SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR4SW::DISABLED => false,
            PWMMR4SW::STOP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMMR4SW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR4SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMMR4SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMMR4SW::DISABLED)
    }
    #[doc = "Stop on PWMMR4: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR4 matches the PWMTC."]
    #[inline]
    pub fn stop(self) -> &'a mut W {
        self.variant(PWMMR4SW::STOP)
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
#[doc = "Values that can be written to the field `PWMMR5I`"]
pub enum PWMMR5IW {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Interrupt on PWMMR5: an interrupt is generated when PWMMR5 matches the value in the PWMTC."]
    INTERRUPT,
}
impl PWMMR5IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR5IW::DISABLED => false,
            PWMMR5IW::INTERRUPT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMMR5IW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR5IW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMMR5IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMMR5IW::DISABLED)
    }
    #[doc = "Interrupt on PWMMR5: an interrupt is generated when PWMMR5 matches the value in the PWMTC."]
    #[inline]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(PWMMR5IW::INTERRUPT)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PWMMR5R`"]
pub enum PWMMR5RW {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Reset on PWMMR5: the PWMTC will be reset if PWMMR5 matches it."]
    RESET,
}
impl PWMMR5RW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR5RW::DISABLED => false,
            PWMMR5RW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMMR5RW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR5RW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMMR5RW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMMR5RW::DISABLED)
    }
    #[doc = "Reset on PWMMR5: the PWMTC will be reset if PWMMR5 matches it."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(PWMMR5RW::RESET)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PWMMR5S`"]
pub enum PWMMR5SW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Stop on PWMMR5: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR5 matches the PWMTC."]
    STOP,
}
impl PWMMR5SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR5SW::DISABLED => false,
            PWMMR5SW::STOP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMMR5SW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR5SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMMR5SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMMR5SW::DISABLED)
    }
    #[doc = "Stop on PWMMR5: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR5 matches the PWMTC."]
    #[inline]
    pub fn stop(self) -> &'a mut W {
        self.variant(PWMMR5SW::STOP)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PWMMR6I`"]
pub enum PWMMR6IW {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Interrupt on PWMMR6: an interrupt is generated when PWMMR6 matches the value in the PWMTC."]
    INTERRUPT,
}
impl PWMMR6IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR6IW::DISABLED => false,
            PWMMR6IW::INTERRUPT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMMR6IW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR6IW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMMR6IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMMR6IW::DISABLED)
    }
    #[doc = "Interrupt on PWMMR6: an interrupt is generated when PWMMR6 matches the value in the PWMTC."]
    #[inline]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(PWMMR6IW::INTERRUPT)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PWMMR6R`"]
pub enum PWMMR6RW {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Reset on PWMMR6: the PWMTC will be reset if PWMMR6 matches it."]
    RESET,
}
impl PWMMR6RW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR6RW::DISABLED => false,
            PWMMR6RW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMMR6RW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR6RW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMMR6RW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMMR6RW::DISABLED)
    }
    #[doc = "Reset on PWMMR6: the PWMTC will be reset if PWMMR6 matches it."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(PWMMR6RW::RESET)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PWMMR6S`"]
pub enum PWMMR6SW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Stop on PWMMR6: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR6 matches the PWMTC."]
    STOP,
}
impl PWMMR6SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMMR6SW::DISABLED => false,
            PWMMR6SW::STOP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMMR6SW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMMR6SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMMR6SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMMR6SW::DISABLED)
    }
    #[doc = "Stop on PWMMR6: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR6 matches the PWMTC."]
    #[inline]
    pub fn stop(self) -> &'a mut W {
        self.variant(PWMMR6SW::STOP)
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
        const OFFSET: u8 = 20;
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
    #[doc = "Bit 0 - Interrupt PWM0"]
    #[inline]
    pub fn pwmmr0i(&self) -> PWMMR0IR {
        PWMMR0IR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Reset PWM0"]
    #[inline]
    pub fn pwmmr0r(&self) -> PWMMR0RR {
        PWMMR0RR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Stop PWM0"]
    #[inline]
    pub fn pwmmr0s(&self) -> PWMMR0SR {
        PWMMR0SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Interrupt PWM1"]
    #[inline]
    pub fn pwmmr1i(&self) -> PWMMR1IR {
        PWMMR1IR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Reset PWM1"]
    #[inline]
    pub fn pwmmr1r(&self) -> PWMMR1RR {
        PWMMR1RR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Stop PWM1"]
    #[inline]
    pub fn pwmmr1s(&self) -> PWMMR1SR {
        PWMMR1SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Interrupt PWM0"]
    #[inline]
    pub fn pwmmr2i(&self) -> PWMMR2IR {
        PWMMR2IR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Reset PWM0"]
    #[inline]
    pub fn pwmmr2r(&self) -> PWMMR2RR {
        PWMMR2RR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Stop PWM0"]
    #[inline]
    pub fn pwmmr2s(&self) -> PWMMR2SR {
        PWMMR2SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Interrupt PWM3"]
    #[inline]
    pub fn pwmmr3i(&self) -> PWMMR3IR {
        PWMMR3IR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Reset PWM3"]
    #[inline]
    pub fn pwmmr3r(&self) -> PWMMR3RR {
        PWMMR3RR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Stop PWM0"]
    #[inline]
    pub fn pwmmr3s(&self) -> PWMMR3SR {
        PWMMR3SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Interrupt PWM4"]
    #[inline]
    pub fn pwmmr4i(&self) -> PWMMR4IR {
        PWMMR4IR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Reset PWM4"]
    #[inline]
    pub fn pwmmr4r(&self) -> PWMMR4RR {
        PWMMR4RR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Stop PWM4"]
    #[inline]
    pub fn pwmmr4s(&self) -> PWMMR4SR {
        PWMMR4SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Interrupt PWM5"]
    #[inline]
    pub fn pwmmr5i(&self) -> PWMMR5IR {
        PWMMR5IR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Reset PWM5"]
    #[inline]
    pub fn pwmmr5r(&self) -> PWMMR5RR {
        PWMMR5RR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Stop PWM5"]
    #[inline]
    pub fn pwmmr5s(&self) -> PWMMR5SR {
        PWMMR5SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Interrupt PWM6"]
    #[inline]
    pub fn pwmmr6i(&self) -> PWMMR6IR {
        PWMMR6IR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Reset PWM6"]
    #[inline]
    pub fn pwmmr6r(&self) -> PWMMR6RR {
        PWMMR6RR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Stop PWM6"]
    #[inline]
    pub fn pwmmr6s(&self) -> PWMMR6SR {
        PWMMR6SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
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
    #[doc = "Bit 0 - Interrupt PWM0"]
    #[inline]
    pub fn pwmmr0i(&mut self) -> _PWMMR0IW {
        _PWMMR0IW { w: self }
    }
    #[doc = "Bit 1 - Reset PWM0"]
    #[inline]
    pub fn pwmmr0r(&mut self) -> _PWMMR0RW {
        _PWMMR0RW { w: self }
    }
    #[doc = "Bit 2 - Stop PWM0"]
    #[inline]
    pub fn pwmmr0s(&mut self) -> _PWMMR0SW {
        _PWMMR0SW { w: self }
    }
    #[doc = "Bit 3 - Interrupt PWM1"]
    #[inline]
    pub fn pwmmr1i(&mut self) -> _PWMMR1IW {
        _PWMMR1IW { w: self }
    }
    #[doc = "Bit 4 - Reset PWM1"]
    #[inline]
    pub fn pwmmr1r(&mut self) -> _PWMMR1RW {
        _PWMMR1RW { w: self }
    }
    #[doc = "Bit 5 - Stop PWM1"]
    #[inline]
    pub fn pwmmr1s(&mut self) -> _PWMMR1SW {
        _PWMMR1SW { w: self }
    }
    #[doc = "Bit 6 - Interrupt PWM0"]
    #[inline]
    pub fn pwmmr2i(&mut self) -> _PWMMR2IW {
        _PWMMR2IW { w: self }
    }
    #[doc = "Bit 7 - Reset PWM0"]
    #[inline]
    pub fn pwmmr2r(&mut self) -> _PWMMR2RW {
        _PWMMR2RW { w: self }
    }
    #[doc = "Bit 8 - Stop PWM0"]
    #[inline]
    pub fn pwmmr2s(&mut self) -> _PWMMR2SW {
        _PWMMR2SW { w: self }
    }
    #[doc = "Bit 9 - Interrupt PWM3"]
    #[inline]
    pub fn pwmmr3i(&mut self) -> _PWMMR3IW {
        _PWMMR3IW { w: self }
    }
    #[doc = "Bit 10 - Reset PWM3"]
    #[inline]
    pub fn pwmmr3r(&mut self) -> _PWMMR3RW {
        _PWMMR3RW { w: self }
    }
    #[doc = "Bit 11 - Stop PWM0"]
    #[inline]
    pub fn pwmmr3s(&mut self) -> _PWMMR3SW {
        _PWMMR3SW { w: self }
    }
    #[doc = "Bit 12 - Interrupt PWM4"]
    #[inline]
    pub fn pwmmr4i(&mut self) -> _PWMMR4IW {
        _PWMMR4IW { w: self }
    }
    #[doc = "Bit 13 - Reset PWM4"]
    #[inline]
    pub fn pwmmr4r(&mut self) -> _PWMMR4RW {
        _PWMMR4RW { w: self }
    }
    #[doc = "Bit 14 - Stop PWM4"]
    #[inline]
    pub fn pwmmr4s(&mut self) -> _PWMMR4SW {
        _PWMMR4SW { w: self }
    }
    #[doc = "Bit 15 - Interrupt PWM5"]
    #[inline]
    pub fn pwmmr5i(&mut self) -> _PWMMR5IW {
        _PWMMR5IW { w: self }
    }
    #[doc = "Bit 16 - Reset PWM5"]
    #[inline]
    pub fn pwmmr5r(&mut self) -> _PWMMR5RW {
        _PWMMR5RW { w: self }
    }
    #[doc = "Bit 17 - Stop PWM5"]
    #[inline]
    pub fn pwmmr5s(&mut self) -> _PWMMR5SW {
        _PWMMR5SW { w: self }
    }
    #[doc = "Bit 18 - Interrupt PWM6"]
    #[inline]
    pub fn pwmmr6i(&mut self) -> _PWMMR6IW {
        _PWMMR6IW { w: self }
    }
    #[doc = "Bit 19 - Reset PWM6"]
    #[inline]
    pub fn pwmmr6r(&mut self) -> _PWMMR6RW {
        _PWMMR6RW { w: self }
    }
    #[doc = "Bit 20 - Stop PWM6"]
    #[inline]
    pub fn pwmmr6s(&mut self) -> _PWMMR6SW {
        _PWMMR6SW { w: self }
    }
}
