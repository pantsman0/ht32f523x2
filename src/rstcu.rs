#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GRSR"]
    pub grsr: GRSR,
    #[doc = "0x04 - AHBPRSTR"]
    pub ahbprstr: AHBPRSTR,
    #[doc = "0x08 - APBPRSTR0"]
    pub apbprstr0: APBPRSTR0,
    #[doc = "0x0c - APBPRSTR1"]
    pub apbprstr1: APBPRSTR1,
}
#[doc = "GRSR"]
pub struct GRSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GRSR"]
pub mod grsr;
#[doc = "AHBPRSTR"]
pub struct AHBPRSTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHBPRSTR"]
pub mod ahbprstr;
#[doc = "APBPRSTR0"]
pub struct APBPRSTR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APBPRSTR0"]
pub mod apbprstr0;
#[doc = "APBPRSTR1"]
pub struct APBPRSTR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APBPRSTR1"]
pub mod apbprstr1;
