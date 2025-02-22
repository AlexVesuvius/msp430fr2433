#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ta0ctl: TA0CTL,
    ta0cctl0: TA0CCTL0,
    ta0cctl1: TA0CCTL1,
    ta0cctl2: TA0CCTL2,
    _reserved4: [u8; 0x08],
    ta0r: TA0R,
    ta0ccr0: TA0CCR0,
    ta0ccr1: TA0CCR1,
    ta0ccr2: TA0CCR2,
    _reserved8: [u8; 0x08],
    ta0ex0: TA0EX0,
    _reserved9: [u8; 0x0c],
    ta0iv: TA0IV,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer0_A3 Control"]
    #[inline(always)]
    pub const fn ta0ctl(&self) -> &TA0CTL {
        &self.ta0ctl
    }
    #[doc = "0x02 - Timer0_A3 Capture/Compare Control 0"]
    #[inline(always)]
    pub const fn ta0cctl0(&self) -> &TA0CCTL0 {
        &self.ta0cctl0
    }
    #[doc = "0x04 - Timer0_A3 Capture/Compare Control 1"]
    #[inline(always)]
    pub const fn ta0cctl1(&self) -> &TA0CCTL1 {
        &self.ta0cctl1
    }
    #[doc = "0x06 - Timer0_A3 Capture/Compare Control 2"]
    #[inline(always)]
    pub const fn ta0cctl2(&self) -> &TA0CCTL2 {
        &self.ta0cctl2
    }
    #[doc = "0x10 - Timer0_A3"]
    #[inline(always)]
    pub const fn ta0r(&self) -> &TA0R {
        &self.ta0r
    }
    #[doc = "0x12 - Timer0_A3 Capture/Compare 0"]
    #[inline(always)]
    pub const fn ta0ccr0(&self) -> &TA0CCR0 {
        &self.ta0ccr0
    }
    #[doc = "0x14 - Timer0_A3 Capture/Compare 1"]
    #[inline(always)]
    pub const fn ta0ccr1(&self) -> &TA0CCR1 {
        &self.ta0ccr1
    }
    #[doc = "0x16 - Timer0_A3 Capture/Compare 2"]
    #[inline(always)]
    pub const fn ta0ccr2(&self) -> &TA0CCR2 {
        &self.ta0ccr2
    }
    #[doc = "0x20 - Timer0_A3 Expansion Register 0"]
    #[inline(always)]
    pub const fn ta0ex0(&self) -> &TA0EX0 {
        &self.ta0ex0
    }
    #[doc = "0x2e - Timer0_A3 Interrupt Vector Word"]
    #[inline(always)]
    pub const fn ta0iv(&self) -> &TA0IV {
        &self.ta0iv
    }
}
#[doc = "TA0CTL (rw) register accessor: Timer0_A3 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta0ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta0ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta0ctl`]
module"]
pub type TA0CTL = crate::Reg<ta0ctl::TA0CTL_SPEC>;
#[doc = "Timer0_A3 Control"]
pub mod ta0ctl;
#[doc = "TA0CCTL0 (rw) register accessor: Timer0_A3 Capture/Compare Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta0cctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta0cctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta0cctl0`]
module"]
pub type TA0CCTL0 = crate::Reg<ta0cctl0::TA0CCTL0_SPEC>;
#[doc = "Timer0_A3 Capture/Compare Control 0"]
pub mod ta0cctl0;
#[doc = "TA0CCTL1 (rw) register accessor: Timer0_A3 Capture/Compare Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta0cctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta0cctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta0cctl1`]
module"]
pub type TA0CCTL1 = crate::Reg<ta0cctl1::TA0CCTL1_SPEC>;
#[doc = "Timer0_A3 Capture/Compare Control 1"]
pub mod ta0cctl1;
#[doc = "TA0CCTL2 (rw) register accessor: Timer0_A3 Capture/Compare Control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta0cctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta0cctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta0cctl2`]
module"]
pub type TA0CCTL2 = crate::Reg<ta0cctl2::TA0CCTL2_SPEC>;
#[doc = "Timer0_A3 Capture/Compare Control 2"]
pub mod ta0cctl2;
#[doc = "TA0R (rw) register accessor: Timer0_A3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta0r`]
module"]
pub type TA0R = crate::Reg<ta0r::TA0R_SPEC>;
#[doc = "Timer0_A3"]
pub mod ta0r;
#[doc = "TA0CCR0 (rw) register accessor: Timer0_A3 Capture/Compare 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta0ccr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta0ccr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta0ccr0`]
module"]
pub type TA0CCR0 = crate::Reg<ta0ccr0::TA0CCR0_SPEC>;
#[doc = "Timer0_A3 Capture/Compare 0"]
pub mod ta0ccr0;
#[doc = "TA0CCR1 (rw) register accessor: Timer0_A3 Capture/Compare 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta0ccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta0ccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta0ccr1`]
module"]
pub type TA0CCR1 = crate::Reg<ta0ccr1::TA0CCR1_SPEC>;
#[doc = "Timer0_A3 Capture/Compare 1"]
pub mod ta0ccr1;
#[doc = "TA0CCR2 (rw) register accessor: Timer0_A3 Capture/Compare 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta0ccr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta0ccr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta0ccr2`]
module"]
pub type TA0CCR2 = crate::Reg<ta0ccr2::TA0CCR2_SPEC>;
#[doc = "Timer0_A3 Capture/Compare 2"]
pub mod ta0ccr2;
#[doc = "TA0EX0 (rw) register accessor: Timer0_A3 Expansion Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta0ex0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta0ex0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta0ex0`]
module"]
pub type TA0EX0 = crate::Reg<ta0ex0::TA0EX0_SPEC>;
#[doc = "Timer0_A3 Expansion Register 0"]
pub mod ta0ex0;
#[doc = "TA0IV (rw) register accessor: Timer0_A3 Interrupt Vector Word\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta0iv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta0iv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta0iv`]
module"]
pub type TA0IV = crate::Reg<ta0iv::TA0IV_SPEC>;
#[doc = "Timer0_A3 Interrupt Vector Word"]
pub mod ta0iv;
