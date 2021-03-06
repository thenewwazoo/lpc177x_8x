#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONTROL {
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
#[doc = "Possible values of the field `E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ER {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled\n                                        (POR and warm reset value)."]
    ENABLED,
}
impl ER {
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
            ER::DISABLED => false,
            ER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ER {
        match value {
            false => ER::DISABLED,
            true => ER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ER::ENABLED
    }
}
#[doc = "Possible values of the field `M`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR {
    #[doc = "Normal memory map."]
    NORMAL,
    #[doc = "Reset memory map. Static memory EMC_CS1 is\n                                        mirrored onto EMC_CS0 and EMC_DYCS0 (POR reset value)."]
    RESET,
}
impl MR {
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
            MR::NORMAL => false,
            MR::RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MR {
        match value {
            false => MR::NORMAL,
            true => MR::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == MR::NORMAL
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == MR::RESET
    }
}
#[doc = "Possible values of the field `L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LR {
    #[doc = "Normal mode (warm\n                                        reset value)."]
    WARMRESET,
    #[doc = "Low-power\n                                        mode. Entering low-power mode reduces memory controller power consumption.\n                                        Dynamic memory is refreshed as necessary. The memory controller\n                                        returns to normal functional mode by clearing the low-power mode\n                                        bit (L), or by POR. This bit must only be modified when the EMC\n                                        is in idle state.[1]"]
    LOWPOWER,
}
impl LR {
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
            LR::WARMRESET => false,
            LR::LOWPOWER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LR {
        match value {
            false => LR::WARMRESET,
            true => LR::LOWPOWER,
        }
    }
    #[doc = "Checks if the value of the field is `WARMRESET`"]
    #[inline]
    pub fn is_warmreset(&self) -> bool {
        *self == LR::WARMRESET
    }
    #[doc = "Checks if the value of the field is `LOWPOWER`"]
    #[inline]
    pub fn is_lowpower(&self) -> bool {
        *self == LR::LOWPOWER
    }
}
#[doc = "Values that can be written to the field `E`"]
pub enum EW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled\n                                        (POR and warm reset value)."]
    ENABLED,
}
impl EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EW::DISABLED => false,
            EW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EW<'a> {
    w: &'a mut W,
}
impl<'a> _EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EW::DISABLED)
    }
    #[doc = "Enabled (POR and warm reset value)."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EW::ENABLED)
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
#[doc = "Values that can be written to the field `M`"]
pub enum MW {
    #[doc = "Normal memory map."]
    NORMAL,
    #[doc = "Reset memory map. Static memory EMC_CS1 is\n                                        mirrored onto EMC_CS0 and EMC_DYCS0 (POR reset value)."]
    RESET,
}
impl MW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MW::NORMAL => false,
            MW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MW<'a> {
    w: &'a mut W,
}
impl<'a> _MW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal memory map."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(MW::NORMAL)
    }
    #[doc = "Reset memory map. Static memory EMC_CS1 is mirrored onto EMC_CS0 and EMC_DYCS0 (POR reset value)."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(MW::RESET)
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
#[doc = "Values that can be written to the field `L`"]
pub enum LW {
    #[doc = "Normal mode (warm\n                                        reset value)."]
    WARMRESET,
    #[doc = "Low-power\n                                        mode. Entering low-power mode reduces memory controller power consumption.\n                                        Dynamic memory is refreshed as necessary. The memory controller\n                                        returns to normal functional mode by clearing the low-power mode\n                                        bit (L), or by POR. This bit must only be modified when the EMC\n                                        is in idle state.[1]"]
    LOWPOWER,
}
impl LW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LW::WARMRESET => false,
            LW::LOWPOWER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LW<'a> {
    w: &'a mut W,
}
impl<'a> _LW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal mode (warm reset value)."]
    #[inline]
    pub fn warmreset(self) -> &'a mut W {
        self.variant(LW::WARMRESET)
    }
    #[doc = "Low-power mode. Entering low-power mode reduces memory controller power consumption. Dynamic memory is refreshed as necessary. The memory controller returns to normal functional mode by clearing the low-power mode bit (L), or by POR. This bit must only be modified when the EMC is in idle state.[1]"]
    #[inline]
    pub fn lowpower(self) -> &'a mut W {
        self.variant(LW::LOWPOWER)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - EMC Enable. Indicates if the EMC is enabled or disabled:"]
    #[inline]
    pub fn e(&self) -> ER {
        ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Address mirror. Indicates normal or reset memory map:"]
    #[inline]
    pub fn m(&self) -> MR {
        MR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Low-power mode. Indicates normal, or low-power mode:"]
    #[inline]
    pub fn l(&self) -> LR {
        LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 3 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - EMC Enable. Indicates if the EMC is enabled or disabled:"]
    #[inline]
    pub fn e(&mut self) -> _EW {
        _EW { w: self }
    }
    #[doc = "Bit 1 - Address mirror. Indicates normal or reset memory map:"]
    #[inline]
    pub fn m(&mut self) -> _MW {
        _MW { w: self }
    }
    #[doc = "Bit 2 - Low-power mode. Indicates normal, or low-power mode:"]
    #[inline]
    pub fn l(&mut self) -> _LW {
        _LW { w: self }
    }
}
