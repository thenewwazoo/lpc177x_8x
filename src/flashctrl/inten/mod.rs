#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::INTEN {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct EE_RW_DONER {
    bits: bool,
}
impl EE_RW_DONER {
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
pub struct EE_PROG_DONER {
    bits: bool,
}
impl EE_PROG_DONER {
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
    #[doc = "Bit 26 - EEPROM read/write operation finished interrupt enable bit. Bit is: - set when 1 is written to the corresponding bit of the EEINTENSET register. - cleared when 1 is written to the corresponding bit of the EEINTENCLR register."]
    #[inline]
    pub fn ee_rw_done(&self) -> EE_RW_DONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EE_RW_DONER { bits }
    }
    #[doc = "Bit 28 - EEPROM program operation finished interrupt enable bit. Bit is: - set when 1 is written in the corresponding bit of the EEINTENSET register. - cleared when 1 is written to the corresponding bit of the EEINTENCLR register."]
    #[inline]
    pub fn ee_prog_done(&self) -> EE_PROG_DONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EE_PROG_DONER { bits }
    }
}
