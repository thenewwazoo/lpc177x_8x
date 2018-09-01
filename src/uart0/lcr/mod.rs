#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LCR {
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
#[doc = "Possible values of the field `WLS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WLSR {
    #[doc = "5-bit character length"]
    _5_BIT,
    #[doc = "6-bit character length"]
    _6_BIT,
    #[doc = "7-bit character length"]
    _7_BIT,
    #[doc = "8-bit character length"]
    _8_BIT,
}
impl WLSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WLSR::_5_BIT => 0,
            WLSR::_6_BIT => 1,
            WLSR::_7_BIT => 2,
            WLSR::_8_BIT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WLSR {
        match value {
            0 => WLSR::_5_BIT,
            1 => WLSR::_6_BIT,
            2 => WLSR::_7_BIT,
            3 => WLSR::_8_BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_5_BIT`"]
    #[inline]
    pub fn is_5_bit(&self) -> bool {
        *self == WLSR::_5_BIT
    }
    #[doc = "Checks if the value of the field is `_6_BIT`"]
    #[inline]
    pub fn is_6_bit(&self) -> bool {
        *self == WLSR::_6_BIT
    }
    #[doc = "Checks if the value of the field is `_7_BIT`"]
    #[inline]
    pub fn is_7_bit(&self) -> bool {
        *self == WLSR::_7_BIT
    }
    #[doc = "Checks if the value of the field is `_8_BIT`"]
    #[inline]
    pub fn is_8_bit(&self) -> bool {
        *self == WLSR::_8_BIT
    }
}
#[doc = "Possible values of the field `SBS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBSR {
    #[doc = "1 stop bit."]
    _1_STOP_BIT,
    #[doc = "2 stop bits (1.5 if UnLCR[1:0]=00)."]
    _2_STOP_BITS,
}
impl SBSR {
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
            SBSR::_1_STOP_BIT => false,
            SBSR::_2_STOP_BITS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SBSR {
        match value {
            false => SBSR::_1_STOP_BIT,
            true => SBSR::_2_STOP_BITS,
        }
    }
    #[doc = "Checks if the value of the field is `_1_STOP_BIT`"]
    #[inline]
    pub fn is_1_stop_bit(&self) -> bool {
        *self == SBSR::_1_STOP_BIT
    }
    #[doc = "Checks if the value of the field is `_2_STOP_BITS`"]
    #[inline]
    pub fn is_2_stop_bits(&self) -> bool {
        *self == SBSR::_2_STOP_BITS
    }
}
#[doc = "Possible values of the field `PE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PER {
    #[doc = "Disable parity generation and checking."]
    DISABLE,
    #[doc = "Enable parity generation and checking."]
    ENABLE,
}
impl PER {
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
            PER::DISABLE => false,
            PER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PER {
        match value {
            false => PER::DISABLE,
            true => PER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == PER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == PER::ENABLE
    }
}
#[doc = "Possible values of the field `PS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSR {
    #[doc = "Odd parity. Number of 1s in the transmitted character and the attached parity bit will be odd."]
    ODD_PARITY,
    #[doc = "Even Parity. Number of 1s in the transmitted character and the attached parity bit will be even."]
    EVEN_PARITY,
    #[doc = "Forced 1 stick parity."]
    FORCED_1,
    #[doc = "Forced 0 stick parity."]
    FORCED_0,
}
impl PSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSR::ODD_PARITY => 0,
            PSR::EVEN_PARITY => 1,
            PSR::FORCED_1 => 2,
            PSR::FORCED_0 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSR {
        match value {
            0 => PSR::ODD_PARITY,
            1 => PSR::EVEN_PARITY,
            2 => PSR::FORCED_1,
            3 => PSR::FORCED_0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ODD_PARITY`"]
    #[inline]
    pub fn is_odd_parity(&self) -> bool {
        *self == PSR::ODD_PARITY
    }
    #[doc = "Checks if the value of the field is `EVEN_PARITY`"]
    #[inline]
    pub fn is_even_parity(&self) -> bool {
        *self == PSR::EVEN_PARITY
    }
    #[doc = "Checks if the value of the field is `FORCED_1`"]
    #[inline]
    pub fn is_forced_1(&self) -> bool {
        *self == PSR::FORCED_1
    }
    #[doc = "Checks if the value of the field is `FORCED_0`"]
    #[inline]
    pub fn is_forced_0(&self) -> bool {
        *self == PSR::FORCED_0
    }
}
#[doc = "Possible values of the field `BC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCR {
    #[doc = "Disable break transmission."]
    DISABLE,
    #[doc = "Enable break transmission. Output pin UARTn TXD is forced to logic 0 when UnLCR[6] is active high."]
    ENABLE,
}
impl BCR {
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
            BCR::DISABLE => false,
            BCR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BCR {
        match value {
            false => BCR::DISABLE,
            true => BCR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == BCR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == BCR::ENABLE
    }
}
#[doc = "Possible values of the field `DLAB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLABR {
    #[doc = "Disable access to Divisor Latches."]
    DISABLE,
    #[doc = "Enable access to Divisor Latches."]
    ENABLE,
}
impl DLABR {
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
            DLABR::DISABLE => false,
            DLABR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DLABR {
        match value {
            false => DLABR::DISABLE,
            true => DLABR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == DLABR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == DLABR::ENABLE
    }
}
#[doc = "Values that can be written to the field `WLS`"]
pub enum WLSW {
    #[doc = "5-bit character length"]
    _5_BIT,
    #[doc = "6-bit character length"]
    _6_BIT,
    #[doc = "7-bit character length"]
    _7_BIT,
    #[doc = "8-bit character length"]
    _8_BIT,
}
impl WLSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WLSW::_5_BIT => 0,
            WLSW::_6_BIT => 1,
            WLSW::_7_BIT => 2,
            WLSW::_8_BIT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WLSW<'a> {
    w: &'a mut W,
}
impl<'a> _WLSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WLSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "5-bit character length"]
    #[inline]
    pub fn _5_bit(self) -> &'a mut W {
        self.variant(WLSW::_5_BIT)
    }
    #[doc = "6-bit character length"]
    #[inline]
    pub fn _6_bit(self) -> &'a mut W {
        self.variant(WLSW::_6_BIT)
    }
    #[doc = "7-bit character length"]
    #[inline]
    pub fn _7_bit(self) -> &'a mut W {
        self.variant(WLSW::_7_BIT)
    }
    #[doc = "8-bit character length"]
    #[inline]
    pub fn _8_bit(self) -> &'a mut W {
        self.variant(WLSW::_8_BIT)
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
#[doc = "Values that can be written to the field `SBS`"]
pub enum SBSW {
    #[doc = "1 stop bit."]
    _1_STOP_BIT,
    #[doc = "2 stop bits (1.5 if UnLCR[1:0]=00)."]
    _2_STOP_BITS,
}
impl SBSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SBSW::_1_STOP_BIT => false,
            SBSW::_2_STOP_BITS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SBSW<'a> {
    w: &'a mut W,
}
impl<'a> _SBSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SBSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "1 stop bit."]
    #[inline]
    pub fn _1_stop_bit(self) -> &'a mut W {
        self.variant(SBSW::_1_STOP_BIT)
    }
    #[doc = "2 stop bits (1.5 if UnLCR[1:0]=00)."]
    #[inline]
    pub fn _2_stop_bits(self) -> &'a mut W {
        self.variant(SBSW::_2_STOP_BITS)
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
#[doc = "Values that can be written to the field `PE`"]
pub enum PEW {
    #[doc = "Disable parity generation and checking."]
    DISABLE,
    #[doc = "Enable parity generation and checking."]
    ENABLE,
}
impl PEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEW::DISABLE => false,
            PEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEW<'a> {
    w: &'a mut W,
}
impl<'a> _PEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable parity generation and checking."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(PEW::DISABLE)
    }
    #[doc = "Enable parity generation and checking."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(PEW::ENABLE)
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
#[doc = "Values that can be written to the field `PS`"]
pub enum PSW {
    #[doc = "Odd parity. Number of 1s in the transmitted character and the attached parity bit will be odd."]
    ODD_PARITY,
    #[doc = "Even Parity. Number of 1s in the transmitted character and the attached parity bit will be even."]
    EVEN_PARITY,
    #[doc = "Forced 1 stick parity."]
    FORCED_1,
    #[doc = "Forced 0 stick parity."]
    FORCED_0,
}
impl PSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PSW::ODD_PARITY => 0,
            PSW::EVEN_PARITY => 1,
            PSW::FORCED_1 => 2,
            PSW::FORCED_0 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSW<'a> {
    w: &'a mut W,
}
impl<'a> _PSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Odd parity. Number of 1s in the transmitted character and the attached parity bit will be odd."]
    #[inline]
    pub fn odd_parity(self) -> &'a mut W {
        self.variant(PSW::ODD_PARITY)
    }
    #[doc = "Even Parity. Number of 1s in the transmitted character and the attached parity bit will be even."]
    #[inline]
    pub fn even_parity(self) -> &'a mut W {
        self.variant(PSW::EVEN_PARITY)
    }
    #[doc = "Forced 1 stick parity."]
    #[inline]
    pub fn forced_1(self) -> &'a mut W {
        self.variant(PSW::FORCED_1)
    }
    #[doc = "Forced 0 stick parity."]
    #[inline]
    pub fn forced_0(self) -> &'a mut W {
        self.variant(PSW::FORCED_0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BC`"]
pub enum BCW {
    #[doc = "Disable break transmission."]
    DISABLE,
    #[doc = "Enable break transmission. Output pin UARTn TXD is forced to logic 0 when UnLCR[6] is active high."]
    ENABLE,
}
impl BCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BCW::DISABLE => false,
            BCW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BCW<'a> {
    w: &'a mut W,
}
impl<'a> _BCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable break transmission."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(BCW::DISABLE)
    }
    #[doc = "Enable break transmission. Output pin UARTn TXD is forced to logic 0 when UnLCR[6] is active high."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(BCW::ENABLE)
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
#[doc = "Values that can be written to the field `DLAB`"]
pub enum DLABW {
    #[doc = "Disable access to Divisor Latches."]
    DISABLE,
    #[doc = "Enable access to Divisor Latches."]
    ENABLE,
}
impl DLABW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DLABW::DISABLE => false,
            DLABW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DLABW<'a> {
    w: &'a mut W,
}
impl<'a> _DLABW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DLABW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable access to Divisor Latches."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(DLABW::DISABLE)
    }
    #[doc = "Enable access to Divisor Latches."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(DLABW::ENABLE)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Word Length Select."]
    #[inline]
    pub fn wls(&self) -> WLSR {
        WLSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Stop Bit Select"]
    #[inline]
    pub fn sbs(&self) -> SBSR {
        SBSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Parity Enable."]
    #[inline]
    pub fn pe(&self) -> PER {
        PER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:5 - Parity Select"]
    #[inline]
    pub fn ps(&self) -> PSR {
        PSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - Break Control"]
    #[inline]
    pub fn bc(&self) -> BCR {
        BCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Divisor Latch Access Bit"]
    #[inline]
    pub fn dlab(&self) -> DLABR {
        DLABR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
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
    #[doc = "Bits 0:1 - Word Length Select."]
    #[inline]
    pub fn wls(&mut self) -> _WLSW {
        _WLSW { w: self }
    }
    #[doc = "Bit 2 - Stop Bit Select"]
    #[inline]
    pub fn sbs(&mut self) -> _SBSW {
        _SBSW { w: self }
    }
    #[doc = "Bit 3 - Parity Enable."]
    #[inline]
    pub fn pe(&mut self) -> _PEW {
        _PEW { w: self }
    }
    #[doc = "Bits 4:5 - Parity Select"]
    #[inline]
    pub fn ps(&mut self) -> _PSW {
        _PSW { w: self }
    }
    #[doc = "Bit 6 - Break Control"]
    #[inline]
    pub fn bc(&mut self) -> _BCW {
        _BCW { w: self }
    }
    #[doc = "Bit 7 - Divisor Latch Access Bit"]
    #[inline]
    pub fn dlab(&mut self) -> _DLABW {
        _DLABW { w: self }
    }
}
