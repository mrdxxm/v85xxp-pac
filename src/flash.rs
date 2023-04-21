#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0xa8],
    #[doc = "0xa8 - ICE protect register."]
    pub iceprot: ICEPROT,
    #[doc = "0xac - Flash read protect status register"]
    pub rdprot: RDPROT,
    #[doc = "0xb0 - Flash write protect control register"]
    pub wrprot: WRPROT,
    _reserved3: [u8; 0x08],
    #[doc = "0xbc - Flash programming status register."]
    pub sts: STS,
    _reserved4: [u8; 0x0c],
    #[doc = "0xcc - FLASH Checksum interrupt status"]
    pub intsts: INTSTS,
    #[doc = "0xd0 - FLASH Checksum start address"]
    pub cssaddr: CSSADDR,
    #[doc = "0xd4 - FLASH Checksum end address."]
    pub cseaddr: CSEADDR,
    #[doc = "0xd8 - FLASH Checksum value register"]
    pub csvalue: CSVALUE,
    #[doc = "0xdc - FLASH Checksum compare value register."]
    pub cscvalue: CSCVALUE,
    #[doc = "0xe0 - FLASH password register"]
    pub pass: PASS,
    #[doc = "0xe4 - FLASH control register."]
    pub ctrl: CTRL,
    #[doc = "0xe8 - FLASH program address register."]
    pub pgaddr: PGADDR,
    #[doc = "0xec - FLASH program word data register."]
    pub pgdata: PGDATA,
    _reserved13: [u8; 0x04],
    #[doc = "0xf4 - FLASH sector erase control register."]
    pub serase: SERASE,
    #[doc = "0xf8 - FLASH chip erase control register."]
    pub cerase: CERASE,
    #[doc = "0xfc - FLASH deep standby control register."]
    pub dstb: DSTB,
}
#[doc = "ICEPROT (rw) register accessor: an alias for `Reg<ICEPROT_SPEC>`"]
pub type ICEPROT = crate::Reg<iceprot::ICEPROT_SPEC>;
#[doc = "ICE protect register."]
pub mod iceprot;
#[doc = "RDPROT (r) register accessor: an alias for `Reg<RDPROT_SPEC>`"]
pub type RDPROT = crate::Reg<rdprot::RDPROT_SPEC>;
#[doc = "Flash read protect status register"]
pub mod rdprot;
#[doc = "WRPROT (rw) register accessor: an alias for `Reg<WRPROT_SPEC>`"]
pub type WRPROT = crate::Reg<wrprot::WRPROT_SPEC>;
#[doc = "Flash write protect control register"]
pub mod wrprot;
#[doc = "STS (r) register accessor: an alias for `Reg<STS_SPEC>`"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "Flash programming status register."]
pub mod sts;
#[doc = "INTSTS (rw) register accessor: an alias for `Reg<INTSTS_SPEC>`"]
pub type INTSTS = crate::Reg<intsts::INTSTS_SPEC>;
#[doc = "FLASH Checksum interrupt status"]
pub mod intsts;
#[doc = "CSSADDR (rw) register accessor: an alias for `Reg<CSSADDR_SPEC>`"]
pub type CSSADDR = crate::Reg<cssaddr::CSSADDR_SPEC>;
#[doc = "FLASH Checksum start address"]
pub mod cssaddr;
#[doc = "CSEADDR (rw) register accessor: an alias for `Reg<CSEADDR_SPEC>`"]
pub type CSEADDR = crate::Reg<cseaddr::CSEADDR_SPEC>;
#[doc = "FLASH Checksum end address."]
pub mod cseaddr;
#[doc = "CSVALUE (r) register accessor: an alias for `Reg<CSVALUE_SPEC>`"]
pub type CSVALUE = crate::Reg<csvalue::CSVALUE_SPEC>;
#[doc = "FLASH Checksum value register"]
pub mod csvalue;
#[doc = "CSCVALUE (rw) register accessor: an alias for `Reg<CSCVALUE_SPEC>`"]
pub type CSCVALUE = crate::Reg<cscvalue::CSCVALUE_SPEC>;
#[doc = "FLASH Checksum compare value register."]
pub mod cscvalue;
#[doc = "PASS (rw) register accessor: an alias for `Reg<PASS_SPEC>`"]
pub type PASS = crate::Reg<pass::PASS_SPEC>;
#[doc = "FLASH password register"]
pub mod pass;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "FLASH control register."]
pub mod ctrl;
#[doc = "PGADDR (rw) register accessor: an alias for `Reg<PGADDR_SPEC>`"]
pub type PGADDR = crate::Reg<pgaddr::PGADDR_SPEC>;
#[doc = "FLASH program address register."]
pub mod pgaddr;
#[doc = "PGDATA (rw) register accessor: an alias for `Reg<PGDATA_SPEC>`"]
pub type PGDATA = crate::Reg<pgdata::PGDATA_SPEC>;
#[doc = "FLASH program word data register."]
pub mod pgdata;
#[doc = "SERASE (rw) register accessor: an alias for `Reg<SERASE_SPEC>`"]
pub type SERASE = crate::Reg<serase::SERASE_SPEC>;
#[doc = "FLASH sector erase control register."]
pub mod serase;
#[doc = "CERASE (rw) register accessor: an alias for `Reg<CERASE_SPEC>`"]
pub type CERASE = crate::Reg<cerase::CERASE_SPEC>;
#[doc = "FLASH chip erase control register."]
pub mod cerase;
#[doc = "DSTB (rw) register accessor: an alias for `Reg<DSTB_SPEC>`"]
pub type DSTB = crate::Reg<dstb::DSTB_SPEC>;
#[doc = "FLASH deep standby control register."]
pub mod dstb;
