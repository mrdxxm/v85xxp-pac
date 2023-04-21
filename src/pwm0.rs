#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register of PWM Timer 0"]
    pub ctl: CTL,
    #[doc = "0x04 - Current count register of PWM Timer x."]
    pub tar: TAR,
    #[doc = "0x08..0x14 - Compare/capture control register x(x=0~3) for PWM timer x."]
    pub cctl: [CCTL; 3],
    #[doc = "0x14..0x20 - Compare/capture data register x for PWM timer x."]
    pub ccr: [CCR; 3],
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control register of PWM Timer 0"]
pub mod ctl;
#[doc = "TAR (r) register accessor: an alias for `Reg<TAR_SPEC>`"]
pub type TAR = crate::Reg<tar::TAR_SPEC>;
#[doc = "Current count register of PWM Timer x."]
pub mod tar;
#[doc = "CCTL (rw) register accessor: an alias for `Reg<CCTL_SPEC>`"]
pub type CCTL = crate::Reg<cctl::CCTL_SPEC>;
#[doc = "Compare/capture control register x(x=0~3) for PWM timer x."]
pub mod cctl;
#[doc = "CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "Compare/capture data register x for PWM timer x."]
pub mod ccr;
