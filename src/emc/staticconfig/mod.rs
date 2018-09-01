#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STATICCONFIG {
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
#[doc = "Possible values of the field `MW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MWR {
    #[doc = "8 bit (POR reset value)."]
    _8_BIT,
    #[doc = "16 bit."]
    _16_BIT,
    #[doc = "32 bit."]
    _32_BIT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MWR::_8_BIT => 0,
            MWR::_16_BIT => 1,
            MWR::_32_BIT => 2,
            MWR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MWR {
        match value {
            0 => MWR::_8_BIT,
            1 => MWR::_16_BIT,
            2 => MWR::_32_BIT,
            i => MWR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_8_BIT`"]
    #[inline]
    pub fn is_8_bit(&self) -> bool {
        *self == MWR::_8_BIT
    }
    #[doc = "Checks if the value of the field is `_16_BIT`"]
    #[inline]
    pub fn is_16_bit(&self) -> bool {
        *self == MWR::_16_BIT
    }
    #[doc = "Checks if the value of the field is `_32_BIT`"]
    #[inline]
    pub fn is_32_bit(&self) -> bool {
        *self == MWR::_32_BIT
    }
}
#[doc = "Possible values of the field `PM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMR {
    #[doc = "Disabled (POR reset value)."]
    DISABLED,
    #[doc = "Asynchronous page mode enabled (page length four)."]
    ASYNCHRONOUS_PAGE_MODE,
}
impl PMR {
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
            PMR::DISABLED => false,
            PMR::ASYNCHRONOUS_PAGE_MODE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PMR {
        match value {
            false => PMR::DISABLED,
            true => PMR::ASYNCHRONOUS_PAGE_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PMR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS_PAGE_MODE`"]
    #[inline]
    pub fn is_asynchronous_page_mode(&self) -> bool {
        *self == PMR::ASYNCHRONOUS_PAGE_MODE
    }
}
#[doc = "Possible values of the field `PC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCR {
    #[doc = "Active LOW chip select."]
    ACTIVE_LOW,
    #[doc = "Active HIGH chip select."]
    ACTIVE_HIGH,
}
impl PCR {
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
            PCR::ACTIVE_LOW => false,
            PCR::ACTIVE_HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PCR {
        match value {
            false => PCR::ACTIVE_LOW,
            true => PCR::ACTIVE_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE_LOW`"]
    #[inline]
    pub fn is_active_low(&self) -> bool {
        *self == PCR::ACTIVE_LOW
    }
    #[doc = "Checks if the value of the field is `ACTIVE_HIGH`"]
    #[inline]
    pub fn is_active_high(&self) -> bool {
        *self == PCR::ACTIVE_HIGH
    }
}
#[doc = "Possible values of the field `PB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PBR {
    #[doc = "For reads all the bits in BLS3:0 are HIGH. For writes the respective active bits in BLS3:0 are LOW (POR reset value)."]
    BLSHIGH,
    #[doc = "For reads the respective active bits in BLS3:0 are LOW. For writes the respective active bits in BLS3:0 are LOW."]
    BLSLOW,
}
impl PBR {
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
            PBR::BLSHIGH => false,
            PBR::BLSLOW => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PBR {
        match value {
            false => PBR::BLSHIGH,
            true => PBR::BLSLOW,
        }
    }
    #[doc = "Checks if the value of the field is `BLSHIGH`"]
    #[inline]
    pub fn is_blshigh(&self) -> bool {
        *self == PBR::BLSHIGH
    }
    #[doc = "Checks if the value of the field is `BLSLOW`"]
    #[inline]
    pub fn is_blslow(&self) -> bool {
        *self == PBR::BLSLOW
    }
}
#[doc = "Possible values of the field `EW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWR {
    #[doc = "Extended wait disabled (POR reset value)."]
    EXTENDED_WAIT_DIS,
    #[doc = "Extended wait enabled."]
    EXTENDED_WAIT_EN,
}
impl EWR {
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
            EWR::EXTENDED_WAIT_DIS => false,
            EWR::EXTENDED_WAIT_EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EWR {
        match value {
            false => EWR::EXTENDED_WAIT_DIS,
            true => EWR::EXTENDED_WAIT_EN,
        }
    }
    #[doc = "Checks if the value of the field is `EXTENDED_WAIT_DIS`"]
    #[inline]
    pub fn is_extended_wait_dis(&self) -> bool {
        *self == EWR::EXTENDED_WAIT_DIS
    }
    #[doc = "Checks if the value of the field is `EXTENDED_WAIT_EN`"]
    #[inline]
    pub fn is_extended_wait_en(&self) -> bool {
        *self == EWR::EXTENDED_WAIT_EN
    }
}
#[doc = "Possible values of the field `B`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BR {
    #[doc = "Buffer disabled (POR reset value)."]
    DISABLED,
    #[doc = "Buffer enabled."]
    ENABLED,
}
impl BR {
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
            BR::DISABLED => false,
            BR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BR {
        match value {
            false => BR::DISABLED,
            true => BR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == BR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == BR::ENABLED
    }
}
#[doc = "Possible values of the field `P`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PR {
    #[doc = "Writes not protected (POR reset value)."]
    WRITES_NOT_PROTECTED,
    #[doc = "Write protected."]
    WRITE_PROTECTED,
}
impl PR {
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
            PR::WRITES_NOT_PROTECTED => false,
            PR::WRITE_PROTECTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PR {
        match value {
            false => PR::WRITES_NOT_PROTECTED,
            true => PR::WRITE_PROTECTED,
        }
    }
    #[doc = "Checks if the value of the field is `WRITES_NOT_PROTECTED`"]
    #[inline]
    pub fn is_writes_not_protected(&self) -> bool {
        *self == PR::WRITES_NOT_PROTECTED
    }
    #[doc = "Checks if the value of the field is `WRITE_PROTECTED`"]
    #[inline]
    pub fn is_write_protected(&self) -> bool {
        *self == PR::WRITE_PROTECTED
    }
}
#[doc = "Values that can be written to the field `MW`"]
pub enum MWW {
    #[doc = "8 bit (POR reset value)."]
    _8_BIT,
    #[doc = "16 bit."]
    _16_BIT,
    #[doc = "32 bit."]
    _32_BIT,
}
impl MWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MWW::_8_BIT => 0,
            MWW::_16_BIT => 1,
            MWW::_32_BIT => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MWW<'a> {
    w: &'a mut W,
}
impl<'a> _MWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MWW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "8 bit (POR reset value)."]
    #[inline]
    pub fn _8_bit(self) -> &'a mut W {
        self.variant(MWW::_8_BIT)
    }
    #[doc = "16 bit."]
    #[inline]
    pub fn _16_bit(self) -> &'a mut W {
        self.variant(MWW::_16_BIT)
    }
    #[doc = "32 bit."]
    #[inline]
    pub fn _32_bit(self) -> &'a mut W {
        self.variant(MWW::_32_BIT)
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
#[doc = "Values that can be written to the field `PM`"]
pub enum PMW {
    #[doc = "Disabled (POR reset value)."]
    DISABLED,
    #[doc = "Asynchronous page mode enabled (page length four)."]
    ASYNCHRONOUS_PAGE_MODE,
}
impl PMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PMW::DISABLED => false,
            PMW::ASYNCHRONOUS_PAGE_MODE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PMW<'a> {
    w: &'a mut W,
}
impl<'a> _PMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled (POR reset value)."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PMW::DISABLED)
    }
    #[doc = "Asynchronous page mode enabled (page length four)."]
    #[inline]
    pub fn asynchronous_page_mode(self) -> &'a mut W {
        self.variant(PMW::ASYNCHRONOUS_PAGE_MODE)
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
#[doc = "Values that can be written to the field `PC`"]
pub enum PCW {
    #[doc = "Active LOW chip select."]
    ACTIVE_LOW,
    #[doc = "Active HIGH chip select."]
    ACTIVE_HIGH,
}
impl PCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PCW::ACTIVE_LOW => false,
            PCW::ACTIVE_HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCW<'a> {
    w: &'a mut W,
}
impl<'a> _PCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Active LOW chip select."]
    #[inline]
    pub fn active_low(self) -> &'a mut W {
        self.variant(PCW::ACTIVE_LOW)
    }
    #[doc = "Active HIGH chip select."]
    #[inline]
    pub fn active_high(self) -> &'a mut W {
        self.variant(PCW::ACTIVE_HIGH)
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
#[doc = "Values that can be written to the field `PB`"]
pub enum PBW {
    #[doc = "For reads all the bits in BLS3:0 are HIGH. For writes the respective active bits in BLS3:0 are LOW (POR reset value)."]
    BLSHIGH,
    #[doc = "For reads the respective active bits in BLS3:0 are LOW. For writes the respective active bits in BLS3:0 are LOW."]
    BLSLOW,
}
impl PBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PBW::BLSHIGH => false,
            PBW::BLSLOW => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PBW<'a> {
    w: &'a mut W,
}
impl<'a> _PBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "For reads all the bits in BLS3:0 are HIGH. For writes the respective active bits in BLS3:0 are LOW (POR reset value)."]
    #[inline]
    pub fn blshigh(self) -> &'a mut W {
        self.variant(PBW::BLSHIGH)
    }
    #[doc = "For reads the respective active bits in BLS3:0 are LOW. For writes the respective active bits in BLS3:0 are LOW."]
    #[inline]
    pub fn blslow(self) -> &'a mut W {
        self.variant(PBW::BLSLOW)
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
#[doc = "Values that can be written to the field `EW`"]
pub enum EWW {
    #[doc = "Extended wait disabled (POR reset value)."]
    EXTENDED_WAIT_DIS,
    #[doc = "Extended wait enabled."]
    EXTENDED_WAIT_EN,
}
impl EWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EWW::EXTENDED_WAIT_DIS => false,
            EWW::EXTENDED_WAIT_EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EWW<'a> {
    w: &'a mut W,
}
impl<'a> _EWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Extended wait disabled (POR reset value)."]
    #[inline]
    pub fn extended_wait_dis(self) -> &'a mut W {
        self.variant(EWW::EXTENDED_WAIT_DIS)
    }
    #[doc = "Extended wait enabled."]
    #[inline]
    pub fn extended_wait_en(self) -> &'a mut W {
        self.variant(EWW::EXTENDED_WAIT_EN)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `B`"]
pub enum BW {
    #[doc = "Buffer disabled (POR reset value)."]
    DISABLED,
    #[doc = "Buffer enabled."]
    ENABLED,
}
impl BW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BW::DISABLED => false,
            BW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BW<'a> {
    w: &'a mut W,
}
impl<'a> _BW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Buffer disabled (POR reset value)."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BW::DISABLED)
    }
    #[doc = "Buffer enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BW::ENABLED)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P`"]
pub enum PW {
    #[doc = "Writes not protected (POR reset value)."]
    WRITES_NOT_PROTECTED,
    #[doc = "Write protected."]
    WRITE_PROTECTED,
}
impl PW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PW::WRITES_NOT_PROTECTED => false,
            PW::WRITE_PROTECTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PW<'a> {
    w: &'a mut W,
}
impl<'a> _PW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Writes not protected (POR reset value)."]
    #[inline]
    pub fn writes_not_protected(self) -> &'a mut W {
        self.variant(PW::WRITES_NOT_PROTECTED)
    }
    #[doc = "Write protected."]
    #[inline]
    pub fn write_protected(self) -> &'a mut W {
        self.variant(PW::WRITE_PROTECTED)
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
        const OFFSET: u8 = 20;
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
    #[doc = "Bits 0:1 - Memory width."]
    #[inline]
    pub fn mw(&self) -> MWR {
        MWR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Page mode. In page mode the EMC can burst up to four external accesses. Therefore devices with asynchronous page mode burst four or higher devices are supported. Asynchronous page mode burst two devices are not supported and must be accessed normally."]
    #[inline]
    pub fn pm(&self) -> PMR {
        PMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Chip select polarity. The value of the chip select polarity on power-on reset is 0."]
    #[inline]
    pub fn pc(&self) -> PCR {
        PCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Byte lane state. The byte lane state bit, PB, enables different types of memory to be connected. For byte-wide static memories the BLS3:0 signal from the EMC is usually connected to WE (write enable). In this case for reads all the BLS3:0 bits must be HIGH. This means that the byte lane state (PB) bit must be LOW. 16 bit wide static memory devices usually have the BLS3:0 signals connected to the UBn and LBn (upper byte and lower byte) signals in the static memory. In this case a write to a particular byte must assert the appropriate UBn or LBn signal LOW. For reads, all the UB and LB signals must be asserted LOW so that the bus is driven. In this case the byte lane state (PB) bit must be HIGH."]
    #[inline]
    pub fn pb(&self) -> PBR {
        PBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Extended wait (EW) uses the EMCStaticExtendedWait register to time both the read and write transfers rather than the EMCStaticWaitRd and EMCStaticWaitWr registers. This enables much longer transactions. [1]"]
    #[inline]
    pub fn ew(&self) -> EWR {
        EWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Buffer enable [2]"]
    #[inline]
    pub fn b(&self) -> BR {
        BR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Write protect"]
    #[inline]
    pub fn p(&self) -> PR {
        PR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
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
    #[doc = "Bits 0:1 - Memory width."]
    #[inline]
    pub fn mw(&mut self) -> _MWW {
        _MWW { w: self }
    }
    #[doc = "Bit 3 - Page mode. In page mode the EMC can burst up to four external accesses. Therefore devices with asynchronous page mode burst four or higher devices are supported. Asynchronous page mode burst two devices are not supported and must be accessed normally."]
    #[inline]
    pub fn pm(&mut self) -> _PMW {
        _PMW { w: self }
    }
    #[doc = "Bit 6 - Chip select polarity. The value of the chip select polarity on power-on reset is 0."]
    #[inline]
    pub fn pc(&mut self) -> _PCW {
        _PCW { w: self }
    }
    #[doc = "Bit 7 - Byte lane state. The byte lane state bit, PB, enables different types of memory to be connected. For byte-wide static memories the BLS3:0 signal from the EMC is usually connected to WE (write enable). In this case for reads all the BLS3:0 bits must be HIGH. This means that the byte lane state (PB) bit must be LOW. 16 bit wide static memory devices usually have the BLS3:0 signals connected to the UBn and LBn (upper byte and lower byte) signals in the static memory. In this case a write to a particular byte must assert the appropriate UBn or LBn signal LOW. For reads, all the UB and LB signals must be asserted LOW so that the bus is driven. In this case the byte lane state (PB) bit must be HIGH."]
    #[inline]
    pub fn pb(&mut self) -> _PBW {
        _PBW { w: self }
    }
    #[doc = "Bit 8 - Extended wait (EW) uses the EMCStaticExtendedWait register to time both the read and write transfers rather than the EMCStaticWaitRd and EMCStaticWaitWr registers. This enables much longer transactions. [1]"]
    #[inline]
    pub fn ew(&mut self) -> _EWW {
        _EWW { w: self }
    }
    #[doc = "Bit 19 - Buffer enable [2]"]
    #[inline]
    pub fn b(&mut self) -> _BW {
        _BW { w: self }
    }
    #[doc = "Bit 20 - Write protect"]
    #[inline]
    pub fn p(&mut self) -> _PW {
        _PW { w: self }
    }
}
