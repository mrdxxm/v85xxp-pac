#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash wait cycle register."]
    pub flashwc: FLASHWC,
    #[doc = "0x04 - Clock selection register."]
    pub clksel: CLKSEL,
    #[doc = "0x08 - AHB clock divider control register."]
    pub clkdivh: CLKDIVH,
    #[doc = "0x0c - APB clock divider control register."]
    pub clkdivp: CLKDIVP,
    #[doc = "0x10 - AHB clock enable control register."]
    pub hclken: HCLKEN,
    #[doc = "0x14 - APB clock enable control register."]
    pub pclken: PCLKEN,
}
#[doc = "FLASHWC (rw) register accessor: an alias for `Reg<FLASHWC_SPEC>`"]
pub type FLASHWC = crate::Reg<flashwc::FLASHWC_SPEC>;
#[doc = "Flash wait cycle register."]
pub mod flashwc;
#[doc = "CLKSEL (rw) register accessor: an alias for `Reg<CLKSEL_SPEC>`"]
pub type CLKSEL = crate::Reg<clksel::CLKSEL_SPEC>;
#[doc = "Clock selection register."]
pub mod clksel;
#[doc = "CLKDIVH (rw) register accessor: an alias for `Reg<CLKDIVH_SPEC>`"]
pub type CLKDIVH = crate::Reg<clkdivh::CLKDIVH_SPEC>;
#[doc = "AHB clock divider control register."]
pub mod clkdivh;
#[doc = "CLKDIVP (rw) register accessor: an alias for `Reg<CLKDIVP_SPEC>`"]
pub type CLKDIVP = crate::Reg<clkdivp::CLKDIVP_SPEC>;
#[doc = "APB clock divider control register."]
pub mod clkdivp;
#[doc = "HCLKEN (rw) register accessor: an alias for `Reg<HCLKEN_SPEC>`"]
pub type HCLKEN = crate::Reg<hclken::HCLKEN_SPEC>;
#[doc = "AHB clock enable control register."]
pub mod hclken;
#[doc = "PCLKEN (rw) register accessor: an alias for `Reg<PCLKEN_SPEC>`"]
pub type PCLKEN = crate::Reg<pclken::PCLKEN_SPEC>;
#[doc = "APB clock enable control register."]
pub mod pclken;
