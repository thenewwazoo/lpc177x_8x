#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYNCCTRL {
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
#[doc = "Possible values of the field `SYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl SYNCR {
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
            SYNCR::DISABLED => false,
            SYNCR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYNCR {
        match value {
            false => SYNCR::DISABLED,
            true => SYNCR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SYNCR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SYNCR::ENABLED
    }
}
#[doc = "Possible values of the field `CSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSRCR {
    #[doc = "Synchronous slave mode (SCLK in)"]
    SLAVE,
    #[doc = "Synchronous master mode (SCLK out)"]
    MASTER,
}
impl CSRCR {
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
            CSRCR::SLAVE => false,
            CSRCR::MASTER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CSRCR {
        match value {
            false => CSRCR::SLAVE,
            true => CSRCR::MASTER,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline]
    pub fn is_slave(&self) -> bool {
        *self == CSRCR::SLAVE
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline]
    pub fn is_master(&self) -> bool {
        *self == CSRCR::MASTER
    }
}
#[doc = "Possible values of the field `FES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FESR {
    #[doc = "RxD is sampled on the rising edge of SCLK"]
    RISING,
    #[doc = "RxD is sampled on the falling edge of SCLK"]
    FALLING,
}
impl FESR {
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
            FESR::RISING => false,
            FESR::FALLING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FESR {
        match value {
            false => FESR::RISING,
            true => FESR::FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == FESR::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == FESR::FALLING
    }
}
#[doc = "Possible values of the field `TSBYPASS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSBYPASSR {
    #[doc = "The input clock is synchronized prior to being used in clock edge detection logic."]
    SYNCHRONIZED,
    #[doc = "The input clock is not synchronized prior to being used in clock edge detection logic. This allows for a high er input clock rate at the expense of potential metastability."]
    NOT_SYNCHRONIZED,
}
impl TSBYPASSR {
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
            TSBYPASSR::SYNCHRONIZED => false,
            TSBYPASSR::NOT_SYNCHRONIZED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSBYPASSR {
        match value {
            false => TSBYPASSR::SYNCHRONIZED,
            true => TSBYPASSR::NOT_SYNCHRONIZED,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONIZED`"]
    #[inline]
    pub fn is_synchronized(&self) -> bool {
        *self == TSBYPASSR::SYNCHRONIZED
    }
    #[doc = "Checks if the value of the field is `NOT_SYNCHRONIZED`"]
    #[inline]
    pub fn is_not_synchronized(&self) -> bool {
        *self == TSBYPASSR::NOT_SYNCHRONIZED
    }
}
#[doc = "Possible values of the field `CSCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSCENR {
    #[doc = "SCLK cycles only when characters are being sent on TxD"]
    ACTIVE,
    #[doc = "SCLK runs continuously (characters can be received on RxD independently from transmission on TxD)"]
    CONTINUOUS,
}
impl CSCENR {
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
            CSCENR::ACTIVE => false,
            CSCENR::CONTINUOUS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CSCENR {
        match value {
            false => CSCENR::ACTIVE,
            true => CSCENR::CONTINUOUS,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline]
    pub fn is_active(&self) -> bool {
        *self == CSCENR::ACTIVE
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline]
    pub fn is_continuous(&self) -> bool {
        *self == CSCENR::CONTINUOUS
    }
}
#[doc = "Possible values of the field `SSSDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSSDISR {
    #[doc = "Send start and stop bits as in other modes."]
    STARTSTOPBIT,
    #[doc = "Do not send start/stop bits."]
    NOSTARTSTOPBIT,
}
impl SSSDISR {
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
            SSSDISR::STARTSTOPBIT => false,
            SSSDISR::NOSTARTSTOPBIT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSSDISR {
        match value {
            false => SSSDISR::STARTSTOPBIT,
            true => SSSDISR::NOSTARTSTOPBIT,
        }
    }
    #[doc = "Checks if the value of the field is `STARTSTOPBIT`"]
    #[inline]
    pub fn is_startstopbit(&self) -> bool {
        *self == SSSDISR::STARTSTOPBIT
    }
    #[doc = "Checks if the value of the field is `NOSTARTSTOPBIT`"]
    #[inline]
    pub fn is_nostartstopbit(&self) -> bool {
        *self == SSSDISR::NOSTARTSTOPBIT
    }
}
#[doc = "Possible values of the field `CCCLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCCLRR {
    #[doc = "CSCEN is under software control."]
    SW_CTRL,
    #[doc = "Hardware clears CSCEN after each character is received."]
    HW_CTRL,
}
impl CCCLRR {
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
            CCCLRR::SW_CTRL => false,
            CCCLRR::HW_CTRL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCCLRR {
        match value {
            false => CCCLRR::SW_CTRL,
            true => CCCLRR::HW_CTRL,
        }
    }
    #[doc = "Checks if the value of the field is `SW_CTRL`"]
    #[inline]
    pub fn is_sw_ctrl(&self) -> bool {
        *self == CCCLRR::SW_CTRL
    }
    #[doc = "Checks if the value of the field is `HW_CTRL`"]
    #[inline]
    pub fn is_hw_ctrl(&self) -> bool {
        *self == CCCLRR::HW_CTRL
    }
}
#[doc = "Values that can be written to the field `SYNC`"]
pub enum SYNCW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl SYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYNCW::DISABLED => false,
            SYNCW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYNCW::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SYNCW::ENABLED)
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
#[doc = "Values that can be written to the field `CSRC`"]
pub enum CSRCW {
    #[doc = "Synchronous slave mode (SCLK in)"]
    SLAVE,
    #[doc = "Synchronous master mode (SCLK out)"]
    MASTER,
}
impl CSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CSRCW::SLAVE => false,
            CSRCW::MASTER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _CSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Synchronous slave mode (SCLK in)"]
    #[inline]
    pub fn slave(self) -> &'a mut W {
        self.variant(CSRCW::SLAVE)
    }
    #[doc = "Synchronous master mode (SCLK out)"]
    #[inline]
    pub fn master(self) -> &'a mut W {
        self.variant(CSRCW::MASTER)
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
#[doc = "Values that can be written to the field `FES`"]
pub enum FESW {
    #[doc = "RxD is sampled on the rising edge of SCLK"]
    RISING,
    #[doc = "RxD is sampled on the falling edge of SCLK"]
    FALLING,
}
impl FESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FESW::RISING => false,
            FESW::FALLING => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FESW<'a> {
    w: &'a mut W,
}
impl<'a> _FESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FESW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RxD is sampled on the rising edge of SCLK"]
    #[inline]
    pub fn rising(self) -> &'a mut W {
        self.variant(FESW::RISING)
    }
    #[doc = "RxD is sampled on the falling edge of SCLK"]
    #[inline]
    pub fn falling(self) -> &'a mut W {
        self.variant(FESW::FALLING)
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
#[doc = "Values that can be written to the field `TSBYPASS`"]
pub enum TSBYPASSW {
    #[doc = "The input clock is synchronized prior to being used in clock edge detection logic."]
    SYNCHRONIZED,
    #[doc = "The input clock is not synchronized prior to being used in clock edge detection logic. This allows for a high er input clock rate at the expense of potential metastability."]
    NOT_SYNCHRONIZED,
}
impl TSBYPASSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSBYPASSW::SYNCHRONIZED => false,
            TSBYPASSW::NOT_SYNCHRONIZED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSBYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _TSBYPASSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSBYPASSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The input clock is synchronized prior to being used in clock edge detection logic."]
    #[inline]
    pub fn synchronized(self) -> &'a mut W {
        self.variant(TSBYPASSW::SYNCHRONIZED)
    }
    #[doc = "The input clock is not synchronized prior to being used in clock edge detection logic. This allows for a high er input clock rate at the expense of potential metastability."]
    #[inline]
    pub fn not_synchronized(self) -> &'a mut W {
        self.variant(TSBYPASSW::NOT_SYNCHRONIZED)
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
#[doc = "Values that can be written to the field `CSCEN`"]
pub enum CSCENW {
    #[doc = "SCLK cycles only when characters are being sent on TxD"]
    ACTIVE,
    #[doc = "SCLK runs continuously (characters can be received on RxD independently from transmission on TxD)"]
    CONTINUOUS,
}
impl CSCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CSCENW::ACTIVE => false,
            CSCENW::CONTINUOUS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSCENW<'a> {
    w: &'a mut W,
}
impl<'a> _CSCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SCLK cycles only when characters are being sent on TxD"]
    #[inline]
    pub fn active(self) -> &'a mut W {
        self.variant(CSCENW::ACTIVE)
    }
    #[doc = "SCLK runs continuously (characters can be received on RxD independently from transmission on TxD)"]
    #[inline]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CSCENW::CONTINUOUS)
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
#[doc = "Values that can be written to the field `SSSDIS`"]
pub enum SSSDISW {
    #[doc = "Send start and stop bits as in other modes."]
    STARTSTOPBIT,
    #[doc = "Do not send start/stop bits."]
    NOSTARTSTOPBIT,
}
impl SSSDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSSDISW::STARTSTOPBIT => false,
            SSSDISW::NOSTARTSTOPBIT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSSDISW<'a> {
    w: &'a mut W,
}
impl<'a> _SSSDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSSDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Send start and stop bits as in other modes."]
    #[inline]
    pub fn startstopbit(self) -> &'a mut W {
        self.variant(SSSDISW::STARTSTOPBIT)
    }
    #[doc = "Do not send start/stop bits."]
    #[inline]
    pub fn nostartstopbit(self) -> &'a mut W {
        self.variant(SSSDISW::NOSTARTSTOPBIT)
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
#[doc = "Values that can be written to the field `CCCLR`"]
pub enum CCCLRW {
    #[doc = "CSCEN is under software control."]
    SW_CTRL,
    #[doc = "Hardware clears CSCEN after each character is received."]
    HW_CTRL,
}
impl CCCLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCCLRW::SW_CTRL => false,
            CCCLRW::HW_CTRL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CCCLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCCLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CSCEN is under software control."]
    #[inline]
    pub fn sw_ctrl(self) -> &'a mut W {
        self.variant(CCCLRW::SW_CTRL)
    }
    #[doc = "Hardware clears CSCEN after each character is received."]
    #[inline]
    pub fn hw_ctrl(self) -> &'a mut W {
        self.variant(CCCLRW::HW_CTRL)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Enables synchronous mode."]
    #[inline]
    pub fn sync(&self) -> SYNCR {
        SYNCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Clock source select."]
    #[inline]
    pub fn csrc(&self) -> CSRCR {
        CSRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Falling edge sampling."]
    #[inline]
    pub fn fes(&self) -> FESR {
        FESR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Transmit synchronization bypass in synchronous slave mode."]
    #[inline]
    pub fn tsbypass(&self) -> TSBYPASSR {
        TSBYPASSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Continuous master clock enable (used only when CSRC is 1)"]
    #[inline]
    pub fn cscen(&self) -> CSCENR {
        CSCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Start/stop bits"]
    #[inline]
    pub fn sssdis(&self) -> SSSDISR {
        SSSDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Continuous clock clear"]
    #[inline]
    pub fn ccclr(&self) -> CCCLRR {
        CCCLRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
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
    #[doc = "Bit 0 - Enables synchronous mode."]
    #[inline]
    pub fn sync(&mut self) -> _SYNCW {
        _SYNCW { w: self }
    }
    #[doc = "Bit 1 - Clock source select."]
    #[inline]
    pub fn csrc(&mut self) -> _CSRCW {
        _CSRCW { w: self }
    }
    #[doc = "Bit 2 - Falling edge sampling."]
    #[inline]
    pub fn fes(&mut self) -> _FESW {
        _FESW { w: self }
    }
    #[doc = "Bit 3 - Transmit synchronization bypass in synchronous slave mode."]
    #[inline]
    pub fn tsbypass(&mut self) -> _TSBYPASSW {
        _TSBYPASSW { w: self }
    }
    #[doc = "Bit 4 - Continuous master clock enable (used only when CSRC is 1)"]
    #[inline]
    pub fn cscen(&mut self) -> _CSCENW {
        _CSCENW { w: self }
    }
    #[doc = "Bit 5 - Start/stop bits"]
    #[inline]
    pub fn sssdis(&mut self) -> _SSSDISW {
        _SSSDISW { w: self }
    }
    #[doc = "Bit 6 - Continuous clock clear"]
    #[inline]
    pub fn ccclr(&mut self) -> _CCCLRW {
        _CCCLRW { w: self }
    }
}
