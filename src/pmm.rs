#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    pmmctl0: PMMCTL0,
    pmmctl1: PMMCTL1,
    pmmctl2: PMMCTL2,
    _reserved3: [u8; 0x04],
    pmmifg: PMMIFG,
    _reserved4: [u8; 0x02],
    pmmie: PMMIE,
    pm5ctl0: PM5CTL0,
}
impl RegisterBlock {
    #[doc = "0x00 - PMM Control 0"]
    #[inline(always)]
    pub const fn pmmctl0(&self) -> &PMMCTL0 {
        &self.pmmctl0
    }
    #[doc = "0x02 - PMM Control 1"]
    #[inline(always)]
    pub const fn pmmctl1(&self) -> &PMMCTL1 {
        &self.pmmctl1
    }
    #[doc = "0x04 - PMM Control 2"]
    #[inline(always)]
    pub const fn pmmctl2(&self) -> &PMMCTL2 {
        &self.pmmctl2
    }
    #[doc = "0x0a - PMM Interrupt Flag"]
    #[inline(always)]
    pub const fn pmmifg(&self) -> &PMMIFG {
        &self.pmmifg
    }
    #[doc = "0x0e - PMM Interrupt Enable"]
    #[inline(always)]
    pub const fn pmmie(&self) -> &PMMIE {
        &self.pmmie
    }
    #[doc = "0x10 - PMM Power Mode 5 Control Register 0"]
    #[inline(always)]
    pub const fn pm5ctl0(&self) -> &PM5CTL0 {
        &self.pm5ctl0
    }
}
#[doc = "PMMCTL0 (rw) register accessor: PMM Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmmctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmmctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmmctl0`]
module"]
pub type PMMCTL0 = crate::Reg<pmmctl0::PMMCTL0_SPEC>;
#[doc = "PMM Control 0"]
pub mod pmmctl0;
#[doc = "PMMCTL1 (rw) register accessor: PMM Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmmctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmmctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmmctl1`]
module"]
pub type PMMCTL1 = crate::Reg<pmmctl1::PMMCTL1_SPEC>;
#[doc = "PMM Control 1"]
pub mod pmmctl1;
#[doc = "PMMCTL2 (rw) register accessor: PMM Control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmmctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmmctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmmctl2`]
module"]
pub type PMMCTL2 = crate::Reg<pmmctl2::PMMCTL2_SPEC>;
#[doc = "PMM Control 2"]
pub mod pmmctl2;
#[doc = "PMMIFG (rw) register accessor: PMM Interrupt Flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmmifg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmmifg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmmifg`]
module"]
pub type PMMIFG = crate::Reg<pmmifg::PMMIFG_SPEC>;
#[doc = "PMM Interrupt Flag"]
pub mod pmmifg;
#[doc = "PMMIE (rw) register accessor: PMM Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmmie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmmie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmmie`]
module"]
pub type PMMIE = crate::Reg<pmmie::PMMIE_SPEC>;
#[doc = "PMM Interrupt Enable"]
pub mod pmmie;
#[doc = "PM5CTL0 (rw) register accessor: PMM Power Mode 5 Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pm5ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pm5ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pm5ctl0`]
module"]
pub type PM5CTL0 = crate::Reg<pm5ctl0::PM5CTL0_SPEC>;
#[doc = "PMM Power Mode 5 Control Register 0"]
pub mod pm5ctl0;
