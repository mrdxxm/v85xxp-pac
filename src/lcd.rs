#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0xa0 - LCD Frame buffer x register"]
    pub fb: [FB; 40],
    _reserved1: [u8; 0x60],
    #[doc = "0x100 - LCD control register."]
    pub ctrl: CTRL,
    #[doc = "0x104 - LCD control register2."]
    pub ctrl2: CTRL2,
    #[doc = "0x108 - LCD segment enable control register 0"]
    pub segctrl0: SEGCTRL0,
    #[doc = "0x10c - LCD segment enable control register 1"]
    pub segctrl1: SEGCTRL1,
    #[doc = "0x110 - LCD segment enable control register 2"]
    pub segctrl2: SEGCTRL2,
}
#[doc = "FB (rw) register accessor: an alias for `Reg<FB_SPEC>`"]
pub type FB = crate::Reg<fb::FB_SPEC>;
#[doc = "LCD Frame buffer x register"]
pub mod fb;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "LCD control register."]
pub mod ctrl;
#[doc = "CTRL2 (rw) register accessor: an alias for `Reg<CTRL2_SPEC>`"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "LCD control register2."]
pub mod ctrl2;
#[doc = "SEGCTRL0 (rw) register accessor: an alias for `Reg<SEGCTRL0_SPEC>`"]
pub type SEGCTRL0 = crate::Reg<segctrl0::SEGCTRL0_SPEC>;
#[doc = "LCD segment enable control register 0"]
pub mod segctrl0;
#[doc = "SEGCTRL1 (rw) register accessor: an alias for `Reg<SEGCTRL1_SPEC>`"]
pub type SEGCTRL1 = crate::Reg<segctrl1::SEGCTRL1_SPEC>;
#[doc = "LCD segment enable control register 1"]
pub mod segctrl1;
#[doc = "SEGCTRL2 (rw) register accessor: an alias for `Reg<SEGCTRL2_SPEC>`"]
pub type SEGCTRL2 = crate::Reg<segctrl2::SEGCTRL2_SPEC>;
#[doc = "LCD segment enable control register 2"]
pub mod segctrl2;
