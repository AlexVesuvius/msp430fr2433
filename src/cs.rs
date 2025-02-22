#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    csctl0: CSCTL0,
    csctl1: CSCTL1,
    csctl2: CSCTL2,
    csctl3: CSCTL3,
    csctl4: CSCTL4,
    csctl5: CSCTL5,
    csctl6: CSCTL6,
    csctl7: CSCTL7,
    csctl8: CSCTL8,
}
impl RegisterBlock {
    #[doc = "0x00 - CS Control Register 0"]
    #[inline(always)]
    pub const fn csctl0(&self) -> &CSCTL0 {
        &self.csctl0
    }
    #[doc = "0x02 - CS Control Register 1"]
    #[inline(always)]
    pub const fn csctl1(&self) -> &CSCTL1 {
        &self.csctl1
    }
    #[doc = "0x04 - CS Control Register 2"]
    #[inline(always)]
    pub const fn csctl2(&self) -> &CSCTL2 {
        &self.csctl2
    }
    #[doc = "0x06 - CS Control Register 3"]
    #[inline(always)]
    pub const fn csctl3(&self) -> &CSCTL3 {
        &self.csctl3
    }
    #[doc = "0x08 - CS Control Register 4"]
    #[inline(always)]
    pub const fn csctl4(&self) -> &CSCTL4 {
        &self.csctl4
    }
    #[doc = "0x0a - CS Control Register 5"]
    #[inline(always)]
    pub const fn csctl5(&self) -> &CSCTL5 {
        &self.csctl5
    }
    #[doc = "0x0c - CS Control Register 6"]
    #[inline(always)]
    pub const fn csctl6(&self) -> &CSCTL6 {
        &self.csctl6
    }
    #[doc = "0x0e - CS Control Register 7"]
    #[inline(always)]
    pub const fn csctl7(&self) -> &CSCTL7 {
        &self.csctl7
    }
    #[doc = "0x10 - CS Control Register 8"]
    #[inline(always)]
    pub const fn csctl8(&self) -> &CSCTL8 {
        &self.csctl8
    }
}
#[doc = "CSCTL0 (rw) register accessor: CS Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csctl0`]
module"]
pub type CSCTL0 = crate::Reg<csctl0::CSCTL0_SPEC>;
#[doc = "CS Control Register 0"]
pub mod csctl0;
#[doc = "CSCTL1 (rw) register accessor: CS Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csctl1`]
module"]
pub type CSCTL1 = crate::Reg<csctl1::CSCTL1_SPEC>;
#[doc = "CS Control Register 1"]
pub mod csctl1;
#[doc = "CSCTL2 (rw) register accessor: CS Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csctl2`]
module"]
pub type CSCTL2 = crate::Reg<csctl2::CSCTL2_SPEC>;
#[doc = "CS Control Register 2"]
pub mod csctl2;
#[doc = "CSCTL3 (rw) register accessor: CS Control Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csctl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csctl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csctl3`]
module"]
pub type CSCTL3 = crate::Reg<csctl3::CSCTL3_SPEC>;
#[doc = "CS Control Register 3"]
pub mod csctl3;
#[doc = "CSCTL4 (rw) register accessor: CS Control Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csctl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csctl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csctl4`]
module"]
pub type CSCTL4 = crate::Reg<csctl4::CSCTL4_SPEC>;
#[doc = "CS Control Register 4"]
pub mod csctl4;
#[doc = "CSCTL5 (rw) register accessor: CS Control Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csctl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csctl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csctl5`]
module"]
pub type CSCTL5 = crate::Reg<csctl5::CSCTL5_SPEC>;
#[doc = "CS Control Register 5"]
pub mod csctl5;
#[doc = "CSCTL6 (rw) register accessor: CS Control Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csctl6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csctl6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csctl6`]
module"]
pub type CSCTL6 = crate::Reg<csctl6::CSCTL6_SPEC>;
#[doc = "CS Control Register 6"]
pub mod csctl6;
#[doc = "CSCTL7 (rw) register accessor: CS Control Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csctl7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csctl7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csctl7`]
module"]
pub type CSCTL7 = crate::Reg<csctl7::CSCTL7_SPEC>;
#[doc = "CS Control Register 7"]
pub mod csctl7;
#[doc = "CSCTL8 (rw) register accessor: CS Control Register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csctl8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csctl8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csctl8`]
module"]
pub type CSCTL8 = crate::Reg<csctl8::CSCTL8_SPEC>;
#[doc = "CS Control Register 8"]
pub mod csctl8;
