#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PMU deep sleep enable register."]
    pub dsleepen: DSLEEPEN,
    #[doc = "0x04 - PMU deep sleep password register."]
    pub dsleeppass: DSLEEPPASS,
    #[doc = "0x08 - PMU control register."]
    pub control: CONTROL,
    #[doc = "0x0c - PMU Status register."]
    pub sts: STS,
    _reserved4: [u8; 0x30],
    #[doc = "0x40 - Watch dog timing unlock register."]
    pub wdtpass: WDTPASS,
    #[doc = "0x44 - Watch dog timer enable register."]
    pub wdten: WDTEN,
    #[doc = "0x48 - Watch dog timer clear register."]
    pub wdtclr: WDTCLR,
    _reserved7: [u8; 0x03b4],
    #[doc = "0x400..0x500 - PMU 32 bits Retention RAM x."]
    pub ram: [RAM; 64],
}
#[doc = "DSLEEPEN (rw) register accessor: an alias for `Reg<DSLEEPEN_SPEC>`"]
pub type DSLEEPEN = crate::Reg<dsleepen::DSLEEPEN_SPEC>;
#[doc = "PMU deep sleep enable register."]
pub mod dsleepen;
#[doc = "DSLEEPPASS (rw) register accessor: an alias for `Reg<DSLEEPPASS_SPEC>`"]
pub type DSLEEPPASS = crate::Reg<dsleeppass::DSLEEPPASS_SPEC>;
#[doc = "PMU deep sleep password register."]
pub mod dsleeppass;
#[doc = "CONTROL (rw) register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "PMU control register."]
pub mod control;
#[doc = "STS (rw) register accessor: an alias for `Reg<STS_SPEC>`"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "PMU Status register."]
pub mod sts;
#[doc = "WDTPASS (rw) register accessor: an alias for `Reg<WDTPASS_SPEC>`"]
pub type WDTPASS = crate::Reg<wdtpass::WDTPASS_SPEC>;
#[doc = "Watch dog timing unlock register."]
pub mod wdtpass;
#[doc = "WDTEN (rw) register accessor: an alias for `Reg<WDTEN_SPEC>`"]
pub type WDTEN = crate::Reg<wdten::WDTEN_SPEC>;
#[doc = "Watch dog timer enable register."]
pub mod wdten;
#[doc = "WDTCLR (rw) register accessor: an alias for `Reg<WDTCLR_SPEC>`"]
pub type WDTCLR = crate::Reg<wdtclr::WDTCLR_SPEC>;
#[doc = "Watch dog timer clear register."]
pub mod wdtclr;
#[doc = "RAM (rw) register accessor: an alias for `Reg<RAM_SPEC>`"]
pub type RAM = crate::Reg<ram::RAM_SPEC>;
#[doc = "PMU 32 bits Retention RAM x."]
pub mod ram;
