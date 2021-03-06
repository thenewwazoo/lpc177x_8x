#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ENR2 {
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
pub struct P2_0ERR {
    bits: bool,
}
impl P2_0ERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct P2_1ERR {
    bits: bool,
}
impl P2_1ERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct P2_2ERR {
    bits: bool,
}
impl P2_2ERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct P2_3ERR {
    bits: bool,
}
impl P2_3ERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct P2_4ERR {
    bits: bool,
}
impl P2_4ERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct P2_5ERR {
    bits: bool,
}
impl P2_5ERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct P2_6ERR {
    bits: bool,
}
impl P2_6ERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct P2_7ERR {
    bits: bool,
}
impl P2_7ERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct P2_8ERR {
    bits: bool,
}
impl P2_8ERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct P2_9ERR {
    bits: bool,
}
impl P2_9ERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct P2_10ERR {
    bits: bool,
}
impl P2_10ERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct P2_11ERR {
    bits: bool,
}
impl P2_11ERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct P2_12ERR {
    bits: bool,
}
impl P2_12ERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct P2_13ERR {
    bits: bool,
}
impl P2_13ERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct P2_14ERR {
    bits: bool,
}
impl P2_14ERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct P2_15ERR {
    bits: bool,
}
impl P2_15ERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct P2_16ERR {
    bits: bool,
}
impl P2_16ERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct P2_17ERR {
    bits: bool,
}
impl P2_17ERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct P2_18ERR {
    bits: bool,
}
impl P2_18ERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct P2_19ERR {
    bits: bool,
}
impl P2_19ERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct P2_20ERR {
    bits: bool,
}
impl P2_20ERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct P2_21ERR {
    bits: bool,
}
impl P2_21ERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct P2_22ERR {
    bits: bool,
}
impl P2_22ERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct P2_23ERR {
    bits: bool,
}
impl P2_23ERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct P2_24ERR {
    bits: bool,
}
impl P2_24ERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct P2_25ERR {
    bits: bool,
}
impl P2_25ERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct P2_26ERR {
    bits: bool,
}
impl P2_26ERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct P2_27ERR {
    bits: bool,
}
impl P2_27ERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct P2_28ERR {
    bits: bool,
}
impl P2_28ERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct P2_29ERR {
    bits: bool,
}
impl P2_29ERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct P2_30ERR {
    bits: bool,
}
impl P2_30ERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct P2_31ERR {
    bits: bool,
}
impl P2_31ERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Proxy"]
pub struct _P2_0ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_0ERW<'a> {
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
pub struct _P2_1ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_1ERW<'a> {
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
pub struct _P2_2ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_2ERW<'a> {
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
pub struct _P2_3ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_3ERW<'a> {
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
pub struct _P2_4ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_4ERW<'a> {
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
pub struct _P2_5ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_5ERW<'a> {
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
pub struct _P2_6ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_6ERW<'a> {
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
pub struct _P2_7ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_7ERW<'a> {
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
pub struct _P2_8ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_8ERW<'a> {
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
pub struct _P2_9ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_9ERW<'a> {
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
pub struct _P2_10ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_10ERW<'a> {
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
#[doc = r" Proxy"]
pub struct _P2_11ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_11ERW<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _P2_12ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_12ERW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _P2_13ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_13ERW<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _P2_14ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_14ERW<'a> {
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _P2_15ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_15ERW<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _P2_16ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_16ERW<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _P2_17ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_17ERW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _P2_18ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_18ERW<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _P2_19ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_19ERW<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _P2_20ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_20ERW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _P2_21ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_21ERW<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _P2_22ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_22ERW<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _P2_23ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_23ERW<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _P2_24ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_24ERW<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _P2_25ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_25ERW<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _P2_26ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_26ERW<'a> {
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
pub struct _P2_27ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_27ERW<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _P2_28ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_28ERW<'a> {
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
#[doc = r" Proxy"]
pub struct _P2_29ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_29ERW<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _P2_30ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_30ERW<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _P2_31ERW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_31ERW<'a> {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Enable rising edge interrupt for P2[0]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_0er(&self) -> P2_0ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_0ERR { bits }
    }
    #[doc = "Bit 1 - Enable rising edge interrupt for P2[1]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_1er(&self) -> P2_1ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_1ERR { bits }
    }
    #[doc = "Bit 2 - Enable rising edge interrupt for P2[2]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_2er(&self) -> P2_2ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_2ERR { bits }
    }
    #[doc = "Bit 3 - Enable rising edge interrupt for P2[3]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_3er(&self) -> P2_3ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_3ERR { bits }
    }
    #[doc = "Bit 4 - Enable rising edge interrupt for P2[4]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_4er(&self) -> P2_4ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_4ERR { bits }
    }
    #[doc = "Bit 5 - Enable rising edge interrupt for P2[5]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_5er(&self) -> P2_5ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_5ERR { bits }
    }
    #[doc = "Bit 6 - Enable rising edge interrupt for P2[6]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_6er(&self) -> P2_6ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_6ERR { bits }
    }
    #[doc = "Bit 7 - Enable rising edge interrupt for P2[7]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_7er(&self) -> P2_7ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_7ERR { bits }
    }
    #[doc = "Bit 8 - Enable rising edge interrupt for P2[8]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_8er(&self) -> P2_8ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_8ERR { bits }
    }
    #[doc = "Bit 9 - Enable rising edge interrupt for P2[9]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_9er(&self) -> P2_9ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_9ERR { bits }
    }
    #[doc = "Bit 10 - Enable rising edge interrupt for P2[10]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_10er(&self) -> P2_10ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_10ERR { bits }
    }
    #[doc = "Bit 11 - Enable rising edge interrupt for P2[11]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_11er(&self) -> P2_11ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_11ERR { bits }
    }
    #[doc = "Bit 12 - Enable rising edge interrupt for P2[12]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_12er(&self) -> P2_12ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_12ERR { bits }
    }
    #[doc = "Bit 13 - Enable rising edge interrupt for P2[13]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_13er(&self) -> P2_13ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_13ERR { bits }
    }
    #[doc = "Bit 14 - Enable rising edge interrupt for P2[14]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_14er(&self) -> P2_14ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_14ERR { bits }
    }
    #[doc = "Bit 15 - Enable rising edge interrupt for P2[15]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_15er(&self) -> P2_15ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_15ERR { bits }
    }
    #[doc = "Bit 16 - Enable rising edge interrupt for P2[16]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_16er(&self) -> P2_16ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_16ERR { bits }
    }
    #[doc = "Bit 17 - Enable rising edge interrupt for P2[17]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_17er(&self) -> P2_17ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_17ERR { bits }
    }
    #[doc = "Bit 18 - Enable rising edge interrupt for P2[18]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_18er(&self) -> P2_18ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_18ERR { bits }
    }
    #[doc = "Bit 19 - Enable rising edge interrupt for P2[19]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_19er(&self) -> P2_19ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_19ERR { bits }
    }
    #[doc = "Bit 20 - Enable rising edge interrupt for P2[20]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_20er(&self) -> P2_20ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_20ERR { bits }
    }
    #[doc = "Bit 21 - Enable rising edge interrupt for P2[21]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_21er(&self) -> P2_21ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_21ERR { bits }
    }
    #[doc = "Bit 22 - Enable rising edge interrupt for P2[22]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_22er(&self) -> P2_22ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_22ERR { bits }
    }
    #[doc = "Bit 23 - Enable rising edge interrupt for P2[23]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_23er(&self) -> P2_23ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_23ERR { bits }
    }
    #[doc = "Bit 24 - Enable rising edge interrupt for P2[24]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_24er(&self) -> P2_24ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_24ERR { bits }
    }
    #[doc = "Bit 25 - Enable rising edge interrupt for P2[25]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_25er(&self) -> P2_25ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_25ERR { bits }
    }
    #[doc = "Bit 26 - Enable rising edge interrupt for P2[26]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_26er(&self) -> P2_26ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_26ERR { bits }
    }
    #[doc = "Bit 27 - Enable rising edge interrupt for P2[27]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_27er(&self) -> P2_27ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_27ERR { bits }
    }
    #[doc = "Bit 28 - Enable rising edge interrupt for P2[28]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_28er(&self) -> P2_28ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_28ERR { bits }
    }
    #[doc = "Bit 29 - Enable rising edge interrupt for P2[29]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_29er(&self) -> P2_29ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_29ERR { bits }
    }
    #[doc = "Bit 30 - Enable rising edge interrupt for P2[30]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_30er(&self) -> P2_30ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_30ERR { bits }
    }
    #[doc = "Bit 31 - Enable rising edge interrupt for P2[31]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_31er(&self) -> P2_31ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_31ERR { bits }
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
    #[doc = "Bit 0 - Enable rising edge interrupt for P2[0]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_0er(&mut self) -> _P2_0ERW {
        _P2_0ERW { w: self }
    }
    #[doc = "Bit 1 - Enable rising edge interrupt for P2[1]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_1er(&mut self) -> _P2_1ERW {
        _P2_1ERW { w: self }
    }
    #[doc = "Bit 2 - Enable rising edge interrupt for P2[2]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_2er(&mut self) -> _P2_2ERW {
        _P2_2ERW { w: self }
    }
    #[doc = "Bit 3 - Enable rising edge interrupt for P2[3]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_3er(&mut self) -> _P2_3ERW {
        _P2_3ERW { w: self }
    }
    #[doc = "Bit 4 - Enable rising edge interrupt for P2[4]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_4er(&mut self) -> _P2_4ERW {
        _P2_4ERW { w: self }
    }
    #[doc = "Bit 5 - Enable rising edge interrupt for P2[5]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_5er(&mut self) -> _P2_5ERW {
        _P2_5ERW { w: self }
    }
    #[doc = "Bit 6 - Enable rising edge interrupt for P2[6]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_6er(&mut self) -> _P2_6ERW {
        _P2_6ERW { w: self }
    }
    #[doc = "Bit 7 - Enable rising edge interrupt for P2[7]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_7er(&mut self) -> _P2_7ERW {
        _P2_7ERW { w: self }
    }
    #[doc = "Bit 8 - Enable rising edge interrupt for P2[8]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_8er(&mut self) -> _P2_8ERW {
        _P2_8ERW { w: self }
    }
    #[doc = "Bit 9 - Enable rising edge interrupt for P2[9]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_9er(&mut self) -> _P2_9ERW {
        _P2_9ERW { w: self }
    }
    #[doc = "Bit 10 - Enable rising edge interrupt for P2[10]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_10er(&mut self) -> _P2_10ERW {
        _P2_10ERW { w: self }
    }
    #[doc = "Bit 11 - Enable rising edge interrupt for P2[11]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_11er(&mut self) -> _P2_11ERW {
        _P2_11ERW { w: self }
    }
    #[doc = "Bit 12 - Enable rising edge interrupt for P2[12]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_12er(&mut self) -> _P2_12ERW {
        _P2_12ERW { w: self }
    }
    #[doc = "Bit 13 - Enable rising edge interrupt for P2[13]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_13er(&mut self) -> _P2_13ERW {
        _P2_13ERW { w: self }
    }
    #[doc = "Bit 14 - Enable rising edge interrupt for P2[14]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_14er(&mut self) -> _P2_14ERW {
        _P2_14ERW { w: self }
    }
    #[doc = "Bit 15 - Enable rising edge interrupt for P2[15]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_15er(&mut self) -> _P2_15ERW {
        _P2_15ERW { w: self }
    }
    #[doc = "Bit 16 - Enable rising edge interrupt for P2[16]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_16er(&mut self) -> _P2_16ERW {
        _P2_16ERW { w: self }
    }
    #[doc = "Bit 17 - Enable rising edge interrupt for P2[17]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_17er(&mut self) -> _P2_17ERW {
        _P2_17ERW { w: self }
    }
    #[doc = "Bit 18 - Enable rising edge interrupt for P2[18]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_18er(&mut self) -> _P2_18ERW {
        _P2_18ERW { w: self }
    }
    #[doc = "Bit 19 - Enable rising edge interrupt for P2[19]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_19er(&mut self) -> _P2_19ERW {
        _P2_19ERW { w: self }
    }
    #[doc = "Bit 20 - Enable rising edge interrupt for P2[20]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_20er(&mut self) -> _P2_20ERW {
        _P2_20ERW { w: self }
    }
    #[doc = "Bit 21 - Enable rising edge interrupt for P2[21]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_21er(&mut self) -> _P2_21ERW {
        _P2_21ERW { w: self }
    }
    #[doc = "Bit 22 - Enable rising edge interrupt for P2[22]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_22er(&mut self) -> _P2_22ERW {
        _P2_22ERW { w: self }
    }
    #[doc = "Bit 23 - Enable rising edge interrupt for P2[23]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_23er(&mut self) -> _P2_23ERW {
        _P2_23ERW { w: self }
    }
    #[doc = "Bit 24 - Enable rising edge interrupt for P2[24]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_24er(&mut self) -> _P2_24ERW {
        _P2_24ERW { w: self }
    }
    #[doc = "Bit 25 - Enable rising edge interrupt for P2[25]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_25er(&mut self) -> _P2_25ERW {
        _P2_25ERW { w: self }
    }
    #[doc = "Bit 26 - Enable rising edge interrupt for P2[26]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_26er(&mut self) -> _P2_26ERW {
        _P2_26ERW { w: self }
    }
    #[doc = "Bit 27 - Enable rising edge interrupt for P2[27]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_27er(&mut self) -> _P2_27ERW {
        _P2_27ERW { w: self }
    }
    #[doc = "Bit 28 - Enable rising edge interrupt for P2[28]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_28er(&mut self) -> _P2_28ERW {
        _P2_28ERW { w: self }
    }
    #[doc = "Bit 29 - Enable rising edge interrupt for P2[29]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_29er(&mut self) -> _P2_29ERW {
        _P2_29ERW { w: self }
    }
    #[doc = "Bit 30 - Enable rising edge interrupt for P2[30]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_30er(&mut self) -> _P2_30ERW {
        _P2_30ERW { w: self }
    }
    #[doc = "Bit 31 - Enable rising edge interrupt for P2[31]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline]
    pub fn p2_31er(&mut self) -> _P2_31ERW {
        _P2_31ERW { w: self }
    }
}
