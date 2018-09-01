#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FCR {
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
#[doc = "Values that can be written to the field `FIFOEN`"]
pub enum FIFOENW {
    #[doc = "Must not be used in the application."]
    DO_NOT_USE,
    #[doc = "Active high enable for both UART1 Rx and TX FIFOs and FCR[7:1] access. This bit must be set for proper UART1 operation. Any transition on this bit will automatically clear the UART1 FIFOs."]
    ACTIVE_HIGH_EN,
}
impl FIFOENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FIFOENW::DO_NOT_USE => false,
            FIFOENW::ACTIVE_HIGH_EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FIFOENW<'a> {
    w: &'a mut W,
}
impl<'a> _FIFOENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FIFOENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Must not be used in the application."]
    #[inline]
    pub fn do_not_use(self) -> &'a mut W {
        self.variant(FIFOENW::DO_NOT_USE)
    }
    #[doc = "Active high enable for both UART1 Rx and TX FIFOs and FCR[7:1] access. This bit must be set for proper UART1 operation. Any transition on this bit will automatically clear the UART1 FIFOs."]
    #[inline]
    pub fn active_high_en(self) -> &'a mut W {
        self.variant(FIFOENW::ACTIVE_HIGH_EN)
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
#[doc = "Values that can be written to the field `RXFIFORES`"]
pub enum RXFIFORESW {
    #[doc = "No impact on either of UART1 FIFOs."]
    NO_OP,
    #[doc = "Writing a logic 1 to FCR[1] will clear all bytes in UART1 Rx FIFO, reset the pointer logic. This bit is self-clearing."]
    CLEAR_ALL_BYTES,
}
impl RXFIFORESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXFIFORESW::NO_OP => false,
            RXFIFORESW::CLEAR_ALL_BYTES => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXFIFORESW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFIFORESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXFIFORESW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No impact on either of UART1 FIFOs."]
    #[inline]
    pub fn no_op(self) -> &'a mut W {
        self.variant(RXFIFORESW::NO_OP)
    }
    #[doc = "Writing a logic 1 to FCR[1] will clear all bytes in UART1 Rx FIFO, reset the pointer logic. This bit is self-clearing."]
    #[inline]
    pub fn clear_all_bytes(self) -> &'a mut W {
        self.variant(RXFIFORESW::CLEAR_ALL_BYTES)
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
#[doc = "Values that can be written to the field `TXFIFORES`"]
pub enum TXFIFORESW {
    #[doc = "No impact on either of UART1 FIFOs."]
    NO_OP,
    #[doc = "Writing a logic 1 to FCR[2] will clear all bytes in UART1 TX FIFO, reset the pointer logic. This bit is self-clearing."]
    CLEAR_ALL_BYTES,
}
impl TXFIFORESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXFIFORESW::NO_OP => false,
            TXFIFORESW::CLEAR_ALL_BYTES => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXFIFORESW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFIFORESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXFIFORESW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No impact on either of UART1 FIFOs."]
    #[inline]
    pub fn no_op(self) -> &'a mut W {
        self.variant(TXFIFORESW::NO_OP)
    }
    #[doc = "Writing a logic 1 to FCR[2] will clear all bytes in UART1 TX FIFO, reset the pointer logic. This bit is self-clearing."]
    #[inline]
    pub fn clear_all_bytes(self) -> &'a mut W {
        self.variant(TXFIFORESW::CLEAR_ALL_BYTES)
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
#[doc = r" Proxy"]
pub struct _DMAMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAMODEW<'a> {
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
#[doc = "Values that can be written to the field `RXTRIGLVL`"]
pub enum RXTRIGLVLW {
    #[doc = "Trigger level 0 (1 character or 0x01)."]
    _1_CHAR,
    #[doc = "Trigger level 1 (4 characters or 0x04)."]
    _4_CHAR,
    #[doc = "Trigger level 2 (8 characters or 0x08)."]
    _8_CHAR,
    #[doc = "Trigger level 3 (14 characters or 0x0E)."]
    _14_CHAR,
}
impl RXTRIGLVLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXTRIGLVLW::_1_CHAR => 0,
            RXTRIGLVLW::_4_CHAR => 1,
            RXTRIGLVLW::_8_CHAR => 2,
            RXTRIGLVLW::_14_CHAR => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXTRIGLVLW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTRIGLVLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXTRIGLVLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Trigger level 0 (1 character or 0x01)."]
    #[inline]
    pub fn _1_char(self) -> &'a mut W {
        self.variant(RXTRIGLVLW::_1_CHAR)
    }
    #[doc = "Trigger level 1 (4 characters or 0x04)."]
    #[inline]
    pub fn _4_char(self) -> &'a mut W {
        self.variant(RXTRIGLVLW::_4_CHAR)
    }
    #[doc = "Trigger level 2 (8 characters or 0x08)."]
    #[inline]
    pub fn _8_char(self) -> &'a mut W {
        self.variant(RXTRIGLVLW::_8_CHAR)
    }
    #[doc = "Trigger level 3 (14 characters or 0x0E)."]
    #[inline]
    pub fn _14_char(self) -> &'a mut W {
        self.variant(RXTRIGLVLW::_14_CHAR)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
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
    #[doc = "Bit 0 - FIFO enable."]
    #[inline]
    pub fn fifoen(&mut self) -> _FIFOENW {
        _FIFOENW { w: self }
    }
    #[doc = "Bit 1 - RX FIFO Reset."]
    #[inline]
    pub fn rxfifores(&mut self) -> _RXFIFORESW {
        _RXFIFORESW { w: self }
    }
    #[doc = "Bit 2 - TX FIFO Reset."]
    #[inline]
    pub fn txfifores(&mut self) -> _TXFIFORESW {
        _TXFIFORESW { w: self }
    }
    #[doc = "Bit 3 - DMA Mode Select. When the FIFO enable bit (bit 0 of this register) is set, this bit selects the DMA mode. See Section 36.6.6.1."]
    #[inline]
    pub fn dmamode(&mut self) -> _DMAMODEW {
        _DMAMODEW { w: self }
    }
    #[doc = "Bits 6:7 - RX Trigger Level. These two bits determine how many receiver UART1 FIFO characters must be written before an interrupt is activated."]
    #[inline]
    pub fn rxtriglvl(&mut self) -> _RXTRIGLVLW {
        _RXTRIGLVLW { w: self }
    }
}
