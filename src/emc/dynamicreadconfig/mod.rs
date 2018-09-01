#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DYNAMICREADCONFIG {
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
#[doc = "Possible values of the field `RD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDR {
    #[doc = "Clock out delayed strategy, using CLKOUT (command not delayed, clock out delayed). POR reset value."]
    CLOCK_OUT_DELAYED,
    #[doc = "Command delayed strategy, using EMCCLKDELAY (command delayed, clock out not delayed)."]
    COMMAND_DELAYED,
    #[doc = "Command delayed strategy plus one clock cycle, using EMCCLKDELAY (command delayed, clock out not delayed)."]
    COMMAND_DELAYED_PLUS_ONE,
    #[doc = "Command delayed strategy plus two clock cycles, using EMCCLKDELAY (command delayed, clock out not delayed)."]
    COMMAND_DELAYED_PLUS_TWO,
}
impl RDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RDR::CLOCK_OUT_DELAYED => 0,
            RDR::COMMAND_DELAYED => 1,
            RDR::COMMAND_DELAYED_PLUS_ONE => 2,
            RDR::COMMAND_DELAYED_PLUS_TWO => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RDR {
        match value {
            0 => RDR::CLOCK_OUT_DELAYED,
            1 => RDR::COMMAND_DELAYED,
            2 => RDR::COMMAND_DELAYED_PLUS_ONE,
            3 => RDR::COMMAND_DELAYED_PLUS_TWO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLOCK_OUT_DELAYED`"]
    #[inline]
    pub fn is_clock_out_delayed(&self) -> bool {
        *self == RDR::CLOCK_OUT_DELAYED
    }
    #[doc = "Checks if the value of the field is `COMMAND_DELAYED`"]
    #[inline]
    pub fn is_command_delayed(&self) -> bool {
        *self == RDR::COMMAND_DELAYED
    }
    #[doc = "Checks if the value of the field is `COMMAND_DELAYED_PLUS_ONE`"]
    #[inline]
    pub fn is_command_delayed_plus_one(&self) -> bool {
        *self == RDR::COMMAND_DELAYED_PLUS_ONE
    }
    #[doc = "Checks if the value of the field is `COMMAND_DELAYED_PLUS_TWO`"]
    #[inline]
    pub fn is_command_delayed_plus_two(&self) -> bool {
        *self == RDR::COMMAND_DELAYED_PLUS_TWO
    }
}
#[doc = "Values that can be written to the field `RD`"]
pub enum RDW {
    #[doc = "Clock out delayed strategy, using CLKOUT (command not delayed, clock out delayed). POR reset value."]
    CLOCK_OUT_DELAYED,
    #[doc = "Command delayed strategy, using EMCCLKDELAY (command delayed, clock out not delayed)."]
    COMMAND_DELAYED,
    #[doc = "Command delayed strategy plus one clock cycle, using EMCCLKDELAY (command delayed, clock out not delayed)."]
    COMMAND_DELAYED_PLUS_ONE,
    #[doc = "Command delayed strategy plus two clock cycles, using EMCCLKDELAY (command delayed, clock out not delayed)."]
    COMMAND_DELAYED_PLUS_TWO,
}
impl RDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RDW::CLOCK_OUT_DELAYED => 0,
            RDW::COMMAND_DELAYED => 1,
            RDW::COMMAND_DELAYED_PLUS_ONE => 2,
            RDW::COMMAND_DELAYED_PLUS_TWO => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RDW<'a> {
    w: &'a mut W,
}
impl<'a> _RDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Clock out delayed strategy, using CLKOUT (command not delayed, clock out delayed). POR reset value."]
    #[inline]
    pub fn clock_out_delayed(self) -> &'a mut W {
        self.variant(RDW::CLOCK_OUT_DELAYED)
    }
    #[doc = "Command delayed strategy, using EMCCLKDELAY (command delayed, clock out not delayed)."]
    #[inline]
    pub fn command_delayed(self) -> &'a mut W {
        self.variant(RDW::COMMAND_DELAYED)
    }
    #[doc = "Command delayed strategy plus one clock cycle, using EMCCLKDELAY (command delayed, clock out not delayed)."]
    #[inline]
    pub fn command_delayed_plus_one(self) -> &'a mut W {
        self.variant(RDW::COMMAND_DELAYED_PLUS_ONE)
    }
    #[doc = "Command delayed strategy plus two clock cycles, using EMCCLKDELAY (command delayed, clock out not delayed)."]
    #[inline]
    pub fn command_delayed_plus_two(self) -> &'a mut W {
        self.variant(RDW::COMMAND_DELAYED_PLUS_TWO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 0:1 - Read data strategy"]
    #[inline]
    pub fn rd(&self) -> RDR {
        RDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:1 - Read data strategy"]
    #[inline]
    pub fn rd(&mut self) -> _RDW {
        _RDW { w: self }
    }
}
