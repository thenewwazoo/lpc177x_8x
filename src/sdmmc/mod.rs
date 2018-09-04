#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power control register."]
    pub pwr: PWR,
    #[doc = "0x04 - Clock control register."]
    pub clk: CLK,
    #[doc = "0x08 - Argument register."]
    pub argument: ARGUMENT,
    #[doc = "0x0c - Command register."]
    pub command: COMMAND,
    #[doc = "0x10 - Response command register."]
    pub respcmd: RESPCMD,
    #[doc = "0x14 - Response register."]
    pub response: [RESPONSE; 4],
    #[doc = "0x24 - Data Timer."]
    pub datatimer: DATATIMER,
    #[doc = "0x28 - Data length register."]
    pub datalength: DATALENGTH,
    #[doc = "0x2c - Data control register."]
    pub datactrl: DATACTRL,
    #[doc = "0x30 - Data counter."]
    pub datacnt: DATACNT,
    #[doc = "0x34 - Status register."]
    pub status: STATUS,
    #[doc = "0x38 - Clear register."]
    pub clear: CLEAR,
    #[doc = "0x3c - Interrupt 0 mask register."]
    pub mask0: MASK0,
    _reserved13: [u8; 8usize],
    #[doc = "0x48 - FIFO Counter."]
    pub fifocnt: FIFOCNT,
    _reserved14: [u8; 52usize],
    #[doc = "0x80 - Data FIFO Register."]
    pub fifo: [FIFO; 16],
}
#[doc = "Power control register."]
pub struct PWR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power control register."]
pub mod pwr;
#[doc = "Clock control register."]
pub struct CLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock control register."]
pub mod clk;
#[doc = "Argument register."]
pub struct ARGUMENT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Argument register."]
pub mod argument;
#[doc = "Command register."]
pub struct COMMAND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command register."]
pub mod command;
#[doc = "Response command register."]
pub struct RESPCMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Response command register."]
pub mod respcmd;
#[doc = "Response register."]
pub struct RESPONSE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Response register."]
pub mod response;
#[doc = "Data Timer."]
pub struct DATATIMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Timer."]
pub mod datatimer;
#[doc = "Data length register."]
pub struct DATALENGTH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data length register."]
pub mod datalength;
#[doc = "Data control register."]
pub struct DATACTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data control register."]
pub mod datactrl;
#[doc = "Data counter."]
pub struct DATACNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data counter."]
pub mod datacnt;
#[doc = "Status register."]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register."]
pub mod status;
#[doc = "Clear register."]
pub struct CLEAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear register."]
pub mod clear;
#[doc = "Interrupt 0 mask register."]
pub struct MASK0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt 0 mask register."]
pub mod mask0;
#[doc = "FIFO Counter."]
pub struct FIFOCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO Counter."]
pub mod fifocnt;
#[doc = "Data FIFO Register."]
pub struct FIFO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data FIFO Register."]
pub mod fifo;
