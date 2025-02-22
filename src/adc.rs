#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    adcctl0: ADCCTL0,
    adcctl1: ADCCTL1,
    adcctl2: ADCCTL2,
    adclo: ADCLO,
    adchi: ADCHI,
    adcmctl0: ADCMCTL0,
    _reserved6: [u8; 0x06],
    adcmem0: ADCMEM0,
    _reserved7: [u8; 0x06],
    adcie: ADCIE,
    adcifg: ADCIFG,
    adciv: ADCIV,
}
impl RegisterBlock {
    #[doc = "0x00 - ADC Control 0"]
    #[inline(always)]
    pub const fn adcctl0(&self) -> &ADCCTL0 {
        &self.adcctl0
    }
    #[doc = "0x02 - ADC Control 1"]
    #[inline(always)]
    pub const fn adcctl1(&self) -> &ADCCTL1 {
        &self.adcctl1
    }
    #[doc = "0x04 - ADC Control 2"]
    #[inline(always)]
    pub const fn adcctl2(&self) -> &ADCCTL2 {
        &self.adcctl2
    }
    #[doc = "0x06 - ADC Window Comparator High Threshold"]
    #[inline(always)]
    pub const fn adclo(&self) -> &ADCLO {
        &self.adclo
    }
    #[doc = "0x08 - ADC Window Comparator High Threshold"]
    #[inline(always)]
    pub const fn adchi(&self) -> &ADCHI {
        &self.adchi
    }
    #[doc = "0x0a - ADC Memory Control 0"]
    #[inline(always)]
    pub const fn adcmctl0(&self) -> &ADCMCTL0 {
        &self.adcmctl0
    }
    #[doc = "0x12 - ADC Conversion Memory 0"]
    #[inline(always)]
    pub const fn adcmem0(&self) -> &ADCMEM0 {
        &self.adcmem0
    }
    #[doc = "0x1a - ADC Interrupt Enable"]
    #[inline(always)]
    pub const fn adcie(&self) -> &ADCIE {
        &self.adcie
    }
    #[doc = "0x1c - ADC Interrupt Flag"]
    #[inline(always)]
    pub const fn adcifg(&self) -> &ADCIFG {
        &self.adcifg
    }
    #[doc = "0x1e - ADC Interrupt Vector Word"]
    #[inline(always)]
    pub const fn adciv(&self) -> &ADCIV {
        &self.adciv
    }
}
#[doc = "ADCCTL0 (rw) register accessor: ADC Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcctl0`]
module"]
pub type ADCCTL0 = crate::Reg<adcctl0::ADCCTL0_SPEC>;
#[doc = "ADC Control 0"]
pub mod adcctl0;
#[doc = "ADCCTL1 (rw) register accessor: ADC Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcctl1`]
module"]
pub type ADCCTL1 = crate::Reg<adcctl1::ADCCTL1_SPEC>;
#[doc = "ADC Control 1"]
pub mod adcctl1;
#[doc = "ADCCTL2 (rw) register accessor: ADC Control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcctl2`]
module"]
pub type ADCCTL2 = crate::Reg<adcctl2::ADCCTL2_SPEC>;
#[doc = "ADC Control 2"]
pub mod adcctl2;
#[doc = "ADCLO (rw) register accessor: ADC Window Comparator High Threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adclo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adclo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adclo`]
module"]
pub type ADCLO = crate::Reg<adclo::ADCLO_SPEC>;
#[doc = "ADC Window Comparator High Threshold"]
pub mod adclo;
#[doc = "ADCHI (rw) register accessor: ADC Window Comparator High Threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adchi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adchi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adchi`]
module"]
pub type ADCHI = crate::Reg<adchi::ADCHI_SPEC>;
#[doc = "ADC Window Comparator High Threshold"]
pub mod adchi;
#[doc = "ADCMCTL0 (rw) register accessor: ADC Memory Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcmctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcmctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmctl0`]
module"]
pub type ADCMCTL0 = crate::Reg<adcmctl0::ADCMCTL0_SPEC>;
#[doc = "ADC Memory Control 0"]
pub mod adcmctl0;
#[doc = "ADCMEM0 (rw) register accessor: ADC Conversion Memory 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcmem0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcmem0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmem0`]
module"]
pub type ADCMEM0 = crate::Reg<adcmem0::ADCMEM0_SPEC>;
#[doc = "ADC Conversion Memory 0"]
pub mod adcmem0;
#[doc = "ADCIE (rw) register accessor: ADC Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcie`]
module"]
pub type ADCIE = crate::Reg<adcie::ADCIE_SPEC>;
#[doc = "ADC Interrupt Enable"]
pub mod adcie;
#[doc = "ADCIFG (rw) register accessor: ADC Interrupt Flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcifg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcifg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcifg`]
module"]
pub type ADCIFG = crate::Reg<adcifg::ADCIFG_SPEC>;
#[doc = "ADC Interrupt Flag"]
pub mod adcifg;
#[doc = "ADCIV (rw) register accessor: ADC Interrupt Vector Word\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adciv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adciv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adciv`]
module"]
pub type ADCIV = crate::Reg<adciv::ADCIV_SPEC>;
#[doc = "ADC Interrupt Vector Word"]
pub mod adciv;
