#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC Control 0"]
    pub adcctl0: crate::Reg<adcctl0::ADCCTL0_SPEC>,
    #[doc = "0x02 - ADC Control 1"]
    pub adcctl1: crate::Reg<adcctl1::ADCCTL1_SPEC>,
    #[doc = "0x04 - ADC Control 2"]
    pub adcctl2: crate::Reg<adcctl2::ADCCTL2_SPEC>,
    #[doc = "0x06 - ADC Window Comparator High Threshold"]
    pub adclo: crate::Reg<adclo::ADCLO_SPEC>,
    #[doc = "0x08 - ADC Window Comparator High Threshold"]
    pub adchi: crate::Reg<adchi::ADCHI_SPEC>,
    #[doc = "0x0a - ADC Memory Control 0"]
    pub adcmctl0: crate::Reg<adcmctl0::ADCMCTL0_SPEC>,
    _reserved6: [u8; 0x06],
    #[doc = "0x12 - ADC Conversion Memory 0"]
    pub adcmem0: crate::Reg<adcmem0::ADCMEM0_SPEC>,
    _reserved7: [u8; 0x06],
    #[doc = "0x1a - ADC Interrupt Enable"]
    pub adcie: crate::Reg<adcie::ADCIE_SPEC>,
    #[doc = "0x1c - ADC Interrupt Flag"]
    pub adcifg: crate::Reg<adcifg::ADCIFG_SPEC>,
    #[doc = "0x1e - ADC Interrupt Vector Word"]
    pub adciv: crate::Reg<adciv::ADCIV_SPEC>,
}
#[doc = "ADCCTL0 register accessor: an alias for `Reg<ADCCTL0_SPEC>`"]
pub type ADCCTL0 = crate::Reg<adcctl0::ADCCTL0_SPEC>;
#[doc = "ADC Control 0"]
pub mod adcctl0;
#[doc = "ADCCTL1 register accessor: an alias for `Reg<ADCCTL1_SPEC>`"]
pub type ADCCTL1 = crate::Reg<adcctl1::ADCCTL1_SPEC>;
#[doc = "ADC Control 1"]
pub mod adcctl1;
#[doc = "ADCCTL2 register accessor: an alias for `Reg<ADCCTL2_SPEC>`"]
pub type ADCCTL2 = crate::Reg<adcctl2::ADCCTL2_SPEC>;
#[doc = "ADC Control 2"]
pub mod adcctl2;
#[doc = "ADCLO register accessor: an alias for `Reg<ADCLO_SPEC>`"]
pub type ADCLO = crate::Reg<adclo::ADCLO_SPEC>;
#[doc = "ADC Window Comparator High Threshold"]
pub mod adclo;
#[doc = "ADCHI register accessor: an alias for `Reg<ADCHI_SPEC>`"]
pub type ADCHI = crate::Reg<adchi::ADCHI_SPEC>;
#[doc = "ADC Window Comparator High Threshold"]
pub mod adchi;
#[doc = "ADCMCTL0 register accessor: an alias for `Reg<ADCMCTL0_SPEC>`"]
pub type ADCMCTL0 = crate::Reg<adcmctl0::ADCMCTL0_SPEC>;
#[doc = "ADC Memory Control 0"]
pub mod adcmctl0;
#[doc = "ADCMEM0 register accessor: an alias for `Reg<ADCMEM0_SPEC>`"]
pub type ADCMEM0 = crate::Reg<adcmem0::ADCMEM0_SPEC>;
#[doc = "ADC Conversion Memory 0"]
pub mod adcmem0;
#[doc = "ADCIE register accessor: an alias for `Reg<ADCIE_SPEC>`"]
pub type ADCIE = crate::Reg<adcie::ADCIE_SPEC>;
#[doc = "ADC Interrupt Enable"]
pub mod adcie;
#[doc = "ADCIFG register accessor: an alias for `Reg<ADCIFG_SPEC>`"]
pub type ADCIFG = crate::Reg<adcifg::ADCIFG_SPEC>;
#[doc = "ADC Interrupt Flag"]
pub mod adcifg;
#[doc = "ADCIV register accessor: an alias for `Reg<ADCIV_SPEC>`"]
pub type ADCIV = crate::Reg<adciv::ADCIV_SPEC>;
#[doc = "ADC Interrupt Vector Word"]
pub mod adciv;
