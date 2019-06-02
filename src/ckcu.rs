#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GCFGR"]
    pub gcfgr: GCFGR,
    #[doc = "0x04 - GCCR"]
    pub gccr: GCCR,
    #[doc = "0x08 - GCSR"]
    pub gcsr: GCSR,
    #[doc = "0x0c - GCIR"]
    pub gcir: GCIR,
    _reserved0: [u8; 8usize],
    #[doc = "0x18 - PLLCFGR"]
    pub pllcfgr: PLLCFGR,
    #[doc = "0x1c - PLLCR"]
    pub pllcr: PLLCR,
    #[doc = "0x20 - AHBCFGR"]
    pub ahbcfgr: AHBCFGR,
    #[doc = "0x24 - AHBCCR"]
    pub ahbccr: AHBCCR,
    #[doc = "0x28 - APBCFGR"]
    pub apbcfgr: APBCFGR,
    #[doc = "0x2c - APBCCR0"]
    pub apbccr0: APBCCR0,
    #[doc = "0x30 - APBCCR1"]
    pub apbccr1: APBCCR1,
    #[doc = "0x34 - CKST"]
    pub ckst: CKST,
    #[doc = "0x38 - APBPCSR0"]
    pub apbpcsr0: APBPCSR0,
    #[doc = "0x3c - APBPCSR1"]
    pub apbpcsr1: APBPCSR1,
    #[doc = "0x40 - HSICR"]
    pub hsicr: HSICR,
    #[doc = "0x44 - HSIATCR"]
    pub hsiatcr: HSIATCR,
    _reserved1: [u8; 696usize],
    #[doc = "0x300 - LPCR"]
    pub lpcr: LPCR,
    #[doc = "0x304 - MCUDBGCR"]
    pub mcudbgcr: MCUDBGCR,
}
#[doc = "GCFGR"]
pub struct GCFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GCFGR"]
pub mod gcfgr;
#[doc = "GCCR"]
pub struct GCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GCCR"]
pub mod gccr;
#[doc = "GCSR"]
pub struct GCSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GCSR"]
pub mod gcsr;
#[doc = "GCIR"]
pub struct GCIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GCIR"]
pub mod gcir;
#[doc = "PLLCFGR"]
pub struct PLLCFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLLCFGR"]
pub mod pllcfgr;
#[doc = "PLLCR"]
pub struct PLLCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLLCR"]
pub mod pllcr;
#[doc = "AHBCFGR"]
pub struct AHBCFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHBCFGR"]
pub mod ahbcfgr;
#[doc = "AHBCCR"]
pub struct AHBCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHBCCR"]
pub mod ahbccr;
#[doc = "APBCFGR"]
pub struct APBCFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APBCFGR"]
pub mod apbcfgr;
#[doc = "APBCCR0"]
pub struct APBCCR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APBCCR0"]
pub mod apbccr0;
#[doc = "APBCCR1"]
pub struct APBCCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APBCCR1"]
pub mod apbccr1;
#[doc = "CKST"]
pub struct CKST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CKST"]
pub mod ckst;
#[doc = "APBPCSR0"]
pub struct APBPCSR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APBPCSR0"]
pub mod apbpcsr0;
#[doc = "APBPCSR1"]
pub struct APBPCSR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APBPCSR1"]
pub mod apbpcsr1;
#[doc = "HSICR"]
pub struct HSICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HSICR"]
pub mod hsicr;
#[doc = "HSIATCR"]
pub struct HSIATCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HSIATCR"]
pub mod hsiatcr;
#[doc = "LPCR"]
pub struct LPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LPCR"]
pub mod lpcr;
#[doc = "MCUDBGCR"]
pub struct MCUDBGCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCUDBGCR"]
pub mod mcudbgcr;
