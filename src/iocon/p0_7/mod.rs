#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::P0_7 {
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
    P0_7,
    #[doc = "I2S transmit clock. It                                             is driven by the master and received by the slave.                                             Corresponds to the signal SCK in the                                                   I2S-bus                                                 specification."]
    I2S_TX_SCK,
    #[doc = "Serial Clock for SSP1."]
    SSP1_SCK,
    #[doc = "Match output for Timer 2, channel 1."]
    T2_MAT1,
    #[doc = "Event input 0 to Event                                             Monitor/Recorder."]
    RTC_EV0,
    #[doc = "LCD data."]
    LCD_VD_9,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FUNCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FUNCR::P0_7 => 0,
            FUNCR::I2S_TX_SCK => 1,
            FUNCR::SSP1_SCK => 2,
            FUNCR::T2_MAT1 => 3,
            FUNCR::RTC_EV0 => 4,
            FUNCR::LCD_VD_9 => 7,
            FUNCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FUNCR {
        match value {
            0 => FUNCR::P0_7,
            1 => FUNCR::I2S_TX_SCK,
            2 => FUNCR::SSP1_SCK,
            3 => FUNCR::T2_MAT1,
            4 => FUNCR::RTC_EV0,
            7 => FUNCR::LCD_VD_9,
            i => FUNCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `P0_7`"]
    #[inline]
    pub fn is_p0_7(&self) -> bool {
        *self == FUNCR::P0_7
    }
    #[doc = "Checks if the value of the field is `I2S_TX_SCK`"]
    #[inline]
    pub fn is_i2s_tx_sck(&self) -> bool {
        *self == FUNCR::I2S_TX_SCK
    }
    #[doc = "Checks if the value of the field is `SSP1_SCK`"]
    #[inline]
    pub fn is_ssp1_sck(&self) -> bool {
        *self == FUNCR::SSP1_SCK
    }
    #[doc = "Checks if the value of the field is `T2_MAT1`"]
    #[inline]
    pub fn is_t2_mat1(&self) -> bool {
        *self == FUNCR::T2_MAT1
    }
    #[doc = "Checks if the value of the field is `RTC_EV0`"]
    #[inline]
    pub fn is_rtc_ev0(&self) -> bool {
        *self == FUNCR::RTC_EV0
    }
    #[doc = "Checks if the value of the field is `LCD_VD_9`"]
    #[inline]
    pub fn is_lcd_vd_9(&self) -> bool {
        *self == FUNCR::LCD_VD_9
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Inactive (no pull-down/pull-up resistor\n                                        enabled)."]
    INACTIVE,
    #[doc = "Pull-down resistor enabled."]
    PULLDOWN_EN,
    #[doc = "Pull-up resistor enabled."]
    PULLUP_EN,
    #[doc = "Repeater mode."]
    REPEATER_MODE,
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::INACTIVE => 0,
            MODER::PULLDOWN_EN => 1,
            MODER::PULLUP_EN => 2,
            MODER::REPEATER_MODE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::INACTIVE,
            1 => MODER::PULLDOWN_EN,
            2 => MODER::PULLUP_EN,
            3 => MODER::REPEATER_MODE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline]
    pub fn is_inactive(&self) -> bool {
        *self == MODER::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PULLDOWN_EN`"]
    #[inline]
    pub fn is_pulldown_en(&self) -> bool {
        *self == MODER::PULLDOWN_EN
    }
    #[doc = "Checks if the value of the field is `PULLUP_EN`"]
    #[inline]
    pub fn is_pullup_en(&self) -> bool {
        *self == MODER::PULLUP_EN
    }
    #[doc = "Checks if the value of the field is `REPEATER_MODE`"]
    #[inline]
    pub fn is_repeater_mode(&self) -> bool {
        *self == MODER::REPEATER_MODE
    }
}
#[doc = "Possible values of the field `HYS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYSR {
    #[doc = "Disable."]
    DISABLE,
    #[doc = "Enable."]
    ENABLE,
}
impl HYSR {
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
            HYSR::DISABLE => false,
            HYSR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HYSR {
        match value {
            false => HYSR::DISABLE,
            true => HYSR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == HYSR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == HYSR::ENABLE
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
#[doc = "Possible values of the field `FILTER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTERR {
    #[doc = "Noise pulses below approximately 10 ns are filtered\n                                        out."]
    ENABLED,
    #[doc = "No input filtering is done."]
    DISABLED,
}
impl FILTERR {
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
            FILTERR::ENABLED => false,
            FILTERR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FILTERR {
        match value {
            false => FILTERR::ENABLED,
            true => FILTERR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == FILTERR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == FILTERR::DISABLED
    }
}
#[doc = "Possible values of the field `SLEW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEWR {
    #[doc = "Standard mode, output slew rate control is enabled. More\n                                        outputs can be switched simultaneously."]
    STANDARD,
    #[doc = "Fast mode, slew rate control is disabled. Refer to the\n                                        appropriate specific device data sheet for details."]
    FAST,
}
impl SLEWR {
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
            SLEWR::STANDARD => false,
            SLEWR::FAST => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLEWR {
        match value {
            false => SLEWR::STANDARD,
            true => SLEWR::FAST,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline]
    pub fn is_standard(&self) -> bool {
        *self == SLEWR::STANDARD
    }
    #[doc = "Checks if the value of the field is `FAST`"]
    #[inline]
    pub fn is_fast(&self) -> bool {
        *self == SLEWR::FAST
    }
}
#[doc = "Possible values of the field `OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ODR {
    #[doc = "Disable."]
    DISABLE,
    #[doc = "Open-drain mode enabled. This is not a true open-drain\n                                        mode. Input cannot be pulled up above VDD."]
    ENABLED,
}
impl ODR {
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
            ODR::DISABLE => false,
            ODR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ODR {
        match value {
            false => ODR::DISABLE,
            true => ODR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ODR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ODR::ENABLED
    }
}
#[doc = "Values that can be written to the field `FUNC`"]
pub enum FUNCW {
    #[doc = "General purpose digital input/output\n                                        pin."]
    P0_7,
    #[doc = "I2S transmit clock. It                                             is driven by the master and received by the slave.                                             Corresponds to the signal SCK in the                                                   I2S-bus                                                 specification."]
    I2S_TX_SCK,
    #[doc = "Serial Clock for SSP1."]
    SSP1_SCK,
    #[doc = "Match output for Timer 2, channel 1."]
    T2_MAT1,
    #[doc = "Event input 0 to Event                                             Monitor/Recorder."]
    RTC_EV0,
    #[doc = "LCD data."]
    LCD_VD_9,
}
impl FUNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FUNCW::P0_7 => 0,
            FUNCW::I2S_TX_SCK => 1,
            FUNCW::SSP1_SCK => 2,
            FUNCW::T2_MAT1 => 3,
            FUNCW::RTC_EV0 => 4,
            FUNCW::LCD_VD_9 => 7,
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
    pub fn p0_7(self) -> &'a mut W {
        self.variant(FUNCW::P0_7)
    }
    #[doc = "I2S transmit clock. It is driven by the master and received by the slave. Corresponds to the signal SCK in the I2S-bus specification."]
    #[inline]
    pub fn i2s_tx_sck(self) -> &'a mut W {
        self.variant(FUNCW::I2S_TX_SCK)
    }
    #[doc = "Serial Clock for SSP1."]
    #[inline]
    pub fn ssp1_sck(self) -> &'a mut W {
        self.variant(FUNCW::SSP1_SCK)
    }
    #[doc = "Match output for Timer 2, channel 1."]
    #[inline]
    pub fn t2_mat1(self) -> &'a mut W {
        self.variant(FUNCW::T2_MAT1)
    }
    #[doc = "Event input 0 to Event Monitor/Recorder."]
    #[inline]
    pub fn rtc_ev0(self) -> &'a mut W {
        self.variant(FUNCW::RTC_EV0)
    }
    #[doc = "LCD data."]
    #[inline]
    pub fn lcd_vd_9(self) -> &'a mut W {
        self.variant(FUNCW::LCD_VD_9)
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
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "Inactive (no pull-down/pull-up resistor\n                                        enabled)."]
    INACTIVE,
    #[doc = "Pull-down resistor enabled."]
    PULLDOWN_EN,
    #[doc = "Pull-up resistor enabled."]
    PULLUP_EN,
    #[doc = "Repeater mode."]
    REPEATER_MODE,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::INACTIVE => 0,
            MODEW::PULLDOWN_EN => 1,
            MODEW::PULLUP_EN => 2,
            MODEW::REPEATER_MODE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Inactive (no pull-down/pull-up resistor enabled)."]
    #[inline]
    pub fn inactive(self) -> &'a mut W {
        self.variant(MODEW::INACTIVE)
    }
    #[doc = "Pull-down resistor enabled."]
    #[inline]
    pub fn pulldown_en(self) -> &'a mut W {
        self.variant(MODEW::PULLDOWN_EN)
    }
    #[doc = "Pull-up resistor enabled."]
    #[inline]
    pub fn pullup_en(self) -> &'a mut W {
        self.variant(MODEW::PULLUP_EN)
    }
    #[doc = "Repeater mode."]
    #[inline]
    pub fn repeater_mode(self) -> &'a mut W {
        self.variant(MODEW::REPEATER_MODE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HYS`"]
pub enum HYSW {
    #[doc = "Disable."]
    DISABLE,
    #[doc = "Enable."]
    ENABLE,
}
impl HYSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HYSW::DISABLE => false,
            HYSW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HYSW<'a> {
    w: &'a mut W,
}
impl<'a> _HYSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HYSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(HYSW::DISABLE)
    }
    #[doc = "Enable."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(HYSW::ENABLE)
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
#[doc = "Values that can be written to the field `FILTER`"]
pub enum FILTERW {
    #[doc = "Noise pulses below approximately 10 ns are filtered\n                                        out."]
    ENABLED,
    #[doc = "No input filtering is done."]
    DISABLED,
}
impl FILTERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FILTERW::ENABLED => false,
            FILTERW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FILTERW<'a> {
    w: &'a mut W,
}
impl<'a> _FILTERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FILTERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Noise pulses below approximately 10 ns are filtered out."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FILTERW::ENABLED)
    }
    #[doc = "No input filtering is done."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FILTERW::DISABLED)
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
#[doc = "Values that can be written to the field `SLEW`"]
pub enum SLEWW {
    #[doc = "Standard mode, output slew rate control is enabled. More\n                                        outputs can be switched simultaneously."]
    STANDARD,
    #[doc = "Fast mode, slew rate control is disabled. Refer to the\n                                        appropriate specific device data sheet for details."]
    FAST,
}
impl SLEWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLEWW::STANDARD => false,
            SLEWW::FAST => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLEWW<'a> {
    w: &'a mut W,
}
impl<'a> _SLEWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLEWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Standard mode, output slew rate control is enabled. More outputs can be switched simultaneously."]
    #[inline]
    pub fn standard(self) -> &'a mut W {
        self.variant(SLEWW::STANDARD)
    }
    #[doc = "Fast mode, slew rate control is disabled. Refer to the appropriate specific device data sheet for details."]
    #[inline]
    pub fn fast(self) -> &'a mut W {
        self.variant(SLEWW::FAST)
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
#[doc = "Values that can be written to the field `OD`"]
pub enum ODW {
    #[doc = "Disable."]
    DISABLE,
    #[doc = "Open-drain mode enabled. This is not a true open-drain\n                                        mode. Input cannot be pulled up above VDD."]
    ENABLED,
}
impl ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ODW::DISABLE => false,
            ODW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ODW<'a> {
    w: &'a mut W,
}
impl<'a> _ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ODW::DISABLE)
    }
    #[doc = "Open-drain mode enabled. This is not a true open-drain mode. Input cannot be pulled up above VDD."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ODW::ENABLED)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Selects pin function for pin P0[7]"]
    #[inline]
    pub fn func(&self) -> FUNCR {
        FUNCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:4 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 5 - Hysteresis."]
    #[inline]
    pub fn hys(&self) -> HYSR {
        HYSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bit 8 - Glitch filter control"]
    #[inline]
    pub fn filter(&self) -> FILTERR {
        FILTERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Driver slew rate"]
    #[inline]
    pub fn slew(&self) -> SLEWR {
        SLEWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Open-drain mode."]
    #[inline]
    pub fn od(&self) -> ODR {
        ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 160 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Selects pin function for pin P0[7]"]
    #[inline]
    pub fn func(&mut self) -> _FUNCW {
        _FUNCW { w: self }
    }
    #[doc = "Bits 3:4 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 5 - Hysteresis."]
    #[inline]
    pub fn hys(&mut self) -> _HYSW {
        _HYSW { w: self }
    }
    #[doc = "Bit 6 - Invert input"]
    #[inline]
    pub fn inv(&mut self) -> _INVW {
        _INVW { w: self }
    }
    #[doc = "Bit 8 - Glitch filter control"]
    #[inline]
    pub fn filter(&mut self) -> _FILTERW {
        _FILTERW { w: self }
    }
    #[doc = "Bit 9 - Driver slew rate"]
    #[inline]
    pub fn slew(&mut self) -> _SLEWW {
        _SLEWW { w: self }
    }
    #[doc = "Bit 10 - Open-drain mode."]
    #[inline]
    pub fn od(&mut self) -> _ODW {
        _ODW { w: self }
    }
}
