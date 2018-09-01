#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TCR {
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
#[doc = "Possible values of the field `CE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CER {
    #[doc = "The PWM Timer Counter and PWM Prescale Counter are enabled for counting."]
    ENABLED,
    #[doc = "The counters are disabled."]
    DISABLED,
}
impl CER {
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
            CER::ENABLED => true,
            CER::DISABLED => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CER {
        match value {
            true => CER::ENABLED,
            false => CER::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CER::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CER::DISABLED
    }
}
#[doc = "Possible values of the field `CR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRR {
    #[doc = "The PWM Timer Counter and the PWM Prescale Counter are synchronously reset on the next positive edge of PCLK. The counters remain reset until this bit is returned to zero."]
    ON_PCLK_POS_EDGE,
    #[doc = "Clear reset."]
    CLEAR_RESET,
}
impl CRR {
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
            CRR::ON_PCLK_POS_EDGE => true,
            CRR::CLEAR_RESET => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRR {
        match value {
            true => CRR::ON_PCLK_POS_EDGE,
            false => CRR::CLEAR_RESET,
        }
    }
    #[doc = "Checks if the value of the field is `ON_PCLK_POS_EDGE`"]
    #[inline]
    pub fn is_on_pclk_pos_edge(&self) -> bool {
        *self == CRR::ON_PCLK_POS_EDGE
    }
    #[doc = "Checks if the value of the field is `CLEAR_RESET`"]
    #[inline]
    pub fn is_clear_reset(&self) -> bool {
        *self == CRR::CLEAR_RESET
    }
}
#[doc = "Possible values of the field `PWMEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMENR {
    #[doc = "PWM mode is enabled (counter resets to 1). PWM mode causes the shadow registers to operate in connection with the Match registers. A program write to a Match register will not have an effect on the Match result until the corresponding bit in PWMLER has been set, followed by the occurrence of a PWM Match 0 event. Note that the PWM Match register that determines the PWM rate (PWM Match Register 0 - MR0) must be set up prior to the PWM being enabled. Otherwise a Match event will not occur to cause shadow register contents to become effective."]
    PWM_MODE,
    #[doc = "Timer mode is enabled (counter resets to 0)."]
    TIMER_MODE,
}
impl PWMENR {
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
            PWMENR::PWM_MODE => true,
            PWMENR::TIMER_MODE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMENR {
        match value {
            true => PWMENR::PWM_MODE,
            false => PWMENR::TIMER_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `PWM_MODE`"]
    #[inline]
    pub fn is_pwm_mode(&self) -> bool {
        *self == PWMENR::PWM_MODE
    }
    #[doc = "Checks if the value of the field is `TIMER_MODE`"]
    #[inline]
    pub fn is_timer_mode(&self) -> bool {
        *self == PWMENR::TIMER_MODE
    }
}
#[doc = "Possible values of the field `MDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDISR {
    #[doc = "Master use. PWM0 is the master, and both PWMs are enabled for counting."]
    MASTER_USE,
    #[doc = "Individual use. The PWMs are used independently, and the individual Counter Enable bits are used to control the PWMs."]
    INDIVIDUAL_USE,
}
impl MDISR {
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
            MDISR::MASTER_USE => true,
            MDISR::INDIVIDUAL_USE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MDISR {
        match value {
            true => MDISR::MASTER_USE,
            false => MDISR::INDIVIDUAL_USE,
        }
    }
    #[doc = "Checks if the value of the field is `MASTER_USE`"]
    #[inline]
    pub fn is_master_use(&self) -> bool {
        *self == MDISR::MASTER_USE
    }
    #[doc = "Checks if the value of the field is `INDIVIDUAL_USE`"]
    #[inline]
    pub fn is_individual_use(&self) -> bool {
        *self == MDISR::INDIVIDUAL_USE
    }
}
#[doc = "Values that can be written to the field `CE`"]
pub enum CEW {
    #[doc = "The PWM Timer Counter and PWM Prescale Counter are enabled for counting."]
    ENABLED,
    #[doc = "The counters are disabled."]
    DISABLED,
}
impl CEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CEW::ENABLED => true,
            CEW::DISABLED => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEW<'a> {
    w: &'a mut W,
}
impl<'a> _CEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The PWM Timer Counter and PWM Prescale Counter are enabled for counting."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CEW::ENABLED)
    }
    #[doc = "The counters are disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CEW::DISABLED)
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
#[doc = "Values that can be written to the field `CR`"]
pub enum CRW {
    #[doc = "The PWM Timer Counter and the PWM Prescale Counter are synchronously reset on the next positive edge of PCLK. The counters remain reset until this bit is returned to zero."]
    ON_PCLK_POS_EDGE,
    #[doc = "Clear reset."]
    CLEAR_RESET,
}
impl CRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRW::ON_PCLK_POS_EDGE => true,
            CRW::CLEAR_RESET => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRW<'a> {
    w: &'a mut W,
}
impl<'a> _CRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The PWM Timer Counter and the PWM Prescale Counter are synchronously reset on the next positive edge of PCLK. The counters remain reset until this bit is returned to zero."]
    #[inline]
    pub fn on_pclk_pos_edge(self) -> &'a mut W {
        self.variant(CRW::ON_PCLK_POS_EDGE)
    }
    #[doc = "Clear reset."]
    #[inline]
    pub fn clear_reset(self) -> &'a mut W {
        self.variant(CRW::CLEAR_RESET)
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
#[doc = "Values that can be written to the field `PWMEN`"]
pub enum PWMENW {
    #[doc = "PWM mode is enabled (counter resets to 1). PWM mode causes the shadow registers to operate in connection with the Match registers. A program write to a Match register will not have an effect on the Match result until the corresponding bit in PWMLER has been set, followed by the occurrence of a PWM Match 0 event. Note that the PWM Match register that determines the PWM rate (PWM Match Register 0 - MR0) must be set up prior to the PWM being enabled. Otherwise a Match event will not occur to cause shadow register contents to become effective."]
    PWM_MODE,
    #[doc = "Timer mode is enabled (counter resets to 0)."]
    TIMER_MODE,
}
impl PWMENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMENW::PWM_MODE => true,
            PWMENW::TIMER_MODE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMENW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PWM mode is enabled (counter resets to 1). PWM mode causes the shadow registers to operate in connection with the Match registers. A program write to a Match register will not have an effect on the Match result until the corresponding bit in PWMLER has been set, followed by the occurrence of a PWM Match 0 event. Note that the PWM Match register that determines the PWM rate (PWM Match Register 0 - MR0) must be set up prior to the PWM being enabled. Otherwise a Match event will not occur to cause shadow register contents to become effective."]
    #[inline]
    pub fn pwm_mode(self) -> &'a mut W {
        self.variant(PWMENW::PWM_MODE)
    }
    #[doc = "Timer mode is enabled (counter resets to 0)."]
    #[inline]
    pub fn timer_mode(self) -> &'a mut W {
        self.variant(PWMENW::TIMER_MODE)
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
#[doc = "Values that can be written to the field `MDIS`"]
pub enum MDISW {
    #[doc = "Master use. PWM0 is the master, and both PWMs are enabled for counting."]
    MASTER_USE,
    #[doc = "Individual use. The PWMs are used independently, and the individual Counter Enable bits are used to control the PWMs."]
    INDIVIDUAL_USE,
}
impl MDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MDISW::MASTER_USE => true,
            MDISW::INDIVIDUAL_USE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MDISW<'a> {
    w: &'a mut W,
}
impl<'a> _MDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Master use. PWM0 is the master, and both PWMs are enabled for counting."]
    #[inline]
    pub fn master_use(self) -> &'a mut W {
        self.variant(MDISW::MASTER_USE)
    }
    #[doc = "Individual use. The PWMs are used independently, and the individual Counter Enable bits are used to control the PWMs."]
    #[inline]
    pub fn individual_use(self) -> &'a mut W {
        self.variant(MDISW::INDIVIDUAL_USE)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Counter Enable"]
    #[inline]
    pub fn ce(&self) -> CER {
        CER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Counter Reset"]
    #[inline]
    pub fn cr(&self) -> CRR {
        CRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - PWM Enable"]
    #[inline]
    pub fn pwmen(&self) -> PWMENR {
        PWMENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Master Disable (PWM0 only). The two PWMs may be synchronized using the Master Disable control bit. The Master disable bit of the Master PWM (PWM0 module) controls a secondary enable input to both PWMs, as shown in Figure 141. This bit has no function in the Slave PWM (PWM1)."]
    #[inline]
    pub fn mdis(&self) -> MDISR {
        MDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
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
    #[doc = "Bit 0 - Counter Enable"]
    #[inline]
    pub fn ce(&mut self) -> _CEW {
        _CEW { w: self }
    }
    #[doc = "Bit 1 - Counter Reset"]
    #[inline]
    pub fn cr(&mut self) -> _CRW {
        _CRW { w: self }
    }
    #[doc = "Bit 3 - PWM Enable"]
    #[inline]
    pub fn pwmen(&mut self) -> _PWMENW {
        _PWMENW { w: self }
    }
    #[doc = "Bit 4 - Master Disable (PWM0 only). The two PWMs may be synchronized using the Master Disable control bit. The Master disable bit of the Master PWM (PWM0 module) controls a secondary enable input to both PWMs, as shown in Figure 141. This bit has no function in the Slave PWM (PWM1)."]
    #[inline]
    pub fn mdis(&mut self) -> _MDISW {
        _MDISW { w: self }
    }
}
