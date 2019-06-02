#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CNTCFR"]
    pub cntcfr: CNTCFR,
    #[doc = "0x04 - MDCFR"]
    pub mdcfr: MDCFR,
    #[doc = "0x08 - TRCFR"]
    pub trcfr: TRCFR,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - CTR"]
    pub ctr: CTR,
    _reserved1: [u8; 12usize],
    #[doc = "0x20 - CH0ICFR"]
    pub ch0icfr: CH0ICFR,
    #[doc = "0x24 - CH1ICFR"]
    pub ch1icfr: CH1ICFR,
    #[doc = "0x28 - CH2ICFR"]
    pub ch2icfr: CH2ICFR,
    #[doc = "0x2c - CH3ICFR"]
    pub ch3icfr: CH3ICFR,
    _reserved2: [u8; 16usize],
    #[doc = "0x40 - CH0OCFR"]
    pub ch0ocfr: CH0OCFR,
    #[doc = "0x44 - CH1OCFR"]
    pub ch1ocfr: CH1OCFR,
    #[doc = "0x48 - CH2OCFR"]
    pub ch2ocfr: CH2OCFR,
    #[doc = "0x4c - CH3OCFR"]
    pub ch3ocfr: CH3OCFR,
    #[doc = "0x50 - CHCTR"]
    pub chctr: CHCTR,
    #[doc = "0x54 - CHPOLR"]
    pub chpolr: CHPOLR,
    _reserved3: [u8; 20usize],
    #[doc = "0x6c - CHBRKCFR"]
    pub chbrkcfr: CHBRKCFR,
    #[doc = "0x70 - CHBRKCTR"]
    pub chbrkctr: CHBRKCTR,
    #[doc = "0x74 - DICTR"]
    pub dictr: DICTR,
    #[doc = "0x78 - EVGR"]
    pub evgr: EVGR,
    #[doc = "0x7c - INTSR"]
    pub intsr: INTSR,
    #[doc = "0x80 - CNTR"]
    pub cntr: CNTR,
    #[doc = "0x84 - PSCR"]
    pub pscr: PSCR,
    #[doc = "0x88 - CRR"]
    pub crr: CRR,
    #[doc = "0x8c - REPR"]
    pub repr: REPR,
    #[doc = "0x90 - CH0CCR"]
    pub ch0ccr: CH0CCR,
    #[doc = "0x94 - CH1CCR"]
    pub ch1ccr: CH1CCR,
    #[doc = "0x98 - CH2CCR"]
    pub ch2ccr: CH2CCR,
    #[doc = "0x9c - CH3CCR"]
    pub ch3ccr: CH3CCR,
    #[doc = "0xa0 - CH0ACR"]
    pub ch0acr: CH0ACR,
    #[doc = "0xa4 - CH1ACR"]
    pub ch1acr: CH1ACR,
    #[doc = "0xa8 - CH2ACR"]
    pub ch2acr: CH2ACR,
    #[doc = "0xac - CH3ACR"]
    pub ch3acr: CH3ACR,
}
#[doc = "CNTCFR"]
pub struct CNTCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNTCFR"]
pub mod cntcfr;
#[doc = "MDCFR"]
pub struct MDCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDCFR"]
pub mod mdcfr;
#[doc = "TRCFR"]
pub struct TRCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRCFR"]
pub mod trcfr;
#[doc = "CTR"]
pub struct CTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CTR"]
pub mod ctr;
#[doc = "CH0ICFR"]
pub struct CH0ICFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH0ICFR"]
pub mod ch0icfr;
#[doc = "CH1ICFR"]
pub struct CH1ICFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH1ICFR"]
pub mod ch1icfr;
#[doc = "CH2ICFR"]
pub struct CH2ICFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH2ICFR"]
pub mod ch2icfr;
#[doc = "CH3ICFR"]
pub struct CH3ICFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH3ICFR"]
pub mod ch3icfr;
#[doc = "CH0OCFR"]
pub struct CH0OCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH0OCFR"]
pub mod ch0ocfr;
#[doc = "CH1OCFR"]
pub struct CH1OCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH1OCFR"]
pub mod ch1ocfr;
#[doc = "CH2OCFR"]
pub struct CH2OCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH2OCFR"]
pub mod ch2ocfr;
#[doc = "CH3OCFR"]
pub struct CH3OCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH3OCFR"]
pub mod ch3ocfr;
#[doc = "CHCTR"]
pub struct CHCTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHCTR"]
pub mod chctr;
#[doc = "CHPOLR"]
pub struct CHPOLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHPOLR"]
pub mod chpolr;
#[doc = "CHBRKCFR"]
pub struct CHBRKCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHBRKCFR"]
pub mod chbrkcfr;
#[doc = "CHBRKCTR"]
pub struct CHBRKCTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHBRKCTR"]
pub mod chbrkctr;
#[doc = "DICTR"]
pub struct DICTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DICTR"]
pub mod dictr;
#[doc = "EVGR"]
pub struct EVGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EVGR"]
pub mod evgr;
#[doc = "INTSR"]
pub struct INTSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "INTSR"]
pub mod intsr;
#[doc = "CNTR"]
pub struct CNTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNTR"]
pub mod cntr;
#[doc = "PSCR"]
pub struct PSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PSCR"]
pub mod pscr;
#[doc = "CRR"]
pub struct CRR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRR"]
pub mod crr;
#[doc = "REPR"]
pub struct REPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "REPR"]
pub mod repr;
#[doc = "CH0CCR"]
pub struct CH0CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH0CCR"]
pub mod ch0ccr;
#[doc = "CH1CCR"]
pub struct CH1CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH1CCR"]
pub mod ch1ccr;
#[doc = "CH2CCR"]
pub struct CH2CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH2CCR"]
pub mod ch2ccr;
#[doc = "CH3CCR"]
pub struct CH3CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH3CCR"]
pub mod ch3ccr;
#[doc = "CH0ACR"]
pub struct CH0ACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH0ACR"]
pub mod ch0acr;
#[doc = "CH1ACR"]
pub struct CH1ACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH1ACR"]
pub mod ch1acr;
#[doc = "CH2ACR"]
pub struct CH2ACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH2ACR"]
pub mod ch2acr;
#[doc = "CH3ACR"]
pub struct CH3ACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH3ACR"]
pub mod ch3acr;
