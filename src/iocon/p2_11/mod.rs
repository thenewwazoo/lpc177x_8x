#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::P2_11 {
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
#[doc = "Possible values of the field `FUNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FUNCR {
    #[doc = "General purpose digital input/output pin. This\n                                        pin includes a 5 ns input glitch filter."]
    P2_11,
    #[doc = "External interrupt 1 input."]
    EINT1,
    #[doc = "Data line 1 for SD card interface."]
    SD_DAT_1,
    #[doc = "Transmit Clock. It is driven by the master and                                             received by the slave. Corresponds to the signal SCK in                                             the                                                 I2S-bus                                                 specification."]
    I2S_TX_SCK,
    #[doc = "LCD clock."]
    LCD_CLKIN,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FUNCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FUNCR::P2_11 => 0,
            FUNCR::EINT1 => 1,
            FUNCR::SD_DAT_1 => 2,
            FUNCR::I2S_TX_SCK => 3,
            FUNCR::LCD_CLKIN => 7,
            FUNCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FUNCR {
        match value {
            0 => FUNCR::P2_11,
            1 => FUNCR::EINT1,
            2 => FUNCR::SD_DAT_1,
            3 => FUNCR::I2S_TX_SCK,
            7 => FUNCR::LCD_CLKIN,
            i => FUNCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `P2_11`"]
    #[inline]
    pub fn is_p2_11(&self) -> bool {
        *self == FUNCR::P2_11
    }
    #[doc = "Checks if the value of the field is `EINT1`"]
    #[inline]
    pub fn is_eint1(&self) -> bool {
        *self == FUNCR::EINT1
    }
    #[doc = "Checks if the value of the field is `SD_DAT_1`"]
    #[inline]
    pub fn is_sd_dat_1(&self) -> bool {
        *self == FUNCR::SD_DAT_1
    }
    #[doc = "Checks if the value of the field is `I2S_TX_SCK`"]
    #[inline]
    pub fn is_i2s_tx_sck(&self) -> bool {
        *self == FUNCR::I2S_TX_SCK
    }
    #[doc = "Checks if the value of the field is `LCD_CLKIN`"]
    #[inline]
    pub fn is_lcd_clkin(&self) -> bool {
        *self == FUNCR::LCD_CLKIN
    }
}
#[doc = "Values that can be written to the field `FUNC`"]
pub enum FUNCW {
    #[doc = "General purpose digital input/output pin. This\n                                        pin includes a 5 ns input glitch filter."]
    P2_11,
    #[doc = "External interrupt 1 input."]
    EINT1,
    #[doc = "Data line 1 for SD card interface."]
    SD_DAT_1,
    #[doc = "Transmit Clock. It is driven by the master and                                             received by the slave. Corresponds to the signal SCK in                                             the                                                 I2S-bus                                                 specification."]
    I2S_TX_SCK,
    #[doc = "LCD clock."]
    LCD_CLKIN,
}
impl FUNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FUNCW::P2_11 => 0,
            FUNCW::EINT1 => 1,
            FUNCW::SD_DAT_1 => 2,
            FUNCW::I2S_TX_SCK => 3,
            FUNCW::LCD_CLKIN => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FUNCW<'a> {
    w: &'a mut W,
}
impl<'a> _FUNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FUNCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "General purpose digital input/output pin. This pin includes a 5 ns input glitch filter."]
    #[inline]
    pub fn p2_11(self) -> &'a mut W {
        self.variant(FUNCW::P2_11)
    }
    #[doc = "External interrupt 1 input."]
    #[inline]
    pub fn eint1(self) -> &'a mut W {
        self.variant(FUNCW::EINT1)
    }
    #[doc = "Data line 1 for SD card interface."]
    #[inline]
    pub fn sd_dat_1(self) -> &'a mut W {
        self.variant(FUNCW::SD_DAT_1)
    }
    #[doc = "Transmit Clock. It is driven by the master and received by the slave. Corresponds to the signal SCK in the I2S-bus specification."]
    #[inline]
    pub fn i2s_tx_sck(self) -> &'a mut W {
        self.variant(FUNCW::I2S_TX_SCK)
    }
    #[doc = "LCD clock."]
    #[inline]
    pub fn lcd_clkin(self) -> &'a mut W {
        self.variant(FUNCW::LCD_CLKIN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:2 - Selects pin function for pin P2[11]"]
    #[inline]
    pub fn func(&self) -> FUNCR {
        FUNCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 48 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Selects pin function for pin P2[11]"]
    #[inline]
    pub fn func(&mut self) -> _FUNCW {
        _FUNCW { w: self }
    }
}
