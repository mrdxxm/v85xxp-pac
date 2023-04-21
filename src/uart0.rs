#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UART data register."]
    pub data: DATA,
    #[doc = "0x04 - UART status register."]
    pub state: STATE,
    #[doc = "0x08 - UART control register."]
    pub ctrl: CTRL,
    #[doc = "0x0c - UART interrupt status register."]
    pub intsts: INTSTS,
    #[doc = "0x10 - UART baud rate divide register."]
    pub bauddiv: BAUDDIV,
    #[doc = "0x14 - UART control register 2."]
    pub ctrl2: CTRL2,
}
#[doc = "DATA (rw) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "UART data register."]
pub mod data;
#[doc = "STATE (rw) register accessor: an alias for `Reg<STATE_SPEC>`"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "UART status register."]
pub mod state;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "UART control register."]
pub mod ctrl;
#[doc = "INTSTS (rw) register accessor: an alias for `Reg<INTSTS_SPEC>`"]
pub type INTSTS = crate::Reg<intsts::INTSTS_SPEC>;
#[doc = "UART interrupt status register."]
pub mod intsts;
#[doc = "BAUDDIV (rw) register accessor: an alias for `Reg<BAUDDIV_SPEC>`"]
pub type BAUDDIV = crate::Reg<bauddiv::BAUDDIV_SPEC>;
#[doc = "UART baud rate divide register."]
pub mod bauddiv;
#[doc = "CTRL2 (rw) register accessor: an alias for `Reg<CTRL2_SPEC>`"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "UART control register 2."]
pub mod ctrl2;
