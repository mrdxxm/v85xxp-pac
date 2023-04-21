#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC second register"]
    pub sec: SEC,
    #[doc = "0x04 - RTC minute register"]
    pub min: MIN,
    #[doc = "0x08 - RTC hour register"]
    pub hour: HOUR,
    #[doc = "0x0c - RTC day register"]
    pub day: DAY,
    #[doc = "0x10 - RTC week register"]
    pub week: WEEK,
    #[doc = "0x14 - RTC mon register"]
    pub mon: MON,
    #[doc = "0x18 - RTC year register"]
    pub year: YEAR,
    #[doc = "0x1c - RTC accurate second/millisecond register"]
    pub time: TIME,
    #[doc = "0x20 - RTC wake-up second register."]
    pub wkusec: WKUSEC,
    #[doc = "0x24 - RTC wake-up minute register"]
    pub wkumin: WKUMIN,
    #[doc = "0x28 - RTC wake-up hour register"]
    pub wkuhour: WKUHOUR,
    #[doc = "0x2c - RTC wake-up counter register"]
    pub wkucnt: WKUCNT,
    #[doc = "0x30 - RTC calibration register"]
    pub cal: CAL,
    #[doc = "0x34 - RTC calibration register"]
    pub div: DIV,
    #[doc = "0x38 - RTC PLL divider control register."]
    pub ctl: CTL,
    #[doc = "0x3c - RTC wake-up interval control"]
    pub itv: ITV,
    #[doc = "0x40 - RTC wake-up second interval control"]
    pub sitv: SITV,
    #[doc = "0x44 - RTC password control register."]
    pub pwd: PWD,
    #[doc = "0x48 - RTC write enable control register."]
    pub ce: CE,
    #[doc = "0x4c - RTC read enable control register"]
    pub load: LOAD,
    #[doc = "0x50 - RTC interrupt status control register"]
    pub intsts: INTSTS,
    #[doc = "0x54 - RTC interrupt enable control register"]
    pub inten: INTEN,
    #[doc = "0x58 - RTC clock pre-scaler control register."]
    pub psca: PSCA,
    _reserved23: [u8; 0x28],
    #[doc = "0x84 - RTC auto-calibration center temperature control register."]
    pub acti: ACTI,
    #[doc = "0x88 - RTC auto-calibration 200*frequency control register."]
    pub acf200: ACF200,
    _reserved25: [u8; 0x04],
    #[doc = "0x90 - RTC parameter P0 register."]
    pub acp0: ACP0,
    #[doc = "0x94 - RTC parameter P1 register."]
    pub acp1: ACP1,
    #[doc = "0x98 - RTC parameter P2 register."]
    pub acp2: ACP2,
    #[doc = "0x9c - RTC parameter P3 register."]
    pub acp3: ACP3,
    #[doc = "0xa0 - RTC parameter P4 register."]
    pub acp4: ACP4,
    #[doc = "0xa4 - RTC parameter P5 register."]
    pub acp5: ACP5,
    #[doc = "0xa8 - RTC parameter P6 register."]
    pub acp6: ACP6,
    #[doc = "0xac - RTC parameter P7 register."]
    pub acp7: ACP7,
    #[doc = "0xb0..0xc4 - RTC auto-calibration parameter Kx control register."]
    pub ack: [ACK; 5],
    _reserved34: [u8; 0x08],
    #[doc = "0xcc - This register is used to represent the current WKUCNT value."]
    pub wkucntr: WKUCNTR,
    #[doc = "0xd0 - RTC auto-calibration k temperature section control register."]
    pub acktemp: ACKTEMP,
    #[doc = "0xd4 - RTC alarm accurate second/millisecond."]
    pub alarmtime: ALARMTIME,
    #[doc = "0xd8 - RTC alarm inaccurate second"]
    pub alarmsec: ALARMSEC,
    #[doc = "0xdc - RTC alarm minute"]
    pub alarmmin: ALARMMIN,
    #[doc = "0xe0 - RTC alarm hour"]
    pub alarmhour: ALARMHOUR,
    #[doc = "0xe4 - RTC alarm control"]
    pub alarmctl: ALARMCTL,
    #[doc = "0xe8 - RTC ADC Ucal K coefficients"]
    pub adcucalk: ADCUCALK,
    #[doc = "0xec - RTC ADC control"]
    pub adcmactl: ADCMACTL,
    #[doc = "0xf0 - RTC ADC data control"]
    pub adcdtctl: ADCDTCTL,
}
#[doc = "SEC (rw) register accessor: an alias for `Reg<SEC_SPEC>`"]
pub type SEC = crate::Reg<sec::SEC_SPEC>;
#[doc = "RTC second register"]
pub mod sec;
#[doc = "MIN (rw) register accessor: an alias for `Reg<MIN_SPEC>`"]
pub type MIN = crate::Reg<min::MIN_SPEC>;
#[doc = "RTC minute register"]
pub mod min;
#[doc = "HOUR (rw) register accessor: an alias for `Reg<HOUR_SPEC>`"]
pub type HOUR = crate::Reg<hour::HOUR_SPEC>;
#[doc = "RTC hour register"]
pub mod hour;
#[doc = "DAY (rw) register accessor: an alias for `Reg<DAY_SPEC>`"]
pub type DAY = crate::Reg<day::DAY_SPEC>;
#[doc = "RTC day register"]
pub mod day;
#[doc = "WEEK (rw) register accessor: an alias for `Reg<WEEK_SPEC>`"]
pub type WEEK = crate::Reg<week::WEEK_SPEC>;
#[doc = "RTC week register"]
pub mod week;
#[doc = "MON (rw) register accessor: an alias for `Reg<MON_SPEC>`"]
pub type MON = crate::Reg<mon::MON_SPEC>;
#[doc = "RTC mon register"]
pub mod mon;
#[doc = "YEAR (rw) register accessor: an alias for `Reg<YEAR_SPEC>`"]
pub type YEAR = crate::Reg<year::YEAR_SPEC>;
#[doc = "RTC year register"]
pub mod year;
#[doc = "TIME (rw) register accessor: an alias for `Reg<TIME_SPEC>`"]
pub type TIME = crate::Reg<time::TIME_SPEC>;
#[doc = "RTC accurate second/millisecond register"]
pub mod time;
#[doc = "WKUSEC (rw) register accessor: an alias for `Reg<WKUSEC_SPEC>`"]
pub type WKUSEC = crate::Reg<wkusec::WKUSEC_SPEC>;
#[doc = "RTC wake-up second register."]
pub mod wkusec;
#[doc = "WKUMIN (rw) register accessor: an alias for `Reg<WKUMIN_SPEC>`"]
pub type WKUMIN = crate::Reg<wkumin::WKUMIN_SPEC>;
#[doc = "RTC wake-up minute register"]
pub mod wkumin;
#[doc = "WKUHOUR (rw) register accessor: an alias for `Reg<WKUHOUR_SPEC>`"]
pub type WKUHOUR = crate::Reg<wkuhour::WKUHOUR_SPEC>;
#[doc = "RTC wake-up hour register"]
pub mod wkuhour;
#[doc = "WKUCNT (rw) register accessor: an alias for `Reg<WKUCNT_SPEC>`"]
pub type WKUCNT = crate::Reg<wkucnt::WKUCNT_SPEC>;
#[doc = "RTC wake-up counter register"]
pub mod wkucnt;
#[doc = "CAL (rw) register accessor: an alias for `Reg<CAL_SPEC>`"]
pub type CAL = crate::Reg<cal::CAL_SPEC>;
#[doc = "RTC calibration register"]
pub mod cal;
#[doc = "DIV (rw) register accessor: an alias for `Reg<DIV_SPEC>`"]
pub type DIV = crate::Reg<div::DIV_SPEC>;
#[doc = "RTC calibration register"]
pub mod div;
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "RTC PLL divider control register."]
pub mod ctl;
#[doc = "ITV (rw) register accessor: an alias for `Reg<ITV_SPEC>`"]
pub type ITV = crate::Reg<itv::ITV_SPEC>;
#[doc = "RTC wake-up interval control"]
pub mod itv;
#[doc = "SITV (rw) register accessor: an alias for `Reg<SITV_SPEC>`"]
pub type SITV = crate::Reg<sitv::SITV_SPEC>;
#[doc = "RTC wake-up second interval control"]
pub mod sitv;
#[doc = "PWD (rw) register accessor: an alias for `Reg<PWD_SPEC>`"]
pub type PWD = crate::Reg<pwd::PWD_SPEC>;
#[doc = "RTC password control register."]
pub mod pwd;
#[doc = "CE (rw) register accessor: an alias for `Reg<CE_SPEC>`"]
pub type CE = crate::Reg<ce::CE_SPEC>;
#[doc = "RTC write enable control register."]
pub mod ce;
#[doc = "LOAD (r) register accessor: an alias for `Reg<LOAD_SPEC>`"]
pub type LOAD = crate::Reg<load::LOAD_SPEC>;
#[doc = "RTC read enable control register"]
pub mod load;
#[doc = "INTSTS (rw) register accessor: an alias for `Reg<INTSTS_SPEC>`"]
pub type INTSTS = crate::Reg<intsts::INTSTS_SPEC>;
#[doc = "RTC interrupt status control register"]
pub mod intsts;
#[doc = "INTEN (rw) register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "RTC interrupt enable control register"]
pub mod inten;
#[doc = "PSCA (rw) register accessor: an alias for `Reg<PSCA_SPEC>`"]
pub type PSCA = crate::Reg<psca::PSCA_SPEC>;
#[doc = "RTC clock pre-scaler control register."]
pub mod psca;
#[doc = "ACTI (rw) register accessor: an alias for `Reg<ACTI_SPEC>`"]
pub type ACTI = crate::Reg<acti::ACTI_SPEC>;
#[doc = "RTC auto-calibration center temperature control register."]
pub mod acti;
#[doc = "ACF200 (rw) register accessor: an alias for `Reg<ACF200_SPEC>`"]
pub type ACF200 = crate::Reg<acf200::ACF200_SPEC>;
#[doc = "RTC auto-calibration 200*frequency control register."]
pub mod acf200;
#[doc = "ACP0 (rw) register accessor: an alias for `Reg<ACP0_SPEC>`"]
pub type ACP0 = crate::Reg<acp0::ACP0_SPEC>;
#[doc = "RTC parameter P0 register."]
pub mod acp0;
#[doc = "ACP1 (rw) register accessor: an alias for `Reg<ACP1_SPEC>`"]
pub type ACP1 = crate::Reg<acp1::ACP1_SPEC>;
#[doc = "RTC parameter P1 register."]
pub mod acp1;
#[doc = "ACP2 (rw) register accessor: an alias for `Reg<ACP2_SPEC>`"]
pub type ACP2 = crate::Reg<acp2::ACP2_SPEC>;
#[doc = "RTC parameter P2 register."]
pub mod acp2;
#[doc = "ACP3 (r) register accessor: an alias for `Reg<ACP3_SPEC>`"]
pub type ACP3 = crate::Reg<acp3::ACP3_SPEC>;
#[doc = "RTC parameter P3 register."]
pub mod acp3;
#[doc = "ACP4 (rw) register accessor: an alias for `Reg<ACP4_SPEC>`"]
pub type ACP4 = crate::Reg<acp4::ACP4_SPEC>;
#[doc = "RTC parameter P4 register."]
pub mod acp4;
#[doc = "ACP5 (rw) register accessor: an alias for `Reg<ACP5_SPEC>`"]
pub type ACP5 = crate::Reg<acp5::ACP5_SPEC>;
#[doc = "RTC parameter P5 register."]
pub mod acp5;
#[doc = "ACP6 (rw) register accessor: an alias for `Reg<ACP6_SPEC>`"]
pub type ACP6 = crate::Reg<acp6::ACP6_SPEC>;
#[doc = "RTC parameter P6 register."]
pub mod acp6;
#[doc = "ACP7 (rw) register accessor: an alias for `Reg<ACP7_SPEC>`"]
pub type ACP7 = crate::Reg<acp7::ACP7_SPEC>;
#[doc = "RTC parameter P7 register."]
pub mod acp7;
#[doc = "ACK (rw) register accessor: an alias for `Reg<ACK_SPEC>`"]
pub type ACK = crate::Reg<ack::ACK_SPEC>;
#[doc = "RTC auto-calibration parameter Kx control register."]
pub mod ack;
#[doc = "WKUCNTR (r) register accessor: an alias for `Reg<WKUCNTR_SPEC>`"]
pub type WKUCNTR = crate::Reg<wkucntr::WKUCNTR_SPEC>;
#[doc = "This register is used to represent the current WKUCNT value."]
pub mod wkucntr;
#[doc = "ACKTEMP (rw) register accessor: an alias for `Reg<ACKTEMP_SPEC>`"]
pub type ACKTEMP = crate::Reg<acktemp::ACKTEMP_SPEC>;
#[doc = "RTC auto-calibration k temperature section control register."]
pub mod acktemp;
#[doc = "ALARMTIME (rw) register accessor: an alias for `Reg<ALARMTIME_SPEC>`"]
pub type ALARMTIME = crate::Reg<alarmtime::ALARMTIME_SPEC>;
#[doc = "RTC alarm accurate second/millisecond."]
pub mod alarmtime;
#[doc = "ALARMSEC (rw) register accessor: an alias for `Reg<ALARMSEC_SPEC>`"]
pub type ALARMSEC = crate::Reg<alarmsec::ALARMSEC_SPEC>;
#[doc = "RTC alarm inaccurate second"]
pub mod alarmsec;
#[doc = "ALARMMIN (rw) register accessor: an alias for `Reg<ALARMMIN_SPEC>`"]
pub type ALARMMIN = crate::Reg<alarmmin::ALARMMIN_SPEC>;
#[doc = "RTC alarm minute"]
pub mod alarmmin;
#[doc = "ALARMHOUR (rw) register accessor: an alias for `Reg<ALARMHOUR_SPEC>`"]
pub type ALARMHOUR = crate::Reg<alarmhour::ALARMHOUR_SPEC>;
#[doc = "RTC alarm hour"]
pub mod alarmhour;
#[doc = "ALARMCTL (rw) register accessor: an alias for `Reg<ALARMCTL_SPEC>`"]
pub type ALARMCTL = crate::Reg<alarmctl::ALARMCTL_SPEC>;
#[doc = "RTC alarm control"]
pub mod alarmctl;
#[doc = "ADCUCALK (rw) register accessor: an alias for `Reg<ADCUCALK_SPEC>`"]
pub type ADCUCALK = crate::Reg<adcucalk::ADCUCALK_SPEC>;
#[doc = "RTC ADC Ucal K coefficients"]
pub mod adcucalk;
#[doc = "ADCMACTL (rw) register accessor: an alias for `Reg<ADCMACTL_SPEC>`"]
pub type ADCMACTL = crate::Reg<adcmactl::ADCMACTL_SPEC>;
#[doc = "RTC ADC control"]
pub mod adcmactl;
#[doc = "ADCDTCTL (rw) register accessor: an alias for `Reg<ADCDTCTL_SPEC>`"]
pub type ADCDTCTL = crate::Reg<adcdtctl::ADCDTCTL_SPEC>;
#[doc = "RTC ADC data control"]
pub mod adcdtctl;
