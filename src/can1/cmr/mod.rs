#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CMR {
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
#[doc = "Values that can be written to the field `TR`"]
pub enum TRW {
    #[doc = "Absent.No transmission request."]
    ABSENT,
    #[doc = "Present. The message, previously written to the CANxTFI, CANxTID, and optionally the CANxTDA and CANxTDB registers, is queued for transmission from the selected Transmit Buffer. If at two or all three of STB1, STB2 and STB3 bits are selected when TR=1 is written, Transmit Buffer will be selected based on the chosen priority scheme (for details see Section 21.5.3 Transmit Buffers (TXB))"]
    PRESENT,
}
impl TRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRW::ABSENT => false,
            TRW::PRESENT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRW<'a> {
    w: &'a mut W,
}
impl<'a> _TRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Absent.No transmission request."]
    #[inline]
    pub fn absent(self) -> &'a mut W {
        self.variant(TRW::ABSENT)
    }
    #[doc = "Present. The message, previously written to the CANxTFI, CANxTID, and optionally the CANxTDA and CANxTDB registers, is queued for transmission from the selected Transmit Buffer. If at two or all three of STB1, STB2 and STB3 bits are selected when TR=1 is written, Transmit Buffer will be selected based on the chosen priority scheme (for details see Section 21.5.3 Transmit Buffers (TXB))"]
    #[inline]
    pub fn present(self) -> &'a mut W {
        self.variant(TRW::PRESENT)
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
#[doc = "Values that can be written to the field `AT`"]
pub enum ATW {
    #[doc = "No action. Do not abort the transmission."]
    NO_ACTION,
    #[doc = "Present. if not already in progress, a pending Transmission Request for the selected Transmit Buffer is cancelled."]
    PRESENT,
}
impl ATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ATW::NO_ACTION => false,
            ATW::PRESENT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ATW<'a> {
    w: &'a mut W,
}
impl<'a> _ATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ATW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action. Do not abort the transmission."]
    #[inline]
    pub fn no_action(self) -> &'a mut W {
        self.variant(ATW::NO_ACTION)
    }
    #[doc = "Present. if not already in progress, a pending Transmission Request for the selected Transmit Buffer is cancelled."]
    #[inline]
    pub fn present(self) -> &'a mut W {
        self.variant(ATW::PRESENT)
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
#[doc = "Values that can be written to the field `RRB`"]
pub enum RRBW {
    #[doc = "No action. Do not release the receive buffer."]
    NO_ACTION,
    #[doc = "Released. The information in the Receive Buffer (consisting of CANxRFS, CANxRID, and if applicable the CANxRDA and CANxRDB registers) is released, and becomes eligible for replacement by the next received frame. If the next received frame is not available, writing this command clears the RBS bit in the Status Register(s)."]
    RELEASED,
}
impl RRBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RRBW::NO_ACTION => false,
            RRBW::RELEASED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RRBW<'a> {
    w: &'a mut W,
}
impl<'a> _RRBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RRBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action. Do not release the receive buffer."]
    #[inline]
    pub fn no_action(self) -> &'a mut W {
        self.variant(RRBW::NO_ACTION)
    }
    #[doc = "Released. The information in the Receive Buffer (consisting of CANxRFS, CANxRID, and if applicable the CANxRDA and CANxRDB registers) is released, and becomes eligible for replacement by the next received frame. If the next received frame is not available, writing this command clears the RBS bit in the Status Register(s)."]
    #[inline]
    pub fn released(self) -> &'a mut W {
        self.variant(RRBW::RELEASED)
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
#[doc = "Values that can be written to the field `CDO`"]
pub enum CDOW {
    #[doc = "No action. Do not clear the data overrun bit."]
    NO_ACTION,
    #[doc = "Clear. The Data Overrun bit in Status Register(s) is cleared."]
    CLEAR,
}
impl CDOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CDOW::NO_ACTION => false,
            CDOW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CDOW<'a> {
    w: &'a mut W,
}
impl<'a> _CDOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CDOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action. Do not clear the data overrun bit."]
    #[inline]
    pub fn no_action(self) -> &'a mut W {
        self.variant(CDOW::NO_ACTION)
    }
    #[doc = "Clear. The Data Overrun bit in Status Register(s) is cleared."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CDOW::CLEAR)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRR`"]
pub enum SRRW {
    #[doc = "Absent. No self reception request."]
    ABSENT,
    #[doc = "Present. The message, previously written to the CANxTFS, CANxTID, and optionally the CANxTDA and CANxTDB registers, is queued for transmission from the selected Transmit Buffer and received simultaneously. This differs from the TR bit above in that the receiver is not disabled during the transmission, so that it receives the message if its Identifier is recognized by the Acceptance Filter."]
    PRESENT,
}
impl SRRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRRW::ABSENT => false,
            SRRW::PRESENT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRRW<'a> {
    w: &'a mut W,
}
impl<'a> _SRRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Absent. No self reception request."]
    #[inline]
    pub fn absent(self) -> &'a mut W {
        self.variant(SRRW::ABSENT)
    }
    #[doc = "Present. The message, previously written to the CANxTFS, CANxTID, and optionally the CANxTDA and CANxTDB registers, is queued for transmission from the selected Transmit Buffer and received simultaneously. This differs from the TR bit above in that the receiver is not disabled during the transmission, so that it receives the message if its Identifier is recognized by the Acceptance Filter."]
    #[inline]
    pub fn present(self) -> &'a mut W {
        self.variant(SRRW::PRESENT)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STB1`"]
pub enum STB1W {
    #[doc = "Not selected. Tx Buffer 1 is not selected for transmission."]
    NOT_SELECTED,
    #[doc = "Selected. Tx Buffer 1 is selected for transmission."]
    SELECTED,
}
impl STB1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STB1W::NOT_SELECTED => false,
            STB1W::SELECTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STB1W<'a> {
    w: &'a mut W,
}
impl<'a> _STB1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STB1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not selected. Tx Buffer 1 is not selected for transmission."]
    #[inline]
    pub fn not_selected(self) -> &'a mut W {
        self.variant(STB1W::NOT_SELECTED)
    }
    #[doc = "Selected. Tx Buffer 1 is selected for transmission."]
    #[inline]
    pub fn selected(self) -> &'a mut W {
        self.variant(STB1W::SELECTED)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STB2`"]
pub enum STB2W {
    #[doc = "Not selected. Tx Buffer 2 is not selected for transmission."]
    NOT_SELECTED,
    #[doc = "Selected. Tx Buffer 2 is selected for transmission."]
    SELECTED,
}
impl STB2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STB2W::NOT_SELECTED => false,
            STB2W::SELECTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STB2W<'a> {
    w: &'a mut W,
}
impl<'a> _STB2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STB2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not selected. Tx Buffer 2 is not selected for transmission."]
    #[inline]
    pub fn not_selected(self) -> &'a mut W {
        self.variant(STB2W::NOT_SELECTED)
    }
    #[doc = "Selected. Tx Buffer 2 is selected for transmission."]
    #[inline]
    pub fn selected(self) -> &'a mut W {
        self.variant(STB2W::SELECTED)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STB3`"]
pub enum STB3W {
    #[doc = "Not selected. Tx Buffer 3 is not selected for transmission."]
    NOT_SELECTED,
    #[doc = "Selected. Tx Buffer 3 is selected for transmission."]
    SELECTED,
}
impl STB3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STB3W::NOT_SELECTED => false,
            STB3W::SELECTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STB3W<'a> {
    w: &'a mut W,
}
impl<'a> _STB3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STB3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not selected. Tx Buffer 3 is not selected for transmission."]
    #[inline]
    pub fn not_selected(self) -> &'a mut W {
        self.variant(STB3W::NOT_SELECTED)
    }
    #[doc = "Selected. Tx Buffer 3 is selected for transmission."]
    #[inline]
    pub fn selected(self) -> &'a mut W {
        self.variant(STB3W::SELECTED)
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
        const OFFSET: u8 = 7;
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
    #[doc = "Bit 0 - Transmission Request."]
    #[inline]
    pub fn tr(&mut self) -> _TRW {
        _TRW { w: self }
    }
    #[doc = "Bit 1 - Abort Transmission."]
    #[inline]
    pub fn at(&mut self) -> _ATW {
        _ATW { w: self }
    }
    #[doc = "Bit 2 - Release Receive Buffer."]
    #[inline]
    pub fn rrb(&mut self) -> _RRBW {
        _RRBW { w: self }
    }
    #[doc = "Bit 3 - Clear Data Overrun."]
    #[inline]
    pub fn cdo(&mut self) -> _CDOW {
        _CDOW { w: self }
    }
    #[doc = "Bit 4 - Self Reception Request."]
    #[inline]
    pub fn srr(&mut self) -> _SRRW {
        _SRRW { w: self }
    }
    #[doc = "Bit 5 - Select Tx Buffer 1."]
    #[inline]
    pub fn stb1(&mut self) -> _STB1W {
        _STB1W { w: self }
    }
    #[doc = "Bit 6 - Select Tx Buffer 2."]
    #[inline]
    pub fn stb2(&mut self) -> _STB2W {
        _STB2W { w: self }
    }
    #[doc = "Bit 7 - Select Tx Buffer 3."]
    #[inline]
    pub fn stb3(&mut self) -> _STB3W {
        _STB3W { w: self }
    }
}
