#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - ISO7816 baud-rate low byte register"]
    pub bauddivl: BAUDDIVL,
    #[doc = "0x08 - ISO7816 baud-rate high byte register"]
    pub bauddivh: BAUDDIVH,
    #[doc = "0x0c - ISO7816 data register."]
    pub data: DATA,
    #[doc = "0x10 - ISO7816 information register."]
    pub info: INFO,
    #[doc = "0x14 - ISO7816 control register."]
    pub cfg: CFG,
    #[doc = "0x18 - ISO7816 clock divider control register."]
    pub clk: CLK,
}
#[doc = "BAUDDIVL (rw) register accessor: an alias for `Reg<BAUDDIVL_SPEC>`"]
pub type BAUDDIVL = crate::Reg<bauddivl::BAUDDIVL_SPEC>;
#[doc = "ISO7816 baud-rate low byte register"]
pub mod bauddivl;
#[doc = "BAUDDIVH (rw) register accessor: an alias for `Reg<BAUDDIVH_SPEC>`"]
pub type BAUDDIVH = crate::Reg<bauddivh::BAUDDIVH_SPEC>;
#[doc = "ISO7816 baud-rate high byte register"]
pub mod bauddivh;
#[doc = "DATA (rw) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "ISO7816 data register."]
pub mod data;
#[doc = "INFO (rw) register accessor: an alias for `Reg<INFO_SPEC>`"]
pub type INFO = crate::Reg<info::INFO_SPEC>;
#[doc = "ISO7816 information register."]
pub mod info;
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "ISO7816 control register."]
pub mod cfg;
#[doc = "CLK (rw) register accessor: an alias for `Reg<CLK_SPEC>`"]
pub type CLK = crate::Reg<clk::CLK_SPEC>;
#[doc = "ISO7816 clock divider control register."]
pub mod clk;
