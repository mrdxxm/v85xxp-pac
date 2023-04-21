#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRYPT control register."]
    pub ctrl: CTRL,
    #[doc = "0x04 - CRYPT pointer A."]
    pub ptra: PTRA,
    #[doc = "0x08 - CRYPT pointer B."]
    pub ptrb: PTRB,
    #[doc = "0x0c - CRYPT pointer O."]
    pub ptro: PTRO,
    #[doc = "0x10 - CRYPT carry/borrow bit register."]
    pub carry: CARRY,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "CRYPT control register."]
pub mod ctrl;
#[doc = "PTRA (rw) register accessor: an alias for `Reg<PTRA_SPEC>`"]
pub type PTRA = crate::Reg<ptra::PTRA_SPEC>;
#[doc = "CRYPT pointer A."]
pub mod ptra;
#[doc = "PTRB (rw) register accessor: an alias for `Reg<PTRB_SPEC>`"]
pub type PTRB = crate::Reg<ptrb::PTRB_SPEC>;
#[doc = "CRYPT pointer B."]
pub mod ptrb;
#[doc = "PTRO (rw) register accessor: an alias for `Reg<PTRO_SPEC>`"]
pub type PTRO = crate::Reg<ptro::PTRO_SPEC>;
#[doc = "CRYPT pointer O."]
pub mod ptro;
#[doc = "CARRY (r) register accessor: an alias for `Reg<CARRY_SPEC>`"]
pub type CARRY = crate::Reg<carry::CARRY_SPEC>;
#[doc = "CRYPT carry/borrow bit register."]
pub mod carry;
