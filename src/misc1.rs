#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SRAM Parity Error Interrupt."]
    pub sramint: SRAMINT,
    #[doc = "0x04 - SRAM initialize register."]
    pub sraminit: SRAMINIT,
    #[doc = "0x08 - SRAM Parity Error address register."]
    pub parerr: PARERR,
    #[doc = "0x0c - IR enable control register."]
    pub iren: IREN,
    #[doc = "0x10 - IR Duty low pulse control register."]
    pub dutyl: DUTYL,
    #[doc = "0x14 - IR Duty high pulse control register."]
    pub dutyh: DUTYH,
    #[doc = "0x18 - Cortex M0 IRQ latency control register."]
    pub irqlat: IRQLAT,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - AHB invalid access address."]
    pub hiaddr: HIADDR,
    #[doc = "0x24 - APB invalid access address."]
    pub piaddr: PIADDR,
}
#[doc = "SRAMINT (rw) register accessor: an alias for `Reg<SRAMINT_SPEC>`"]
pub type SRAMINT = crate::Reg<sramint::SRAMINT_SPEC>;
#[doc = "SRAM Parity Error Interrupt."]
pub mod sramint;
#[doc = "SRAMINIT (rw) register accessor: an alias for `Reg<SRAMINIT_SPEC>`"]
pub type SRAMINIT = crate::Reg<sraminit::SRAMINIT_SPEC>;
#[doc = "SRAM initialize register."]
pub mod sraminit;
#[doc = "PARERR (r) register accessor: an alias for `Reg<PARERR_SPEC>`"]
pub type PARERR = crate::Reg<parerr::PARERR_SPEC>;
#[doc = "SRAM Parity Error address register."]
pub mod parerr;
#[doc = "IREN (rw) register accessor: an alias for `Reg<IREN_SPEC>`"]
pub type IREN = crate::Reg<iren::IREN_SPEC>;
#[doc = "IR enable control register."]
pub mod iren;
#[doc = "DUTYL (rw) register accessor: an alias for `Reg<DUTYL_SPEC>`"]
pub type DUTYL = crate::Reg<dutyl::DUTYL_SPEC>;
#[doc = "IR Duty low pulse control register."]
pub mod dutyl;
#[doc = "DUTYH (rw) register accessor: an alias for `Reg<DUTYH_SPEC>`"]
pub type DUTYH = crate::Reg<dutyh::DUTYH_SPEC>;
#[doc = "IR Duty high pulse control register."]
pub mod dutyh;
#[doc = "IRQLAT (rw) register accessor: an alias for `Reg<IRQLAT_SPEC>`"]
pub type IRQLAT = crate::Reg<irqlat::IRQLAT_SPEC>;
#[doc = "Cortex M0 IRQ latency control register."]
pub mod irqlat;
#[doc = "HIADDR (r) register accessor: an alias for `Reg<HIADDR_SPEC>`"]
pub type HIADDR = crate::Reg<hiaddr::HIADDR_SPEC>;
#[doc = "AHB invalid access address."]
pub mod hiaddr;
#[doc = "PIADDR (r) register accessor: an alias for `Reg<PIADDR_SPEC>`"]
pub type PIADDR = crate::Reg<piaddr::PIADDR_SPEC>;
#[doc = "APB invalid access address."]
pub mod piaddr;
