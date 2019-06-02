#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CH0CR"]
    pub ch0cr: CH0CR,
    #[doc = "0x04 - CH0SADR"]
    pub ch0sadr: CH0SADR,
    #[doc = "0x08 - CH0DADR"]
    pub ch0dadr: CH0DADR,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - CH0TSR"]
    pub ch0tsr: CH0TSR,
    #[doc = "0x14 - CH0CTSR"]
    pub ch0ctsr: CH0CTSR,
    #[doc = "0x18 - CH1CR"]
    pub ch1cr: CH1CR,
    #[doc = "0x1c - CH1SADR"]
    pub ch1sadr: CH1SADR,
    #[doc = "0x20 - CH1DADR"]
    pub ch1dadr: CH1DADR,
    _reserved1: [u8; 4usize],
    #[doc = "0x28 - CH1TSR"]
    pub ch1tsr: CH1TSR,
    #[doc = "0x2c - CH1CTSR"]
    pub ch1ctsr: CH1CTSR,
    #[doc = "0x30 - CH2CR"]
    pub ch2cr: CH2CR,
    #[doc = "0x34 - CH2SADR"]
    pub ch2sadr: CH2SADR,
    #[doc = "0x38 - CH2DADR"]
    pub ch2dadr: CH2DADR,
    _reserved2: [u8; 4usize],
    #[doc = "0x40 - CH2TSR"]
    pub ch2tsr: CH2TSR,
    #[doc = "0x44 - CH2CTSR"]
    pub ch2ctsr: CH2CTSR,
    #[doc = "0x48 - CH3CR"]
    pub ch3cr: CH3CR,
    #[doc = "0x4c - CH3SADR"]
    pub ch3sadr: CH3SADR,
    #[doc = "0x50 - CH3DADR"]
    pub ch3dadr: CH3DADR,
    _reserved3: [u8; 4usize],
    #[doc = "0x58 - CH3TSR"]
    pub ch3tsr: CH3TSR,
    #[doc = "0x5c - CH3CTSR"]
    pub ch3ctsr: CH3CTSR,
    #[doc = "0x60 - CH4CR"]
    pub ch4cr: CH4CR,
    #[doc = "0x64 - CH4SADR"]
    pub ch4sadr: CH4SADR,
    #[doc = "0x68 - CH4DADR"]
    pub ch4dadr: CH4DADR,
    _reserved4: [u8; 4usize],
    #[doc = "0x70 - CH4TSR"]
    pub ch4tsr: CH4TSR,
    #[doc = "0x74 - CH4CTSR"]
    pub ch4ctsr: CH4CTSR,
    #[doc = "0x78 - CH5CR"]
    pub ch5cr: CH5CR,
    #[doc = "0x7c - CH5SADR"]
    pub ch5sadr: CH5SADR,
    #[doc = "0x80 - CH5DADR"]
    pub ch5dadr: CH5DADR,
    _reserved5: [u8; 4usize],
    #[doc = "0x88 - CH5TSR"]
    pub ch5tsr: CH5TSR,
    #[doc = "0x8c - CH5CTSR"]
    pub ch5ctsr: CH5CTSR,
    _reserved6: [u8; 144usize],
    #[doc = "0x120 - ISR"]
    pub isr: ISR,
    _reserved7: [u8; 4usize],
    #[doc = "0x128 - ISCR"]
    pub iscr: ISCR,
    _reserved8: [u8; 4usize],
    #[doc = "0x130 - IER"]
    pub ier: IER,
}
#[doc = "CH0CR"]
pub struct CH0CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH0CR"]
pub mod ch0cr;
#[doc = "CH0SADR"]
pub struct CH0SADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH0SADR"]
pub mod ch0sadr;
#[doc = "CH0DADR"]
pub struct CH0DADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH0DADR"]
pub mod ch0dadr;
#[doc = "CH0TSR"]
pub struct CH0TSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH0TSR"]
pub mod ch0tsr;
#[doc = "CH0CTSR"]
pub struct CH0CTSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH0CTSR"]
pub mod ch0ctsr;
#[doc = "CH1CR"]
pub struct CH1CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH1CR"]
pub mod ch1cr;
#[doc = "CH1SADR"]
pub struct CH1SADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH1SADR"]
pub mod ch1sadr;
#[doc = "CH1DADR"]
pub struct CH1DADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH1DADR"]
pub mod ch1dadr;
#[doc = "CH1TSR"]
pub struct CH1TSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH1TSR"]
pub mod ch1tsr;
#[doc = "CH1CTSR"]
pub struct CH1CTSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH1CTSR"]
pub mod ch1ctsr;
#[doc = "CH2CR"]
pub struct CH2CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH2CR"]
pub mod ch2cr;
#[doc = "CH2SADR"]
pub struct CH2SADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH2SADR"]
pub mod ch2sadr;
#[doc = "CH2DADR"]
pub struct CH2DADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH2DADR"]
pub mod ch2dadr;
#[doc = "CH2TSR"]
pub struct CH2TSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH2TSR"]
pub mod ch2tsr;
#[doc = "CH2CTSR"]
pub struct CH2CTSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH2CTSR"]
pub mod ch2ctsr;
#[doc = "CH3CR"]
pub struct CH3CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH3CR"]
pub mod ch3cr;
#[doc = "CH3SADR"]
pub struct CH3SADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH3SADR"]
pub mod ch3sadr;
#[doc = "CH3DADR"]
pub struct CH3DADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH3DADR"]
pub mod ch3dadr;
#[doc = "CH3TSR"]
pub struct CH3TSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH3TSR"]
pub mod ch3tsr;
#[doc = "CH3CTSR"]
pub struct CH3CTSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH3CTSR"]
pub mod ch3ctsr;
#[doc = "CH4CR"]
pub struct CH4CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH4CR"]
pub mod ch4cr;
#[doc = "CH4SADR"]
pub struct CH4SADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH4SADR"]
pub mod ch4sadr;
#[doc = "CH4DADR"]
pub struct CH4DADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH4DADR"]
pub mod ch4dadr;
#[doc = "CH4TSR"]
pub struct CH4TSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH4TSR"]
pub mod ch4tsr;
#[doc = "CH4CTSR"]
pub struct CH4CTSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH4CTSR"]
pub mod ch4ctsr;
#[doc = "CH5CR"]
pub struct CH5CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH5CR"]
pub mod ch5cr;
#[doc = "CH5SADR"]
pub struct CH5SADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH5SADR"]
pub mod ch5sadr;
#[doc = "CH5DADR"]
pub struct CH5DADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH5DADR"]
pub mod ch5dadr;
#[doc = "CH5TSR"]
pub struct CH5TSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH5TSR"]
pub mod ch5tsr;
#[doc = "CH5CTSR"]
pub struct CH5CTSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CH5CTSR"]
pub mod ch5ctsr;
#[doc = "ISR"]
pub struct ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ISR"]
pub mod isr;
#[doc = "ISCR"]
pub struct ISCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ISCR"]
pub mod iscr;
#[doc = "IER"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IER"]
pub mod ier;
