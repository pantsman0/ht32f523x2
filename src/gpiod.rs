#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DIRCR"]
    pub dircr: DIRCR,
    #[doc = "0x04 - INER"]
    pub iner: INER,
    #[doc = "0x08 - PUR"]
    pub pur: PUR,
    #[doc = "0x0c - PDR"]
    pub pdr: PDR,
    #[doc = "0x10 - ODR"]
    pub odr: ODR,
    #[doc = "0x14 - DRVR"]
    pub drvr: DRVR,
    #[doc = "0x18 - LOCKR"]
    pub lockr: LOCKR,
    #[doc = "0x1c - DINR"]
    pub dinr: DINR,
    #[doc = "0x20 - DOUTR"]
    pub doutr: DOUTR,
    #[doc = "0x24 - SRR"]
    pub srr: SRR,
    #[doc = "0x28 - RR"]
    pub rr: RR,
}
#[doc = "DIRCR"]
pub struct DIRCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DIRCR"]
pub mod dircr;
#[doc = "INER"]
pub struct INER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "INER"]
pub mod iner;
#[doc = "PUR"]
pub struct PUR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PUR"]
pub mod pur;
#[doc = "PDR"]
pub struct PDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDR"]
pub mod pdr;
#[doc = "ODR"]
pub struct ODR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ODR"]
pub mod odr;
#[doc = "DRVR"]
pub struct DRVR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DRVR"]
pub mod drvr;
#[doc = "LOCKR"]
pub struct LOCKR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LOCKR"]
pub mod lockr;
#[doc = "DINR"]
pub struct DINR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DINR"]
pub mod dinr;
#[doc = "DOUTR"]
pub struct DOUTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DOUTR"]
pub mod doutr;
#[doc = "SRR"]
pub struct SRR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRR"]
pub mod srr;
#[doc = "RR"]
pub struct RR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RR"]
pub mod rr;
