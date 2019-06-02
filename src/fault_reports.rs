#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DFSR"]
    pub dfsr: DFSR,
}
#[doc = "DFSR"]
pub struct DFSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSR"]
pub mod dfsr;
