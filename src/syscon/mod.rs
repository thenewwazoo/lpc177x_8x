#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash Accelerator Configuration Register. Controls flash access timing."]
    pub flashcfg: FLASHCFG,
    _reserved0: [u8; 124usize],
    #[doc = "0x80 - PLL0 Control register"]
    pub pll0con: PLLCON,
    #[doc = "0x84 - PLL0 Configuration register"]
    pub pll0cfg: PLLCFG,
    #[doc = "0x88 - PLL0 Status register"]
    pub pll0stat: PLLSTAT,
    #[doc = "0x8c - PLL0 Feed register"]
    pub pll0feed: PLLFEED,
    _reserved1: [u8; 16usize],
    #[doc = "0xa0 - PLL0 Control register"]
    pub pll1con: PLLCON,
    #[doc = "0xa4 - PLL0 Configuration register"]
    pub pll1cfg: PLLCFG,
    #[doc = "0xa8 - PLL0 Status register"]
    pub pll1stat: PLLSTAT,
    #[doc = "0xac - PLL0 Feed register"]
    pub pll1feed: PLLFEED,
    _reserved2: [u8; 16usize],
    #[doc = "0xc0 - Power Control register"]
    pub pcon: PCON,
    #[doc = "0xc4 - Power Control for Peripherals"]
    pub pconp: PCONP,
    _reserved3: [u8; 56usize],
    #[doc = "0x100 - External Memory Controller Clock Selection register"]
    pub emcclksel: EMCCLKSEL,
    #[doc = "0x104 - CPU Clock Selection register"]
    pub cclksel: CCLKSEL,
    #[doc = "0x108 - USB Clock Selection register"]
    pub usbclksel: USBCLKSEL,
    #[doc = "0x10c - Clock Source Select Register"]
    pub clksrcsel: CLKSRCSEL,
    #[doc = "0x110 - Allows clearing the current CAN channel sleep state as well as reading that state."]
    pub cansleepclr: CANSLEEPCLR,
    #[doc = "0x114 - Allows reading the wake-up state of the CAN channels."]
    pub canwakeflags: CANWAKEFLAGS,
    _reserved4: [u8; 40usize],
    #[doc = "0x140 - External Interrupt Flag Register"]
    pub extint: EXTINT,
    _reserved5: [u8; 4usize],
    #[doc = "0x148 - External Interrupt Mode register"]
    pub extmode: EXTMODE,
    #[doc = "0x14c - External Interrupt Polarity Register"]
    pub extpolar: EXTPOLAR,
    _reserved6: [u8; 48usize],
    #[doc = "0x180 - Reset Source Identification Register"]
    pub rsid: RSID,
    _reserved7: [u8; 4usize],
    #[doc = "0x188 - Matrix arbitration register"]
    pub matrixarb: MATRIXARB,
    _reserved8: [u8; 20usize],
    #[doc = "0x1a0 - System Control and Status"]
    pub scs: SCS,
    _reserved9: [u8; 4usize],
    #[doc = "0x1a8 - Peripheral Clock Selection register"]
    pub pclksel: PCLKSEL,
    _reserved10: [u8; 4usize],
    #[doc = "0x1b0 - Power boost register"]
    pub pboost: PBOOST,
    #[doc = "0x1b4 - SPIFI Clock Selection register"]
    pub spificlksel: SPIFICLKSEL,
    #[doc = "0x1b8 - LCD Clock configuration register"]
    pub lcd_cfg: LCD_CFG,
    _reserved11: [u8; 4usize],
    #[doc = "0x1c0 - USB Interrupt Status"]
    pub usbintst: USBINTST,
    #[doc = "0x1c4 - Selects between alternative requests on DMA channels 0 through 7 and 10 through 15"]
    pub dmacreqsel: DMACREQSEL,
    #[doc = "0x1c8 - Clock Output Configuration register"]
    pub clkoutcfg: CLKOUTCFG,
    #[doc = "0x1cc - Individual peripheral reset control bits"]
    pub rstcon0: RSTCON0,
    #[doc = "0x1d0 - Individual peripheral reset control bits"]
    pub rstcon1: RSTCON1,
    _reserved12: [u8; 8usize],
    #[doc = "0x1dc - Values for the 4 programmable delays associated with SDRAM operation."]
    pub emcdlyctl: EMCDLYCTL,
    #[doc = "0x1e0 - Controls the calibration counter for programmable delays and returns the result value."]
    pub emccal: EMCCAL,
}
#[doc = "Flash Accelerator Configuration Register. Controls flash access timing."]
pub struct FLASHCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Accelerator Configuration Register. Controls flash access timing."]
pub mod flashcfg;
#[doc = "PLL0 Control register"]
pub struct PLLCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL0 Control register"]
pub mod pllcon;
#[doc = "PLL0 Configuration register"]
pub struct PLLCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL0 Configuration register"]
pub mod pllcfg;
#[doc = "PLL0 Status register"]
pub struct PLLSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL0 Status register"]
pub mod pllstat;
#[doc = "PLL0 Feed register"]
pub struct PLLFEED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL0 Feed register"]
pub mod pllfeed;
#[doc = "Power Control register"]
pub struct PCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Control register"]
pub mod pcon;
#[doc = "Power Control for Peripherals"]
pub struct PCONP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Control for Peripherals"]
pub mod pconp;
#[doc = "External Memory Controller Clock Selection register"]
pub struct EMCCLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External Memory Controller Clock Selection register"]
pub mod emcclksel;
#[doc = "CPU Clock Selection register"]
pub struct CCLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU Clock Selection register"]
pub mod cclksel;
#[doc = "USB Clock Selection register"]
pub struct USBCLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Clock Selection register"]
pub mod usbclksel;
#[doc = "Clock Source Select Register"]
pub struct CLKSRCSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Source Select Register"]
pub mod clksrcsel;
#[doc = "Allows clearing the current CAN channel sleep state as well as reading that state."]
pub struct CANSLEEPCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Allows clearing the current CAN channel sleep state as well as reading that state."]
pub mod cansleepclr;
#[doc = "Allows reading the wake-up state of the CAN channels."]
pub struct CANWAKEFLAGS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Allows reading the wake-up state of the CAN channels."]
pub mod canwakeflags;
#[doc = "External Interrupt Flag Register"]
pub struct EXTINT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External Interrupt Flag Register"]
pub mod extint;
#[doc = "External Interrupt Mode register"]
pub struct EXTMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External Interrupt Mode register"]
pub mod extmode;
#[doc = "External Interrupt Polarity Register"]
pub struct EXTPOLAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External Interrupt Polarity Register"]
pub mod extpolar;
#[doc = "Reset Source Identification Register"]
pub struct RSID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset Source Identification Register"]
pub mod rsid;
#[doc = "Matrix arbitration register"]
pub struct MATRIXARB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Matrix arbitration register"]
pub mod matrixarb;
#[doc = "System Control and Status"]
pub struct SCS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Control and Status"]
pub mod scs;
#[doc = "Peripheral Clock Selection register"]
pub struct PCLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Clock Selection register"]
pub mod pclksel;
#[doc = "Power boost register"]
pub struct PBOOST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power boost register"]
pub mod pboost;
#[doc = "SPIFI Clock Selection register"]
pub struct SPIFICLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPIFI Clock Selection register"]
pub mod spificlksel;
#[doc = "LCD Clock configuration register"]
pub struct LCD_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LCD Clock configuration register"]
pub mod lcd_cfg;
#[doc = "USB Interrupt Status"]
pub struct USBINTST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Interrupt Status"]
pub mod usbintst;
#[doc = "Selects between alternative requests on DMA channels 0 through 7 and 10 through 15"]
pub struct DMACREQSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Selects between alternative requests on DMA channels 0 through 7 and 10 through 15"]
pub mod dmacreqsel;
#[doc = "Clock Output Configuration register"]
pub struct CLKOUTCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Output Configuration register"]
pub mod clkoutcfg;
#[doc = "Individual peripheral reset control bits"]
pub struct RSTCON0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Individual peripheral reset control bits"]
pub mod rstcon0;
#[doc = "Individual peripheral reset control bits"]
pub struct RSTCON1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Individual peripheral reset control bits"]
pub mod rstcon1;
#[doc = "Values for the 4 programmable delays associated with SDRAM operation."]
pub struct EMCDLYCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Values for the 4 programmable delays associated with SDRAM operation."]
pub mod emcdlyctl;
#[doc = "Controls the calibration counter for programmable delays and returns the result value."]
pub struct EMCCAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Controls the calibration counter for programmable delays and returns the result value."]
pub mod emccal;
