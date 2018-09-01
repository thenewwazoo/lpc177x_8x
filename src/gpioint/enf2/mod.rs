#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ENF2 {
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
pub struct P2_0EFR {
    bits: bool,
}
impl P2_0EFR {
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
pub struct P2_1EFR {
    bits: bool,
}
impl P2_1EFR {
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
pub struct P2_2EFR {
    bits: bool,
}
impl P2_2EFR {
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
pub struct P2_3EFR {
    bits: bool,
}
impl P2_3EFR {
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
pub struct P2_4EFR {
    bits: bool,
}
impl P2_4EFR {
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
pub struct P2_5EFR {
    bits: bool,
}
impl P2_5EFR {
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
pub struct P2_6EFR {
    bits: bool,
}
impl P2_6EFR {
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
pub struct P2_7EFR {
    bits: bool,
}
impl P2_7EFR {
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
pub struct P2_8EFR {
    bits: bool,
}
impl P2_8EFR {
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
pub struct P2_9EFR {
    bits: bool,
}
impl P2_9EFR {
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
pub struct P2_10EFR {
    bits: bool,
}
impl P2_10EFR {
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
pub struct P2_11EFR {
    bits: bool,
}
impl P2_11EFR {
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
pub struct P2_12EFR {
    bits: bool,
}
impl P2_12EFR {
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
pub struct P2_13EFR {
    bits: bool,
}
impl P2_13EFR {
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
pub struct P2_14EFR {
    bits: bool,
}
impl P2_14EFR {
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
pub struct P2_15EFR {
    bits: bool,
}
impl P2_15EFR {
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
pub struct P2_16EFR {
    bits: bool,
}
impl P2_16EFR {
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
pub struct P2_17EFR {
    bits: bool,
}
impl P2_17EFR {
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
pub struct P2_18EFR {
    bits: bool,
}
impl P2_18EFR {
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
pub struct P2_19EFR {
    bits: bool,
}
impl P2_19EFR {
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
pub struct P2_20EFR {
    bits: bool,
}
impl P2_20EFR {
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
pub struct P2_21EFR {
    bits: bool,
}
impl P2_21EFR {
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
pub struct P2_22EFR {
    bits: bool,
}
impl P2_22EFR {
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
pub struct P2_23EFR {
    bits: bool,
}
impl P2_23EFR {
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
pub struct P2_24EFR {
    bits: bool,
}
impl P2_24EFR {
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
pub struct P2_25EFR {
    bits: bool,
}
impl P2_25EFR {
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
pub struct P2_26EFR {
    bits: bool,
}
impl P2_26EFR {
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
pub struct P2_27EFR {
    bits: bool,
}
impl P2_27EFR {
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
pub struct P2_28EFR {
    bits: bool,
}
impl P2_28EFR {
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
pub struct P2_29EFR {
    bits: bool,
}
impl P2_29EFR {
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
pub struct P2_30EFR {
    bits: bool,
}
impl P2_30EFR {
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
pub struct P2_31EFR {
    bits: bool,
}
impl P2_31EFR {
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
pub struct _P2_0EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_0EFW<'a> {
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
pub struct _P2_1EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_1EFW<'a> {
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
pub struct _P2_2EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_2EFW<'a> {
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
pub struct _P2_3EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_3EFW<'a> {
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
pub struct _P2_4EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_4EFW<'a> {
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
pub struct _P2_5EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_5EFW<'a> {
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
pub struct _P2_6EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_6EFW<'a> {
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
pub struct _P2_7EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_7EFW<'a> {
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
pub struct _P2_8EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_8EFW<'a> {
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
pub struct _P2_9EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_9EFW<'a> {
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
pub struct _P2_10EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_10EFW<'a> {
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
pub struct _P2_11EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_11EFW<'a> {
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
pub struct _P2_12EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_12EFW<'a> {
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
pub struct _P2_13EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_13EFW<'a> {
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
pub struct _P2_14EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_14EFW<'a> {
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
pub struct _P2_15EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_15EFW<'a> {
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
pub struct _P2_16EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_16EFW<'a> {
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
pub struct _P2_17EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_17EFW<'a> {
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
pub struct _P2_18EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_18EFW<'a> {
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
pub struct _P2_19EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_19EFW<'a> {
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
pub struct _P2_20EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_20EFW<'a> {
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
pub struct _P2_21EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_21EFW<'a> {
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
pub struct _P2_22EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_22EFW<'a> {
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
pub struct _P2_23EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_23EFW<'a> {
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
pub struct _P2_24EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_24EFW<'a> {
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
pub struct _P2_25EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_25EFW<'a> {
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
pub struct _P2_26EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_26EFW<'a> {
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
pub struct _P2_27EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_27EFW<'a> {
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
pub struct _P2_28EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_28EFW<'a> {
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
pub struct _P2_29EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_29EFW<'a> {
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
pub struct _P2_30EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_30EFW<'a> {
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
pub struct _P2_31EFW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_31EFW<'a> {
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
    #[doc = "Bit 0 - Enable falling edge interrupt for P2[0]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_0ef(&self) -> P2_0EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_0EFR { bits }
    }
    #[doc = "Bit 1 - Enable falling edge interrupt for P2[1]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_1ef(&self) -> P2_1EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_1EFR { bits }
    }
    #[doc = "Bit 2 - Enable falling edge interrupt for P2[2]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_2ef(&self) -> P2_2EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_2EFR { bits }
    }
    #[doc = "Bit 3 - Enable falling edge interrupt for P2[3]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_3ef(&self) -> P2_3EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_3EFR { bits }
    }
    #[doc = "Bit 4 - Enable falling edge interrupt for P2[4]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_4ef(&self) -> P2_4EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_4EFR { bits }
    }
    #[doc = "Bit 5 - Enable falling edge interrupt for P2[5]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_5ef(&self) -> P2_5EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_5EFR { bits }
    }
    #[doc = "Bit 6 - Enable falling edge interrupt for P2[6]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_6ef(&self) -> P2_6EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_6EFR { bits }
    }
    #[doc = "Bit 7 - Enable falling edge interrupt for P2[7]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_7ef(&self) -> P2_7EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_7EFR { bits }
    }
    #[doc = "Bit 8 - Enable falling edge interrupt for P2[8]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_8ef(&self) -> P2_8EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_8EFR { bits }
    }
    #[doc = "Bit 9 - Enable falling edge interrupt for P2[9]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_9ef(&self) -> P2_9EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_9EFR { bits }
    }
    #[doc = "Bit 10 - Enable falling edge interrupt for P2[10]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_10ef(&self) -> P2_10EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_10EFR { bits }
    }
    #[doc = "Bit 11 - Enable falling edge interrupt for P2[11]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_11ef(&self) -> P2_11EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_11EFR { bits }
    }
    #[doc = "Bit 12 - Enable falling edge interrupt for P2[12]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_12ef(&self) -> P2_12EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_12EFR { bits }
    }
    #[doc = "Bit 13 - Enable falling edge interrupt for P2[13]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_13ef(&self) -> P2_13EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_13EFR { bits }
    }
    #[doc = "Bit 14 - Enable falling edge interrupt for P2[14]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_14ef(&self) -> P2_14EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_14EFR { bits }
    }
    #[doc = "Bit 15 - Enable falling edge interrupt for P2[15]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_15ef(&self) -> P2_15EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_15EFR { bits }
    }
    #[doc = "Bit 16 - Enable falling edge interrupt for P2[16]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_16ef(&self) -> P2_16EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_16EFR { bits }
    }
    #[doc = "Bit 17 - Enable falling edge interrupt for P2[17]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_17ef(&self) -> P2_17EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_17EFR { bits }
    }
    #[doc = "Bit 18 - Enable falling edge interrupt for P2[18]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_18ef(&self) -> P2_18EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_18EFR { bits }
    }
    #[doc = "Bit 19 - Enable falling edge interrupt for P2[19]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_19ef(&self) -> P2_19EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_19EFR { bits }
    }
    #[doc = "Bit 20 - Enable falling edge interrupt for P2[20]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_20ef(&self) -> P2_20EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_20EFR { bits }
    }
    #[doc = "Bit 21 - Enable falling edge interrupt for P2[21]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_21ef(&self) -> P2_21EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_21EFR { bits }
    }
    #[doc = "Bit 22 - Enable falling edge interrupt for P2[22]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_22ef(&self) -> P2_22EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_22EFR { bits }
    }
    #[doc = "Bit 23 - Enable falling edge interrupt for P2[23]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_23ef(&self) -> P2_23EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_23EFR { bits }
    }
    #[doc = "Bit 24 - Enable falling edge interrupt for P2[24]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_24ef(&self) -> P2_24EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_24EFR { bits }
    }
    #[doc = "Bit 25 - Enable falling edge interrupt for P2[25]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_25ef(&self) -> P2_25EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_25EFR { bits }
    }
    #[doc = "Bit 26 - Enable falling edge interrupt for P2[26]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_26ef(&self) -> P2_26EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_26EFR { bits }
    }
    #[doc = "Bit 27 - Enable falling edge interrupt for P2[27]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_27ef(&self) -> P2_27EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_27EFR { bits }
    }
    #[doc = "Bit 28 - Enable falling edge interrupt for P2[28]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_28ef(&self) -> P2_28EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_28EFR { bits }
    }
    #[doc = "Bit 29 - Enable falling edge interrupt for P2[29]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_29ef(&self) -> P2_29EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_29EFR { bits }
    }
    #[doc = "Bit 30 - Enable falling edge interrupt for P2[30]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_30ef(&self) -> P2_30EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_30EFR { bits }
    }
    #[doc = "Bit 31 - Enable falling edge interrupt for P2[31]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_31ef(&self) -> P2_31EFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P2_31EFR { bits }
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
    #[doc = "Bit 0 - Enable falling edge interrupt for P2[0]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_0ef(&mut self) -> _P2_0EFW {
        _P2_0EFW { w: self }
    }
    #[doc = "Bit 1 - Enable falling edge interrupt for P2[1]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_1ef(&mut self) -> _P2_1EFW {
        _P2_1EFW { w: self }
    }
    #[doc = "Bit 2 - Enable falling edge interrupt for P2[2]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_2ef(&mut self) -> _P2_2EFW {
        _P2_2EFW { w: self }
    }
    #[doc = "Bit 3 - Enable falling edge interrupt for P2[3]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_3ef(&mut self) -> _P2_3EFW {
        _P2_3EFW { w: self }
    }
    #[doc = "Bit 4 - Enable falling edge interrupt for P2[4]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_4ef(&mut self) -> _P2_4EFW {
        _P2_4EFW { w: self }
    }
    #[doc = "Bit 5 - Enable falling edge interrupt for P2[5]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_5ef(&mut self) -> _P2_5EFW {
        _P2_5EFW { w: self }
    }
    #[doc = "Bit 6 - Enable falling edge interrupt for P2[6]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_6ef(&mut self) -> _P2_6EFW {
        _P2_6EFW { w: self }
    }
    #[doc = "Bit 7 - Enable falling edge interrupt for P2[7]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_7ef(&mut self) -> _P2_7EFW {
        _P2_7EFW { w: self }
    }
    #[doc = "Bit 8 - Enable falling edge interrupt for P2[8]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_8ef(&mut self) -> _P2_8EFW {
        _P2_8EFW { w: self }
    }
    #[doc = "Bit 9 - Enable falling edge interrupt for P2[9]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_9ef(&mut self) -> _P2_9EFW {
        _P2_9EFW { w: self }
    }
    #[doc = "Bit 10 - Enable falling edge interrupt for P2[10]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_10ef(&mut self) -> _P2_10EFW {
        _P2_10EFW { w: self }
    }
    #[doc = "Bit 11 - Enable falling edge interrupt for P2[11]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_11ef(&mut self) -> _P2_11EFW {
        _P2_11EFW { w: self }
    }
    #[doc = "Bit 12 - Enable falling edge interrupt for P2[12]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_12ef(&mut self) -> _P2_12EFW {
        _P2_12EFW { w: self }
    }
    #[doc = "Bit 13 - Enable falling edge interrupt for P2[13]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_13ef(&mut self) -> _P2_13EFW {
        _P2_13EFW { w: self }
    }
    #[doc = "Bit 14 - Enable falling edge interrupt for P2[14]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_14ef(&mut self) -> _P2_14EFW {
        _P2_14EFW { w: self }
    }
    #[doc = "Bit 15 - Enable falling edge interrupt for P2[15]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_15ef(&mut self) -> _P2_15EFW {
        _P2_15EFW { w: self }
    }
    #[doc = "Bit 16 - Enable falling edge interrupt for P2[16]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_16ef(&mut self) -> _P2_16EFW {
        _P2_16EFW { w: self }
    }
    #[doc = "Bit 17 - Enable falling edge interrupt for P2[17]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_17ef(&mut self) -> _P2_17EFW {
        _P2_17EFW { w: self }
    }
    #[doc = "Bit 18 - Enable falling edge interrupt for P2[18]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_18ef(&mut self) -> _P2_18EFW {
        _P2_18EFW { w: self }
    }
    #[doc = "Bit 19 - Enable falling edge interrupt for P2[19]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_19ef(&mut self) -> _P2_19EFW {
        _P2_19EFW { w: self }
    }
    #[doc = "Bit 20 - Enable falling edge interrupt for P2[20]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_20ef(&mut self) -> _P2_20EFW {
        _P2_20EFW { w: self }
    }
    #[doc = "Bit 21 - Enable falling edge interrupt for P2[21]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_21ef(&mut self) -> _P2_21EFW {
        _P2_21EFW { w: self }
    }
    #[doc = "Bit 22 - Enable falling edge interrupt for P2[22]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_22ef(&mut self) -> _P2_22EFW {
        _P2_22EFW { w: self }
    }
    #[doc = "Bit 23 - Enable falling edge interrupt for P2[23]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_23ef(&mut self) -> _P2_23EFW {
        _P2_23EFW { w: self }
    }
    #[doc = "Bit 24 - Enable falling edge interrupt for P2[24]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_24ef(&mut self) -> _P2_24EFW {
        _P2_24EFW { w: self }
    }
    #[doc = "Bit 25 - Enable falling edge interrupt for P2[25]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_25ef(&mut self) -> _P2_25EFW {
        _P2_25EFW { w: self }
    }
    #[doc = "Bit 26 - Enable falling edge interrupt for P2[26]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_26ef(&mut self) -> _P2_26EFW {
        _P2_26EFW { w: self }
    }
    #[doc = "Bit 27 - Enable falling edge interrupt for P2[27]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_27ef(&mut self) -> _P2_27EFW {
        _P2_27EFW { w: self }
    }
    #[doc = "Bit 28 - Enable falling edge interrupt for P2[28]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_28ef(&mut self) -> _P2_28EFW {
        _P2_28EFW { w: self }
    }
    #[doc = "Bit 29 - Enable falling edge interrupt for P2[29]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_29ef(&mut self) -> _P2_29EFW {
        _P2_29EFW { w: self }
    }
    #[doc = "Bit 30 - Enable falling edge interrupt for P2[30]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_30ef(&mut self) -> _P2_30EFW {
        _P2_30EFW { w: self }
    }
    #[doc = "Bit 31 - Enable falling edge interrupt for P2[31]. 0 = Disable falling edge interrupt. 1 = Enable falling edge interrupt."]
    #[inline]
    pub fn p2_31ef(&mut self) -> _P2_31EFW {
        _P2_31EFW { w: self }
    }
}
