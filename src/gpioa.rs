#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IOA output enable register"]
    pub oen: OEN,
    #[doc = "0x04 - IOA input enable register"]
    pub ie: IE,
    #[doc = "0x08 - IOA data register"]
    pub dat: DAT,
    #[doc = "0x0c - IOA attribute register"]
    pub att: ATT,
    #[doc = "0x10 - IOA wake-up enable register"]
    pub ioawkuen: IOAWKUEN,
    #[doc = "0x14 - IOA input status register"]
    pub sts: STS,
    #[doc = "0x18 - IOA interrupt status register."]
    pub ioaintsts: IOAINTSTS,
    _reserved7: [u8; 0x0c],
    #[doc = "0x28 - IOA special function select register."]
    pub sel: SEL,
    _reserved8: [u8; 0x14],
    #[doc = "0x40 - IOA no-deglitch control register."]
    pub ioanodeg: IOANODEG,
}
#[doc = "OEN (rw) register accessor: an alias for `Reg<OEN_SPEC>`"]
pub type OEN = crate::Reg<oen::OEN_SPEC>;
#[doc = "IOA output enable register"]
pub mod oen;
#[doc = "IE (rw) register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "IOA input enable register"]
pub mod ie;
#[doc = "DAT (rw) register accessor: an alias for `Reg<DAT_SPEC>`"]
pub type DAT = crate::Reg<dat::DAT_SPEC>;
#[doc = "IOA data register"]
pub mod dat;
#[doc = "ATT (rw) register accessor: an alias for `Reg<ATT_SPEC>`"]
pub type ATT = crate::Reg<att::ATT_SPEC>;
#[doc = "IOA attribute register"]
pub mod att;
#[doc = "IOAWKUEN (rw) register accessor: an alias for `Reg<IOAWKUEN_SPEC>`"]
pub type IOAWKUEN = crate::Reg<ioawkuen::IOAWKUEN_SPEC>;
#[doc = "IOA wake-up enable register"]
pub mod ioawkuen;
#[doc = "STS (r) register accessor: an alias for `Reg<STS_SPEC>`"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "IOA input status register"]
pub mod sts;
#[doc = "IOAINTSTS (rw) register accessor: an alias for `Reg<IOAINTSTS_SPEC>`"]
pub type IOAINTSTS = crate::Reg<ioaintsts::IOAINTSTS_SPEC>;
#[doc = "IOA interrupt status register."]
pub mod ioaintsts;
#[doc = "SEL (rw) register accessor: an alias for `Reg<SEL_SPEC>`"]
pub type SEL = crate::Reg<sel::SEL_SPEC>;
#[doc = "IOA special function select register."]
pub mod sel;
#[doc = "IOANODEG (rw) register accessor: an alias for `Reg<IOANODEG_SPEC>`"]
pub type IOANODEG = crate::Reg<ioanodeg::IOANODEG_SPEC>;
#[doc = "IOA no-deglitch control register."]
pub mod ioanodeg;
