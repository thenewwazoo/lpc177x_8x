#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MODE {
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
#[doc = "Possible values of the field `CRC_POLY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_POLYR {
    #[doc = "CRC-CCITT polynomial"]
    CRC_CCITT_POLYNOMIAL,
    #[doc = "CRC-16 polynomial"]
    CRC_16_POLYNOMIAL,
    #[doc = "CRC-32 polynomial"]
    CRC_32_POLYNOMIAL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CRC_POLYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CRC_POLYR::CRC_CCITT_POLYNOMIAL => 0,
            CRC_POLYR::CRC_16_POLYNOMIAL => 1,
            CRC_POLYR::CRC_32_POLYNOMIAL => 2,
            CRC_POLYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CRC_POLYR {
        match value {
            0 => CRC_POLYR::CRC_CCITT_POLYNOMIAL,
            1 => CRC_POLYR::CRC_16_POLYNOMIAL,
            2 => CRC_POLYR::CRC_32_POLYNOMIAL,
            i => CRC_POLYR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CRC_CCITT_POLYNOMIAL`"]
    #[inline]
    pub fn is_crc_ccitt_polynomial(&self) -> bool {
        *self == CRC_POLYR::CRC_CCITT_POLYNOMIAL
    }
    #[doc = "Checks if the value of the field is `CRC_16_POLYNOMIAL`"]
    #[inline]
    pub fn is_crc_16_polynomial(&self) -> bool {
        *self == CRC_POLYR::CRC_16_POLYNOMIAL
    }
    #[doc = "Checks if the value of the field is `CRC_32_POLYNOMIAL`"]
    #[inline]
    pub fn is_crc_32_polynomial(&self) -> bool {
        *self == CRC_POLYR::CRC_32_POLYNOMIAL
    }
}
#[doc = "Possible values of the field `BIT_RVS_WR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIT_RVS_WRR {
    #[doc = "No bit order reverse for CRC_WR_DATA (per byte)"]
    NO_BIT_ORDER_REVERSE,
    #[doc = "Bit order reverse for CRC_WR_DATA (per byte)"]
    BIT_ORDER_REVERSED,
}
impl BIT_RVS_WRR {
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
            BIT_RVS_WRR::NO_BIT_ORDER_REVERSE => false,
            BIT_RVS_WRR::BIT_ORDER_REVERSED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BIT_RVS_WRR {
        match value {
            false => BIT_RVS_WRR::NO_BIT_ORDER_REVERSE,
            true => BIT_RVS_WRR::BIT_ORDER_REVERSED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_BIT_ORDER_REVERSE`"]
    #[inline]
    pub fn is_no_bit_order_reverse(&self) -> bool {
        *self == BIT_RVS_WRR::NO_BIT_ORDER_REVERSE
    }
    #[doc = "Checks if the value of the field is `BIT_ORDER_REVERSED`"]
    #[inline]
    pub fn is_bit_order_reversed(&self) -> bool {
        *self == BIT_RVS_WRR::BIT_ORDER_REVERSED
    }
}
#[doc = "Possible values of the field `CMPL_WR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPL_WRR {
    #[doc = "No one's complement for CRC_WR_DATA"]
    NO_ONES_COMPLEMENT,
    #[doc = "One's complement for CRC_WR_DATA"]
    ONES_COMPLEMENT,
}
impl CMPL_WRR {
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
            CMPL_WRR::NO_ONES_COMPLEMENT => false,
            CMPL_WRR::ONES_COMPLEMENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMPL_WRR {
        match value {
            false => CMPL_WRR::NO_ONES_COMPLEMENT,
            true => CMPL_WRR::ONES_COMPLEMENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ONES_COMPLEMENT`"]
    #[inline]
    pub fn is_no_ones_complement(&self) -> bool {
        *self == CMPL_WRR::NO_ONES_COMPLEMENT
    }
    #[doc = "Checks if the value of the field is `ONES_COMPLEMENT`"]
    #[inline]
    pub fn is_ones_complement(&self) -> bool {
        *self == CMPL_WRR::ONES_COMPLEMENT
    }
}
#[doc = "Possible values of the field `BIT_RVS_SUM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIT_RVS_SUMR {
    #[doc = "No bit order reverse for CRC_SUM"]
    NO_BIT_ORDER_REVERSED,
    #[doc = "Bit order reverse for CRC_SUM"]
    BIT_ORDER_REVERSED,
}
impl BIT_RVS_SUMR {
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
            BIT_RVS_SUMR::NO_BIT_ORDER_REVERSED => false,
            BIT_RVS_SUMR::BIT_ORDER_REVERSED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BIT_RVS_SUMR {
        match value {
            false => BIT_RVS_SUMR::NO_BIT_ORDER_REVERSED,
            true => BIT_RVS_SUMR::BIT_ORDER_REVERSED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_BIT_ORDER_REVERSED`"]
    #[inline]
    pub fn is_no_bit_order_reversed(&self) -> bool {
        *self == BIT_RVS_SUMR::NO_BIT_ORDER_REVERSED
    }
    #[doc = "Checks if the value of the field is `BIT_ORDER_REVERSED`"]
    #[inline]
    pub fn is_bit_order_reversed(&self) -> bool {
        *self == BIT_RVS_SUMR::BIT_ORDER_REVERSED
    }
}
#[doc = "Possible values of the field `CMPL_SUM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPL_SUMR {
    #[doc = "No one's complement for CRC_SUM"]
    NO_ONES_COMPLEMENT,
    #[doc = "One's complement for CRC_SUM"]
    ONES_COMPLEMENT,
}
impl CMPL_SUMR {
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
            CMPL_SUMR::NO_ONES_COMPLEMENT => false,
            CMPL_SUMR::ONES_COMPLEMENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMPL_SUMR {
        match value {
            false => CMPL_SUMR::NO_ONES_COMPLEMENT,
            true => CMPL_SUMR::ONES_COMPLEMENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ONES_COMPLEMENT`"]
    #[inline]
    pub fn is_no_ones_complement(&self) -> bool {
        *self == CMPL_SUMR::NO_ONES_COMPLEMENT
    }
    #[doc = "Checks if the value of the field is `ONES_COMPLEMENT`"]
    #[inline]
    pub fn is_ones_complement(&self) -> bool {
        *self == CMPL_SUMR::ONES_COMPLEMENT
    }
}
#[doc = "Values that can be written to the field `CRC_POLY`"]
pub enum CRC_POLYW {
    #[doc = "CRC-CCITT polynomial"]
    CRC_CCITT_POLYNOMIAL,
    #[doc = "CRC-16 polynomial"]
    CRC_16_POLYNOMIAL,
    #[doc = "CRC-32 polynomial"]
    CRC_32_POLYNOMIAL,
}
impl CRC_POLYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CRC_POLYW::CRC_CCITT_POLYNOMIAL => 0,
            CRC_POLYW::CRC_16_POLYNOMIAL => 1,
            CRC_POLYW::CRC_32_POLYNOMIAL => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRC_POLYW<'a> {
    w: &'a mut W,
}
impl<'a> _CRC_POLYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRC_POLYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "CRC-CCITT polynomial"]
    #[inline]
    pub fn crc_ccitt_polynomial(self) -> &'a mut W {
        self.variant(CRC_POLYW::CRC_CCITT_POLYNOMIAL)
    }
    #[doc = "CRC-16 polynomial"]
    #[inline]
    pub fn crc_16_polynomial(self) -> &'a mut W {
        self.variant(CRC_POLYW::CRC_16_POLYNOMIAL)
    }
    #[doc = "CRC-32 polynomial"]
    #[inline]
    pub fn crc_32_polynomial(self) -> &'a mut W {
        self.variant(CRC_POLYW::CRC_32_POLYNOMIAL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BIT_RVS_WR`"]
pub enum BIT_RVS_WRW {
    #[doc = "No bit order reverse for CRC_WR_DATA (per byte)"]
    NO_BIT_ORDER_REVERSE,
    #[doc = "Bit order reverse for CRC_WR_DATA (per byte)"]
    BIT_ORDER_REVERSED,
}
impl BIT_RVS_WRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BIT_RVS_WRW::NO_BIT_ORDER_REVERSE => false,
            BIT_RVS_WRW::BIT_ORDER_REVERSED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BIT_RVS_WRW<'a> {
    w: &'a mut W,
}
impl<'a> _BIT_RVS_WRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BIT_RVS_WRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No bit order reverse for CRC_WR_DATA (per byte)"]
    #[inline]
    pub fn no_bit_order_reverse(self) -> &'a mut W {
        self.variant(BIT_RVS_WRW::NO_BIT_ORDER_REVERSE)
    }
    #[doc = "Bit order reverse for CRC_WR_DATA (per byte)"]
    #[inline]
    pub fn bit_order_reversed(self) -> &'a mut W {
        self.variant(BIT_RVS_WRW::BIT_ORDER_REVERSED)
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
#[doc = "Values that can be written to the field `CMPL_WR`"]
pub enum CMPL_WRW {
    #[doc = "No one's complement for CRC_WR_DATA"]
    NO_ONES_COMPLEMENT,
    #[doc = "One's complement for CRC_WR_DATA"]
    ONES_COMPLEMENT,
}
impl CMPL_WRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMPL_WRW::NO_ONES_COMPLEMENT => false,
            CMPL_WRW::ONES_COMPLEMENT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMPL_WRW<'a> {
    w: &'a mut W,
}
impl<'a> _CMPL_WRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMPL_WRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No one's complement for CRC_WR_DATA"]
    #[inline]
    pub fn no_ones_complement(self) -> &'a mut W {
        self.variant(CMPL_WRW::NO_ONES_COMPLEMENT)
    }
    #[doc = "One's complement for CRC_WR_DATA"]
    #[inline]
    pub fn ones_complement(self) -> &'a mut W {
        self.variant(CMPL_WRW::ONES_COMPLEMENT)
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
#[doc = "Values that can be written to the field `BIT_RVS_SUM`"]
pub enum BIT_RVS_SUMW {
    #[doc = "No bit order reverse for CRC_SUM"]
    NO_BIT_ORDER_REVERSED,
    #[doc = "Bit order reverse for CRC_SUM"]
    BIT_ORDER_REVERSED,
}
impl BIT_RVS_SUMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BIT_RVS_SUMW::NO_BIT_ORDER_REVERSED => false,
            BIT_RVS_SUMW::BIT_ORDER_REVERSED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BIT_RVS_SUMW<'a> {
    w: &'a mut W,
}
impl<'a> _BIT_RVS_SUMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BIT_RVS_SUMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No bit order reverse for CRC_SUM"]
    #[inline]
    pub fn no_bit_order_reversed(self) -> &'a mut W {
        self.variant(BIT_RVS_SUMW::NO_BIT_ORDER_REVERSED)
    }
    #[doc = "Bit order reverse for CRC_SUM"]
    #[inline]
    pub fn bit_order_reversed(self) -> &'a mut W {
        self.variant(BIT_RVS_SUMW::BIT_ORDER_REVERSED)
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
#[doc = "Values that can be written to the field `CMPL_SUM`"]
pub enum CMPL_SUMW {
    #[doc = "No one's complement for CRC_SUM"]
    NO_ONES_COMPLEMENT,
    #[doc = "One's complement for CRC_SUM"]
    ONES_COMPLEMENT,
}
impl CMPL_SUMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMPL_SUMW::NO_ONES_COMPLEMENT => false,
            CMPL_SUMW::ONES_COMPLEMENT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMPL_SUMW<'a> {
    w: &'a mut W,
}
impl<'a> _CMPL_SUMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMPL_SUMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No one's complement for CRC_SUM"]
    #[inline]
    pub fn no_ones_complement(self) -> &'a mut W {
        self.variant(CMPL_SUMW::NO_ONES_COMPLEMENT)
    }
    #[doc = "One's complement for CRC_SUM"]
    #[inline]
    pub fn ones_complement(self) -> &'a mut W {
        self.variant(CMPL_SUMW::ONES_COMPLEMENT)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Select CRC polynomial"]
    #[inline]
    pub fn crc_poly(&self) -> CRC_POLYR {
        CRC_POLYR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Select bit order for CRC_WR_DATA"]
    #[inline]
    pub fn bit_rvs_wr(&self) -> BIT_RVS_WRR {
        BIT_RVS_WRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Select one's complement for CRC_WR_DATA"]
    #[inline]
    pub fn cmpl_wr(&self) -> CMPL_WRR {
        CMPL_WRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Select bit order revers for CRC_SUM"]
    #[inline]
    pub fn bit_rvs_sum(&self) -> BIT_RVS_SUMR {
        BIT_RVS_SUMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Select one's complement for CRC_SUM"]
    #[inline]
    pub fn cmpl_sum(&self) -> CMPL_SUMR {
        CMPL_SUMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
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
    #[doc = "Bits 0:1 - Select CRC polynomial"]
    #[inline]
    pub fn crc_poly(&mut self) -> _CRC_POLYW {
        _CRC_POLYW { w: self }
    }
    #[doc = "Bit 2 - Select bit order for CRC_WR_DATA"]
    #[inline]
    pub fn bit_rvs_wr(&mut self) -> _BIT_RVS_WRW {
        _BIT_RVS_WRW { w: self }
    }
    #[doc = "Bit 3 - Select one's complement for CRC_WR_DATA"]
    #[inline]
    pub fn cmpl_wr(&mut self) -> _CMPL_WRW {
        _CMPL_WRW { w: self }
    }
    #[doc = "Bit 4 - Select bit order revers for CRC_SUM"]
    #[inline]
    pub fn bit_rvs_sum(&mut self) -> _BIT_RVS_SUMW {
        _BIT_RVS_SUMW { w: self }
    }
    #[doc = "Bit 5 - Select one's complement for CRC_SUM"]
    #[inline]
    pub fn cmpl_sum(&mut self) -> _CMPL_SUMW {
        _CMPL_SUMW { w: self }
    }
}
