#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IO output enable register"]
    pub oen: OEN,
    #[doc = "0x04 - IO input enable register"]
    pub ie: IE,
    #[doc = "0x08 - IO data register"]
    pub dat: DAT,
    #[doc = "0x0c - IO attribute register"]
    pub att: ATT,
    #[doc = "0x10 - IO input status register"]
    pub sts: STS,
}
#[doc = "OEN (rw) register accessor: an alias for `Reg<OEN_SPEC>`"]
pub type OEN = crate::Reg<oen::OEN_SPEC>;
#[doc = "IO output enable register"]
pub mod oen;
#[doc = "IE (rw) register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "IO input enable register"]
pub mod ie;
#[doc = "DAT (rw) register accessor: an alias for `Reg<DAT_SPEC>`"]
pub type DAT = crate::Reg<dat::DAT_SPEC>;
#[doc = "IO data register"]
pub mod dat;
#[doc = "ATT (rw) register accessor: an alias for `Reg<ATT_SPEC>`"]
pub type ATT = crate::Reg<att::ATT_SPEC>;
#[doc = "IO attribute register"]
pub mod att;
#[doc = "STS (r) register accessor: an alias for `Reg<STS_SPEC>`"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "IO input status register"]
pub mod sts;
