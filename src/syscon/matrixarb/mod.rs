#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MATRIXARB {
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
pub struct PRI_ICODER {
    bits: u8,
}
impl PRI_ICODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRI_DCODER {
    bits: u8,
}
impl PRI_DCODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRI_SYSR {
    bits: u8,
}
impl PRI_SYSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRI_GPDMAR {
    bits: u8,
}
impl PRI_GPDMAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRI_ETHR {
    bits: u8,
}
impl PRI_ETHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRI_LCDR {
    bits: u8,
}
impl PRI_LCDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRI_USBR {
    bits: u8,
}
impl PRI_USBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ROM_LATR {
    bits: bool,
}
impl ROM_LATR {
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
#[doc = r" Proxy"]
pub struct _PRI_ICODEW<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_ICODEW<'a> {
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
#[doc = r" Proxy"]
pub struct _PRI_DCODEW<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_DCODEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRI_SYSW<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_SYSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRI_GPDMAW<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_GPDMAW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRI_ETHW<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_ETHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRI_LCDW<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_LCDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRI_USBW<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_USBW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ROM_LATW<'a> {
    w: &'a mut W,
}
impl<'a> _ROM_LATW<'a> {
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
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:1 - I-Code bus priority. Should be lower than PRI_DCODE for proper operation."]
    #[inline]
    pub fn pri_icode(&self) -> PRI_ICODER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_ICODER { bits }
    }
    #[doc = "Bits 2:3 - D-Code bus priority."]
    #[inline]
    pub fn pri_dcode(&self) -> PRI_DCODER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_DCODER { bits }
    }
    #[doc = "Bits 4:5 - System bus priority."]
    #[inline]
    pub fn pri_sys(&self) -> PRI_SYSR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_SYSR { bits }
    }
    #[doc = "Bits 6:7 - General Purpose DMA controller priority."]
    #[inline]
    pub fn pri_gpdma(&self) -> PRI_GPDMAR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_GPDMAR { bits }
    }
    #[doc = "Bits 8:9 - Ethernet DMA priority."]
    #[inline]
    pub fn pri_eth(&self) -> PRI_ETHR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_ETHR { bits }
    }
    #[doc = "Bits 10:11 - LCD DMA priority."]
    #[inline]
    pub fn pri_lcd(&self) -> PRI_LCDR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_LCDR { bits }
    }
    #[doc = "Bits 12:13 - USB DMA priority."]
    #[inline]
    pub fn pri_usb(&self) -> PRI_USBR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_USBR { bits }
    }
    #[doc = "Bit 16 - ROM latency select. Should always be 0."]
    #[inline]
    pub fn rom_lat(&self) -> ROM_LATR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ROM_LATR { bits }
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
    #[doc = "Bits 0:1 - I-Code bus priority. Should be lower than PRI_DCODE for proper operation."]
    #[inline]
    pub fn pri_icode(&mut self) -> _PRI_ICODEW {
        _PRI_ICODEW { w: self }
    }
    #[doc = "Bits 2:3 - D-Code bus priority."]
    #[inline]
    pub fn pri_dcode(&mut self) -> _PRI_DCODEW {
        _PRI_DCODEW { w: self }
    }
    #[doc = "Bits 4:5 - System bus priority."]
    #[inline]
    pub fn pri_sys(&mut self) -> _PRI_SYSW {
        _PRI_SYSW { w: self }
    }
    #[doc = "Bits 6:7 - General Purpose DMA controller priority."]
    #[inline]
    pub fn pri_gpdma(&mut self) -> _PRI_GPDMAW {
        _PRI_GPDMAW { w: self }
    }
    #[doc = "Bits 8:9 - Ethernet DMA priority."]
    #[inline]
    pub fn pri_eth(&mut self) -> _PRI_ETHW {
        _PRI_ETHW { w: self }
    }
    #[doc = "Bits 10:11 - LCD DMA priority."]
    #[inline]
    pub fn pri_lcd(&mut self) -> _PRI_LCDW {
        _PRI_LCDW { w: self }
    }
    #[doc = "Bits 12:13 - USB DMA priority."]
    #[inline]
    pub fn pri_usb(&mut self) -> _PRI_USBW {
        _PRI_USBW { w: self }
    }
    #[doc = "Bit 16 - ROM latency select. Should always be 0."]
    #[inline]
    pub fn rom_lat(&mut self) -> _ROM_LATW {
        _ROM_LATW { w: self }
    }
}
