#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLEAR {
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
pub struct _CMDCRCFAILCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDCRCFAILCLRW<'a> {
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
#[doc = r" Proxy"]
pub struct _DATACRCFAILCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _DATACRCFAILCLRW<'a> {
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
#[doc = r" Proxy"]
pub struct _CMDTIMEOUTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDTIMEOUTCLRW<'a> {
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
#[doc = r" Proxy"]
pub struct _DATATIMEOUTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _DATATIMEOUTCLRW<'a> {
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
#[doc = r" Proxy"]
pub struct _TXUNDERRUNCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _TXUNDERRUNCLRW<'a> {
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
#[doc = r" Proxy"]
pub struct _RXOVERRUNCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _RXOVERRUNCLRW<'a> {
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
#[doc = r" Proxy"]
pub struct _CMDRESPENDCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDRESPENDCLRW<'a> {
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
#[doc = r" Proxy"]
pub struct _CMDSENTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDSENTCLRW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DATAENDCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _DATAENDCLRW<'a> {
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
#[doc = r" Proxy"]
pub struct _STARTBITERRCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _STARTBITERRCLRW<'a> {
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
#[doc = r" Proxy"]
pub struct _DATABLOCKENDCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _DATABLOCKENDCLRW<'a> {
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
    #[doc = "Bit 0 - Clears CmdCrcFail flag."]
    #[inline]
    pub fn cmdcrcfailclr(&mut self) -> _CMDCRCFAILCLRW {
        _CMDCRCFAILCLRW { w: self }
    }
    #[doc = "Bit 1 - Clears DataCrcFail flag."]
    #[inline]
    pub fn datacrcfailclr(&mut self) -> _DATACRCFAILCLRW {
        _DATACRCFAILCLRW { w: self }
    }
    #[doc = "Bit 2 - Clears CmdTimeOut flag."]
    #[inline]
    pub fn cmdtimeoutclr(&mut self) -> _CMDTIMEOUTCLRW {
        _CMDTIMEOUTCLRW { w: self }
    }
    #[doc = "Bit 3 - Clears DataTimeOut flag."]
    #[inline]
    pub fn datatimeoutclr(&mut self) -> _DATATIMEOUTCLRW {
        _DATATIMEOUTCLRW { w: self }
    }
    #[doc = "Bit 4 - Clears TxUnderrun flag."]
    #[inline]
    pub fn txunderrunclr(&mut self) -> _TXUNDERRUNCLRW {
        _TXUNDERRUNCLRW { w: self }
    }
    #[doc = "Bit 5 - Clears RxOverrun flag."]
    #[inline]
    pub fn rxoverrunclr(&mut self) -> _RXOVERRUNCLRW {
        _RXOVERRUNCLRW { w: self }
    }
    #[doc = "Bit 6 - Clears CmdRespEnd flag."]
    #[inline]
    pub fn cmdrespendclr(&mut self) -> _CMDRESPENDCLRW {
        _CMDRESPENDCLRW { w: self }
    }
    #[doc = "Bit 7 - Clears CmdSent flag."]
    #[inline]
    pub fn cmdsentclr(&mut self) -> _CMDSENTCLRW {
        _CMDSENTCLRW { w: self }
    }
    #[doc = "Bit 8 - Clears DataEnd flag."]
    #[inline]
    pub fn dataendclr(&mut self) -> _DATAENDCLRW {
        _DATAENDCLRW { w: self }
    }
    #[doc = "Bit 9 - Clears StartBitErr flag."]
    #[inline]
    pub fn startbiterrclr(&mut self) -> _STARTBITERRCLRW {
        _STARTBITERRCLRW { w: self }
    }
    #[doc = "Bit 10 - Clears DataBlockEnd flag."]
    #[inline]
    pub fn datablockendclr(&mut self) -> _DATABLOCKENDCLRW {
        _DATABLOCKENDCLRW { w: self }
    }
}
