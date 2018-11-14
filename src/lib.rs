#![doc = "Peripheral access API for LPC178X7X microcontrollers (generated using svd2rust v0.13.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.13.1/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
#![feature(untagged_unions)]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
extern "C" {
    fn WWDT();
    fn TIMER0();
    fn TIMER1();
    fn TIMER2();
    fn TIMER3();
    fn UART0();
    fn UART1();
    fn UART2();
    fn UART3();
    fn PWM1();
    fn I2C0();
    fn I2C1();
    fn I2C2();
    fn SSP0();
    fn SSP1();
    fn RTC();
    fn EINT0();
    fn EINT1();
    fn EINT2();
    fn EINT3();
    fn ADC();
    fn BOD();
    fn USB();
    fn CAN();
    fn GPDMA();
    fn I2S();
    fn ETHERNET();
    fn SDMMC();
    fn MCPWM();
    fn QEI();
    fn USB_NEED_CLK();
    fn UART4();
    fn SSP2();
    fn LCD();
    fn GPIOINT();
    fn PWM0();
    fn EEPROM();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 41] = [
    Vector { _handler: WWDT },
    Vector { _handler: TIMER0 },
    Vector { _handler: TIMER1 },
    Vector { _handler: TIMER2 },
    Vector { _handler: TIMER3 },
    Vector { _handler: UART0 },
    Vector { _handler: UART1 },
    Vector { _handler: UART2 },
    Vector { _handler: UART3 },
    Vector { _handler: PWM1 },
    Vector { _handler: I2C0 },
    Vector { _handler: I2C1 },
    Vector { _handler: I2C2 },
    Vector { _reserved: 0 },
    Vector { _handler: SSP0 },
    Vector { _handler: SSP1 },
    Vector { _reserved: 0 },
    Vector { _handler: RTC },
    Vector { _handler: EINT0 },
    Vector { _handler: EINT1 },
    Vector { _handler: EINT2 },
    Vector { _handler: EINT3 },
    Vector { _handler: ADC },
    Vector { _handler: BOD },
    Vector { _handler: USB },
    Vector { _handler: CAN },
    Vector { _handler: GPDMA },
    Vector { _handler: I2S },
    Vector { _handler: ETHERNET },
    Vector { _handler: SDMMC },
    Vector { _handler: MCPWM },
    Vector { _handler: QEI },
    Vector { _reserved: 0 },
    Vector {
        _handler: USB_NEED_CLK,
    },
    Vector { _reserved: 0 },
    Vector { _handler: UART4 },
    Vector { _handler: SSP2 },
    Vector { _handler: LCD },
    Vector { _handler: GPIOINT },
    Vector { _handler: PWM0 },
    Vector { _handler: EEPROM },
];
#[doc = r" Macro to override a device specific interrupt handler"]
#[doc = r""]
#[doc = r" # Syntax"]
#[doc = r""]
#[doc = r" ``` ignore"]
#[doc = r" interrupt!("]
#[doc = r"     // Name of the interrupt"]
#[doc = r"     $Name:ident,"]
#[doc = r""]
#[doc = r"     // Path to the interrupt handler (a function)"]
#[doc = r"     $handler:path,"]
#[doc = r""]
#[doc = r"     // Optional, state preserved across invocations of the handler"]
#[doc = r"     state: $State:ty = $initial_state:expr,"]
#[doc = r" );"]
#[doc = r" ```"]
#[doc = r""]
#[doc = r" Where `$Name` must match the name of one of the variants of the `Interrupt`"]
#[doc = r" enum."]
#[doc = r""]
#[doc = r" The handler must have signature `fn()` is no state was associated to it;"]
#[doc = r" otherwise its signature must be `fn(&mut $State)`."]
#[cfg(feature = "rt")]
#[macro_export]
macro_rules! interrupt {
    ( $ Name : ident , $ handler : path , state : $ State : ty = $ initial_state : expr ) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)]
        #[no_mangle]
        pub unsafe extern "C" fn $Name() {
            static mut STATE: $State = $initial_state;
            let _ = $crate::Interrupt::$Name;
            let f: fn(&mut $State) = $handler;
            f(&mut STATE)
        }
    };
    ( $ Name : ident , $ handler : path ) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)]
        #[no_mangle]
        pub unsafe extern "C" fn $Name() {
            let _ = $crate::Interrupt::$Name;
            let f: fn() = $handler;
            f()
        }
    };
}
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - WWDT"]
    WWDT,
    #[doc = "1 - TIMER0"]
    TIMER0,
    #[doc = "2 - TIMER1"]
    TIMER1,
    #[doc = "3 - TIMER2"]
    TIMER2,
    #[doc = "4 - TIMER3"]
    TIMER3,
    #[doc = "5 - UART0"]
    UART0,
    #[doc = "6 - UART1"]
    UART1,
    #[doc = "7 - UART2"]
    UART2,
    #[doc = "8 - UART3"]
    UART3,
    #[doc = "9 - PWM1"]
    PWM1,
    #[doc = "10 - I2C0"]
    I2C0,
    #[doc = "11 - I2C1"]
    I2C1,
    #[doc = "12 - I2C2"]
    I2C2,
    #[doc = "14 - SSP0"]
    SSP0,
    #[doc = "15 - SSP1"]
    SSP1,
    #[doc = "17 - RTC"]
    RTC,
    #[doc = "18 - EINT0"]
    EINT0,
    #[doc = "19 - EINT1"]
    EINT1,
    #[doc = "20 - EINT2"]
    EINT2,
    #[doc = "21 - EINT3"]
    EINT3,
    #[doc = "22 - ADC"]
    ADC,
    #[doc = "23 - BOD"]
    BOD,
    #[doc = "24 - USB"]
    USB,
    #[doc = "25 - CAN"]
    CAN,
    #[doc = "26 - GPDMA"]
    GPDMA,
    #[doc = "27 - I2S"]
    I2S,
    #[doc = "28 - ETHERNET"]
    ETHERNET,
    #[doc = "29 - SDMMC"]
    SDMMC,
    #[doc = "30 - MCPWM"]
    MCPWM,
    #[doc = "31 - QEI"]
    QEI,
    #[doc = "33 - USB_NEED_CLK"]
    USB_NEED_CLK,
    #[doc = "35 - UART4"]
    UART4,
    #[doc = "36 - SSP2"]
    SSP2,
    #[doc = "37 - LCD"]
    LCD,
    #[doc = "38 - GPIOINT"]
    GPIOINT,
    #[doc = "39 - PWM0"]
    PWM0,
    #[doc = "40 - EEPROM"]
    EEPROM,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::WWDT => 0,
            Interrupt::TIMER0 => 1,
            Interrupt::TIMER1 => 2,
            Interrupt::TIMER2 => 3,
            Interrupt::TIMER3 => 4,
            Interrupt::UART0 => 5,
            Interrupt::UART1 => 6,
            Interrupt::UART2 => 7,
            Interrupt::UART3 => 8,
            Interrupt::PWM1 => 9,
            Interrupt::I2C0 => 10,
            Interrupt::I2C1 => 11,
            Interrupt::I2C2 => 12,
            Interrupt::SSP0 => 14,
            Interrupt::SSP1 => 15,
            Interrupt::RTC => 17,
            Interrupt::EINT0 => 18,
            Interrupt::EINT1 => 19,
            Interrupt::EINT2 => 20,
            Interrupt::EINT3 => 21,
            Interrupt::ADC => 22,
            Interrupt::BOD => 23,
            Interrupt::USB => 24,
            Interrupt::CAN => 25,
            Interrupt::GPDMA => 26,
            Interrupt::I2S => 27,
            Interrupt::ETHERNET => 28,
            Interrupt::SDMMC => 29,
            Interrupt::MCPWM => 30,
            Interrupt::QEI => 31,
            Interrupt::USB_NEED_CLK => 33,
            Interrupt::UART4 => 35,
            Interrupt::SSP2 => 36,
            Interrupt::LCD => 37,
            Interrupt::GPIOINT => 38,
            Interrupt::PWM0 => 39,
            Interrupt::EEPROM => 40,
        }
    }
}
#[doc(hidden)]
pub mod interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{
    CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU,
};
#[doc = "Flash control block"]
pub struct FLASHCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASHCTRL {}
impl FLASHCTRL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const flashctrl::RegisterBlock {
        2097152 as *const _
    }
}
impl Deref for FLASHCTRL {
    type Target = flashctrl::RegisterBlock;
    fn deref(&self) -> &flashctrl::RegisterBlock {
        unsafe { &*FLASHCTRL::ptr() }
    }
}
#[doc = "Flash control block"]
pub mod flashctrl;
#[doc = "General purpose DMA controller"]
pub struct GPDMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPDMA {}
impl GPDMA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpdma::RegisterBlock {
        537395200 as *const _
    }
}
impl Deref for GPDMA {
    type Target = gpdma::RegisterBlock;
    fn deref(&self) -> &gpdma::RegisterBlock {
        unsafe { &*GPDMA::ptr() }
    }
}
#[doc = "General purpose DMA controller"]
pub mod gpdma;
#[doc = "Ethernet"]
pub struct ETHERNET {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ETHERNET {}
impl ETHERNET {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ethernet::RegisterBlock {
        537411584 as *const _
    }
}
impl Deref for ETHERNET {
    type Target = ethernet::RegisterBlock;
    fn deref(&self) -> &ethernet::RegisterBlock {
        unsafe { &*ETHERNET::ptr() }
    }
}
#[doc = "Ethernet"]
pub mod ethernet;
#[doc = "LCD controller"]
pub struct LCD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LCD {}
impl LCD {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lcd::RegisterBlock {
        537427968 as *const _
    }
}
impl Deref for LCD {
    type Target = lcd::RegisterBlock;
    fn deref(&self) -> &lcd::RegisterBlock {
        unsafe { &*LCD::ptr() }
    }
}
#[doc = "LCD controller"]
pub mod lcd;
#[doc = "USB device/host/OTG controller"]
pub struct USB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB {}
impl USB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb::RegisterBlock {
        537444352 as *const _
    }
}
impl Deref for USB {
    type Target = usb::RegisterBlock;
    fn deref(&self) -> &usb::RegisterBlock {
        unsafe { &*USB::ptr() }
    }
}
#[doc = "USB device/host/OTG controller"]
pub mod usb;
#[doc = "CRC engine"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const crc::RegisterBlock {
        537460736 as *const _
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    fn deref(&self) -> &crc::RegisterBlock {
        unsafe { &*CRC::ptr() }
    }
}
#[doc = "CRC engine"]
pub mod crc;
#[doc = "General Purpose I/O"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpio::RegisterBlock {
        537493504 as *const _
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    fn deref(&self) -> &gpio::RegisterBlock {
        unsafe { &*GPIO::ptr() }
    }
}
#[doc = "General Purpose I/O"]
pub mod gpio;
#[doc = "ExternalMemory Controller (EMC)"]
pub struct EMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EMC {}
impl EMC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const emc::RegisterBlock {
        537509888 as *const _
    }
}
impl Deref for EMC {
    type Target = emc::RegisterBlock;
    fn deref(&self) -> &emc::RegisterBlock {
        unsafe { &*EMC::ptr() }
    }
}
#[doc = "ExternalMemory Controller (EMC)"]
pub mod emc;
#[doc = "Windowed Watchdog Timer (WWDT)"]
pub struct WWDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WWDT {}
impl WWDT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wwdt::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for WWDT {
    type Target = wwdt::RegisterBlock;
    fn deref(&self) -> &wwdt::RegisterBlock {
        unsafe { &*WWDT::ptr() }
    }
}
#[doc = "Windowed Watchdog Timer (WWDT)"]
pub mod wwdt;
#[doc = "Timer0/1/2/3"]
pub struct TIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER0 {}
impl TIMER0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer0::RegisterBlock {
        1073758208 as *const _
    }
}
impl Deref for TIMER0 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &timer0::RegisterBlock {
        unsafe { &*TIMER0::ptr() }
    }
}
#[doc = "Timer0/1/2/3"]
pub mod timer0;
#[doc = "TIMER1"]
pub struct TIMER1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER1 {}
impl TIMER1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer0::RegisterBlock {
        1073774592 as *const _
    }
}
impl Deref for TIMER1 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &timer0::RegisterBlock {
        unsafe { &*TIMER1::ptr() }
    }
}
#[doc = "UART0/2/3"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart0::RegisterBlock {
        1073790976 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &uart0::RegisterBlock {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "UART0/2/3"]
pub mod uart0;
#[doc = "UART1"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart1::RegisterBlock {
        1073807360 as *const _
    }
}
impl Deref for UART1 {
    type Target = uart1::RegisterBlock;
    fn deref(&self) -> &uart1::RegisterBlock {
        unsafe { &*UART1::ptr() }
    }
}
#[doc = "UART1"]
pub mod uart1;
#[doc = "Pulse Width Modulators (PWM0/1)"]
pub struct PWM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM0 {}
impl PWM0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwm0::RegisterBlock {
        1073823744 as *const _
    }
}
impl Deref for PWM0 {
    type Target = pwm0::RegisterBlock;
    fn deref(&self) -> &pwm0::RegisterBlock {
        unsafe { &*PWM0::ptr() }
    }
}
#[doc = "Pulse Width Modulators (PWM0/1)"]
pub mod pwm0;
#[doc = "PWM1"]
pub struct PWM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM1 {}
impl PWM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwm0::RegisterBlock {
        1073840128 as *const _
    }
}
impl Deref for PWM1 {
    type Target = pwm0::RegisterBlock;
    fn deref(&self) -> &pwm0::RegisterBlock {
        unsafe { &*PWM1::ptr() }
    }
}
#[doc = "I2C bus interface"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c0::RegisterBlock {
        1073856512 as *const _
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &i2c0::RegisterBlock {
        unsafe { &*I2C0::ptr() }
    }
}
#[doc = "I2C bus interface"]
pub mod i2c0;
#[doc = "Real Time Clock (RTC)"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc::RegisterBlock {
        1073889280 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &rtc::RegisterBlock {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real Time Clock (RTC)"]
pub mod rtc;
#[doc = "GPIO"]
pub struct GPIOINT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOINT {}
impl GPIOINT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioint::RegisterBlock {
        1073905792 as *const _
    }
}
impl Deref for GPIOINT {
    type Target = gpioint::RegisterBlock;
    fn deref(&self) -> &gpioint::RegisterBlock {
        unsafe { &*GPIOINT::ptr() }
    }
}
#[doc = "GPIO"]
pub mod gpioint;
#[doc = "IOCON pin configuration"]
pub struct IOCON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IOCON {}
impl IOCON {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const iocon::RegisterBlock {
        1073922048 as *const _
    }
}
impl Deref for IOCON {
    type Target = iocon::RegisterBlock;
    fn deref(&self) -> &iocon::RegisterBlock {
        unsafe { &*IOCON::ptr() }
    }
}
#[doc = "IOCON pin configuration"]
pub mod iocon;
#[doc = "SSP1 controller"]
pub struct SSP1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSP1 {}
impl SSP1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ssp1::RegisterBlock {
        1073938432 as *const _
    }
}
impl Deref for SSP1 {
    type Target = ssp1::RegisterBlock;
    fn deref(&self) -> &ssp1::RegisterBlock {
        unsafe { &*SSP1::ptr() }
    }
}
#[doc = "SSP1 controller"]
pub mod ssp1;
#[doc = "Analog-to-Digital Converter (ADC)"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc::RegisterBlock {
        1073954816 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    fn deref(&self) -> &adc::RegisterBlock {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "Analog-to-Digital Converter (ADC)"]
pub mod adc;
#[doc = "CAN acceptance filter RAM"]
pub struct CANAFRAM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CANAFRAM {}
impl CANAFRAM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const canafram::RegisterBlock {
        1073971200 as *const _
    }
}
impl Deref for CANAFRAM {
    type Target = canafram::RegisterBlock;
    fn deref(&self) -> &canafram::RegisterBlock {
        unsafe { &*CANAFRAM::ptr() }
    }
}
#[doc = "CAN acceptance filter RAM"]
pub mod canafram;
#[doc = "CAN controller acceptance filter"]
pub struct CANAF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CANAF {}
impl CANAF {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const canaf::RegisterBlock {
        1073987584 as *const _
    }
}
impl Deref for CANAF {
    type Target = canaf::RegisterBlock;
    fn deref(&self) -> &canaf::RegisterBlock {
        unsafe { &*CANAF::ptr() }
    }
}
#[doc = "CAN controller acceptance filter"]
pub mod canaf;
#[doc = "Central CAN controller"]
pub struct CCAN {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCAN {}
impl CCAN {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccan::RegisterBlock {
        1074003968 as *const _
    }
}
impl Deref for CCAN {
    type Target = ccan::RegisterBlock;
    fn deref(&self) -> &ccan::RegisterBlock {
        unsafe { &*CCAN::ptr() }
    }
}
#[doc = "Central CAN controller"]
pub mod ccan;
#[doc = "CAN1 controller"]
pub struct CAN1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN1 {}
impl CAN1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can1::RegisterBlock {
        1074020352 as *const _
    }
}
impl Deref for CAN1 {
    type Target = can1::RegisterBlock;
    fn deref(&self) -> &can1::RegisterBlock {
        unsafe { &*CAN1::ptr() }
    }
}
#[doc = "CAN1 controller"]
pub mod can1;
#[doc = "CAN2"]
pub struct CAN2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN2 {}
impl CAN2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can1::RegisterBlock {
        1074036736 as *const _
    }
}
impl Deref for CAN2 {
    type Target = can1::RegisterBlock;
    fn deref(&self) -> &can1::RegisterBlock {
        unsafe { &*CAN2::ptr() }
    }
}
#[doc = "I2C1"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c0::RegisterBlock {
        1074118656 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &i2c0::RegisterBlock {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "SSP controller"]
pub struct SSP0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSP0 {}
impl SSP0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ssp1::RegisterBlock {
        1074298880 as *const _
    }
}
impl Deref for SSP0 {
    type Target = ssp1::RegisterBlock;
    fn deref(&self) -> &ssp1::RegisterBlock {
        unsafe { &*SSP0::ptr() }
    }
}
#[doc = "Digital-to-Analog Converter (DAC)"]
pub struct DAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC {}
impl DAC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dac::RegisterBlock {
        1074315264 as *const _
    }
}
impl Deref for DAC {
    type Target = dac::RegisterBlock;
    fn deref(&self) -> &dac::RegisterBlock {
        unsafe { &*DAC::ptr() }
    }
}
#[doc = "Digital-to-Analog Converter (DAC)"]
pub mod dac;
#[doc = "TIMER2"]
pub struct TIMER2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER2 {}
impl TIMER2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer0::RegisterBlock {
        1074331648 as *const _
    }
}
impl Deref for TIMER2 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &timer0::RegisterBlock {
        unsafe { &*TIMER2::ptr() }
    }
}
#[doc = "TIMER3"]
pub struct TIMER3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER3 {}
impl TIMER3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer0::RegisterBlock {
        1074348032 as *const _
    }
}
impl Deref for TIMER3 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &timer0::RegisterBlock {
        unsafe { &*TIMER3::ptr() }
    }
}
#[doc = "UART2"]
pub struct UART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART2 {}
impl UART2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart0::RegisterBlock {
        1074364416 as *const _
    }
}
impl Deref for UART2 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &uart0::RegisterBlock {
        unsafe { &*UART2::ptr() }
    }
}
#[doc = "UART3"]
pub struct UART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART3 {}
impl UART3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart0::RegisterBlock {
        1074380800 as *const _
    }
}
impl Deref for UART3 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &uart0::RegisterBlock {
        unsafe { &*UART3::ptr() }
    }
}
#[doc = "I2C2"]
pub struct I2C2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C2 {}
impl I2C2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c0::RegisterBlock {
        1074397184 as *const _
    }
}
impl Deref for I2C2 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &i2c0::RegisterBlock {
        unsafe { &*I2C2::ptr() }
    }
}
#[doc = "UART4"]
pub struct UART4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART4 {}
impl UART4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart4::RegisterBlock {
        1074413568 as *const _
    }
}
impl Deref for UART4 {
    type Target = uart4::RegisterBlock;
    fn deref(&self) -> &uart4::RegisterBlock {
        unsafe { &*UART4::ptr() }
    }
}
#[doc = "UART4"]
pub mod uart4;
#[doc = "I2S interface"]
pub struct I2S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S {}
impl I2S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2s::RegisterBlock {
        1074429952 as *const _
    }
}
impl Deref for I2S {
    type Target = i2s::RegisterBlock;
    fn deref(&self) -> &i2s::RegisterBlock {
        unsafe { &*I2S::ptr() }
    }
}
#[doc = "I2S interface"]
pub mod i2s;
#[doc = "SSP2"]
pub struct SSP2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSP2 {}
impl SSP2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ssp1::RegisterBlock {
        1074446336 as *const _
    }
}
impl Deref for SSP2 {
    type Target = ssp1::RegisterBlock;
    fn deref(&self) -> &ssp1::RegisterBlock {
        unsafe { &*SSP2::ptr() }
    }
}
#[doc = "Motor Control PWM"]
pub struct MCPWM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MCPWM {}
impl MCPWM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mcpwm::RegisterBlock {
        1074495488 as *const _
    }
}
impl Deref for MCPWM {
    type Target = mcpwm::RegisterBlock;
    fn deref(&self) -> &mcpwm::RegisterBlock {
        unsafe { &*MCPWM::ptr() }
    }
}
#[doc = "Motor Control PWM"]
pub mod mcpwm;
#[doc = "Quadrature Encoder Interface (QEI)"]
pub struct QEI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QEI {}
impl QEI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const qei::RegisterBlock {
        1074511872 as *const _
    }
}
impl Deref for QEI {
    type Target = qei::RegisterBlock;
    fn deref(&self) -> &qei::RegisterBlock {
        unsafe { &*QEI::ptr() }
    }
}
#[doc = "Quadrature Encoder Interface (QEI)"]
pub mod qei;
#[doc = "SD card interface"]
pub struct SDMMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDMMC {}
impl SDMMC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sdmmc::RegisterBlock {
        1074528256 as *const _
    }
}
impl Deref for SDMMC {
    type Target = sdmmc::RegisterBlock;
    fn deref(&self) -> &sdmmc::RegisterBlock {
        unsafe { &*SDMMC::ptr() }
    }
}
#[doc = "SD card interface"]
pub mod sdmmc;
#[doc = "System and clock control"]
pub struct SYSCON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCON {}
impl SYSCON {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const syscon::RegisterBlock {
        1074774016 as *const _
    }
}
impl Deref for SYSCON {
    type Target = syscon::RegisterBlock;
    fn deref(&self) -> &syscon::RegisterBlock {
        unsafe { &*SYSCON::ptr() }
    }
}
#[doc = "System and clock control"]
pub mod syscon;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "FLASHCTRL"]
    pub FLASHCTRL: FLASHCTRL,
    #[doc = "GPDMA"]
    pub GPDMA: GPDMA,
    #[doc = "ETHERNET"]
    pub ETHERNET: ETHERNET,
    #[doc = "LCD"]
    pub LCD: LCD,
    #[doc = "USB"]
    pub USB: USB,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "EMC"]
    pub EMC: EMC,
    #[doc = "WWDT"]
    pub WWDT: WWDT,
    #[doc = "TIMER0"]
    pub TIMER0: TIMER0,
    #[doc = "TIMER1"]
    pub TIMER1: TIMER1,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "PWM0"]
    pub PWM0: PWM0,
    #[doc = "PWM1"]
    pub PWM1: PWM1,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "GPIOINT"]
    pub GPIOINT: GPIOINT,
    #[doc = "IOCON"]
    pub IOCON: IOCON,
    #[doc = "SSP1"]
    pub SSP1: SSP1,
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "CANAFRAM"]
    pub CANAFRAM: CANAFRAM,
    #[doc = "CANAF"]
    pub CANAF: CANAF,
    #[doc = "CCAN"]
    pub CCAN: CCAN,
    #[doc = "CAN1"]
    pub CAN1: CAN1,
    #[doc = "CAN2"]
    pub CAN2: CAN2,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "SSP0"]
    pub SSP0: SSP0,
    #[doc = "DAC"]
    pub DAC: DAC,
    #[doc = "TIMER2"]
    pub TIMER2: TIMER2,
    #[doc = "TIMER3"]
    pub TIMER3: TIMER3,
    #[doc = "UART2"]
    pub UART2: UART2,
    #[doc = "UART3"]
    pub UART3: UART3,
    #[doc = "I2C2"]
    pub I2C2: I2C2,
    #[doc = "UART4"]
    pub UART4: UART4,
    #[doc = "I2S"]
    pub I2S: I2S,
    #[doc = "SSP2"]
    pub SSP2: SSP2,
    #[doc = "MCPWM"]
    pub MCPWM: MCPWM,
    #[doc = "QEI"]
    pub QEI: QEI,
    #[doc = "SDMMC"]
    pub SDMMC: SDMMC,
    #[doc = "SYSCON"]
    pub SYSCON: SYSCON,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            FLASHCTRL: FLASHCTRL {
                _marker: PhantomData,
            },
            GPDMA: GPDMA {
                _marker: PhantomData,
            },
            ETHERNET: ETHERNET {
                _marker: PhantomData,
            },
            LCD: LCD {
                _marker: PhantomData,
            },
            USB: USB {
                _marker: PhantomData,
            },
            CRC: CRC {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
            EMC: EMC {
                _marker: PhantomData,
            },
            WWDT: WWDT {
                _marker: PhantomData,
            },
            TIMER0: TIMER0 {
                _marker: PhantomData,
            },
            TIMER1: TIMER1 {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            PWM0: PWM0 {
                _marker: PhantomData,
            },
            PWM1: PWM1 {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            GPIOINT: GPIOINT {
                _marker: PhantomData,
            },
            IOCON: IOCON {
                _marker: PhantomData,
            },
            SSP1: SSP1 {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
            CANAFRAM: CANAFRAM {
                _marker: PhantomData,
            },
            CANAF: CANAF {
                _marker: PhantomData,
            },
            CCAN: CCAN {
                _marker: PhantomData,
            },
            CAN1: CAN1 {
                _marker: PhantomData,
            },
            CAN2: CAN2 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            SSP0: SSP0 {
                _marker: PhantomData,
            },
            DAC: DAC {
                _marker: PhantomData,
            },
            TIMER2: TIMER2 {
                _marker: PhantomData,
            },
            TIMER3: TIMER3 {
                _marker: PhantomData,
            },
            UART2: UART2 {
                _marker: PhantomData,
            },
            UART3: UART3 {
                _marker: PhantomData,
            },
            I2C2: I2C2 {
                _marker: PhantomData,
            },
            UART4: UART4 {
                _marker: PhantomData,
            },
            I2S: I2S {
                _marker: PhantomData,
            },
            SSP2: SSP2 {
                _marker: PhantomData,
            },
            MCPWM: MCPWM {
                _marker: PhantomData,
            },
            QEI: QEI {
                _marker: PhantomData,
            },
            SDMMC: SDMMC {
                _marker: PhantomData,
            },
            SYSCON: SYSCON {
                _marker: PhantomData,
            },
        }
    }
}
