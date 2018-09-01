#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::P2_13 {
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
    P2_13,
    #[doc = "External interrupt 3 input."]
    EINT3,
    #[doc = "Data line 3 for SD card interface."]
    SD_DAT_3,
    #[doc = "Transmit data. It is driven by the transmitter                                             and read by the receiver. Corresponds to the signal SD                                             in the                                                 I2S-bus                                                 specification."]
    I2S_TX_SDA,
    #[doc = "LCD data."]
    LCD_VD_5,
    #[doc = "LCD data."]
    LCD_VD_9,
    #[doc = "LCD data."]
    LCD_VD_19,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FUNCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FUNCR::P2_13 => 0,
            FUNCR::EINT3 => 1,
            FUNCR::SD_DAT_3 => 2,
            FUNCR::I2S_TX_SDA => 3,
            FUNCR::LCD_VD_5 => 5,
            FUNCR::LCD_VD_9 => 6,
            FUNCR::LCD_VD_19 => 7,
            FUNCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FUNCR {
        match value {
            0 => FUNCR::P2_13,
            1 => FUNCR::EINT3,
            2 => FUNCR::SD_DAT_3,
            3 => FUNCR::I2S_TX_SDA,
            5 => FUNCR::LCD_VD_5,
            6 => FUNCR::LCD_VD_9,
            7 => FUNCR::LCD_VD_19,
            i => FUNCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `P2_13`"]
    #[inline]
    pub fn is_p2_13(&self) -> bool {
        *self == FUNCR::P2_13
    }
    #[doc = "Checks if the value of the field is `EINT3`"]
    #[inline]
    pub fn is_eint3(&self) -> bool {
        *self == FUNCR::EINT3
    }
    #[doc = "Checks if the value of the field is `SD_DAT_3`"]
    #[inline]
    pub fn is_sd_dat_3(&self) -> bool {
        *self == FUNCR::SD_DAT_3
    }
    #[doc = "Checks if the value of the field is `I2S_TX_SDA`"]
    #[inline]
    pub fn is_i2s_tx_sda(&self) -> bool {
        *self == FUNCR::I2S_TX_SDA
    }
    #[doc = "Checks if the value of the field is `LCD_VD_5`"]
    #[inline]
    pub fn is_lcd_vd_5(&self) -> bool {
        *self == FUNCR::LCD_VD_5
    }
    #[doc = "Checks if the value of the field is `LCD_VD_9`"]
    #[inline]
    pub fn is_lcd_vd_9(&self) -> bool {
        *self == FUNCR::LCD_VD_9
    }
    #[doc = "Checks if the value of the field is `LCD_VD_19`"]
    #[inline]
    pub fn is_lcd_vd_19(&self) -> bool {
        *self == FUNCR::LCD_VD_19
    }
}
#[doc = "Values that can be written to the field `FUNC`"]
pub enum FUNCW {
    #[doc = "General purpose digital input/output pin. This\n                                        pin includes a 5 ns input glitch filter."]
    P2_13,
    #[doc = "External interrupt 3 input."]
    EINT3,
    #[doc = "Data line 3 for SD card interface."]
    SD_DAT_3,
    #[doc = "Transmit data. It is driven by the transmitter                                             and read by the receiver. Corresponds to the signal SD                                             in the                                                 I2S-bus                                                 specification."]
    I2S_TX_SDA,
    #[doc = "LCD data."]
    LCD_VD_5,
    #[doc = "LCD data."]
    LCD_VD_9,
    #[doc = "LCD data."]
    LCD_VD_19,
}
impl FUNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FUNCW::P2_13 => 0,
            FUNCW::EINT3 => 1,
            FUNCW::SD_DAT_3 => 2,
            FUNCW::I2S_TX_SDA => 3,
            FUNCW::LCD_VD_5 => 5,
            FUNCW::LCD_VD_9 => 6,
            FUNCW::LCD_VD_19 => 7,
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
    pub fn p2_13(self) -> &'a mut W {
        self.variant(FUNCW::P2_13)
    }
    #[doc = "External interrupt 3 input."]
    #[inline]
    pub fn eint3(self) -> &'a mut W {
        self.variant(FUNCW::EINT3)
    }
    #[doc = "Data line 3 for SD card interface."]
    #[inline]
    pub fn sd_dat_3(self) -> &'a mut W {
        self.variant(FUNCW::SD_DAT_3)
    }
    #[doc = "Transmit data. It is driven by the transmitter and read by the receiver. Corresponds to the signal SD in the I2S-bus specification."]
    #[inline]
    pub fn i2s_tx_sda(self) -> &'a mut W {
        self.variant(FUNCW::I2S_TX_SDA)
    }
    #[doc = "LCD data."]
    #[inline]
    pub fn lcd_vd_5(self) -> &'a mut W {
        self.variant(FUNCW::LCD_VD_5)
    }
    #[doc = "LCD data."]
    #[inline]
    pub fn lcd_vd_9(self) -> &'a mut W {
        self.variant(FUNCW::LCD_VD_9)
    }
    #[doc = "LCD data."]
    #[inline]
    pub fn lcd_vd_19(self) -> &'a mut W {
        self.variant(FUNCW::LCD_VD_19)
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
    #[doc = "Bits 0:2 - Selects pin function for pin P2[13]"]
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
    #[doc = "Bits 0:2 - Selects pin function for pin P2[13]"]
    #[inline]
    pub fn func(&mut self) -> _FUNCW {
        _FUNCW { w: self }
    }
}
