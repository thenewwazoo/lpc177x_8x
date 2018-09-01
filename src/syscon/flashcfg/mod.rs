#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FLASHCFG {
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
#[doc = "Possible values of the field `FLASHTIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHTIMR {
    #[doc = "Flash accesses use 1 CPU clock. Use for up to 20 MHz CPU clock with power boost off."]
    _1_CLK,
    #[doc = "Flash accesses use 2 CPU clocks. Use for up to 40 MHz CPU clock with power boost off."]
    _2_CLK,
    #[doc = "Flash accesses use 3 CPU clocks. Use for up to 60 MHz CPU clock with power boost off."]
    _3_CLK,
    #[doc = "Flash accesses use 4 CPU clocks. Use for up to 80 MHz CPU clock with power boost off. Use this setting for operation from 100 to 120 MHz operation with power boost on."]
    _4_CLK,
    #[doc = "Flash accesses use 5 CPU clocks. Use for up to 100 MHz CPU clock with power boost off."]
    _5_CLK,
    #[doc = "Flash accesses use 6 CPU clocks. Safe setting for any allowed conditions."]
    _6_CLK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FLASHTIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FLASHTIMR::_1_CLK => 0,
            FLASHTIMR::_2_CLK => 1,
            FLASHTIMR::_3_CLK => 2,
            FLASHTIMR::_4_CLK => 3,
            FLASHTIMR::_5_CLK => 4,
            FLASHTIMR::_6_CLK => 5,
            FLASHTIMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FLASHTIMR {
        match value {
            0 => FLASHTIMR::_1_CLK,
            1 => FLASHTIMR::_2_CLK,
            2 => FLASHTIMR::_3_CLK,
            3 => FLASHTIMR::_4_CLK,
            4 => FLASHTIMR::_5_CLK,
            5 => FLASHTIMR::_6_CLK,
            i => FLASHTIMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1_CLK`"]
    #[inline]
    pub fn is_1_clk(&self) -> bool {
        *self == FLASHTIMR::_1_CLK
    }
    #[doc = "Checks if the value of the field is `_2_CLK`"]
    #[inline]
    pub fn is_2_clk(&self) -> bool {
        *self == FLASHTIMR::_2_CLK
    }
    #[doc = "Checks if the value of the field is `_3_CLK`"]
    #[inline]
    pub fn is_3_clk(&self) -> bool {
        *self == FLASHTIMR::_3_CLK
    }
    #[doc = "Checks if the value of the field is `_4_CLK`"]
    #[inline]
    pub fn is_4_clk(&self) -> bool {
        *self == FLASHTIMR::_4_CLK
    }
    #[doc = "Checks if the value of the field is `_5_CLK`"]
    #[inline]
    pub fn is_5_clk(&self) -> bool {
        *self == FLASHTIMR::_5_CLK
    }
    #[doc = "Checks if the value of the field is `_6_CLK`"]
    #[inline]
    pub fn is_6_clk(&self) -> bool {
        *self == FLASHTIMR::_6_CLK
    }
}
#[doc = "Values that can be written to the field `FLASHTIM`"]
pub enum FLASHTIMW {
    #[doc = "Flash accesses use 1 CPU clock. Use for up to 20 MHz CPU clock with power boost off."]
    _1_CLK,
    #[doc = "Flash accesses use 2 CPU clocks. Use for up to 40 MHz CPU clock with power boost off."]
    _2_CLK,
    #[doc = "Flash accesses use 3 CPU clocks. Use for up to 60 MHz CPU clock with power boost off."]
    _3_CLK,
    #[doc = "Flash accesses use 4 CPU clocks. Use for up to 80 MHz CPU clock with power boost off. Use this setting for operation from 100 to 120 MHz operation with power boost on."]
    _4_CLK,
    #[doc = "Flash accesses use 5 CPU clocks. Use for up to 100 MHz CPU clock with power boost off."]
    _5_CLK,
    #[doc = "Flash accesses use 6 CPU clocks. Safe setting for any allowed conditions."]
    _6_CLK,
}
impl FLASHTIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLASHTIMW::_1_CLK => 0,
            FLASHTIMW::_2_CLK => 1,
            FLASHTIMW::_3_CLK => 2,
            FLASHTIMW::_4_CLK => 3,
            FLASHTIMW::_5_CLK => 4,
            FLASHTIMW::_6_CLK => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLASHTIMW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASHTIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLASHTIMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Flash accesses use 1 CPU clock. Use for up to 20 MHz CPU clock with power boost off."]
    #[inline]
    pub fn _1_clk(self) -> &'a mut W {
        self.variant(FLASHTIMW::_1_CLK)
    }
    #[doc = "Flash accesses use 2 CPU clocks. Use for up to 40 MHz CPU clock with power boost off."]
    #[inline]
    pub fn _2_clk(self) -> &'a mut W {
        self.variant(FLASHTIMW::_2_CLK)
    }
    #[doc = "Flash accesses use 3 CPU clocks. Use for up to 60 MHz CPU clock with power boost off."]
    #[inline]
    pub fn _3_clk(self) -> &'a mut W {
        self.variant(FLASHTIMW::_3_CLK)
    }
    #[doc = "Flash accesses use 4 CPU clocks. Use for up to 80 MHz CPU clock with power boost off. Use this setting for operation from 100 to 120 MHz operation with power boost on."]
    #[inline]
    pub fn _4_clk(self) -> &'a mut W {
        self.variant(FLASHTIMW::_4_CLK)
    }
    #[doc = "Flash accesses use 5 CPU clocks. Use for up to 100 MHz CPU clock with power boost off."]
    #[inline]
    pub fn _5_clk(self) -> &'a mut W {
        self.variant(FLASHTIMW::_5_CLK)
    }
    #[doc = "Flash accesses use 6 CPU clocks. Safe setting for any allowed conditions."]
    #[inline]
    pub fn _6_clk(self) -> &'a mut W {
        self.variant(FLASHTIMW::_6_CLK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
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
    #[doc = "Bits 12:15 - Flash access time. The value of this field plus 1 gives the number of CPU clocks used for a flash access. Warning: improper setting of this value may result in incorrect operation of the device. All other values are reserved."]
    #[inline]
    pub fn flashtim(&self) -> FLASHTIMR {
        FLASHTIMR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 12:15 - Flash access time. The value of this field plus 1 gives the number of CPU clocks used for a flash access. Warning: improper setting of this value may result in incorrect operation of the device. All other values are reserved."]
    #[inline]
    pub fn flashtim(&mut self) -> _FLASHTIMW {
        _FLASHTIMW { w: self }
    }
}
