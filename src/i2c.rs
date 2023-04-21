#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C data register."]
    pub data: DATA,
    #[doc = "0x04 - I2C address register."]
    pub addr: ADDR,
    #[doc = "0x08 - I2C control/status register."]
    pub ctrl: CTRL,
    #[doc = "0x0c - I2C status register."]
    pub sts: STS,
    _reserved4: [u8; 0x08],
    #[doc = "0x18 - I2C interrupt enable register."]
    pub ctrl2: CTRL2,
}
#[doc = "DATA (rw) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "I2C data register."]
pub mod data;
#[doc = "ADDR (rw) register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "I2C address register."]
pub mod addr;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "I2C control/status register."]
pub mod ctrl;
#[doc = "STS (r) register accessor: an alias for `Reg<STS_SPEC>`"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "I2C status register."]
pub mod sts;
#[doc = "CTRL2 (rw) register accessor: an alias for `Reg<CTRL2_SPEC>`"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "I2C interrupt enable register."]
pub mod ctrl2;
