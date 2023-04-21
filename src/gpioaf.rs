#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IOB special function select register."]
    pub iob_sel: IOB_SEL,
    _reserved1: [u8; 0x08],
    #[doc = "0x0c - IOE special function select register."]
    pub ioe_sel: IOE_SEL,
    _reserved2: [u8; 0x10],
    #[doc = "0x20 - IO misc. control register."]
    pub io_misc: IO_MISC,
}
#[doc = "IOB_SEL (rw) register accessor: an alias for `Reg<IOB_SEL_SPEC>`"]
pub type IOB_SEL = crate::Reg<iob_sel::IOB_SEL_SPEC>;
#[doc = "IOB special function select register."]
pub mod iob_sel;
#[doc = "IOE_SEL (rw) register accessor: an alias for `Reg<IOE_SEL_SPEC>`"]
pub type IOE_SEL = crate::Reg<ioe_sel::IOE_SEL_SPEC>;
#[doc = "IOE special function select register."]
pub mod ioe_sel;
#[doc = "IO_MISC (rw) register accessor: an alias for `Reg<IO_MISC_SPEC>`"]
pub type IO_MISC = crate::Reg<io_misc::IO_MISC_SPEC>;
#[doc = "IO misc. control register."]
pub mod io_misc;
