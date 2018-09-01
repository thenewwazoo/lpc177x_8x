#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EMCCLKSEL {
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
#[doc = "Possible values of the field `EMCDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMCDIVR {
    #[doc = "The EMC uses the same clock as the CPU."]
    CPU_CLK,
    #[doc = "The EMC uses a clock at half the rate of the CPU."]
    HALF_CPU_CLK,
}
impl EMCDIVR {
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
            EMCDIVR::CPU_CLK => false,
            EMCDIVR::HALF_CPU_CLK => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EMCDIVR {
        match value {
            false => EMCDIVR::CPU_CLK,
            true => EMCDIVR::HALF_CPU_CLK,
        }
    }
    #[doc = "Checks if the value of the field is `CPU_CLK`"]
    #[inline]
    pub fn is_cpu_clk(&self) -> bool {
        *self == EMCDIVR::CPU_CLK
    }
    #[doc = "Checks if the value of the field is `HALF_CPU_CLK`"]
    #[inline]
    pub fn is_half_cpu_clk(&self) -> bool {
        *self == EMCDIVR::HALF_CPU_CLK
    }
}
#[doc = "Values that can be written to the field `EMCDIV`"]
pub enum EMCDIVW {
    #[doc = "The EMC uses the same clock as the CPU."]
    CPU_CLK,
    #[doc = "The EMC uses a clock at half the rate of the CPU."]
    HALF_CPU_CLK,
}
impl EMCDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EMCDIVW::CPU_CLK => false,
            EMCDIVW::HALF_CPU_CLK => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EMCDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _EMCDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EMCDIVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The EMC uses the same clock as the CPU."]
    #[inline]
    pub fn cpu_clk(self) -> &'a mut W {
        self.variant(EMCDIVW::CPU_CLK)
    }
    #[doc = "The EMC uses a clock at half the rate of the CPU."]
    #[inline]
    pub fn half_cpu_clk(self) -> &'a mut W {
        self.variant(EMCDIVW::HALF_CPU_CLK)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Selects the EMC clock rate relative to the CPU clock."]
    #[inline]
    pub fn emcdiv(&self) -> EMCDIVR {
        EMCDIVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
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
    #[doc = "Bit 0 - Selects the EMC clock rate relative to the CPU clock."]
    #[inline]
    pub fn emcdiv(&mut self) -> _EMCDIVW {
        _EMCDIVW { w: self }
    }
}
