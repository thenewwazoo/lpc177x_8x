#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCS {
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
#[doc = "Possible values of the field `EMCSC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMCSCR {
    #[doc = "Static memory addresses are shifted to match the data bus width. For example, when accessing a 32-bit wide data bus, the address is shifted right 2 places such that bit 2 is the LSB. In this mode, address bit 0 for the this device is connected to address bit 0 of the memory device, thus simplifying memory connections. This also makes a larger memory address range possible, because additional upper address bits can appear on the higher address pins due to the shift."]
    MATCH_BUS_WIDTH,
    #[doc = "Static memory addresses are always output as byte addresses regardless of the data bus width. For example, when word data is accessed on a 32-bit bus, address bits 1 and 0 will always be 0. In this mode, one or both lower address bits may not be connected to memories that are part of a bus that is wider than 8 bits. This mode matches the operation of LPC23xx and LPC24xx devices."]
    ALWAYS_BYTE_ADDRS,
}
impl EMCSCR {
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
            EMCSCR::MATCH_BUS_WIDTH => false,
            EMCSCR::ALWAYS_BYTE_ADDRS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EMCSCR {
        match value {
            false => EMCSCR::MATCH_BUS_WIDTH,
            true => EMCSCR::ALWAYS_BYTE_ADDRS,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH_BUS_WIDTH`"]
    #[inline]
    pub fn is_match_bus_width(&self) -> bool {
        *self == EMCSCR::MATCH_BUS_WIDTH
    }
    #[doc = "Checks if the value of the field is `ALWAYS_BYTE_ADDRS`"]
    #[inline]
    pub fn is_always_byte_addrs(&self) -> bool {
        *self == EMCSCR::ALWAYS_BYTE_ADDRS
    }
}
#[doc = "Possible values of the field `EMCRD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMCRDR {
    #[doc = "Both EMC resets are asserted when any type of chip reset event occurs. In this mode, all registers and functions of the EMC are initialized upon any reset condition."]
    RESET_EMC,
    #[doc = "Many portions of the EMC are only reset by a power-on or brown-out event, in order to allow the EMC to retain its state through a warm reset (external reset or watchdog reset). If the EMC is configured correctly, auto-refresh can be maintained through a warm reset."]
    MAINTAIN_STATE,
}
impl EMCRDR {
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
            EMCRDR::RESET_EMC => false,
            EMCRDR::MAINTAIN_STATE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EMCRDR {
        match value {
            false => EMCRDR::RESET_EMC,
            true => EMCRDR::MAINTAIN_STATE,
        }
    }
    #[doc = "Checks if the value of the field is `RESET_EMC`"]
    #[inline]
    pub fn is_reset_emc(&self) -> bool {
        *self == EMCRDR::RESET_EMC
    }
    #[doc = "Checks if the value of the field is `MAINTAIN_STATE`"]
    #[inline]
    pub fn is_maintain_state(&self) -> bool {
        *self == EMCRDR::MAINTAIN_STATE
    }
}
#[doc = "Possible values of the field `EMCBC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMCBCR {
    #[doc = "Burst enabled."]
    BURST_ENABLED,
    #[doc = "Burst disabled. This mode can be used to prevent multiple sequential accesses to memory mapped I/O devices connected to EMC static memory chip selects. These unrequested accesses can cause issues with some I/O devices."]
    BURST_DISABLED,
}
impl EMCBCR {
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
            EMCBCR::BURST_ENABLED => false,
            EMCBCR::BURST_DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EMCBCR {
        match value {
            false => EMCBCR::BURST_ENABLED,
            true => EMCBCR::BURST_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `BURST_ENABLED`"]
    #[inline]
    pub fn is_burst_enabled(&self) -> bool {
        *self == EMCBCR::BURST_ENABLED
    }
    #[doc = "Checks if the value of the field is `BURST_DISABLED`"]
    #[inline]
    pub fn is_burst_disabled(&self) -> bool {
        *self == EMCBCR::BURST_DISABLED
    }
}
#[doc = "Possible values of the field `MCIPWRAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCIPWRALR {
    #[doc = "SD_PWR is active low (inverted output of the SD Card interface block)."]
    ACTIVE_LOW,
    #[doc = "SD_PWR is active high (follows the output of the SD Card interface block)."]
    ACTIVE_HIGH,
}
impl MCIPWRALR {
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
            MCIPWRALR::ACTIVE_LOW => false,
            MCIPWRALR::ACTIVE_HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MCIPWRALR {
        match value {
            false => MCIPWRALR::ACTIVE_LOW,
            true => MCIPWRALR::ACTIVE_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE_LOW`"]
    #[inline]
    pub fn is_active_low(&self) -> bool {
        *self == MCIPWRALR::ACTIVE_LOW
    }
    #[doc = "Checks if the value of the field is `ACTIVE_HIGH`"]
    #[inline]
    pub fn is_active_high(&self) -> bool {
        *self == MCIPWRALR::ACTIVE_HIGH
    }
}
#[doc = "Possible values of the field `OSCRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCRSR {
    #[doc = "The frequency range of the main oscillator is 1 MHz to 20 MHz."]
    _1_TO_20_MHZ,
    #[doc = "The frequency range of the main oscillator is 15 MHz to 25 MHz."]
    _15_TO_25_MHZ,
}
impl OSCRSR {
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
            OSCRSR::_1_TO_20_MHZ => false,
            OSCRSR::_15_TO_25_MHZ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OSCRSR {
        match value {
            false => OSCRSR::_1_TO_20_MHZ,
            true => OSCRSR::_15_TO_25_MHZ,
        }
    }
    #[doc = "Checks if the value of the field is `_1_TO_20_MHZ`"]
    #[inline]
    pub fn is_1_to_20_mhz(&self) -> bool {
        *self == OSCRSR::_1_TO_20_MHZ
    }
    #[doc = "Checks if the value of the field is `_15_TO_25_MHZ`"]
    #[inline]
    pub fn is_15_to_25_mhz(&self) -> bool {
        *self == OSCRSR::_15_TO_25_MHZ
    }
}
#[doc = "Possible values of the field `OSCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCENR {
    #[doc = "The main oscillator is disabled."]
    DISABLED,
    #[doc = "The main oscillator is enabled, and will start up if the correct external circuitry is connected to the XTAL1 and XTAL2 pins."]
    ENABLED,
}
impl OSCENR {
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
            OSCENR::DISABLED => false,
            OSCENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OSCENR {
        match value {
            false => OSCENR::DISABLED,
            true => OSCENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == OSCENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == OSCENR::ENABLED
    }
}
#[doc = "Possible values of the field `OSCSTAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCSTATR {
    #[doc = "The main oscillator is not ready to be used as a clock source."]
    NOT_READY,
    #[doc = "The main oscillator is ready to be used as a clock source. The main oscillator must be enabled via the OSCEN bit."]
    READY,
}
impl OSCSTATR {
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
            OSCSTATR::NOT_READY => false,
            OSCSTATR::READY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OSCSTATR {
        match value {
            false => OSCSTATR::NOT_READY,
            true => OSCSTATR::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_READY`"]
    #[inline]
    pub fn is_not_ready(&self) -> bool {
        *self == OSCSTATR::NOT_READY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline]
    pub fn is_ready(&self) -> bool {
        *self == OSCSTATR::READY
    }
}
#[doc = "Values that can be written to the field `EMCSC`"]
pub enum EMCSCW {
    #[doc = "Static memory addresses are shifted to match the data bus width. For example, when accessing a 32-bit wide data bus, the address is shifted right 2 places such that bit 2 is the LSB. In this mode, address bit 0 for the this device is connected to address bit 0 of the memory device, thus simplifying memory connections. This also makes a larger memory address range possible, because additional upper address bits can appear on the higher address pins due to the shift."]
    MATCH_BUS_WIDTH,
    #[doc = "Static memory addresses are always output as byte addresses regardless of the data bus width. For example, when word data is accessed on a 32-bit bus, address bits 1 and 0 will always be 0. In this mode, one or both lower address bits may not be connected to memories that are part of a bus that is wider than 8 bits. This mode matches the operation of LPC23xx and LPC24xx devices."]
    ALWAYS_BYTE_ADDRS,
}
impl EMCSCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EMCSCW::MATCH_BUS_WIDTH => false,
            EMCSCW::ALWAYS_BYTE_ADDRS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EMCSCW<'a> {
    w: &'a mut W,
}
impl<'a> _EMCSCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EMCSCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Static memory addresses are shifted to match the data bus width. For example, when accessing a 32-bit wide data bus, the address is shifted right 2 places such that bit 2 is the LSB. In this mode, address bit 0 for the this device is connected to address bit 0 of the memory device, thus simplifying memory connections. This also makes a larger memory address range possible, because additional upper address bits can appear on the higher address pins due to the shift."]
    #[inline]
    pub fn match_bus_width(self) -> &'a mut W {
        self.variant(EMCSCW::MATCH_BUS_WIDTH)
    }
    #[doc = "Static memory addresses are always output as byte addresses regardless of the data bus width. For example, when word data is accessed on a 32-bit bus, address bits 1 and 0 will always be 0. In this mode, one or both lower address bits may not be connected to memories that are part of a bus that is wider than 8 bits. This mode matches the operation of LPC23xx and LPC24xx devices."]
    #[inline]
    pub fn always_byte_addrs(self) -> &'a mut W {
        self.variant(EMCSCW::ALWAYS_BYTE_ADDRS)
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
#[doc = "Values that can be written to the field `EMCRD`"]
pub enum EMCRDW {
    #[doc = "Both EMC resets are asserted when any type of chip reset event occurs. In this mode, all registers and functions of the EMC are initialized upon any reset condition."]
    RESET_EMC,
    #[doc = "Many portions of the EMC are only reset by a power-on or brown-out event, in order to allow the EMC to retain its state through a warm reset (external reset or watchdog reset). If the EMC is configured correctly, auto-refresh can be maintained through a warm reset."]
    MAINTAIN_STATE,
}
impl EMCRDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EMCRDW::RESET_EMC => false,
            EMCRDW::MAINTAIN_STATE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EMCRDW<'a> {
    w: &'a mut W,
}
impl<'a> _EMCRDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EMCRDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Both EMC resets are asserted when any type of chip reset event occurs. In this mode, all registers and functions of the EMC are initialized upon any reset condition."]
    #[inline]
    pub fn reset_emc(self) -> &'a mut W {
        self.variant(EMCRDW::RESET_EMC)
    }
    #[doc = "Many portions of the EMC are only reset by a power-on or brown-out event, in order to allow the EMC to retain its state through a warm reset (external reset or watchdog reset). If the EMC is configured correctly, auto-refresh can be maintained through a warm reset."]
    #[inline]
    pub fn maintain_state(self) -> &'a mut W {
        self.variant(EMCRDW::MAINTAIN_STATE)
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
#[doc = "Values that can be written to the field `EMCBC`"]
pub enum EMCBCW {
    #[doc = "Burst enabled."]
    BURST_ENABLED,
    #[doc = "Burst disabled. This mode can be used to prevent multiple sequential accesses to memory mapped I/O devices connected to EMC static memory chip selects. These unrequested accesses can cause issues with some I/O devices."]
    BURST_DISABLED,
}
impl EMCBCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EMCBCW::BURST_ENABLED => false,
            EMCBCW::BURST_DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EMCBCW<'a> {
    w: &'a mut W,
}
impl<'a> _EMCBCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EMCBCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Burst enabled."]
    #[inline]
    pub fn burst_enabled(self) -> &'a mut W {
        self.variant(EMCBCW::BURST_ENABLED)
    }
    #[doc = "Burst disabled. This mode can be used to prevent multiple sequential accesses to memory mapped I/O devices connected to EMC static memory chip selects. These unrequested accesses can cause issues with some I/O devices."]
    #[inline]
    pub fn burst_disabled(self) -> &'a mut W {
        self.variant(EMCBCW::BURST_DISABLED)
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
#[doc = "Values that can be written to the field `MCIPWRAL`"]
pub enum MCIPWRALW {
    #[doc = "SD_PWR is active low (inverted output of the SD Card interface block)."]
    ACTIVE_LOW,
    #[doc = "SD_PWR is active high (follows the output of the SD Card interface block)."]
    ACTIVE_HIGH,
}
impl MCIPWRALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MCIPWRALW::ACTIVE_LOW => false,
            MCIPWRALW::ACTIVE_HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCIPWRALW<'a> {
    w: &'a mut W,
}
impl<'a> _MCIPWRALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCIPWRALW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SD_PWR is active low (inverted output of the SD Card interface block)."]
    #[inline]
    pub fn active_low(self) -> &'a mut W {
        self.variant(MCIPWRALW::ACTIVE_LOW)
    }
    #[doc = "SD_PWR is active high (follows the output of the SD Card interface block)."]
    #[inline]
    pub fn active_high(self) -> &'a mut W {
        self.variant(MCIPWRALW::ACTIVE_HIGH)
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
#[doc = "Values that can be written to the field `OSCRS`"]
pub enum OSCRSW {
    #[doc = "The frequency range of the main oscillator is 1 MHz to 20 MHz."]
    _1_TO_20_MHZ,
    #[doc = "The frequency range of the main oscillator is 15 MHz to 25 MHz."]
    _15_TO_25_MHZ,
}
impl OSCRSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OSCRSW::_1_TO_20_MHZ => false,
            OSCRSW::_15_TO_25_MHZ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSCRSW<'a> {
    w: &'a mut W,
}
impl<'a> _OSCRSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSCRSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The frequency range of the main oscillator is 1 MHz to 20 MHz."]
    #[inline]
    pub fn _1_to_20_mhz(self) -> &'a mut W {
        self.variant(OSCRSW::_1_TO_20_MHZ)
    }
    #[doc = "The frequency range of the main oscillator is 15 MHz to 25 MHz."]
    #[inline]
    pub fn _15_to_25_mhz(self) -> &'a mut W {
        self.variant(OSCRSW::_15_TO_25_MHZ)
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
#[doc = "Values that can be written to the field `OSCEN`"]
pub enum OSCENW {
    #[doc = "The main oscillator is disabled."]
    DISABLED,
    #[doc = "The main oscillator is enabled, and will start up if the correct external circuitry is connected to the XTAL1 and XTAL2 pins."]
    ENABLED,
}
impl OSCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OSCENW::DISABLED => false,
            OSCENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSCENW<'a> {
    w: &'a mut W,
}
impl<'a> _OSCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The main oscillator is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OSCENW::DISABLED)
    }
    #[doc = "The main oscillator is enabled, and will start up if the correct external circuitry is connected to the XTAL1 and XTAL2 pins."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OSCENW::ENABLED)
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
#[doc = "Values that can be written to the field `OSCSTAT`"]
pub enum OSCSTATW {
    #[doc = "The main oscillator is not ready to be used as a clock source."]
    NOT_READY,
    #[doc = "The main oscillator is ready to be used as a clock source. The main oscillator must be enabled via the OSCEN bit."]
    READY,
}
impl OSCSTATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OSCSTATW::NOT_READY => false,
            OSCSTATW::READY => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSCSTATW<'a> {
    w: &'a mut W,
}
impl<'a> _OSCSTATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSCSTATW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The main oscillator is not ready to be used as a clock source."]
    #[inline]
    pub fn not_ready(self) -> &'a mut W {
        self.variant(OSCSTATW::NOT_READY)
    }
    #[doc = "The main oscillator is ready to be used as a clock source. The main oscillator must be enabled via the OSCEN bit."]
    #[inline]
    pub fn ready(self) -> &'a mut W {
        self.variant(OSCSTATW::READY)
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
    #[doc = "Bit 0 - EMC Shift Control. Controls how addresses are output on the EMC address pins for static memories. Also see Section 10.9 in the EMC chapter."]
    #[inline]
    pub fn emcsc(&self) -> EMCSCR {
        EMCSCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - EMC Reset Disable[1]. External Memory Controller Reset Disable. Also see Section 10.8 in the EMC chapter."]
    #[inline]
    pub fn emcrd(&self) -> EMCRDR {
        EMCRDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - External Memory Controller burst control. Also see Section 10.10 in the EMC chapter."]
    #[inline]
    pub fn emcbc(&self) -> EMCBCR {
        EMCBCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - MCIPWR Active Level[1]. Selects the active level of the SD card interface signal SD_PWR."]
    #[inline]
    pub fn mcipwral(&self) -> MCIPWRALR {
        MCIPWRALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Main oscillator range select."]
    #[inline]
    pub fn oscrs(&self) -> OSCRSR {
        OSCRSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Main oscillator enable."]
    #[inline]
    pub fn oscen(&self) -> OSCENR {
        OSCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Main oscillator status."]
    #[inline]
    pub fn oscstat(&self) -> OSCSTATR {
        OSCSTATR::_from({
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
    #[doc = "Bit 0 - EMC Shift Control. Controls how addresses are output on the EMC address pins for static memories. Also see Section 10.9 in the EMC chapter."]
    #[inline]
    pub fn emcsc(&mut self) -> _EMCSCW {
        _EMCSCW { w: self }
    }
    #[doc = "Bit 1 - EMC Reset Disable[1]. External Memory Controller Reset Disable. Also see Section 10.8 in the EMC chapter."]
    #[inline]
    pub fn emcrd(&mut self) -> _EMCRDW {
        _EMCRDW { w: self }
    }
    #[doc = "Bit 2 - External Memory Controller burst control. Also see Section 10.10 in the EMC chapter."]
    #[inline]
    pub fn emcbc(&mut self) -> _EMCBCW {
        _EMCBCW { w: self }
    }
    #[doc = "Bit 3 - MCIPWR Active Level[1]. Selects the active level of the SD card interface signal SD_PWR."]
    #[inline]
    pub fn mcipwral(&mut self) -> _MCIPWRALW {
        _MCIPWRALW { w: self }
    }
    #[doc = "Bit 4 - Main oscillator range select."]
    #[inline]
    pub fn oscrs(&mut self) -> _OSCRSW {
        _OSCRSW { w: self }
    }
    #[doc = "Bit 5 - Main oscillator enable."]
    #[inline]
    pub fn oscen(&mut self) -> _OSCENW {
        _OSCENW { w: self }
    }
    #[doc = "Bit 6 - Main oscillator status."]
    #[inline]
    pub fn oscstat(&mut self) -> _OSCSTATW {
        _OSCSTATW { w: self }
    }
}
