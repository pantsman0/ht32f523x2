#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPTM_CNTCFR"]
    pub gptm_cntcfr: GPTM_CNTCFR,
    #[doc = "0x04 - GPTM_MDCFR"]
    pub gptm_mdcfr: GPTM_MDCFR,
    #[doc = "0x08 - GPTM_TRCFR"]
    pub gptm_trcfr: GPTM_TRCFR,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - GPTM_CTR"]
    pub gptm_ctr: GPTM_CTR,
    _reserved1: [u8; 12usize],
    #[doc = "0x20 - GPTM_CH0ICFR"]
    pub gptm_ch0icfr: GPTM_CH0ICFR,
    #[doc = "0x24 - GPTM_CH1ICFR"]
    pub gptm_ch1icfr: GPTM_CH1ICFR,
    #[doc = "0x28 - GPTM_CH2ICFR"]
    pub gptm_ch2icfr: GPTM_CH2ICFR,
    #[doc = "0x2c - GPTM_CH3ICFR"]
    pub gptm_ch3icfr: GPTM_CH3ICFR,
    _reserved2: [u8; 16usize],
    #[doc = "0x40 - GPTM_CH0OCFR"]
    pub gptm_ch0ocfr: GPTM_CH0OCFR,
    #[doc = "0x44 - GPTM_CH1OCFR"]
    pub gptm_ch1ocfr: GPTM_CH1OCFR,
    #[doc = "0x48 - GPTM_CH2OCFR"]
    pub gptm_ch2ocfr: GPTM_CH2OCFR,
    #[doc = "0x4c - GPTM_CH3OCFR"]
    pub gptm_ch3ocfr: GPTM_CH3OCFR,
    #[doc = "0x50 - GPTM_CHCTR"]
    pub gptm_chctr: GPTM_CHCTR,
    #[doc = "0x54 - GPTM_CHPOLR"]
    pub gptm_chpolr: GPTM_CHPOLR,
    _reserved3: [u8; 28usize],
    #[doc = "0x74 - GPTM_DICTR"]
    pub gptm_dictr: GPTM_DICTR,
    #[doc = "0x78 - GPTM_EVGR"]
    pub gptm_evgr: GPTM_EVGR,
    #[doc = "0x7c - GPTM_INTSR"]
    pub gptm_intsr: GPTM_INTSR,
    #[doc = "0x80 - GPTM_CNTR"]
    pub gptm_cntr: GPTM_CNTR,
    #[doc = "0x84 - GPTM_PSCR"]
    pub gptm_pscr: GPTM_PSCR,
    #[doc = "0x88 - GPTM_CRR"]
    pub gptm_crr: GPTM_CRR,
    _reserved4: [u8; 4usize],
    #[doc = "0x90 - GPTM_CH0CCR"]
    pub gptm_ch0ccr: GPTM_CH0CCR,
    #[doc = "0x94 - GPTM_CH1CCR"]
    pub gptm_ch1ccr: GPTM_CH1CCR,
    #[doc = "0x98 - GPTM_CH2CCR"]
    pub gptm_ch2ccr: GPTM_CH2CCR,
    #[doc = "0x9c - GPTM_CH3CCR"]
    pub gptm_ch3ccr: GPTM_CH3CCR,
    #[doc = "0xa0 - GPTM_CH0ACR"]
    pub gptm_ch0acr: GPTM_CH0ACR,
    #[doc = "0xa4 - GPTM_CH1ACR"]
    pub gptm_ch1acr: GPTM_CH1ACR,
    #[doc = "0xa8 - GPTM_CH2ACR"]
    pub gptm_ch2acr: GPTM_CH2ACR,
    #[doc = "0xac - GPTM_CH3ACR"]
    pub gptm_ch3acr: GPTM_CH3ACR,
}
#[doc = "GPTM_CNTCFR"]
pub struct GPTM_CNTCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_CNTCFR"]
pub mod gptm_cntcfr;
#[doc = "GPTM_MDCFR"]
pub struct GPTM_MDCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_MDCFR"]
pub mod gptm_mdcfr;
#[doc = "GPTM_TRCFR"]
pub struct GPTM_TRCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_TRCFR"]
pub mod gptm_trcfr;
#[doc = "GPTM_CTR"]
pub struct GPTM_CTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_CTR"]
pub mod gptm_ctr;
#[doc = "GPTM_CH0ICFR"]
pub struct GPTM_CH0ICFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_CH0ICFR"]
pub mod gptm_ch0icfr;
#[doc = "GPTM_CH1ICFR"]
pub struct GPTM_CH1ICFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_CH1ICFR"]
pub mod gptm_ch1icfr;
#[doc = "GPTM_CH2ICFR"]
pub struct GPTM_CH2ICFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_CH2ICFR"]
pub mod gptm_ch2icfr;
#[doc = "GPTM_CH3ICFR"]
pub struct GPTM_CH3ICFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_CH3ICFR"]
pub mod gptm_ch3icfr;
#[doc = "GPTM_CH0OCFR"]
pub struct GPTM_CH0OCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_CH0OCFR"]
pub mod gptm_ch0ocfr;
#[doc = "GPTM_CH1OCFR"]
pub struct GPTM_CH1OCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_CH1OCFR"]
pub mod gptm_ch1ocfr;
#[doc = "GPTM_CH2OCFR"]
pub struct GPTM_CH2OCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_CH2OCFR"]
pub mod gptm_ch2ocfr;
#[doc = "GPTM_CH3OCFR"]
pub struct GPTM_CH3OCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_CH3OCFR"]
pub mod gptm_ch3ocfr;
#[doc = "GPTM_CHCTR"]
pub struct GPTM_CHCTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_CHCTR"]
pub mod gptm_chctr;
#[doc = "GPTM_CHPOLR"]
pub struct GPTM_CHPOLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_CHPOLR"]
pub mod gptm_chpolr;
#[doc = "GPTM_DICTR"]
pub struct GPTM_DICTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_DICTR"]
pub mod gptm_dictr;
#[doc = "GPTM_EVGR"]
pub struct GPTM_EVGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_EVGR"]
pub mod gptm_evgr;
#[doc = "GPTM_INTSR"]
pub struct GPTM_INTSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_INTSR"]
pub mod gptm_intsr;
#[doc = "GPTM_CNTR"]
pub struct GPTM_CNTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_CNTR"]
pub mod gptm_cntr;
#[doc = "GPTM_PSCR"]
pub struct GPTM_PSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_PSCR"]
pub mod gptm_pscr;
#[doc = "GPTM_CRR"]
pub struct GPTM_CRR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_CRR"]
pub mod gptm_crr;
#[doc = "GPTM_CH0CCR"]
pub struct GPTM_CH0CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_CH0CCR"]
pub mod gptm_ch0ccr;
#[doc = "GPTM_CH1CCR"]
pub struct GPTM_CH1CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_CH1CCR"]
pub mod gptm_ch1ccr;
#[doc = "GPTM_CH2CCR"]
pub struct GPTM_CH2CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_CH2CCR"]
pub mod gptm_ch2ccr;
#[doc = "GPTM_CH3CCR"]
pub struct GPTM_CH3CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_CH3CCR"]
pub mod gptm_ch3ccr;
#[doc = "GPTM_CH0ACR"]
pub struct GPTM_CH0ACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_CH0ACR"]
pub mod gptm_ch0acr;
#[doc = "GPTM_CH1ACR"]
pub struct GPTM_CH1ACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_CH1ACR"]
pub mod gptm_ch1acr;
#[doc = "GPTM_CH2ACR"]
pub struct GPTM_CH2ACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_CH2ACR"]
pub mod gptm_ch2acr;
#[doc = "GPTM_CH3ACR"]
pub struct GPTM_CH3ACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPTM_CH3ACR"]
pub mod gptm_ch3acr;
