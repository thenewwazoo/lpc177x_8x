#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO Port0 Direction control register."]
    pub dir0: DIR,
    _reserved0: [u8; 12usize],
    #[doc = "0x10 - Mask register for Port0."]
    pub mask0: MASK,
    #[doc = "0x14 - Port0 Pin value register using FIOMASK."]
    pub pin0: PIN,
    #[doc = "0x18 - Port0 Output Set register using FIOMASK."]
    pub set0: SET,
    #[doc = "0x1c - Port0 Output Clear register using FIOMASK."]
    pub clr0: CLR,
    #[doc = "0x20 - GPIO Port0 Direction control register."]
    pub dir1: DIR,
    _reserved1: [u8; 12usize],
    #[doc = "0x30 - Mask register for Port0."]
    pub mask1: MASK,
    #[doc = "0x34 - Port0 Pin value register using FIOMASK."]
    pub pin1: PIN,
    #[doc = "0x38 - Port0 Output Set register using FIOMASK."]
    pub set1: SET,
    #[doc = "0x3c - Port0 Output Clear register using FIOMASK."]
    pub clr1: CLR,
    #[doc = "0x40 - GPIO Port0 Direction control register."]
    pub dir2: DIR,
    _reserved2: [u8; 12usize],
    #[doc = "0x50 - Mask register for Port0."]
    pub mask2: MASK,
    #[doc = "0x54 - Port0 Pin value register using FIOMASK."]
    pub pin2: PIN,
    #[doc = "0x58 - Port0 Output Set register using FIOMASK."]
    pub set2: SET,
    #[doc = "0x5c - Port0 Output Clear register using FIOMASK."]
    pub clr2: CLR,
    #[doc = "0x60 - GPIO Port0 Direction control register."]
    pub dir3: DIR,
    _reserved3: [u8; 12usize],
    #[doc = "0x70 - Mask register for Port0."]
    pub mask3: MASK,
    #[doc = "0x74 - Port0 Pin value register using FIOMASK."]
    pub pin3: PIN,
    #[doc = "0x78 - Port0 Output Set register using FIOMASK."]
    pub set3: SET,
    #[doc = "0x7c - Port0 Output Clear register using FIOMASK."]
    pub clr3: CLR,
    #[doc = "0x80 - GPIO Port0 Direction control register."]
    pub dir4: DIR,
    _reserved4: [u8; 12usize],
    #[doc = "0x90 - Mask register for Port0."]
    pub mask4: MASK,
    #[doc = "0x94 - Port0 Pin value register using FIOMASK."]
    pub pin4: PIN,
    #[doc = "0x98 - Port0 Output Set register using FIOMASK."]
    pub set4: SET,
    #[doc = "0x9c - Port0 Output Clear register using FIOMASK."]
    pub clr4: CLR,
    #[doc = "0xa0 - GPIO Port0 Direction control register."]
    pub dir5: DIR,
    _reserved5: [u8; 12usize],
    #[doc = "0xb0 - Mask register for Port0."]
    pub mask5: MASK,
    #[doc = "0xb4 - Port0 Pin value register using FIOMASK."]
    pub pin5: PIN,
    #[doc = "0xb8 - Port0 Output Set register using FIOMASK."]
    pub set5: SET,
    #[doc = "0xbc - Port0 Output Clear register using FIOMASK."]
    pub clr5: CLR,
}
#[doc = "GPIO Port0 Direction control register."]
pub struct DIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Port0 Direction control register."]
pub mod dir;
#[doc = "Mask register for Port0."]
pub struct MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mask register for Port0."]
pub mod mask;
#[doc = "Port0 Pin value register using FIOMASK."]
pub struct PIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port0 Pin value register using FIOMASK."]
pub mod pin;
#[doc = "Port0 Output Set register using FIOMASK."]
pub struct SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port0 Output Set register using FIOMASK."]
pub mod set;
#[doc = "Port0 Output Clear register using FIOMASK."]
pub struct CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port0 Output Clear register using FIOMASK."]
pub mod clr;
