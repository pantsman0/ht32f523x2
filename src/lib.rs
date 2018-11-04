#![doc = "Peripheral access API for HT32F52342_52 microcontrollers (generated using svd2rust v0.13.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.13.1/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
extern "C" {
    fn LVD_BOD();
    fn RTC();
    fn FMC();
    fn EVWUP();
    fn EXTI0_1();
    fn EXTI2_3();
    fn EXTI4_15();
    fn CMP0_1();
    fn ADC();
    fn MCTM0();
    fn GPTM1();
    fn GPTM0();
    fn SCTM0();
    fn SCTM1();
    fn BFTM0();
    fn BFTM1();
    fn I2C0();
    fn I2C1();
    fn SPI0();
    fn SPI1();
    fn USART0();
    fn USART1();
    fn UART0();
    fn UART1();
    fn SCI();
    fn I2S();
    fn USB();
    fn PDMACH0_1();
    fn PDMACH2_5();
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
pub static __INTERRUPTS: [Vector; 32] = [
    Vector { _handler: LVD_BOD },
    Vector { _handler: RTC },
    Vector { _handler: FMC },
    Vector { _handler: EVWUP },
    Vector { _handler: EXTI0_1 },
    Vector { _handler: EXTI2_3 },
    Vector { _handler: EXTI4_15 },
    Vector { _handler: CMP0_1 },
    Vector { _handler: ADC },
    Vector { _reserved: 0 },
    Vector { _handler: MCTM0 },
    Vector { _handler: GPTM1 },
    Vector { _handler: GPTM0 },
    Vector { _handler: SCTM0 },
    Vector { _handler: SCTM1 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: BFTM0 },
    Vector { _handler: BFTM1 },
    Vector { _handler: I2C0 },
    Vector { _handler: I2C1 },
    Vector { _handler: SPI0 },
    Vector { _handler: SPI1 },
    Vector { _handler: USART0 },
    Vector { _handler: USART1 },
    Vector { _handler: UART0 },
    Vector { _handler: UART1 },
    Vector { _handler: SCI },
    Vector { _handler: I2S },
    Vector { _handler: USB },
    Vector {
        _handler: PDMACH0_1,
    },
    Vector {
        _handler: PDMACH2_5,
    },
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
    #[doc = "0 - LVD_BOD"]
    LVD_BOD,
    #[doc = "1 - RTC"]
    RTC,
    #[doc = "2 - FMC"]
    FMC,
    #[doc = "3 - EVWUP"]
    EVWUP,
    #[doc = "4 - EXTI0_1"]
    EXTI0_1,
    #[doc = "5 - EXTI2_3"]
    EXTI2_3,
    #[doc = "6 - EXTI4_15"]
    EXTI4_15,
    #[doc = "7 - CMP0_1"]
    CMP0_1,
    #[doc = "8 - ADC"]
    ADC,
    #[doc = "10 - MCTM0"]
    MCTM0,
    #[doc = "11 - GPTM1"]
    GPTM1,
    #[doc = "12 - GPTM0"]
    GPTM0,
    #[doc = "13 - SCTM0"]
    SCTM0,
    #[doc = "14 - SCTM1"]
    SCTM1,
    #[doc = "17 - BFTM0"]
    BFTM0,
    #[doc = "18 - BFTM1"]
    BFTM1,
    #[doc = "19 - I2C0"]
    I2C0,
    #[doc = "20 - I2C1"]
    I2C1,
    #[doc = "21 - SPI0"]
    SPI0,
    #[doc = "22 - SPI1"]
    SPI1,
    #[doc = "23 - USART0"]
    USART0,
    #[doc = "24 - USART1"]
    USART1,
    #[doc = "25 - UART0"]
    UART0,
    #[doc = "26 - UART1"]
    UART1,
    #[doc = "27 - SCI"]
    SCI,
    #[doc = "28 - I2S"]
    I2S,
    #[doc = "29 - USB"]
    USB,
    #[doc = "30 - PDMACH0_1"]
    PDMACH0_1,
    #[doc = "31 - PDMACH2_5"]
    PDMACH2_5,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::LVD_BOD => 0,
            Interrupt::RTC => 1,
            Interrupt::FMC => 2,
            Interrupt::EVWUP => 3,
            Interrupt::EXTI0_1 => 4,
            Interrupt::EXTI2_3 => 5,
            Interrupt::EXTI4_15 => 6,
            Interrupt::CMP0_1 => 7,
            Interrupt::ADC => 8,
            Interrupt::MCTM0 => 10,
            Interrupt::GPTM1 => 11,
            Interrupt::GPTM0 => 12,
            Interrupt::SCTM0 => 13,
            Interrupt::SCTM1 => 14,
            Interrupt::BFTM0 => 17,
            Interrupt::BFTM1 => 18,
            Interrupt::I2C0 => 19,
            Interrupt::I2C1 => 20,
            Interrupt::SPI0 => 21,
            Interrupt::SPI1 => 22,
            Interrupt::USART0 => 23,
            Interrupt::USART1 => 24,
            Interrupt::UART0 => 25,
            Interrupt::UART1 => 26,
            Interrupt::SCI => 27,
            Interrupt::I2S => 28,
            Interrupt::USB => 29,
            Interrupt::PDMACH0_1 => 30,
            Interrupt::PDMACH2_5 => 31,
        }
    }
}
#[doc(hidden)]
pub mod interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[doc = "SysTick"]
pub struct SYSTICK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSTICK {}
impl SYSTICK {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sys_tick::RegisterBlock {
        3758153744 as *const _
    }
}
impl Deref for SYSTICK {
    type Target = sys_tick::RegisterBlock;
    fn deref(&self) -> &sys_tick::RegisterBlock {
        unsafe { &*SYSTICK::ptr() }
    }
}
#[doc = "SysTick"]
pub mod sys_tick;
#[doc = "Fault_Reports"]
pub struct FAULT_REPORTS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FAULT_REPORTS {}
impl FAULT_REPORTS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fault_reports::RegisterBlock {
        3758157104 as *const _
    }
}
impl Deref for FAULT_REPORTS {
    type Target = fault_reports::RegisterBlock;
    fn deref(&self) -> &fault_reports::RegisterBlock {
        unsafe { &*FAULT_REPORTS::ptr() }
    }
}
#[doc = "Fault_Reports"]
pub mod fault_reports;
#[doc = "FMC"]
pub struct FMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FMC {}
impl FMC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fmc::RegisterBlock {
        1074266112 as *const _
    }
}
impl Deref for FMC {
    type Target = fmc::RegisterBlock;
    fn deref(&self) -> &fmc::RegisterBlock {
        unsafe { &*FMC::ptr() }
    }
}
#[doc = "FMC"]
pub mod fmc;
#[doc = "PWRCU"]
pub struct PWRCU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWRCU {}
impl PWRCU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwrcu::RegisterBlock {
        1074176256 as *const _
    }
}
impl Deref for PWRCU {
    type Target = pwrcu::RegisterBlock;
    fn deref(&self) -> &pwrcu::RegisterBlock {
        unsafe { &*PWRCU::ptr() }
    }
}
#[doc = "PWRCU"]
pub mod pwrcu;
#[doc = "CKCU"]
pub struct CKCU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CKCU {}
impl CKCU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ckcu::RegisterBlock {
        1074298880 as *const _
    }
}
impl Deref for CKCU {
    type Target = ckcu::RegisterBlock;
    fn deref(&self) -> &ckcu::RegisterBlock {
        unsafe { &*CKCU::ptr() }
    }
}
#[doc = "CKCU"]
pub mod ckcu;
#[doc = "RSTCU"]
pub struct RSTCU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RSTCU {}
impl RSTCU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rstcu::RegisterBlock {
        1074299136 as *const _
    }
}
impl Deref for RSTCU {
    type Target = rstcu::RegisterBlock;
    fn deref(&self) -> &rstcu::RegisterBlock {
        unsafe { &*RSTCU::ptr() }
    }
}
#[doc = "RSTCU"]
pub mod rstcu;
#[doc = "GPIOA"]
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioa::RegisterBlock {
        1074462720 as *const _
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &gpioa::RegisterBlock {
        unsafe { &*GPIOA::ptr() }
    }
}
#[doc = "GPIOA"]
pub mod gpioa;
#[doc = "GPIOB"]
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOB {}
impl GPIOB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiob::RegisterBlock {
        1074470912 as *const _
    }
}
impl Deref for GPIOB {
    type Target = gpiob::RegisterBlock;
    fn deref(&self) -> &gpiob::RegisterBlock {
        unsafe { &*GPIOB::ptr() }
    }
}
#[doc = "GPIOB"]
pub mod gpiob;
#[doc = "GPIOC"]
pub struct GPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOC {}
impl GPIOC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioc::RegisterBlock {
        1074479104 as *const _
    }
}
impl Deref for GPIOC {
    type Target = gpioc::RegisterBlock;
    fn deref(&self) -> &gpioc::RegisterBlock {
        unsafe { &*GPIOC::ptr() }
    }
}
#[doc = "GPIOC"]
pub mod gpioc;
#[doc = "GPIOD"]
pub struct GPIOD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOD {}
impl GPIOD {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiod::RegisterBlock {
        1074487296 as *const _
    }
}
impl Deref for GPIOD {
    type Target = gpiod::RegisterBlock;
    fn deref(&self) -> &gpiod::RegisterBlock {
        unsafe { &*GPIOD::ptr() }
    }
}
#[doc = "GPIOD"]
pub mod gpiod;
#[doc = "AFIO"]
pub struct AFIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AFIO {}
impl AFIO {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const afio::RegisterBlock {
        1073881088 as *const _
    }
}
impl Deref for AFIO {
    type Target = afio::RegisterBlock;
    fn deref(&self) -> &afio::RegisterBlock {
        unsafe { &*AFIO::ptr() }
    }
}
#[doc = "AFIO"]
pub mod afio;
#[doc = "EXTI"]
pub struct EXTI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EXTI {}
impl EXTI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const exti::RegisterBlock {
        1073889280 as *const _
    }
}
impl Deref for EXTI {
    type Target = exti::RegisterBlock;
    fn deref(&self) -> &exti::RegisterBlock {
        unsafe { &*EXTI::ptr() }
    }
}
#[doc = "EXTI"]
pub mod exti;
#[doc = "ADC"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc::RegisterBlock {
        1073807360 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    fn deref(&self) -> &adc::RegisterBlock {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "ADC"]
pub mod adc;
#[doc = "CMP"]
pub struct CMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMP {}
impl CMP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const cmp::RegisterBlock {
        1074102272 as *const _
    }
}
impl Deref for CMP {
    type Target = cmp::RegisterBlock;
    fn deref(&self) -> &cmp::RegisterBlock {
        unsafe { &*CMP::ptr() }
    }
}
#[doc = "CMP"]
pub mod cmp;
#[doc = "MCTM0"]
pub struct MCTM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MCTM0 {}
impl MCTM0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mctm0::RegisterBlock {
        1073922048 as *const _
    }
}
impl Deref for MCTM0 {
    type Target = mctm0::RegisterBlock;
    fn deref(&self) -> &mctm0::RegisterBlock {
        unsafe { &*MCTM0::ptr() }
    }
}
#[doc = "MCTM0"]
pub mod mctm0;
#[doc = "GPTM0"]
pub struct GPTM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPTM0 {}
impl GPTM0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gptm0::RegisterBlock {
        1074192384 as *const _
    }
}
impl Deref for GPTM0 {
    type Target = gptm0::RegisterBlock;
    fn deref(&self) -> &gptm0::RegisterBlock {
        unsafe { &*GPTM0::ptr() }
    }
}
#[doc = "GPTM0"]
pub mod gptm0;
#[doc = "GPTM1"]
pub struct GPTM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPTM1 {}
impl GPTM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gptm0::RegisterBlock {
        1074196480 as *const _
    }
}
impl Deref for GPTM1 {
    type Target = gptm0::RegisterBlock;
    fn deref(&self) -> &gptm0::RegisterBlock {
        unsafe { &*GPTM1::ptr() }
    }
}
#[doc = "SCTM0"]
pub struct SCTM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCTM0 {}
impl SCTM0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sctm0::RegisterBlock {
        1073954816 as *const _
    }
}
impl Deref for SCTM0 {
    type Target = sctm0::RegisterBlock;
    fn deref(&self) -> &sctm0::RegisterBlock {
        unsafe { &*SCTM0::ptr() }
    }
}
#[doc = "SCTM0"]
pub mod sctm0;
#[doc = "SCTM1"]
pub struct SCTM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCTM1 {}
impl SCTM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sctm0::RegisterBlock {
        1074216960 as *const _
    }
}
impl Deref for SCTM1 {
    type Target = sctm0::RegisterBlock;
    fn deref(&self) -> &sctm0::RegisterBlock {
        unsafe { &*SCTM1::ptr() }
    }
}
#[doc = "BFTM0"]
pub struct BFTM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BFTM0 {}
impl BFTM0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const bftm0::RegisterBlock {
        1074225152 as *const _
    }
}
impl Deref for BFTM0 {
    type Target = bftm0::RegisterBlock;
    fn deref(&self) -> &bftm0::RegisterBlock {
        unsafe { &*BFTM0::ptr() }
    }
}
#[doc = "BFTM0"]
pub mod bftm0;
#[doc = "BFTM1"]
pub struct BFTM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BFTM1 {}
impl BFTM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const bftm0::RegisterBlock {
        1074229248 as *const _
    }
}
impl Deref for BFTM1 {
    type Target = bftm0::RegisterBlock;
    fn deref(&self) -> &bftm0::RegisterBlock {
        unsafe { &*BFTM1::ptr() }
    }
}
#[doc = "RTC"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc::RegisterBlock {
        1074176000 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &rtc::RegisterBlock {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "RTC"]
pub mod rtc;
#[doc = "WDT"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wdt::RegisterBlock {
        1074167808 as *const _
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    fn deref(&self) -> &wdt::RegisterBlock {
        unsafe { &*WDT::ptr() }
    }
}
#[doc = "WDT"]
pub mod wdt;
#[doc = "I2C0"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c0::RegisterBlock {
        1074036736 as *const _
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &i2c0::RegisterBlock {
        unsafe { &*I2C0::ptr() }
    }
}
#[doc = "I2C0"]
pub mod i2c0;
#[doc = "I2C1"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c0::RegisterBlock {
        1074040832 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &i2c0::RegisterBlock {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "SPI0"]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi0::RegisterBlock {
        1073758208 as *const _
    }
}
impl Deref for SPI0 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &spi0::RegisterBlock {
        unsafe { &*SPI0::ptr() }
    }
}
#[doc = "SPI0"]
pub mod spi0;
#[doc = "SPI1"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi0::RegisterBlock {
        1074020352 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &spi0::RegisterBlock {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "USART0"]
pub struct USART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART0 {}
impl USART0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart0::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for USART0 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &usart0::RegisterBlock {
        unsafe { &*USART0::ptr() }
    }
}
#[doc = "USART0"]
pub mod usart0;
#[doc = "USART1"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart0::RegisterBlock {
        1074003968 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &usart0::RegisterBlock {
        unsafe { &*USART1::ptr() }
    }
}
#[doc = "UART0"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart0::RegisterBlock {
        1073745920 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &uart0::RegisterBlock {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "UART0"]
pub mod uart0;
#[doc = "UART1"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart0::RegisterBlock {
        1074008064 as *const _
    }
}
impl Deref for UART1 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &uart0::RegisterBlock {
        unsafe { &*UART1::ptr() }
    }
}
#[doc = "SCI"]
pub struct SCI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCI {}
impl SCI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sci::RegisterBlock {
        1074016256 as *const _
    }
}
impl Deref for SCI {
    type Target = sci::RegisterBlock;
    fn deref(&self) -> &sci::RegisterBlock {
        unsafe { &*SCI::ptr() }
    }
}
#[doc = "SCI"]
pub mod sci;
#[doc = "USB"]
pub struct USB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB {}
impl USB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb::RegisterBlock {
        1074429952 as *const _
    }
}
impl Deref for USB {
    type Target = usb::RegisterBlock;
    fn deref(&self) -> &usb::RegisterBlock {
        unsafe { &*USB::ptr() }
    }
}
#[doc = "USB"]
pub mod usb;
#[doc = "PDMA"]
pub struct PDMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PDMA {}
impl PDMA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pdma::RegisterBlock {
        1074331648 as *const _
    }
}
impl Deref for PDMA {
    type Target = pdma::RegisterBlock;
    fn deref(&self) -> &pdma::RegisterBlock {
        unsafe { &*PDMA::ptr() }
    }
}
#[doc = "PDMA"]
pub mod pdma;
#[doc = "EBI"]
pub struct EBI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EBI {}
impl EBI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ebi::RegisterBlock {
        1074364416 as *const _
    }
}
impl Deref for EBI {
    type Target = ebi::RegisterBlock;
    fn deref(&self) -> &ebi::RegisterBlock {
        unsafe { &*EBI::ptr() }
    }
}
#[doc = "EBI"]
pub mod ebi;
#[doc = "I2S"]
pub struct I2S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S {}
impl I2S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2s::RegisterBlock {
        1073897472 as *const _
    }
}
impl Deref for I2S {
    type Target = i2s::RegisterBlock;
    fn deref(&self) -> &i2s::RegisterBlock {
        unsafe { &*I2S::ptr() }
    }
}
#[doc = "I2S"]
pub mod i2s;
#[doc = "CRC"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const crc::RegisterBlock {
        1074307072 as *const _
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    fn deref(&self) -> &crc::RegisterBlock {
        unsafe { &*CRC::ptr() }
    }
}
#[doc = "CRC"]
pub mod crc;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "SYSTICK"]
    pub SYSTICK: SYSTICK,
    #[doc = "FAULT_REPORTS"]
    pub FAULT_REPORTS: FAULT_REPORTS,
    #[doc = "FMC"]
    pub FMC: FMC,
    #[doc = "PWRCU"]
    pub PWRCU: PWRCU,
    #[doc = "CKCU"]
    pub CKCU: CKCU,
    #[doc = "RSTCU"]
    pub RSTCU: RSTCU,
    #[doc = "GPIOA"]
    pub GPIOA: GPIOA,
    #[doc = "GPIOB"]
    pub GPIOB: GPIOB,
    #[doc = "GPIOC"]
    pub GPIOC: GPIOC,
    #[doc = "GPIOD"]
    pub GPIOD: GPIOD,
    #[doc = "AFIO"]
    pub AFIO: AFIO,
    #[doc = "EXTI"]
    pub EXTI: EXTI,
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "CMP"]
    pub CMP: CMP,
    #[doc = "MCTM0"]
    pub MCTM0: MCTM0,
    #[doc = "GPTM0"]
    pub GPTM0: GPTM0,
    #[doc = "GPTM1"]
    pub GPTM1: GPTM1,
    #[doc = "SCTM0"]
    pub SCTM0: SCTM0,
    #[doc = "SCTM1"]
    pub SCTM1: SCTM1,
    #[doc = "BFTM0"]
    pub BFTM0: BFTM0,
    #[doc = "BFTM1"]
    pub BFTM1: BFTM1,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "WDT"]
    pub WDT: WDT,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "USART0"]
    pub USART0: USART0,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "SCI"]
    pub SCI: SCI,
    #[doc = "USB"]
    pub USB: USB,
    #[doc = "PDMA"]
    pub PDMA: PDMA,
    #[doc = "EBI"]
    pub EBI: EBI,
    #[doc = "I2S"]
    pub I2S: I2S,
    #[doc = "CRC"]
    pub CRC: CRC,
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
            SYSTICK: SYSTICK {
                _marker: PhantomData,
            },
            FAULT_REPORTS: FAULT_REPORTS {
                _marker: PhantomData,
            },
            FMC: FMC {
                _marker: PhantomData,
            },
            PWRCU: PWRCU {
                _marker: PhantomData,
            },
            CKCU: CKCU {
                _marker: PhantomData,
            },
            RSTCU: RSTCU {
                _marker: PhantomData,
            },
            GPIOA: GPIOA {
                _marker: PhantomData,
            },
            GPIOB: GPIOB {
                _marker: PhantomData,
            },
            GPIOC: GPIOC {
                _marker: PhantomData,
            },
            GPIOD: GPIOD {
                _marker: PhantomData,
            },
            AFIO: AFIO {
                _marker: PhantomData,
            },
            EXTI: EXTI {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
            CMP: CMP {
                _marker: PhantomData,
            },
            MCTM0: MCTM0 {
                _marker: PhantomData,
            },
            GPTM0: GPTM0 {
                _marker: PhantomData,
            },
            GPTM1: GPTM1 {
                _marker: PhantomData,
            },
            SCTM0: SCTM0 {
                _marker: PhantomData,
            },
            SCTM1: SCTM1 {
                _marker: PhantomData,
            },
            BFTM0: BFTM0 {
                _marker: PhantomData,
            },
            BFTM1: BFTM1 {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            WDT: WDT {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            USART0: USART0 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            SCI: SCI {
                _marker: PhantomData,
            },
            USB: USB {
                _marker: PhantomData,
            },
            PDMA: PDMA {
                _marker: PhantomData,
            },
            EBI: EBI {
                _marker: PhantomData,
            },
            I2S: I2S {
                _marker: PhantomData,
            },
            CRC: CRC {
                _marker: PhantomData,
            },
        }
    }
}
