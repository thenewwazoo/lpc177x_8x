#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DATACTRL {
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
pub struct ENABLER {
    bits: bool,
}
impl ENABLER {
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
#[doc = "Possible values of the field `DIRECTION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRECTIONR {
    #[doc = "From controller to card."]
    CONTROLLER_TO_CARD,
    #[doc = "From card to controller."]
    CARD_TO_CONTROLLER,
}
impl DIRECTIONR {
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
            DIRECTIONR::CONTROLLER_TO_CARD => false,
            DIRECTIONR::CARD_TO_CONTROLLER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIRECTIONR {
        match value {
            false => DIRECTIONR::CONTROLLER_TO_CARD,
            true => DIRECTIONR::CARD_TO_CONTROLLER,
        }
    }
    #[doc = "Checks if the value of the field is `CONTROLLER_TO_CARD`"]
    #[inline]
    pub fn is_controller_to_card(&self) -> bool {
        *self == DIRECTIONR::CONTROLLER_TO_CARD
    }
    #[doc = "Checks if the value of the field is `CARD_TO_CONTROLLER`"]
    #[inline]
    pub fn is_card_to_controller(&self) -> bool {
        *self == DIRECTIONR::CARD_TO_CONTROLLER
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Block data transfer."]
    BLOCK_DATA_TRANSFER,
    #[doc = "Stream data transfer."]
    STREAM_DATA_TRANSFER,
}
impl MODER {
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
            MODER::BLOCK_DATA_TRANSFER => false,
            MODER::STREAM_DATA_TRANSFER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MODER {
        match value {
            false => MODER::BLOCK_DATA_TRANSFER,
            true => MODER::STREAM_DATA_TRANSFER,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCK_DATA_TRANSFER`"]
    #[inline]
    pub fn is_block_data_transfer(&self) -> bool {
        *self == MODER::BLOCK_DATA_TRANSFER
    }
    #[doc = "Checks if the value of the field is `STREAM_DATA_TRANSFER`"]
    #[inline]
    pub fn is_stream_data_transfer(&self) -> bool {
        *self == MODER::STREAM_DATA_TRANSFER
    }
}
#[doc = "Possible values of the field `DMAENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAENABLER {
    #[doc = "DMA disabled."]
    DMA_DISABLED,
    #[doc = "DMA enabled."]
    DMA_ENABLED,
}
impl DMAENABLER {
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
            DMAENABLER::DMA_DISABLED => false,
            DMAENABLER::DMA_ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAENABLER {
        match value {
            false => DMAENABLER::DMA_DISABLED,
            true => DMAENABLER::DMA_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_DISABLED`"]
    #[inline]
    pub fn is_dma_disabled(&self) -> bool {
        *self == DMAENABLER::DMA_DISABLED
    }
    #[doc = "Checks if the value of the field is `DMA_ENABLED`"]
    #[inline]
    pub fn is_dma_enabled(&self) -> bool {
        *self == DMAENABLER::DMA_ENABLED
    }
}
#[doc = r" Value of the field"]
pub struct BLOCKSIZER {
    bits: u8,
}
impl BLOCKSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLEW<'a> {
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
#[doc = "Values that can be written to the field `DIRECTION`"]
pub enum DIRECTIONW {
    #[doc = "From controller to card."]
    CONTROLLER_TO_CARD,
    #[doc = "From card to controller."]
    CARD_TO_CONTROLLER,
}
impl DIRECTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIRECTIONW::CONTROLLER_TO_CARD => false,
            DIRECTIONW::CARD_TO_CONTROLLER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIRECTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _DIRECTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIRECTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "From controller to card."]
    #[inline]
    pub fn controller_to_card(self) -> &'a mut W {
        self.variant(DIRECTIONW::CONTROLLER_TO_CARD)
    }
    #[doc = "From card to controller."]
    #[inline]
    pub fn card_to_controller(self) -> &'a mut W {
        self.variant(DIRECTIONW::CARD_TO_CONTROLLER)
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
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "Block data transfer."]
    BLOCK_DATA_TRANSFER,
    #[doc = "Stream data transfer."]
    STREAM_DATA_TRANSFER,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MODEW::BLOCK_DATA_TRANSFER => false,
            MODEW::STREAM_DATA_TRANSFER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Block data transfer."]
    #[inline]
    pub fn block_data_transfer(self) -> &'a mut W {
        self.variant(MODEW::BLOCK_DATA_TRANSFER)
    }
    #[doc = "Stream data transfer."]
    #[inline]
    pub fn stream_data_transfer(self) -> &'a mut W {
        self.variant(MODEW::STREAM_DATA_TRANSFER)
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
#[doc = "Values that can be written to the field `DMAENABLE`"]
pub enum DMAENABLEW {
    #[doc = "DMA disabled."]
    DMA_DISABLED,
    #[doc = "DMA enabled."]
    DMA_ENABLED,
}
impl DMAENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAENABLEW::DMA_DISABLED => false,
            DMAENABLEW::DMA_ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA disabled."]
    #[inline]
    pub fn dma_disabled(self) -> &'a mut W {
        self.variant(DMAENABLEW::DMA_DISABLED)
    }
    #[doc = "DMA enabled."]
    #[inline]
    pub fn dma_enabled(self) -> &'a mut W {
        self.variant(DMAENABLEW::DMA_ENABLED)
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
#[doc = r" Proxy"]
pub struct _BLOCKSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _BLOCKSIZEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
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
    #[doc = "Bit 0 - Data transfer enable."]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLER { bits }
    }
    #[doc = "Bit 1 - Data transfer direction"]
    #[inline]
    pub fn direction(&self) -> DIRECTIONR {
        DIRECTIONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Data transfer mode"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enable DMA"]
    #[inline]
    pub fn dmaenable(&self) -> DMAENABLER {
        DMAENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:7 - Data block length"]
    #[inline]
    pub fn blocksize(&self) -> BLOCKSIZER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BLOCKSIZER { bits }
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
    #[doc = "Bit 0 - Data transfer enable."]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
    #[doc = "Bit 1 - Data transfer direction"]
    #[inline]
    pub fn direction(&mut self) -> _DIRECTIONW {
        _DIRECTIONW { w: self }
    }
    #[doc = "Bit 2 - Data transfer mode"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 3 - Enable DMA"]
    #[inline]
    pub fn dmaenable(&mut self) -> _DMAENABLEW {
        _DMAENABLEW { w: self }
    }
    #[doc = "Bits 4:7 - Data block length"]
    #[inline]
    pub fn blocksize(&mut self) -> _BLOCKSIZEW {
        _BLOCKSIZEW { w: self }
    }
}
