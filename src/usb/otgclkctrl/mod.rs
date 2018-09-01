#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OTGCLKCTRL {
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
#[doc = "Possible values of the field `HOST_CLK_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HOST_CLK_ENR {
    #[doc = "Disable the Host clock."]
    DISABLE,
    #[doc = "Enable the Host clock."]
    ENABLE,
}
impl HOST_CLK_ENR {
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
            HOST_CLK_ENR::DISABLE => false,
            HOST_CLK_ENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HOST_CLK_ENR {
        match value {
            false => HOST_CLK_ENR::DISABLE,
            true => HOST_CLK_ENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == HOST_CLK_ENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == HOST_CLK_ENR::ENABLE
    }
}
#[doc = "Possible values of the field `DEV_CLK_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEV_CLK_ENR {
    #[doc = "Disable the Device clock."]
    DISABLE,
    #[doc = "Enable the Device clock."]
    ENABLE,
}
impl DEV_CLK_ENR {
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
            DEV_CLK_ENR::DISABLE => false,
            DEV_CLK_ENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DEV_CLK_ENR {
        match value {
            false => DEV_CLK_ENR::DISABLE,
            true => DEV_CLK_ENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == DEV_CLK_ENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == DEV_CLK_ENR::ENABLE
    }
}
#[doc = "Possible values of the field `I2C_CLK_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C_CLK_ENR {
    #[doc = "Disable the I2C clock."]
    DISABLE,
    #[doc = "Enable the I2C clock."]
    ENABLE,
}
impl I2C_CLK_ENR {
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
            I2C_CLK_ENR::DISABLE => false,
            I2C_CLK_ENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> I2C_CLK_ENR {
        match value {
            false => I2C_CLK_ENR::DISABLE,
            true => I2C_CLK_ENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == I2C_CLK_ENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == I2C_CLK_ENR::ENABLE
    }
}
#[doc = "Possible values of the field `OTG_CLK_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OTG_CLK_ENR {
    #[doc = "Disable the OTG clock."]
    DISABLE,
    #[doc = "Enable the OTG clock."]
    ENABLE,
}
impl OTG_CLK_ENR {
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
            OTG_CLK_ENR::DISABLE => false,
            OTG_CLK_ENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OTG_CLK_ENR {
        match value {
            false => OTG_CLK_ENR::DISABLE,
            true => OTG_CLK_ENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == OTG_CLK_ENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == OTG_CLK_ENR::ENABLE
    }
}
#[doc = "Possible values of the field `AHB_CLK_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHB_CLK_ENR {
    #[doc = "Disable the AHB clock."]
    DISABLE,
    #[doc = "Enable the AHB clock."]
    ENABLE,
}
impl AHB_CLK_ENR {
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
            AHB_CLK_ENR::DISABLE => false,
            AHB_CLK_ENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AHB_CLK_ENR {
        match value {
            false => AHB_CLK_ENR::DISABLE,
            true => AHB_CLK_ENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == AHB_CLK_ENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == AHB_CLK_ENR::ENABLE
    }
}
#[doc = "Values that can be written to the field `HOST_CLK_EN`"]
pub enum HOST_CLK_ENW {
    #[doc = "Disable the Host clock."]
    DISABLE,
    #[doc = "Enable the Host clock."]
    ENABLE,
}
impl HOST_CLK_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HOST_CLK_ENW::DISABLE => false,
            HOST_CLK_ENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HOST_CLK_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _HOST_CLK_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HOST_CLK_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the Host clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(HOST_CLK_ENW::DISABLE)
    }
    #[doc = "Enable the Host clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(HOST_CLK_ENW::ENABLE)
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
#[doc = "Values that can be written to the field `DEV_CLK_EN`"]
pub enum DEV_CLK_ENW {
    #[doc = "Disable the Device clock."]
    DISABLE,
    #[doc = "Enable the Device clock."]
    ENABLE,
}
impl DEV_CLK_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DEV_CLK_ENW::DISABLE => false,
            DEV_CLK_ENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEV_CLK_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DEV_CLK_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DEV_CLK_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the Device clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(DEV_CLK_ENW::DISABLE)
    }
    #[doc = "Enable the Device clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(DEV_CLK_ENW::ENABLE)
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
#[doc = "Values that can be written to the field `I2C_CLK_EN`"]
pub enum I2C_CLK_ENW {
    #[doc = "Disable the I2C clock."]
    DISABLE,
    #[doc = "Enable the I2C clock."]
    ENABLE,
}
impl I2C_CLK_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            I2C_CLK_ENW::DISABLE => false,
            I2C_CLK_ENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2C_CLK_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_CLK_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2C_CLK_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the I2C clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(I2C_CLK_ENW::DISABLE)
    }
    #[doc = "Enable the I2C clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(I2C_CLK_ENW::ENABLE)
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
#[doc = "Values that can be written to the field `OTG_CLK_EN`"]
pub enum OTG_CLK_ENW {
    #[doc = "Disable the OTG clock."]
    DISABLE,
    #[doc = "Enable the OTG clock."]
    ENABLE,
}
impl OTG_CLK_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OTG_CLK_ENW::DISABLE => false,
            OTG_CLK_ENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OTG_CLK_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _OTG_CLK_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OTG_CLK_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the OTG clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(OTG_CLK_ENW::DISABLE)
    }
    #[doc = "Enable the OTG clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(OTG_CLK_ENW::ENABLE)
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
#[doc = "Values that can be written to the field `AHB_CLK_EN`"]
pub enum AHB_CLK_ENW {
    #[doc = "Disable the AHB clock."]
    DISABLE,
    #[doc = "Enable the AHB clock."]
    ENABLE,
}
impl AHB_CLK_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AHB_CLK_ENW::DISABLE => false,
            AHB_CLK_ENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AHB_CLK_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _AHB_CLK_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AHB_CLK_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the AHB clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(AHB_CLK_ENW::DISABLE)
    }
    #[doc = "Enable the AHB clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(AHB_CLK_ENW::ENABLE)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Host clock enable"]
    #[inline]
    pub fn host_clk_en(&self) -> HOST_CLK_ENR {
        HOST_CLK_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Device clock enable"]
    #[inline]
    pub fn dev_clk_en(&self) -> DEV_CLK_ENR {
        DEV_CLK_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - I2C clock enable"]
    #[inline]
    pub fn i2c_clk_en(&self) -> I2C_CLK_ENR {
        I2C_CLK_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - OTG clock enable. In device-only applications, this bit enables access to the PORTSEL register."]
    #[inline]
    pub fn otg_clk_en(&self) -> OTG_CLK_ENR {
        OTG_CLK_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - AHB master clock enable"]
    #[inline]
    pub fn ahb_clk_en(&self) -> AHB_CLK_ENR {
        AHB_CLK_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
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
    #[doc = "Bit 0 - Host clock enable"]
    #[inline]
    pub fn host_clk_en(&mut self) -> _HOST_CLK_ENW {
        _HOST_CLK_ENW { w: self }
    }
    #[doc = "Bit 1 - Device clock enable"]
    #[inline]
    pub fn dev_clk_en(&mut self) -> _DEV_CLK_ENW {
        _DEV_CLK_ENW { w: self }
    }
    #[doc = "Bit 2 - I2C clock enable"]
    #[inline]
    pub fn i2c_clk_en(&mut self) -> _I2C_CLK_ENW {
        _I2C_CLK_ENW { w: self }
    }
    #[doc = "Bit 3 - OTG clock enable. In device-only applications, this bit enables access to the PORTSEL register."]
    #[inline]
    pub fn otg_clk_en(&mut self) -> _OTG_CLK_ENW {
        _OTG_CLK_ENW { w: self }
    }
    #[doc = "Bit 4 - AHB master clock enable"]
    #[inline]
    pub fn ahb_clk_en(&mut self) -> _AHB_CLK_ENW {
        _AHB_CLK_ENW { w: self }
    }
}
