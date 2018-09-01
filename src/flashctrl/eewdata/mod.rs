#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EEWDATA {
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
pub struct _WDATAW<'a> {
    w: &'a mut W,
}
impl<'a> _WDATAW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:31 - Write data. In case of: 8-bit write operations: bits [7:0] must contain valid write data. 16-bit write operations: bits [15:0] must contain valid write data. 32-bit write operations: bits [31:0] must contain valid write data."]
    #[inline]
    pub fn wdata(&mut self) -> _WDATAW {
        _WDATAW { w: self }
    }
}
