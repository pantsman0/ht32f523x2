#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CR"]
    pub cr: CR,
    #[doc = "0x04 - IER"]
    pub ier: IER,
    #[doc = "0x08 - ADDR"]
    pub addr: ADDR,
    #[doc = "0x0c - SR"]
    pub sr: SR,
    #[doc = "0x10 - SHPGR"]
    pub shpgr: SHPGR,
    #[doc = "0x14 - SLPGR"]
    pub slpgr: SLPGR,
    #[doc = "0x18 - DR"]
    pub dr: DR,
    #[doc = "0x1c - TAR"]
    pub tar: TAR,
    #[doc = "0x20 - ADDMR"]
    pub addmr: ADDMR,
    #[doc = "0x24 - ADDSR"]
    pub addsr: ADDSR,
    #[doc = "0x28 - TOUT"]
    pub tout: TOUT,
}
#[doc = "CR"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CR"]
pub mod cr;
#[doc = "IER"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IER"]
pub mod ier;
#[doc = "ADDR"]
pub struct ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADDR"]
pub mod addr;
#[doc = "SR"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SR"]
pub mod sr;
#[doc = "SHPGR"]
pub struct SHPGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SHPGR"]
pub mod shpgr;
#[doc = "SLPGR"]
pub struct SLPGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SLPGR"]
pub mod slpgr;
#[doc = "DR"]
pub struct DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DR"]
pub mod dr;
#[doc = "TAR"]
pub struct TAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TAR"]
pub mod tar;
#[doc = "ADDMR"]
pub struct ADDMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADDMR"]
pub mod addmr;
#[doc = "ADDSR"]
pub struct ADDSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADDSR"]
pub mod addsr;
#[doc = "TOUT"]
pub struct TOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TOUT"]
pub mod tout;
