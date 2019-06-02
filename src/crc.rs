#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CR"]
    pub cr: CR,
    #[doc = "0x04 - SDR"]
    pub sdr: SDR,
    #[doc = "0x08 - CSR"]
    pub csr: CSR,
    #[doc = "0x0c - DR"]
    pub dr: DR,
}
#[doc = "CR"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CR"]
pub mod cr;
#[doc = "SDR"]
pub struct SDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDR"]
pub mod sdr;
#[doc = "CSR"]
pub struct CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CSR"]
pub mod csr;
#[doc = "DR"]
pub struct DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DR"]
pub mod dr;
