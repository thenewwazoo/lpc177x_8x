#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::EERDATA {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct RDATAR {
    bits: u32,
}
impl RDATAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - Read data. In case of: 8-bit read operations: bits [7:0] contain read data, others are zero. 16-bit read operations: bits [15:0] contain read data, others are zero. 32-bit read operations: bits [31:0] contain read data."]
    #[inline]
    pub fn rdata(&self) -> RDATAR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RDATAR { bits }
    }
}
