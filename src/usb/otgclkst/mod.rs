#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::OTGCLKST {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `HOST_CLK_ON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HOST_CLK_ONR {
    #[doc = "Host clock is not available."]
    UNAVAILABLE,
    #[doc = "Host clock is available."]
    AVAILABLE,
}
impl HOST_CLK_ONR {
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
            HOST_CLK_ONR::UNAVAILABLE => false,
            HOST_CLK_ONR::AVAILABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HOST_CLK_ONR {
        match value {
            false => HOST_CLK_ONR::UNAVAILABLE,
            true => HOST_CLK_ONR::AVAILABLE,
        }
    }
    #[doc = "Checks if the value of the field is `UNAVAILABLE`"]
    #[inline]
    pub fn is_unavailable(&self) -> bool {
        *self == HOST_CLK_ONR::UNAVAILABLE
    }
    #[doc = "Checks if the value of the field is `AVAILABLE`"]
    #[inline]
    pub fn is_available(&self) -> bool {
        *self == HOST_CLK_ONR::AVAILABLE
    }
}
#[doc = "Possible values of the field `DEV_CLK_ON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEV_CLK_ONR {
    #[doc = "Device clock is not available."]
    UNAVAILABLE,
    #[doc = "Device clock is available."]
    AVAILABLE,
}
impl DEV_CLK_ONR {
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
            DEV_CLK_ONR::UNAVAILABLE => false,
            DEV_CLK_ONR::AVAILABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DEV_CLK_ONR {
        match value {
            false => DEV_CLK_ONR::UNAVAILABLE,
            true => DEV_CLK_ONR::AVAILABLE,
        }
    }
    #[doc = "Checks if the value of the field is `UNAVAILABLE`"]
    #[inline]
    pub fn is_unavailable(&self) -> bool {
        *self == DEV_CLK_ONR::UNAVAILABLE
    }
    #[doc = "Checks if the value of the field is `AVAILABLE`"]
    #[inline]
    pub fn is_available(&self) -> bool {
        *self == DEV_CLK_ONR::AVAILABLE
    }
}
#[doc = "Possible values of the field `I2C_CLK_ON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C_CLK_ONR {
    #[doc = "I2C clock is not available."]
    UNAVAILABLE,
    #[doc = "I2C clock is available."]
    AVAILABLE,
}
impl I2C_CLK_ONR {
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
            I2C_CLK_ONR::UNAVAILABLE => false,
            I2C_CLK_ONR::AVAILABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> I2C_CLK_ONR {
        match value {
            false => I2C_CLK_ONR::UNAVAILABLE,
            true => I2C_CLK_ONR::AVAILABLE,
        }
    }
    #[doc = "Checks if the value of the field is `UNAVAILABLE`"]
    #[inline]
    pub fn is_unavailable(&self) -> bool {
        *self == I2C_CLK_ONR::UNAVAILABLE
    }
    #[doc = "Checks if the value of the field is `AVAILABLE`"]
    #[inline]
    pub fn is_available(&self) -> bool {
        *self == I2C_CLK_ONR::AVAILABLE
    }
}
#[doc = "Possible values of the field `OTG_CLK_ON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OTG_CLK_ONR {
    #[doc = "OTG clock is not available."]
    UNAVAILABLE,
    #[doc = "OTG clock is available."]
    AVAILABLE,
}
impl OTG_CLK_ONR {
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
            OTG_CLK_ONR::UNAVAILABLE => false,
            OTG_CLK_ONR::AVAILABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OTG_CLK_ONR {
        match value {
            false => OTG_CLK_ONR::UNAVAILABLE,
            true => OTG_CLK_ONR::AVAILABLE,
        }
    }
    #[doc = "Checks if the value of the field is `UNAVAILABLE`"]
    #[inline]
    pub fn is_unavailable(&self) -> bool {
        *self == OTG_CLK_ONR::UNAVAILABLE
    }
    #[doc = "Checks if the value of the field is `AVAILABLE`"]
    #[inline]
    pub fn is_available(&self) -> bool {
        *self == OTG_CLK_ONR::AVAILABLE
    }
}
#[doc = "Possible values of the field `AHB_CLK_ON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHB_CLK_ONR {
    #[doc = "AHB clock is not available."]
    UNAVAILABLE,
    #[doc = "AHB clock is available."]
    AVAILABLE,
}
impl AHB_CLK_ONR {
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
            AHB_CLK_ONR::UNAVAILABLE => false,
            AHB_CLK_ONR::AVAILABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AHB_CLK_ONR {
        match value {
            false => AHB_CLK_ONR::UNAVAILABLE,
            true => AHB_CLK_ONR::AVAILABLE,
        }
    }
    #[doc = "Checks if the value of the field is `UNAVAILABLE`"]
    #[inline]
    pub fn is_unavailable(&self) -> bool {
        *self == AHB_CLK_ONR::UNAVAILABLE
    }
    #[doc = "Checks if the value of the field is `AVAILABLE`"]
    #[inline]
    pub fn is_available(&self) -> bool {
        *self == AHB_CLK_ONR::AVAILABLE
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Host clock status."]
    #[inline]
    pub fn host_clk_on(&self) -> HOST_CLK_ONR {
        HOST_CLK_ONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Device clock status."]
    #[inline]
    pub fn dev_clk_on(&self) -> DEV_CLK_ONR {
        DEV_CLK_ONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - I2C clock status."]
    #[inline]
    pub fn i2c_clk_on(&self) -> I2C_CLK_ONR {
        I2C_CLK_ONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - OTG clock status."]
    #[inline]
    pub fn otg_clk_on(&self) -> OTG_CLK_ONR {
        OTG_CLK_ONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - AHB master clock status."]
    #[inline]
    pub fn ahb_clk_on(&self) -> AHB_CLK_ONR {
        AHB_CLK_ONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
