#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWRCU_BAKSR"]
    pub pwrcu_baksr: PWRCU_BAKSR,
    #[doc = "0x04 - PWRCU_BAKCR"]
    pub pwrcu_bakcr: PWRCU_BAKCR,
    #[doc = "0x08 - PWRCU_BAKTEST"]
    pub pwrcu_baktest: PWRCU_BAKTEST,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - PWRCU_LVDCSR"]
    pub pwrcu_lvdcsr: PWRCU_LVDCSR,
    _reserved1: [u8; 236usize],
    #[doc = "0x100 - PWRCU_BAKREG0"]
    pub pwrcu_bakreg0: PWRCU_BAKREG0,
    #[doc = "0x104 - PWRCU_BAKREG1"]
    pub pwrcu_bakreg1: PWRCU_BAKREG1,
    #[doc = "0x108 - PWRCU_BAKREG2"]
    pub pwrcu_bakreg2: PWRCU_BAKREG2,
    #[doc = "0x10c - PWRCU_BAKREG3"]
    pub pwrcu_bakreg3: PWRCU_BAKREG3,
    #[doc = "0x110 - PWRCU_BAKREG4"]
    pub pwrcu_bakreg4: PWRCU_BAKREG4,
    #[doc = "0x114 - PWRCU_BAKREG5"]
    pub pwrcu_bakreg5: PWRCU_BAKREG5,
    #[doc = "0x118 - PWRCU_BAKREG6"]
    pub pwrcu_bakreg6: PWRCU_BAKREG6,
    #[doc = "0x11c - PWRCU_BAKREG7"]
    pub pwrcu_bakreg7: PWRCU_BAKREG7,
    #[doc = "0x120 - PWRCU_BAKREG8"]
    pub pwrcu_bakreg8: PWRCU_BAKREG8,
    #[doc = "0x124 - PWRCU_BAKREG9"]
    pub pwrcu_bakreg9: PWRCU_BAKREG9,
}
#[doc = "PWRCU_BAKSR"]
pub struct PWRCU_BAKSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWRCU_BAKSR"]
pub mod pwrcu_baksr;
#[doc = "PWRCU_BAKCR"]
pub struct PWRCU_BAKCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWRCU_BAKCR"]
pub mod pwrcu_bakcr;
#[doc = "PWRCU_BAKTEST"]
pub struct PWRCU_BAKTEST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWRCU_BAKTEST"]
pub mod pwrcu_baktest;
#[doc = "PWRCU_LVDCSR"]
pub struct PWRCU_LVDCSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWRCU_LVDCSR"]
pub mod pwrcu_lvdcsr;
#[doc = "PWRCU_BAKREG0"]
pub struct PWRCU_BAKREG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWRCU_BAKREG0"]
pub mod pwrcu_bakreg0;
#[doc = "PWRCU_BAKREG1"]
pub struct PWRCU_BAKREG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWRCU_BAKREG1"]
pub mod pwrcu_bakreg1;
#[doc = "PWRCU_BAKREG2"]
pub struct PWRCU_BAKREG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWRCU_BAKREG2"]
pub mod pwrcu_bakreg2;
#[doc = "PWRCU_BAKREG3"]
pub struct PWRCU_BAKREG3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWRCU_BAKREG3"]
pub mod pwrcu_bakreg3;
#[doc = "PWRCU_BAKREG4"]
pub struct PWRCU_BAKREG4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWRCU_BAKREG4"]
pub mod pwrcu_bakreg4;
#[doc = "PWRCU_BAKREG5"]
pub struct PWRCU_BAKREG5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWRCU_BAKREG5"]
pub mod pwrcu_bakreg5;
#[doc = "PWRCU_BAKREG6"]
pub struct PWRCU_BAKREG6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWRCU_BAKREG6"]
pub mod pwrcu_bakreg6;
#[doc = "PWRCU_BAKREG7"]
pub struct PWRCU_BAKREG7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWRCU_BAKREG7"]
pub mod pwrcu_bakreg7;
#[doc = "PWRCU_BAKREG8"]
pub struct PWRCU_BAKREG8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWRCU_BAKREG8"]
pub mod pwrcu_bakreg8;
#[doc = "PWRCU_BAKREG9"]
pub struct PWRCU_BAKREG9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWRCU_BAKREG9"]
pub mod pwrcu_bakreg9;
