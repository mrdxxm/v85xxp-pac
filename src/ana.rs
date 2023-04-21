#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Analog register 0."]
    pub reg0: REG0,
    #[doc = "0x04 - Analog register 1."]
    pub reg1: REG1,
    #[doc = "0x08 - Analog register 2."]
    pub reg2: REG2,
    #[doc = "0x0c - Analog register 3."]
    pub reg3: REG3,
    #[doc = "0x10 - Analog register 4."]
    pub reg4: REG4,
    #[doc = "0x14 - Analog register 5."]
    pub reg5: REG5,
    #[doc = "0x18 - Analog register 6."]
    pub reg6: REG6,
    #[doc = "0x1c - Analog register 7."]
    pub reg7: REG7,
    #[doc = "0x20 - Analog register 8."]
    pub reg8: REG8,
    #[doc = "0x24 - Analog register 9."]
    pub reg9: REG9,
    #[doc = "0x28 - Analog register 10."]
    pub rega: REGA,
    #[doc = "0x2c - Analog register 11."]
    pub regb: REGB,
    #[doc = "0x30 - Analog register 12."]
    pub regc: REGC,
    #[doc = "0x34 - Analog register 13."]
    pub regd: REGD,
    #[doc = "0x38 - Analog register 14."]
    pub rege: REGE,
    #[doc = "0x3c - Analog register 15."]
    pub regf: REGF,
    #[doc = "0x40 - Analog register 16."]
    pub reg10: REG10,
    #[doc = "0x44 - Analog register 17."]
    pub reg11: REG11,
    _reserved18: [u8; 0x08],
    #[doc = "0x50 - Analog control register."]
    pub ctrl: CTRL,
    #[doc = "0x54 - Comparator result register."]
    pub cmpout: CMPOUT,
    _reserved20: [u8; 0x04],
    #[doc = "0x5c - ADC State register."]
    pub adcstate: ADCSTATE,
    #[doc = "0x60 - Analog interrupt status register."]
    pub intsts: INTSTS,
    #[doc = "0x64 - Analog interrupt enable register."]
    pub inten: INTEN,
    #[doc = "0x68 - ADC control register."]
    pub adcctrl0: ADCCTRL0,
    #[doc = "0x6c - CMP1/CMP2 control register."]
    pub cmpctl: CMPCTL,
    #[doc = "0x70..0xb0 - ADC channel x data register."]
    pub adcdata: [ADCDATA; 16],
    #[doc = "0xb0..0xb8 - Comparator x counter."]
    pub cmpcnt: [CMPCNT; 2],
    #[doc = "0xb8 - Analog MISC control register."]
    pub misc: MISC,
    _reserved28: [u8; 0x04],
    #[doc = "0xc0 - ANA_ADCDOS."]
    pub adcdos: ADCDOS,
    _reserved29: [u8; 0x08],
    #[doc = "0xcc - ANA_ADCDCPN."]
    pub adcdcpn: ADCDCPN,
    _reserved30: [u8; 0x08],
    #[doc = "0xd8 - ANA_ADCDCNM0."]
    pub adcdcnm0: ADCDCNM0,
    _reserved31: [u8; 0x04],
    #[doc = "0xe0 - ANA_ADCDATADMA."]
    pub adcdatadma: ADCDATADMA,
    #[doc = "0xe4 - CMP1/CMP2 threshold register."]
    pub cmpthr: CMPTHR,
    #[doc = "0xe8 - ANA_ADCCTRL1."]
    pub adcctrl1: ADCCTRL1,
    #[doc = "0xec - ANA_ADCCTRL2."]
    pub adcctrl2: ADCCTRL2,
    _reserved35: [u8; 0x04],
    #[doc = "0xf4 - ANA_ADCDATATHD1_0."]
    pub adcdatathd1_0: ADCDATATHD1_0,
    #[doc = "0xf8 - ANA_ADCDATATHD3_2."]
    pub adcdatathd3_2: ADCDATATHD3_2,
    #[doc = "0xfc - ANA_ADCDATATHD_CH."]
    pub adcdatathd_ch: ADCDATATHD_CH,
}
impl RegisterBlock {
    #[doc = "0xb0 - Comparator x counter."]
    #[inline(always)]
    pub fn cmpcnt1(&self) -> &CMPCNT {
        &self.cmpcnt[0]
    }
    #[doc = "0xb4 - Comparator x counter."]
    #[inline(always)]
    pub fn cmpcnt2(&self) -> &CMPCNT {
        &self.cmpcnt[1]
    }
}
#[doc = "REG0 (rw) register accessor: an alias for `Reg<REG0_SPEC>`"]
pub type REG0 = crate::Reg<reg0::REG0_SPEC>;
#[doc = "Analog register 0."]
pub mod reg0;
#[doc = "REG1 (rw) register accessor: an alias for `Reg<REG1_SPEC>`"]
pub type REG1 = crate::Reg<reg1::REG1_SPEC>;
#[doc = "Analog register 1."]
pub mod reg1;
#[doc = "REG2 (rw) register accessor: an alias for `Reg<REG2_SPEC>`"]
pub type REG2 = crate::Reg<reg2::REG2_SPEC>;
#[doc = "Analog register 2."]
pub mod reg2;
#[doc = "REG3 (rw) register accessor: an alias for `Reg<REG3_SPEC>`"]
pub type REG3 = crate::Reg<reg3::REG3_SPEC>;
#[doc = "Analog register 3."]
pub mod reg3;
#[doc = "REG4 (rw) register accessor: an alias for `Reg<REG4_SPEC>`"]
pub type REG4 = crate::Reg<reg4::REG4_SPEC>;
#[doc = "Analog register 4."]
pub mod reg4;
#[doc = "REG5 (rw) register accessor: an alias for `Reg<REG5_SPEC>`"]
pub type REG5 = crate::Reg<reg5::REG5_SPEC>;
#[doc = "Analog register 5."]
pub mod reg5;
#[doc = "REG6 (rw) register accessor: an alias for `Reg<REG6_SPEC>`"]
pub type REG6 = crate::Reg<reg6::REG6_SPEC>;
#[doc = "Analog register 6."]
pub mod reg6;
#[doc = "REG7 (rw) register accessor: an alias for `Reg<REG7_SPEC>`"]
pub type REG7 = crate::Reg<reg7::REG7_SPEC>;
#[doc = "Analog register 7."]
pub mod reg7;
#[doc = "REG8 (rw) register accessor: an alias for `Reg<REG8_SPEC>`"]
pub type REG8 = crate::Reg<reg8::REG8_SPEC>;
#[doc = "Analog register 8."]
pub mod reg8;
#[doc = "REG9 (rw) register accessor: an alias for `Reg<REG9_SPEC>`"]
pub type REG9 = crate::Reg<reg9::REG9_SPEC>;
#[doc = "Analog register 9."]
pub mod reg9;
#[doc = "REGA (rw) register accessor: an alias for `Reg<REGA_SPEC>`"]
pub type REGA = crate::Reg<rega::REGA_SPEC>;
#[doc = "Analog register 10."]
pub mod rega;
#[doc = "REGB (rw) register accessor: an alias for `Reg<REGB_SPEC>`"]
pub type REGB = crate::Reg<regb::REGB_SPEC>;
#[doc = "Analog register 11."]
pub mod regb;
#[doc = "REGC (rw) register accessor: an alias for `Reg<REGC_SPEC>`"]
pub type REGC = crate::Reg<regc::REGC_SPEC>;
#[doc = "Analog register 12."]
pub mod regc;
#[doc = "REGD (rw) register accessor: an alias for `Reg<REGD_SPEC>`"]
pub type REGD = crate::Reg<regd::REGD_SPEC>;
#[doc = "Analog register 13."]
pub mod regd;
#[doc = "REGE (rw) register accessor: an alias for `Reg<REGE_SPEC>`"]
pub type REGE = crate::Reg<rege::REGE_SPEC>;
#[doc = "Analog register 14."]
pub mod rege;
#[doc = "REGF (rw) register accessor: an alias for `Reg<REGF_SPEC>`"]
pub type REGF = crate::Reg<regf::REGF_SPEC>;
#[doc = "Analog register 15."]
pub mod regf;
#[doc = "REG10 (rw) register accessor: an alias for `Reg<REG10_SPEC>`"]
pub type REG10 = crate::Reg<reg10::REG10_SPEC>;
#[doc = "Analog register 16."]
pub mod reg10;
#[doc = "REG11 (rw) register accessor: an alias for `Reg<REG11_SPEC>`"]
pub type REG11 = crate::Reg<reg11::REG11_SPEC>;
#[doc = "Analog register 17."]
pub mod reg11;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Analog control register."]
pub mod ctrl;
#[doc = "CMPOUT (r) register accessor: an alias for `Reg<CMPOUT_SPEC>`"]
pub type CMPOUT = crate::Reg<cmpout::CMPOUT_SPEC>;
#[doc = "Comparator result register."]
pub mod cmpout;
#[doc = "ADCSTATE (r) register accessor: an alias for `Reg<ADCSTATE_SPEC>`"]
pub type ADCSTATE = crate::Reg<adcstate::ADCSTATE_SPEC>;
#[doc = "ADC State register."]
pub mod adcstate;
#[doc = "INTSTS (rw) register accessor: an alias for `Reg<INTSTS_SPEC>`"]
pub type INTSTS = crate::Reg<intsts::INTSTS_SPEC>;
#[doc = "Analog interrupt status register."]
pub mod intsts;
#[doc = "INTEN (rw) register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Analog interrupt enable register."]
pub mod inten;
#[doc = "ADCCTRL0 (rw) register accessor: an alias for `Reg<ADCCTRL0_SPEC>`"]
pub type ADCCTRL0 = crate::Reg<adcctrl0::ADCCTRL0_SPEC>;
#[doc = "ADC control register."]
pub mod adcctrl0;
#[doc = "CMPCTL (rw) register accessor: an alias for `Reg<CMPCTL_SPEC>`"]
pub type CMPCTL = crate::Reg<cmpctl::CMPCTL_SPEC>;
#[doc = "CMP1/CMP2 control register."]
pub mod cmpctl;
#[doc = "ADCDATA (r) register accessor: an alias for `Reg<ADCDATA_SPEC>`"]
pub type ADCDATA = crate::Reg<adcdata::ADCDATA_SPEC>;
#[doc = "ADC channel x data register."]
pub mod adcdata;
#[doc = "CMPCNT (rw) register accessor: an alias for `Reg<CMPCNT_SPEC>`"]
pub type CMPCNT = crate::Reg<cmpcnt::CMPCNT_SPEC>;
#[doc = "Comparator x counter."]
pub mod cmpcnt;
#[doc = "MISC (rw) register accessor: an alias for `Reg<MISC_SPEC>`"]
pub type MISC = crate::Reg<misc::MISC_SPEC>;
#[doc = "Analog MISC control register."]
pub mod misc;
#[doc = "ADCDOS (r) register accessor: an alias for `Reg<ADCDOS_SPEC>`"]
pub type ADCDOS = crate::Reg<adcdos::ADCDOS_SPEC>;
#[doc = "ANA_ADCDOS."]
pub mod adcdos;
#[doc = "ADCDCPN (r) register accessor: an alias for `Reg<ADCDCPN_SPEC>`"]
pub type ADCDCPN = crate::Reg<adcdcpn::ADCDCPN_SPEC>;
#[doc = "ANA_ADCDCPN."]
pub mod adcdcpn;
#[doc = "ADCDCNM0 (r) register accessor: an alias for `Reg<ADCDCNM0_SPEC>`"]
pub type ADCDCNM0 = crate::Reg<adcdcnm0::ADCDCNM0_SPEC>;
#[doc = "ANA_ADCDCNM0."]
pub mod adcdcnm0;
#[doc = "ADCDATADMA (r) register accessor: an alias for `Reg<ADCDATADMA_SPEC>`"]
pub type ADCDATADMA = crate::Reg<adcdatadma::ADCDATADMA_SPEC>;
#[doc = "ANA_ADCDATADMA."]
pub mod adcdatadma;
#[doc = "CMPTHR (rw) register accessor: an alias for `Reg<CMPTHR_SPEC>`"]
pub type CMPTHR = crate::Reg<cmpthr::CMPTHR_SPEC>;
#[doc = "CMP1/CMP2 threshold register."]
pub mod cmpthr;
#[doc = "ADCCTRL1 (rw) register accessor: an alias for `Reg<ADCCTRL1_SPEC>`"]
pub type ADCCTRL1 = crate::Reg<adcctrl1::ADCCTRL1_SPEC>;
#[doc = "ANA_ADCCTRL1."]
pub mod adcctrl1;
#[doc = "ADCCTRL2 (rw) register accessor: an alias for `Reg<ADCCTRL2_SPEC>`"]
pub type ADCCTRL2 = crate::Reg<adcctrl2::ADCCTRL2_SPEC>;
#[doc = "ANA_ADCCTRL2."]
pub mod adcctrl2;
#[doc = "ADCDATATHD1_0 (rw) register accessor: an alias for `Reg<ADCDATATHD1_0_SPEC>`"]
pub type ADCDATATHD1_0 = crate::Reg<adcdatathd1_0::ADCDATATHD1_0_SPEC>;
#[doc = "ANA_ADCDATATHD1_0."]
pub mod adcdatathd1_0;
#[doc = "ADCDATATHD3_2 (rw) register accessor: an alias for `Reg<ADCDATATHD3_2_SPEC>`"]
pub type ADCDATATHD3_2 = crate::Reg<adcdatathd3_2::ADCDATATHD3_2_SPEC>;
#[doc = "ANA_ADCDATATHD3_2."]
pub mod adcdatathd3_2;
#[doc = "ADCDATATHD_CH (rw) register accessor: an alias for `Reg<ADCDATATHD_CH_SPEC>`"]
pub type ADCDATATHD_CH = crate::Reg<adcdatathd_ch::ADCDATATHD_CH_SPEC>;
#[doc = "ANA_ADCDATATHD_CH."]
pub mod adcdatathd_ch;
