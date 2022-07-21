#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 3 Input"]
    pub p3in: crate::Reg<p3in::P3IN_SPEC>,
    _reserved1: [u8; 0x01],
    #[doc = "0x02 - Port 3 Output"]
    pub p3out: crate::Reg<p3out::P3OUT_SPEC>,
    _reserved2: [u8; 0x01],
    #[doc = "0x04 - Port 3 Direction"]
    pub p3dir: crate::Reg<p3dir::P3DIR_SPEC>,
    _reserved3: [u8; 0x01],
    #[doc = "0x06 - Port 3 Resistor Enable"]
    pub p3ren: crate::Reg<p3ren::P3REN_SPEC>,
    _reserved4: [u8; 0x03],
    #[doc = "0x0a - Port 3 Selection0"]
    pub p3sel0: crate::Reg<p3sel0::P3SEL0_SPEC>,
    _reserved5: [u8; 0x01],
    #[doc = "0x0c - Port 3 Selection1"]
    pub p3sel1: crate::Reg<p3sel1::P3SEL1_SPEC>,
}
#[doc = "P3IN register accessor: an alias for `Reg<P3IN_SPEC>`"]
pub type P3IN = crate::Reg<p3in::P3IN_SPEC>;
#[doc = "Port 3 Input"]
pub mod p3in;
#[doc = "P3OUT register accessor: an alias for `Reg<P3OUT_SPEC>`"]
pub type P3OUT = crate::Reg<p3out::P3OUT_SPEC>;
#[doc = "Port 3 Output"]
pub mod p3out;
#[doc = "P3DIR register accessor: an alias for `Reg<P3DIR_SPEC>`"]
pub type P3DIR = crate::Reg<p3dir::P3DIR_SPEC>;
#[doc = "Port 3 Direction"]
pub mod p3dir;
#[doc = "P3REN register accessor: an alias for `Reg<P3REN_SPEC>`"]
pub type P3REN = crate::Reg<p3ren::P3REN_SPEC>;
#[doc = "Port 3 Resistor Enable"]
pub mod p3ren;
#[doc = "P3SEL0 register accessor: an alias for `Reg<P3SEL0_SPEC>`"]
pub type P3SEL0 = crate::Reg<p3sel0::P3SEL0_SPEC>;
#[doc = "Port 3 Selection0"]
pub mod p3sel0;
#[doc = "P3SEL1 register accessor: an alias for `Reg<P3SEL1_SPEC>`"]
pub type P3SEL1 = crate::Reg<p3sel1::P3SEL1_SPEC>;
#[doc = "Port 3 Selection1"]
pub mod p3sel1;
