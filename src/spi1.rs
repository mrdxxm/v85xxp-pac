#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI Control Register."]
    pub ctrl: CTRL,
    #[doc = "0x04 - SPI Transmit Status Register."]
    pub txsts: TXSTS,
    #[doc = "0x08 - SPI Transmit FIFO register."]
    pub txdat: TXDAT,
    #[doc = "0x0c - SPI Receive Status Register."]
    pub rxsts: RXSTS,
    #[doc = "0x10 - SPI Receive FIFO Register."]
    pub rxdat: RXDAT,
    #[doc = "0x14 - SPI Misc. Control Register."]
    pub misc: MISC,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "SPI Control Register."]
pub mod ctrl;
#[doc = "TXSTS (rw) register accessor: an alias for `Reg<TXSTS_SPEC>`"]
pub type TXSTS = crate::Reg<txsts::TXSTS_SPEC>;
#[doc = "SPI Transmit Status Register."]
pub mod txsts;
#[doc = "TXDAT (rw) register accessor: an alias for `Reg<TXDAT_SPEC>`"]
pub type TXDAT = crate::Reg<txdat::TXDAT_SPEC>;
#[doc = "SPI Transmit FIFO register."]
pub mod txdat;
#[doc = "RXSTS (rw) register accessor: an alias for `Reg<RXSTS_SPEC>`"]
pub type RXSTS = crate::Reg<rxsts::RXSTS_SPEC>;
#[doc = "SPI Receive Status Register."]
pub mod rxsts;
#[doc = "RXDAT (r) register accessor: an alias for `Reg<RXDAT_SPEC>`"]
pub type RXDAT = crate::Reg<rxdat::RXDAT_SPEC>;
#[doc = "SPI Receive FIFO Register."]
pub mod rxdat;
#[doc = "MISC (rw) register accessor: an alias for `Reg<MISC_SPEC>`"]
pub type MISC = crate::Reg<misc::MISC_SPEC>;
#[doc = "SPI Misc. Control Register."]
pub mod misc;
