#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PCLKSEL {
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
pub struct PCLKDIVR {
    bits: u8,
}
impl PCLKDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _PCLKDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _PCLKDIVW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:4 - Selects the divide value for the clock used for all APB peripherals. 0 = The divider is turned off., no clock will be provided to APB peripherals.. 1 = The input clock is divided by 1 to produce the APB peripheral clock. 2 = The input clock is divided by 2 to produce the APB peripheral clock. 3 = The input clock is divided by 3 to produce the APB peripheral clock. ... 31 = The input clock is divided by 31 to produce the APB peripheral clock."]
    #[inline]
    pub fn pclkdiv(&self) -> PCLKDIVR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PCLKDIVR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 16 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - Selects the divide value for the clock used for all APB peripherals. 0 = The divider is turned off., no clock will be provided to APB peripherals.. 1 = The input clock is divided by 1 to produce the APB peripheral clock. 2 = The input clock is divided by 2 to produce the APB peripheral clock. 3 = The input clock is divided by 3 to produce the APB peripheral clock. ... 31 = The input clock is divided by 31 to produce the APB peripheral clock."]
    #[inline]
    pub fn pclkdiv(&mut self) -> _PCLKDIVW {
        _PCLKDIVW { w: self }
    }
}
