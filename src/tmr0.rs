#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register of Timer x"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Current count register of Timer x"]
    pub value: VALUE,
    #[doc = "0x08 - Reload register of Timer x."]
    pub reload: RELOAD,
    #[doc = "0x0c - Interrupt status register of Timer x."]
    pub intsts: INTSTS,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control register of Timer x"]
pub mod ctrl;
#[doc = "VALUE (rw) register accessor: an alias for `Reg<VALUE_SPEC>`"]
pub type VALUE = crate::Reg<value::VALUE_SPEC>;
#[doc = "Current count register of Timer x"]
pub mod value;
#[doc = "RELOAD (rw) register accessor: an alias for `Reg<RELOAD_SPEC>`"]
pub type RELOAD = crate::Reg<reload::RELOAD_SPEC>;
#[doc = "Reload register of Timer x."]
pub mod reload;
#[doc = "INTSTS (rw) register accessor: an alias for `Reg<INTSTS_SPEC>`"]
pub type INTSTS = crate::Reg<intsts::INTSTS_SPEC>;
#[doc = "Interrupt status register of Timer x."]
pub mod intsts;
