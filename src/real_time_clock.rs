#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC control Register"]
    pub rtcctl: crate::Reg<rtcctl::RTCCTL_SPEC>,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - RTC interrupt vector"]
    pub rtciv: crate::Reg<rtciv::RTCIV_SPEC>,
    _reserved2: [u8; 0x02],
    #[doc = "0x08 - RTC moduloRegister"]
    pub rtcmod: crate::Reg<rtcmod::RTCMOD_SPEC>,
    _reserved3: [u8; 0x02],
    #[doc = "0x0c - RTC counter Register"]
    pub rtccnt: crate::Reg<rtccnt::RTCCNT_SPEC>,
}
#[doc = "RTCCTL register accessor: an alias for `Reg<RTCCTL_SPEC>`"]
pub type RTCCTL = crate::Reg<rtcctl::RTCCTL_SPEC>;
#[doc = "RTC control Register"]
pub mod rtcctl;
#[doc = "RTCIV register accessor: an alias for `Reg<RTCIV_SPEC>`"]
pub type RTCIV = crate::Reg<rtciv::RTCIV_SPEC>;
#[doc = "RTC interrupt vector"]
pub mod rtciv;
#[doc = "RTCMOD register accessor: an alias for `Reg<RTCMOD_SPEC>`"]
pub type RTCMOD = crate::Reg<rtcmod::RTCMOD_SPEC>;
#[doc = "RTC moduloRegister"]
pub mod rtcmod;
#[doc = "RTCCNT register accessor: an alias for `Reg<RTCCNT_SPEC>`"]
pub type RTCCNT = crate::Reg<rtccnt::RTCCNT_SPEC>;
#[doc = "RTC counter Register"]
pub mod rtccnt;
