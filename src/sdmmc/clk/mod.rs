#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLK {
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
#[doc = r" Value of the field"]
pub struct CLKDIVR {
    bits: u8,
}
impl CLKDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLER {
    #[doc = "Clock disabled."]
    CLOCK_DISABLED,
    #[doc = "Clock enabled."]
    CLOCK_ENABLED,
}
impl ENABLER {
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
            ENABLER::CLOCK_DISABLED => false,
            ENABLER::CLOCK_ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENABLER {
        match value {
            false => ENABLER::CLOCK_DISABLED,
            true => ENABLER::CLOCK_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `CLOCK_DISABLED`"]
    #[inline]
    pub fn is_clock_disabled(&self) -> bool {
        *self == ENABLER::CLOCK_DISABLED
    }
    #[doc = "Checks if the value of the field is `CLOCK_ENABLED`"]
    #[inline]
    pub fn is_clock_enabled(&self) -> bool {
        *self == ENABLER::CLOCK_ENABLED
    }
}
#[doc = "Possible values of the field `PWRSAVE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRSAVER {
    #[doc = "Always enabled."]
    ALWAYS_ENABLED,
    #[doc = "Clock enabled when bus is active."]
    WHEN_ACTIVE,
}
impl PWRSAVER {
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
            PWRSAVER::ALWAYS_ENABLED => false,
            PWRSAVER::WHEN_ACTIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWRSAVER {
        match value {
            false => PWRSAVER::ALWAYS_ENABLED,
            true => PWRSAVER::WHEN_ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ALWAYS_ENABLED`"]
    #[inline]
    pub fn is_always_enabled(&self) -> bool {
        *self == PWRSAVER::ALWAYS_ENABLED
    }
    #[doc = "Checks if the value of the field is `WHEN_ACTIVE`"]
    #[inline]
    pub fn is_when_active(&self) -> bool {
        *self == PWRSAVER::WHEN_ACTIVE
    }
}
#[doc = "Possible values of the field `BYPASS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASSR {
    #[doc = "Disable bypass."]
    DISABLE,
    #[doc = "Enable bypass. MCLK driven to card bus output (SD_CLK)."]
    ENABLE,
}
impl BYPASSR {
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
            BYPASSR::DISABLE => false,
            BYPASSR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BYPASSR {
        match value {
            false => BYPASSR::DISABLE,
            true => BYPASSR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == BYPASSR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == BYPASSR::ENABLE
    }
}
#[doc = "Possible values of the field `WIDEBUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WIDEBUSR {
    #[doc = "Standard bus mode (only SD_DAT[0] used)."]
    STANDARD_BUS_MODE,
    #[doc = "Wide bus mode (SD_DAT[3:0] used)"]
    WIDE_BUS_MODE,
}
impl WIDEBUSR {
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
            WIDEBUSR::STANDARD_BUS_MODE => false,
            WIDEBUSR::WIDE_BUS_MODE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WIDEBUSR {
        match value {
            false => WIDEBUSR::STANDARD_BUS_MODE,
            true => WIDEBUSR::WIDE_BUS_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD_BUS_MODE`"]
    #[inline]
    pub fn is_standard_bus_mode(&self) -> bool {
        *self == WIDEBUSR::STANDARD_BUS_MODE
    }
    #[doc = "Checks if the value of the field is `WIDE_BUS_MODE`"]
    #[inline]
    pub fn is_wide_bus_mode(&self) -> bool {
        *self == WIDEBUSR::WIDE_BUS_MODE
    }
}
#[doc = r" Proxy"]
pub struct _CLKDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKDIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ENABLE`"]
pub enum ENABLEW {
    #[doc = "Clock disabled."]
    CLOCK_DISABLED,
    #[doc = "Clock enabled."]
    CLOCK_ENABLED,
}
impl ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENABLEW::CLOCK_DISABLED => false,
            ENABLEW::CLOCK_ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled."]
    #[inline]
    pub fn clock_disabled(self) -> &'a mut W {
        self.variant(ENABLEW::CLOCK_DISABLED)
    }
    #[doc = "Clock enabled."]
    #[inline]
    pub fn clock_enabled(self) -> &'a mut W {
        self.variant(ENABLEW::CLOCK_ENABLED)
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
#[doc = "Values that can be written to the field `PWRSAVE`"]
pub enum PWRSAVEW {
    #[doc = "Always enabled."]
    ALWAYS_ENABLED,
    #[doc = "Clock enabled when bus is active."]
    WHEN_ACTIVE,
}
impl PWRSAVEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWRSAVEW::ALWAYS_ENABLED => false,
            PWRSAVEW::WHEN_ACTIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWRSAVEW<'a> {
    w: &'a mut W,
}
impl<'a> _PWRSAVEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWRSAVEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Always enabled."]
    #[inline]
    pub fn always_enabled(self) -> &'a mut W {
        self.variant(PWRSAVEW::ALWAYS_ENABLED)
    }
    #[doc = "Clock enabled when bus is active."]
    #[inline]
    pub fn when_active(self) -> &'a mut W {
        self.variant(PWRSAVEW::WHEN_ACTIVE)
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
#[doc = "Values that can be written to the field `BYPASS`"]
pub enum BYPASSW {
    #[doc = "Disable bypass."]
    DISABLE,
    #[doc = "Enable bypass. MCLK driven to card bus output (SD_CLK)."]
    ENABLE,
}
impl BYPASSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BYPASSW::DISABLE => false,
            BYPASSW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _BYPASSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BYPASSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable bypass."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(BYPASSW::DISABLE)
    }
    #[doc = "Enable bypass. MCLK driven to card bus output (SD_CLK)."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(BYPASSW::ENABLE)
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
#[doc = "Values that can be written to the field `WIDEBUS`"]
pub enum WIDEBUSW {
    #[doc = "Standard bus mode (only SD_DAT[0] used)."]
    STANDARD_BUS_MODE,
    #[doc = "Wide bus mode (SD_DAT[3:0] used)"]
    WIDE_BUS_MODE,
}
impl WIDEBUSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WIDEBUSW::STANDARD_BUS_MODE => false,
            WIDEBUSW::WIDE_BUS_MODE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WIDEBUSW<'a> {
    w: &'a mut W,
}
impl<'a> _WIDEBUSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WIDEBUSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Standard bus mode (only SD_DAT[0] used)."]
    #[inline]
    pub fn standard_bus_mode(self) -> &'a mut W {
        self.variant(WIDEBUSW::STANDARD_BUS_MODE)
    }
    #[doc = "Wide bus mode (SD_DAT[3:0] used)"]
    #[inline]
    pub fn wide_bus_mode(self) -> &'a mut W {
        self.variant(WIDEBUSW::WIDE_BUS_MODE)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Bus clock period: SD_CLK frequency = MCLK / [2x(ClkDiv+1)]."]
    #[inline]
    pub fn clkdiv(&self) -> CLKDIVR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CLKDIVR { bits }
    }
    #[doc = "Bit 8 - Enable SD card bus clock:"]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Disable SD_CLK output when bus is idle:"]
    #[inline]
    pub fn pwrsave(&self) -> PWRSAVER {
        PWRSAVER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Enable bypass of clock divide logic:"]
    #[inline]
    pub fn bypass(&self) -> BYPASSR {
        BYPASSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Enable wide bus mode."]
    #[inline]
    pub fn widebus(&self) -> WIDEBUSR {
        WIDEBUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
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
    #[doc = "Bits 0:7 - Bus clock period: SD_CLK frequency = MCLK / [2x(ClkDiv+1)]."]
    #[inline]
    pub fn clkdiv(&mut self) -> _CLKDIVW {
        _CLKDIVW { w: self }
    }
    #[doc = "Bit 8 - Enable SD card bus clock:"]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
    #[doc = "Bit 9 - Disable SD_CLK output when bus is idle:"]
    #[inline]
    pub fn pwrsave(&mut self) -> _PWRSAVEW {
        _PWRSAVEW { w: self }
    }
    #[doc = "Bit 10 - Enable bypass of clock divide logic:"]
    #[inline]
    pub fn bypass(&mut self) -> _BYPASSW {
        _BYPASSW { w: self }
    }
    #[doc = "Bit 11 - Enable wide bus mode."]
    #[inline]
    pub fn widebus(&mut self) -> _WIDEBUSW {
        _WIDEBUSW { w: self }
    }
}
