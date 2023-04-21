#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWM output selection register."]
    pub o_sel: O_SEL,
    #[doc = "0x04 - Input of PWM0 and PWM1 selection register."]
    pub i_sel01: I_SEL01,
    #[doc = "0x08 - Input of PWM2 and PWM3 selection register."]
    pub i_sel23: I_SEL23,
}
#[doc = "O_SEL (rw) register accessor: an alias for `Reg<O_SEL_SPEC>`"]
pub type O_SEL = crate::Reg<o_sel::O_SEL_SPEC>;
#[doc = "PWM output selection register."]
pub mod o_sel;
#[doc = "I_SEL01 (rw) register accessor: an alias for `Reg<I_SEL01_SPEC>`"]
pub type I_SEL01 = crate::Reg<i_sel01::I_SEL01_SPEC>;
#[doc = "Input of PWM0 and PWM1 selection register."]
pub mod i_sel01;
#[doc = "I_SEL23 (rw) register accessor: an alias for `Reg<I_SEL23_SPEC>`"]
pub type I_SEL23 = crate::Reg<i_sel23::I_SEL23_SPEC>;
#[doc = "Input of PWM2 and PWM3 selection register."]
pub mod i_sel23;
