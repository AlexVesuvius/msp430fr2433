#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    frctl0: FRCTL0,
    _reserved1: [u8; 0x02],
    gcctl0: GCCTL0,
    gcctl1: GCCTL1,
}
impl RegisterBlock {
    #[doc = "0x00 - FRAM Controller Control 0"]
    #[inline(always)]
    pub const fn frctl0(&self) -> &FRCTL0 {
        &self.frctl0
    }
    #[doc = "0x04 - General Control 0"]
    #[inline(always)]
    pub const fn gcctl0(&self) -> &GCCTL0 {
        &self.gcctl0
    }
    #[doc = "0x06 - General Control 1"]
    #[inline(always)]
    pub const fn gcctl1(&self) -> &GCCTL1 {
        &self.gcctl1
    }
}
#[doc = "FRCTL0 (rw) register accessor: FRAM Controller Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frctl0`]
module"]
pub type FRCTL0 = crate::Reg<frctl0::FRCTL0_SPEC>;
#[doc = "FRAM Controller Control 0"]
pub mod frctl0;
#[doc = "GCCTL0 (rw) register accessor: General Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcctl0`]
module"]
pub type GCCTL0 = crate::Reg<gcctl0::GCCTL0_SPEC>;
#[doc = "General Control 0"]
pub mod gcctl0;
#[doc = "GCCTL1 (rw) register accessor: General Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcctl1`]
module"]
pub type GCCTL1 = crate::Reg<gcctl1::GCCTL1_SPEC>;
#[doc = "General Control 1"]
pub mod gcctl1;
