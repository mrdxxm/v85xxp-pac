#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA interrupt enable register."]
    pub ie: IE,
    #[doc = "0x04 - DMA status register."]
    pub sts: STS,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - DMA channel x control register."]
    pub c0ctl: CCTL,
    #[doc = "0x14 - DMA source address register."]
    pub c0src: CSRC,
    #[doc = "0x18 - DMA channel x destination register."]
    pub c0dst: CDST,
    #[doc = "0x1c - DMA channel x transfer length register."]
    pub c0len: CLEN,
    #[doc = "0x20 - DMA channel x control register."]
    pub c1ctl: CCTL,
    #[doc = "0x24 - DMA source address register."]
    pub c1src: CSRC,
    #[doc = "0x28 - DMA channel x destination register."]
    pub c1dst: CDST,
    #[doc = "0x2c - DMA channel x transfer length register."]
    pub c1len: CLEN,
    #[doc = "0x30 - DMA channel x control register."]
    pub c2ctl: CCTL,
    #[doc = "0x34 - DMA source address register."]
    pub c2src: CSRC,
    #[doc = "0x38 - DMA channel x destination register."]
    pub c2dst: CDST,
    #[doc = "0x3c - DMA channel x transfer length register."]
    pub c2len: CLEN,
    #[doc = "0x40 - DMA channel x control register."]
    pub c3ctl: CCTL,
    #[doc = "0x44 - DMA source address register."]
    pub c3src: CSRC,
    #[doc = "0x48 - DMA channel x destination register."]
    pub c3dst: CDST,
    #[doc = "0x4c - DMA channel x transfer length register."]
    pub c3len: CLEN,
    #[doc = "0x50 - DMA AES control register."]
    pub aesctl: AESCTL,
    _reserved19: [u8; 0x0c],
    #[doc = "0x60..0x80 - DMA AES key x register. When mode is AES128, only register KEY3~KEY0 is used. When mode is AES192, only register KEY5~KEY0 is used. When mode is AES256, register KEY7~KEY0 is used."]
    pub aeskey: [AESKEY; 8],
}
#[doc = "IE (rw) register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "DMA interrupt enable register."]
pub mod ie;
#[doc = "STS (rw) register accessor: an alias for `Reg<STS_SPEC>`"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "DMA status register."]
pub mod sts;
#[doc = "CCTL (rw) register accessor: an alias for `Reg<CCTL_SPEC>`"]
pub type CCTL = crate::Reg<cctl::CCTL_SPEC>;
#[doc = "DMA channel x control register."]
pub mod cctl;
#[doc = "CSRC (rw) register accessor: an alias for `Reg<CSRC_SPEC>`"]
pub type CSRC = crate::Reg<csrc::CSRC_SPEC>;
#[doc = "DMA source address register."]
pub mod csrc;
#[doc = "CDST (rw) register accessor: an alias for `Reg<CDST_SPEC>`"]
pub type CDST = crate::Reg<cdst::CDST_SPEC>;
#[doc = "DMA channel x destination register."]
pub mod cdst;
#[doc = "CLEN (rw) register accessor: an alias for `Reg<CLEN_SPEC>`"]
pub type CLEN = crate::Reg<clen::CLEN_SPEC>;
#[doc = "DMA channel x transfer length register."]
pub mod clen;
#[doc = "AESCTL (rw) register accessor: an alias for `Reg<AESCTL_SPEC>`"]
pub type AESCTL = crate::Reg<aesctl::AESCTL_SPEC>;
#[doc = "DMA AES control register."]
pub mod aesctl;
#[doc = "AESKEY (rw) register accessor: an alias for `Reg<AESKEY_SPEC>`"]
pub type AESKEY = crate::Reg<aeskey::AESKEY_SPEC>;
#[doc = "DMA AES key x register. When mode is AES128, only register KEY3~KEY0 is used. When mode is AES192, only register KEY5~KEY0 is used. When mode is AES256, register KEY7~KEY0 is used."]
pub mod aeskey;
