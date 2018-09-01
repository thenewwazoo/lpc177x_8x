#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 32usize],
    #[doc = "0x20 - Signature start address register"]
    pub fmsstart: FMSSTART,
    #[doc = "0x24 - Signature stop-address register"]
    pub fmsstop: FMSSTOP,
    _reserved1: [u8; 4usize],
    #[doc = "0x2c - 128-bit signature Word 0"]
    pub fmsw0: FMSW0,
    #[doc = "0x30 - 128-bit signature Word 1"]
    pub fmsw1: FMSW1,
    #[doc = "0x34 - 128-bit signature Word 2"]
    pub fmsw2: FMSW2,
    #[doc = "0x38 - 128-bit signature Word 3"]
    pub fmsw3: FMSW3,
    _reserved2: [u8; 68usize],
    #[doc = "0x80 - EEPROM command register"]
    pub eecmd: EECMD,
    #[doc = "0x84 - EEPROM address register"]
    pub eeaddr: EEADDR,
    #[doc = "0x88 - EEPROM write data register"]
    pub eewdata: EEWDATA,
    #[doc = "0x8c - EEPROM read data register"]
    pub eerdata: EERDATA,
    #[doc = "0x90 - EEPROM wait state register"]
    pub eewstate: EEWSTATE,
    #[doc = "0x94 - EEPROM clock divider register"]
    pub eeclkdiv: EECLKDIV,
    #[doc = "0x98 - EEPROM power-down register"]
    pub eepwrdwn: EEPWRDWN,
    _reserved3: [u8; 3900usize],
    #[doc = "0xfd8 - EEPROM interrupt enable clear"]
    pub enclr: ENCLR,
    #[doc = "0xfdc - EEPROM interrupt enable set"]
    pub enset: ENSET,
    #[doc = "0xfe0 - Signature generation status register"]
    pub stat: STAT,
    #[doc = "0xfe4 - EEPROM interrupt enable"]
    pub inten: INTEN,
    #[doc = "0xfe8 - Signature generation status clear register"]
    pub statclr: STATCLR,
}
#[doc = "Signature start address register"]
pub struct FMSSTART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Signature start address register"]
pub mod fmsstart;
#[doc = "Signature stop-address register"]
pub struct FMSSTOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Signature stop-address register"]
pub mod fmsstop;
#[doc = "128-bit signature Word 0"]
pub struct FMSW0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "128-bit signature Word 0"]
pub mod fmsw0;
#[doc = "128-bit signature Word 1"]
pub struct FMSW1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "128-bit signature Word 1"]
pub mod fmsw1;
#[doc = "128-bit signature Word 2"]
pub struct FMSW2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "128-bit signature Word 2"]
pub mod fmsw2;
#[doc = "128-bit signature Word 3"]
pub struct FMSW3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "128-bit signature Word 3"]
pub mod fmsw3;
#[doc = "EEPROM command register"]
pub struct EECMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM command register"]
pub mod eecmd;
#[doc = "EEPROM address register"]
pub struct EEADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM address register"]
pub mod eeaddr;
#[doc = "EEPROM write data register"]
pub struct EEWDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM write data register"]
pub mod eewdata;
#[doc = "EEPROM read data register"]
pub struct EERDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM read data register"]
pub mod eerdata;
#[doc = "EEPROM wait state register"]
pub struct EEWSTATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM wait state register"]
pub mod eewstate;
#[doc = "EEPROM clock divider register"]
pub struct EECLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM clock divider register"]
pub mod eeclkdiv;
#[doc = "EEPROM power-down register"]
pub struct EEPWRDWN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM power-down register"]
pub mod eepwrdwn;
#[doc = "Signature generation status register"]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Signature generation status register"]
pub mod stat;
#[doc = "EEPROM interrupt enable"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM interrupt enable"]
pub mod inten;
#[doc = "Signature generation status clear register"]
pub struct STATCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Signature generation status clear register"]
pub mod statclr;
#[doc = "EEPROM interrupt enable clear"]
pub struct ENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM interrupt enable clear"]
pub mod enclr;
#[doc = "EEPROM interrupt enable set"]
pub struct ENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEPROM interrupt enable set"]
pub mod enset;
