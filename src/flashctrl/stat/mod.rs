#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::STAT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct SIG_DONER {
    bits: bool,
}
impl SIG_DONER {
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
pub struct END_OF_RDWRR {
    bits: bool,
}
impl END_OF_RDWRR {
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
pub struct END_OF_PROG1R {
    bits: bool,
}
impl END_OF_PROG1R {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 2 - When 1, a previously started signature generation has completed. See FMSTATCLR register description for clearing this flag."]
    #[inline]
    pub fn sig_done(&self) -> SIG_DONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SIG_DONER { bits }
    }
    #[doc = "Bit 26 - EEPROM read/write operation finished interrupt status bit. Bit is: - set when this operation has finished OR when 1 is written in the corresponding bit of the EEINTSTATSET register. - cleared when 1 is written to the corresponding bit of the EEINTSTATCLR register."]
    #[inline]
    pub fn end_of_rdwr(&self) -> END_OF_RDWRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        END_OF_RDWRR { bits }
    }
    #[doc = "Bit 28 - EEPROM program operation finished interrupt status bit. Bit is: - set when this operation has finished OR when 1 is written to the corresponding bit of the EEINTSTATSET register. - cleared when 1 is written to the corresponding bit of the EEINTSTATCLR register."]
    #[inline]
    pub fn end_of_prog1(&self) -> END_OF_PROG1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        END_OF_PROG1R { bits }
    }
}
