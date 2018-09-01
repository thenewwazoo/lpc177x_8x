#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EMCDLYCTL {
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
pub struct CMDDLYR {
    bits: u8,
}
impl CMDDLYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FBCLKDLYR {
    bits: u8,
}
impl FBCLKDLYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CLKOUT0DLYR {
    bits: u8,
}
impl CLKOUT0DLYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CLKOUT1DLYR {
    bits: u8,
}
impl CLKOUT1DLYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CMDDLYW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDDLYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FBCLKDLYW<'a> {
    w: &'a mut W,
}
impl<'a> _FBCLKDLYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLKOUT0DLYW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKOUT0DLYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLKOUT1DLYW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKOUT1DLYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:4 - Programmable delay value for EMC outputs in command delayed mode. See Section 10.12.6. The delay amount is roughly (CMDDLY+1) * 250 picoseconds. This field applies only when the command delayed read strategy is selected in the EMCDynamicReadConfig register. In this mode, all control outputs from the EMC are delayed, but the output clock is not. Delaying the control outputs changes dynamic characteristics defined in the device data sheet."]
    #[inline]
    pub fn cmddly(&self) -> CMDDLYR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CMDDLYR { bits }
    }
    #[doc = "Bits 8:12 - Programmable delay value for the feedback clock that controls input data sampling. See Section 10.5.3. The delay amount is roughly (FBCLKDLY+1) * 250 picoseconds."]
    #[inline]
    pub fn fbclkdly(&self) -> FBCLKDLYR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FBCLKDLYR { bits }
    }
    #[doc = "Bits 16:20 - Programmable delay value for the CLKOUT0 output. This would typically be used in clock delayed mode. See Section 10.12.6 The delay amount is roughly (CLKOUT0DLY+1) * 250 picoseconds. Delaying the clock output changes dynamic characteristics defined in the device data sheet."]
    #[inline]
    pub fn clkout0dly(&self) -> CLKOUT0DLYR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CLKOUT0DLYR { bits }
    }
    #[doc = "Bits 24:28 - Programmable delay value for the CLKOUT1 output. This would typically be used in clock delayed mode. See Section 10.12.6 The delay amount is roughly (CLKOUT1DLY+1) * 250 picoseconds."]
    #[inline]
    pub fn clkout1dly(&self) -> CLKOUT1DLYR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CLKOUT1DLYR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 528 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - Programmable delay value for EMC outputs in command delayed mode. See Section 10.12.6. The delay amount is roughly (CMDDLY+1) * 250 picoseconds. This field applies only when the command delayed read strategy is selected in the EMCDynamicReadConfig register. In this mode, all control outputs from the EMC are delayed, but the output clock is not. Delaying the control outputs changes dynamic characteristics defined in the device data sheet."]
    #[inline]
    pub fn cmddly(&mut self) -> _CMDDLYW {
        _CMDDLYW { w: self }
    }
    #[doc = "Bits 8:12 - Programmable delay value for the feedback clock that controls input data sampling. See Section 10.5.3. The delay amount is roughly (FBCLKDLY+1) * 250 picoseconds."]
    #[inline]
    pub fn fbclkdly(&mut self) -> _FBCLKDLYW {
        _FBCLKDLYW { w: self }
    }
    #[doc = "Bits 16:20 - Programmable delay value for the CLKOUT0 output. This would typically be used in clock delayed mode. See Section 10.12.6 The delay amount is roughly (CLKOUT0DLY+1) * 250 picoseconds. Delaying the clock output changes dynamic characteristics defined in the device data sheet."]
    #[inline]
    pub fn clkout0dly(&mut self) -> _CLKOUT0DLYW {
        _CLKOUT0DLYW { w: self }
    }
    #[doc = "Bits 24:28 - Programmable delay value for the CLKOUT1 output. This would typically be used in clock delayed mode. See Section 10.12.6 The delay amount is roughly (CLKOUT1DLY+1) * 250 picoseconds."]
    #[inline]
    pub fn clkout1dly(&mut self) -> _CLKOUT1DLYW {
        _CLKOUT1DLYW { w: self }
    }
}
