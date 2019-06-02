#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UART_URDR"]
    pub uart_urdr: UART_URDR,
    #[doc = "0x04 - UART_URCR"]
    pub uart_urcr: UART_URCR,
    _reserved0: [u8; 4usize],
    #[doc = "0x0c - UART_URIER"]
    pub uart_urier: UART_URIER,
    #[doc = "0x10 - UART_URSIFR"]
    pub uart_ursifr: UART_URSIFR,
    _reserved1: [u8; 16usize],
    #[doc = "0x24 - UART_URDLR"]
    pub uart_urdlr: UART_URDLR,
    #[doc = "0x28 - UART_URTSTR"]
    pub uart_urtstr: UART_URTSTR,
}
#[doc = "UART_URDR"]
pub struct UART_URDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART_URDR"]
pub mod uart_urdr;
#[doc = "UART_URCR"]
pub struct UART_URCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART_URCR"]
pub mod uart_urcr;
#[doc = "UART_URIER"]
pub struct UART_URIER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART_URIER"]
pub mod uart_urier;
#[doc = "UART_URSIFR"]
pub struct UART_URSIFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART_URSIFR"]
pub mod uart_ursifr;
#[doc = "UART_URDLR"]
pub struct UART_URDLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART_URDLR"]
pub mod uart_urdlr;
#[doc = "UART_URTSTR"]
pub struct UART_URTSTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART_URTSTR"]
pub mod uart_urtstr;
