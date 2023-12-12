#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    bakmem0: BAKMEM0,
    bakmem1: BAKMEM1,
    bakmem2: BAKMEM2,
    bakmem3: BAKMEM3,
    bakmem4: BAKMEM4,
    bakmem5: BAKMEM5,
    bakmem6: BAKMEM6,
    bakmem7: BAKMEM7,
    bakmem8: BAKMEM8,
    bakmem9: BAKMEM9,
    bakmem10: BAKMEM10,
    bakmem11: BAKMEM11,
    bakmem12: BAKMEM12,
    bakmem13: BAKMEM13,
    bakmem14: BAKMEM14,
    bakmem15: BAKMEM15,
}
impl RegisterBlock {
    #[doc = "0x00 - Battery Backup Memory 0"]
    #[inline(always)]
    pub const fn bakmem0(&self) -> &BAKMEM0 {
        &self.bakmem0
    }
    #[doc = "0x02 - Battery Backup Memory 1"]
    #[inline(always)]
    pub const fn bakmem1(&self) -> &BAKMEM1 {
        &self.bakmem1
    }
    #[doc = "0x04 - Battery Backup Memory 2"]
    #[inline(always)]
    pub const fn bakmem2(&self) -> &BAKMEM2 {
        &self.bakmem2
    }
    #[doc = "0x06 - Battery Backup Memory 3"]
    #[inline(always)]
    pub const fn bakmem3(&self) -> &BAKMEM3 {
        &self.bakmem3
    }
    #[doc = "0x08 - Battery Backup Memory 4"]
    #[inline(always)]
    pub const fn bakmem4(&self) -> &BAKMEM4 {
        &self.bakmem4
    }
    #[doc = "0x0a - Battery Backup Memory 5"]
    #[inline(always)]
    pub const fn bakmem5(&self) -> &BAKMEM5 {
        &self.bakmem5
    }
    #[doc = "0x0c - Battery Backup Memory 6"]
    #[inline(always)]
    pub const fn bakmem6(&self) -> &BAKMEM6 {
        &self.bakmem6
    }
    #[doc = "0x0e - Battery Backup Memory 7"]
    #[inline(always)]
    pub const fn bakmem7(&self) -> &BAKMEM7 {
        &self.bakmem7
    }
    #[doc = "0x10 - Battery Backup Memory 8"]
    #[inline(always)]
    pub const fn bakmem8(&self) -> &BAKMEM8 {
        &self.bakmem8
    }
    #[doc = "0x12 - Battery Backup Memory 9"]
    #[inline(always)]
    pub const fn bakmem9(&self) -> &BAKMEM9 {
        &self.bakmem9
    }
    #[doc = "0x14 - Battery Backup Memory 10"]
    #[inline(always)]
    pub const fn bakmem10(&self) -> &BAKMEM10 {
        &self.bakmem10
    }
    #[doc = "0x16 - Battery Backup Memory 11"]
    #[inline(always)]
    pub const fn bakmem11(&self) -> &BAKMEM11 {
        &self.bakmem11
    }
    #[doc = "0x18 - Battery Backup Memory 12"]
    #[inline(always)]
    pub const fn bakmem12(&self) -> &BAKMEM12 {
        &self.bakmem12
    }
    #[doc = "0x1a - Battery Backup Memory 13"]
    #[inline(always)]
    pub const fn bakmem13(&self) -> &BAKMEM13 {
        &self.bakmem13
    }
    #[doc = "0x1c - Battery Backup Memory 14"]
    #[inline(always)]
    pub const fn bakmem14(&self) -> &BAKMEM14 {
        &self.bakmem14
    }
    #[doc = "0x1e - Battery Backup Memory 15"]
    #[inline(always)]
    pub const fn bakmem15(&self) -> &BAKMEM15 {
        &self.bakmem15
    }
}
#[doc = "BAKMEM0 (rw) register accessor: Battery Backup Memory 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bakmem0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bakmem0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bakmem0`]
module"]
pub type BAKMEM0 = crate::Reg<bakmem0::BAKMEM0_SPEC>;
#[doc = "Battery Backup Memory 0"]
pub mod bakmem0;
#[doc = "BAKMEM1 (rw) register accessor: Battery Backup Memory 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bakmem1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bakmem1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bakmem1`]
module"]
pub type BAKMEM1 = crate::Reg<bakmem1::BAKMEM1_SPEC>;
#[doc = "Battery Backup Memory 1"]
pub mod bakmem1;
#[doc = "BAKMEM2 (rw) register accessor: Battery Backup Memory 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bakmem2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bakmem2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bakmem2`]
module"]
pub type BAKMEM2 = crate::Reg<bakmem2::BAKMEM2_SPEC>;
#[doc = "Battery Backup Memory 2"]
pub mod bakmem2;
#[doc = "BAKMEM3 (rw) register accessor: Battery Backup Memory 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bakmem3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bakmem3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bakmem3`]
module"]
pub type BAKMEM3 = crate::Reg<bakmem3::BAKMEM3_SPEC>;
#[doc = "Battery Backup Memory 3"]
pub mod bakmem3;
#[doc = "BAKMEM4 (rw) register accessor: Battery Backup Memory 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bakmem4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bakmem4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bakmem4`]
module"]
pub type BAKMEM4 = crate::Reg<bakmem4::BAKMEM4_SPEC>;
#[doc = "Battery Backup Memory 4"]
pub mod bakmem4;
#[doc = "BAKMEM5 (rw) register accessor: Battery Backup Memory 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bakmem5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bakmem5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bakmem5`]
module"]
pub type BAKMEM5 = crate::Reg<bakmem5::BAKMEM5_SPEC>;
#[doc = "Battery Backup Memory 5"]
pub mod bakmem5;
#[doc = "BAKMEM6 (rw) register accessor: Battery Backup Memory 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bakmem6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bakmem6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bakmem6`]
module"]
pub type BAKMEM6 = crate::Reg<bakmem6::BAKMEM6_SPEC>;
#[doc = "Battery Backup Memory 6"]
pub mod bakmem6;
#[doc = "BAKMEM7 (rw) register accessor: Battery Backup Memory 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bakmem7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bakmem7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bakmem7`]
module"]
pub type BAKMEM7 = crate::Reg<bakmem7::BAKMEM7_SPEC>;
#[doc = "Battery Backup Memory 7"]
pub mod bakmem7;
#[doc = "BAKMEM8 (rw) register accessor: Battery Backup Memory 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bakmem8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bakmem8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bakmem8`]
module"]
pub type BAKMEM8 = crate::Reg<bakmem8::BAKMEM8_SPEC>;
#[doc = "Battery Backup Memory 8"]
pub mod bakmem8;
#[doc = "BAKMEM9 (rw) register accessor: Battery Backup Memory 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bakmem9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bakmem9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bakmem9`]
module"]
pub type BAKMEM9 = crate::Reg<bakmem9::BAKMEM9_SPEC>;
#[doc = "Battery Backup Memory 9"]
pub mod bakmem9;
#[doc = "BAKMEM10 (rw) register accessor: Battery Backup Memory 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bakmem10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bakmem10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bakmem10`]
module"]
pub type BAKMEM10 = crate::Reg<bakmem10::BAKMEM10_SPEC>;
#[doc = "Battery Backup Memory 10"]
pub mod bakmem10;
#[doc = "BAKMEM11 (rw) register accessor: Battery Backup Memory 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bakmem11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bakmem11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bakmem11`]
module"]
pub type BAKMEM11 = crate::Reg<bakmem11::BAKMEM11_SPEC>;
#[doc = "Battery Backup Memory 11"]
pub mod bakmem11;
#[doc = "BAKMEM12 (rw) register accessor: Battery Backup Memory 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bakmem12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bakmem12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bakmem12`]
module"]
pub type BAKMEM12 = crate::Reg<bakmem12::BAKMEM12_SPEC>;
#[doc = "Battery Backup Memory 12"]
pub mod bakmem12;
#[doc = "BAKMEM13 (rw) register accessor: Battery Backup Memory 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bakmem13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bakmem13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bakmem13`]
module"]
pub type BAKMEM13 = crate::Reg<bakmem13::BAKMEM13_SPEC>;
#[doc = "Battery Backup Memory 13"]
pub mod bakmem13;
#[doc = "BAKMEM14 (rw) register accessor: Battery Backup Memory 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bakmem14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bakmem14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bakmem14`]
module"]
pub type BAKMEM14 = crate::Reg<bakmem14::BAKMEM14_SPEC>;
#[doc = "Battery Backup Memory 14"]
pub mod bakmem14;
#[doc = "BAKMEM15 (rw) register accessor: Battery Backup Memory 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bakmem15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bakmem15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bakmem15`]
module"]
pub type BAKMEM15 = crate::Reg<bakmem15::BAKMEM15_SPEC>;
#[doc = "Battery Backup Memory 15"]
pub mod bakmem15;
