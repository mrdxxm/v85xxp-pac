#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UART 32K x control register 0."]
    pub ctrl0: CTRL0,
    #[doc = "0x04 - UART 32K x control register 1."]
    pub ctrl1: CTRL1,
    #[doc = "0x08 - UART 32K x baud rate control register."]
    pub bauddiv: BAUDDIV,
    #[doc = "0x0c - UART 32K x receive data buffer."]
    pub data: DATA,
    #[doc = "0x10 - UART 32K x interrupt status register."]
    pub sts: STS,
}
#[doc = "CTRL0 (rw) register accessor: an alias for `Reg<CTRL0_SPEC>`"]
pub type CTRL0 = crate::Reg<ctrl0::CTRL0_SPEC>;
#[doc = "UART 32K x control register 0."]
pub mod ctrl0;
#[doc = "CTRL1 (rw) register accessor: an alias for `Reg<CTRL1_SPEC>`"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "UART 32K x control register 1."]
pub mod ctrl1;
#[doc = "BAUDDIV (rw) register accessor: an alias for `Reg<BAUDDIV_SPEC>`"]
pub type BAUDDIV = crate::Reg<bauddiv::BAUDDIV_SPEC>;
#[doc = "UART 32K x baud rate control register."]
pub mod bauddiv;
#[doc = "DATA (r) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "UART 32K x receive data buffer."]
pub mod data;
#[doc = "STS (rw) register accessor: an alias for `Reg<STS_SPEC>`"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "UART 32K x interrupt status register."]
pub mod sts;
