#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ENCLR {
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
}
#[doc = r" Proxy"]
pub struct _RDWR_CLR_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RDWR_CLR_ENW<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PROG1_CLR_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PROG1_CLR_ENW<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[doc = "Bit 26 - Clear read/write operation finished interrupt enable bit (EEPROM). 0: leave corresponding bit unchanged. 1: clear corresponding bit."]
    #[inline]
    pub fn rdwr_clr_en(&mut self) -> _RDWR_CLR_ENW {
        _RDWR_CLR_ENW { w: self }
    }
    #[doc = "Bit 28 - Clear program operation finished interrupt enable bit for EEPROM device 1. 0: leave corresponding bit unchanged. 1: clear corresponding bit."]
    #[inline]
    pub fn prog1_clr_en(&mut self) -> _PROG1_CLR_ENW {
        _PROG1_CLR_ENW { w: self }
    }
}
