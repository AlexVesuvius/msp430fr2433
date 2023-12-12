#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    rtcctl: RTCCTL,
    _reserved1: [u8; 0x02],
    rtciv: RTCIV,
    _reserved2: [u8; 0x02],
    rtcmod: RTCMOD,
    _reserved3: [u8; 0x02],
    rtccnt: RTCCNT,
}
impl RegisterBlock {
    #[doc = "0x00 - RTC control Register"]
    #[inline(always)]
    pub const fn rtcctl(&self) -> &RTCCTL {
        &self.rtcctl
    }
    #[doc = "0x04 - RTC interrupt vector"]
    #[inline(always)]
    pub const fn rtciv(&self) -> &RTCIV {
        &self.rtciv
    }
    #[doc = "0x08 - RTC moduloRegister"]
    #[inline(always)]
    pub const fn rtcmod(&self) -> &RTCMOD {
        &self.rtcmod
    }
    #[doc = "0x0c - RTC counter Register"]
    #[inline(always)]
    pub const fn rtccnt(&self) -> &RTCCNT {
        &self.rtccnt
    }
}
#[doc = "RTCCTL (rw) register accessor: RTC control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcctl`]
module"]
pub type RTCCTL = crate::Reg<rtcctl::RTCCTL_SPEC>;
#[doc = "RTC control Register"]
pub mod rtcctl;
#[doc = "RTCIV (rw) register accessor: RTC interrupt vector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtciv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtciv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtciv`]
module"]
pub type RTCIV = crate::Reg<rtciv::RTCIV_SPEC>;
#[doc = "RTC interrupt vector"]
pub mod rtciv;
#[doc = "RTCMOD (rw) register accessor: RTC moduloRegister\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcmod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcmod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcmod`]
module"]
pub type RTCMOD = crate::Reg<rtcmod::RTCMOD_SPEC>;
#[doc = "RTC moduloRegister"]
pub mod rtcmod;
#[doc = "RTCCNT (rw) register accessor: RTC counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccnt`]
module"]
pub type RTCCNT = crate::Reg<rtccnt::RTCCNT_SPEC>;
#[doc = "RTC counter Register"]
pub mod rtccnt;
