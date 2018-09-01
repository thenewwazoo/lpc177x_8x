#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DMAINTST {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `EOT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOTR {
    #[doc = "All bits in the USBEoTIntSt register are 0."]
    ALL_BITS_ZERO,
    #[doc = "At least one bit in the USBEoTIntSt is set."]
    SINGLE_BIT,
}
impl EOTR {
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
            EOTR::ALL_BITS_ZERO => false,
            EOTR::SINGLE_BIT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EOTR {
        match value {
            false => EOTR::ALL_BITS_ZERO,
            true => EOTR::SINGLE_BIT,
        }
    }
    #[doc = "Checks if the value of the field is `ALL_BITS_ZERO`"]
    #[inline]
    pub fn is_all_bits_zero(&self) -> bool {
        *self == EOTR::ALL_BITS_ZERO
    }
    #[doc = "Checks if the value of the field is `SINGLE_BIT`"]
    #[inline]
    pub fn is_single_bit(&self) -> bool {
        *self == EOTR::SINGLE_BIT
    }
}
#[doc = "Possible values of the field `NDDR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NDDRR {
    #[doc = "All bits in the USBNDDRIntSt register are 0."]
    ALL_BITS_ZERO,
    #[doc = "At least one bit in the USBNDDRIntSt is set."]
    SINGLE_BIT,
}
impl NDDRR {
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
            NDDRR::ALL_BITS_ZERO => false,
            NDDRR::SINGLE_BIT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NDDRR {
        match value {
            false => NDDRR::ALL_BITS_ZERO,
            true => NDDRR::SINGLE_BIT,
        }
    }
    #[doc = "Checks if the value of the field is `ALL_BITS_ZERO`"]
    #[inline]
    pub fn is_all_bits_zero(&self) -> bool {
        *self == NDDRR::ALL_BITS_ZERO
    }
    #[doc = "Checks if the value of the field is `SINGLE_BIT`"]
    #[inline]
    pub fn is_single_bit(&self) -> bool {
        *self == NDDRR::SINGLE_BIT
    }
}
#[doc = "Possible values of the field `ERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRR {
    #[doc = "All bits in the USBSysErrIntSt register are 0."]
    ALL_BITS_ZERO,
    #[doc = "At least one bit in the USBSysErrIntSt is set."]
    SINGLE_BIT,
}
impl ERRR {
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
            ERRR::ALL_BITS_ZERO => false,
            ERRR::SINGLE_BIT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERRR {
        match value {
            false => ERRR::ALL_BITS_ZERO,
            true => ERRR::SINGLE_BIT,
        }
    }
    #[doc = "Checks if the value of the field is `ALL_BITS_ZERO`"]
    #[inline]
    pub fn is_all_bits_zero(&self) -> bool {
        *self == ERRR::ALL_BITS_ZERO
    }
    #[doc = "Checks if the value of the field is `SINGLE_BIT`"]
    #[inline]
    pub fn is_single_bit(&self) -> bool {
        *self == ERRR::SINGLE_BIT
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - End of Transfer Interrupt bit."]
    #[inline]
    pub fn eot(&self) -> EOTR {
        EOTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - New DD Request Interrupt bit."]
    #[inline]
    pub fn nddr(&self) -> NDDRR {
        NDDRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - System Error Interrupt bit."]
    #[inline]
    pub fn err(&self) -> ERRR {
        ERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
