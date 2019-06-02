#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USART_USRDR"]
    pub usart_usrdr: USART_USRDR,
    #[doc = "0x04 - USART_USRCR"]
    pub usart_usrcr: USART_USRCR,
    #[doc = "0x08 - USART_USRFCR"]
    pub usart_usrfcr: USART_USRFCR,
    #[doc = "0x0c - USART_USRIER"]
    pub usart_usrier: USART_USRIER,
    #[doc = "0x10 - USART_USRSIFR"]
    pub usart_usrsifr: USART_USRSIFR,
    #[doc = "0x14 - USART_USRTPR"]
    pub usart_usrtpr: USART_USRTPR,
    #[doc = "0x18 - USART_IrDACR"]
    pub usart_ir_dacr: USART_IRDACR,
    #[doc = "0x1c - USART_RS485CR"]
    pub usart_rs485cr: USART_RS485CR,
    #[doc = "0x20 - USART_SYNCR"]
    pub usart_syncr: USART_SYNCR,
    #[doc = "0x24 - USART_USRDLR"]
    pub usart_usrdlr: USART_USRDLR,
    #[doc = "0x28 - USART_USRTSTR"]
    pub usart_usrtstr: USART_USRTSTR,
}
#[doc = "USART_USRDR"]
pub struct USART_USRDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USART_USRDR"]
pub mod usart_usrdr;
#[doc = "USART_USRCR"]
pub struct USART_USRCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USART_USRCR"]
pub mod usart_usrcr;
#[doc = "USART_USRFCR"]
pub struct USART_USRFCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USART_USRFCR"]
pub mod usart_usrfcr;
#[doc = "USART_USRIER"]
pub struct USART_USRIER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USART_USRIER"]
pub mod usart_usrier;
#[doc = "USART_USRSIFR"]
pub struct USART_USRSIFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USART_USRSIFR"]
pub mod usart_usrsifr;
#[doc = "USART_USRTPR"]
pub struct USART_USRTPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USART_USRTPR"]
pub mod usart_usrtpr;
#[doc = "USART_IrDACR"]
pub struct USART_IRDACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USART_IrDACR"]
pub mod usart_ir_dacr;
#[doc = "USART_RS485CR"]
pub struct USART_RS485CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USART_RS485CR"]
pub mod usart_rs485cr;
#[doc = "USART_SYNCR"]
pub struct USART_SYNCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USART_SYNCR"]
pub mod usart_syncr;
#[doc = "USART_USRDLR"]
pub struct USART_USRDLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USART_USRDLR"]
pub mod usart_usrdlr;
#[doc = "USART_USRTSTR"]
pub struct USART_USRTSTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USART_USRTSTR"]
pub mod usart_usrtstr;
