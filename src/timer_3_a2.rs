#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ta3ctl: TA3CTL,
    ta3cctl0: TA3CCTL0,
    ta3cctl1: TA3CCTL1,
    _reserved3: [u8; 0x0a],
    ta3r: TA3R,
    ta3ccr0: TA3CCR0,
    ta3ccr1: TA3CCR1,
    _reserved6: [u8; 0x0a],
    ta3ex0: TA3EX0,
    _reserved7: [u8; 0x0c],
    ta3iv: TA3IV,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer3_A2 Control"]
    #[inline(always)]
    pub const fn ta3ctl(&self) -> &TA3CTL {
        &self.ta3ctl
    }
    #[doc = "0x02 - Timer3_A2 Capture/Compare Control 0"]
    #[inline(always)]
    pub const fn ta3cctl0(&self) -> &TA3CCTL0 {
        &self.ta3cctl0
    }
    #[doc = "0x04 - Timer3_A2 Capture/Compare Control 1"]
    #[inline(always)]
    pub const fn ta3cctl1(&self) -> &TA3CCTL1 {
        &self.ta3cctl1
    }
    #[doc = "0x10 - Timer3_A2"]
    #[inline(always)]
    pub const fn ta3r(&self) -> &TA3R {
        &self.ta3r
    }
    #[doc = "0x12 - Timer3_A2 Capture/Compare 0"]
    #[inline(always)]
    pub const fn ta3ccr0(&self) -> &TA3CCR0 {
        &self.ta3ccr0
    }
    #[doc = "0x14 - Timer3_A2 Capture/Compare 1"]
    #[inline(always)]
    pub const fn ta3ccr1(&self) -> &TA3CCR1 {
        &self.ta3ccr1
    }
    #[doc = "0x20 - Timer3_A2 Expansion Register 0"]
    #[inline(always)]
    pub const fn ta3ex0(&self) -> &TA3EX0 {
        &self.ta3ex0
    }
    #[doc = "0x2e - Timer3_A2 Interrupt Vector Word"]
    #[inline(always)]
    pub const fn ta3iv(&self) -> &TA3IV {
        &self.ta3iv
    }
}
#[doc = "TA3CTL (rw) register accessor: Timer3_A2 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta3ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta3ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta3ctl`]
module"]
pub type TA3CTL = crate::Reg<ta3ctl::TA3CTL_SPEC>;
#[doc = "Timer3_A2 Control"]
pub mod ta3ctl;
#[doc = "TA3CCTL0 (rw) register accessor: Timer3_A2 Capture/Compare Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta3cctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta3cctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta3cctl0`]
module"]
pub type TA3CCTL0 = crate::Reg<ta3cctl0::TA3CCTL0_SPEC>;
#[doc = "Timer3_A2 Capture/Compare Control 0"]
pub mod ta3cctl0;
#[doc = "TA3CCTL1 (rw) register accessor: Timer3_A2 Capture/Compare Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta3cctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta3cctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta3cctl1`]
module"]
pub type TA3CCTL1 = crate::Reg<ta3cctl1::TA3CCTL1_SPEC>;
#[doc = "Timer3_A2 Capture/Compare Control 1"]
pub mod ta3cctl1;
#[doc = "TA3R (rw) register accessor: Timer3_A2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta3r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta3r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta3r`]
module"]
pub type TA3R = crate::Reg<ta3r::TA3R_SPEC>;
#[doc = "Timer3_A2"]
pub mod ta3r;
#[doc = "TA3CCR0 (rw) register accessor: Timer3_A2 Capture/Compare 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta3ccr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta3ccr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta3ccr0`]
module"]
pub type TA3CCR0 = crate::Reg<ta3ccr0::TA3CCR0_SPEC>;
#[doc = "Timer3_A2 Capture/Compare 0"]
pub mod ta3ccr0;
#[doc = "TA3CCR1 (rw) register accessor: Timer3_A2 Capture/Compare 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta3ccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta3ccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta3ccr1`]
module"]
pub type TA3CCR1 = crate::Reg<ta3ccr1::TA3CCR1_SPEC>;
#[doc = "Timer3_A2 Capture/Compare 1"]
pub mod ta3ccr1;
#[doc = "TA3EX0 (rw) register accessor: Timer3_A2 Expansion Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta3ex0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta3ex0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta3ex0`]
module"]
pub type TA3EX0 = crate::Reg<ta3ex0::TA3EX0_SPEC>;
#[doc = "Timer3_A2 Expansion Register 0"]
pub mod ta3ex0;
#[doc = "TA3IV (rw) register accessor: Timer3_A2 Interrupt Vector Word\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ta3iv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ta3iv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ta3iv`]
module"]
pub type TA3IV = crate::Reg<ta3iv::TA3IV_SPEC>;
#[doc = "Timer3_A2 Interrupt Vector Word"]
pub mod ta3iv;
