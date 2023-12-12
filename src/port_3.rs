#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    p3in: P3IN,
    _reserved1: [u8; 0x01],
    p3out: P3OUT,
    _reserved2: [u8; 0x01],
    p3dir: P3DIR,
    _reserved3: [u8; 0x01],
    p3ren: P3REN,
    _reserved4: [u8; 0x03],
    p3sel0: P3SEL0,
    _reserved5: [u8; 0x01],
    p3sel1: P3SEL1,
}
impl RegisterBlock {
    #[doc = "0x00 - Port 3 Input"]
    #[inline(always)]
    pub const fn p3in(&self) -> &P3IN {
        &self.p3in
    }
    #[doc = "0x02 - Port 3 Output"]
    #[inline(always)]
    pub const fn p3out(&self) -> &P3OUT {
        &self.p3out
    }
    #[doc = "0x04 - Port 3 Direction"]
    #[inline(always)]
    pub const fn p3dir(&self) -> &P3DIR {
        &self.p3dir
    }
    #[doc = "0x06 - Port 3 Resistor Enable"]
    #[inline(always)]
    pub const fn p3ren(&self) -> &P3REN {
        &self.p3ren
    }
    #[doc = "0x0a - Port 3 Selection0"]
    #[inline(always)]
    pub const fn p3sel0(&self) -> &P3SEL0 {
        &self.p3sel0
    }
    #[doc = "0x0c - Port 3 Selection1"]
    #[inline(always)]
    pub const fn p3sel1(&self) -> &P3SEL1 {
        &self.p3sel1
    }
}
#[doc = "P3IN (rw) register accessor: Port 3 Input\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p3in::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p3in::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3in`]
module"]
pub type P3IN = crate::Reg<p3in::P3IN_SPEC>;
#[doc = "Port 3 Input"]
pub mod p3in;
#[doc = "P3OUT (rw) register accessor: Port 3 Output\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p3out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p3out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3out`]
module"]
pub type P3OUT = crate::Reg<p3out::P3OUT_SPEC>;
#[doc = "Port 3 Output"]
pub mod p3out;
#[doc = "P3DIR (rw) register accessor: Port 3 Direction\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p3dir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p3dir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3dir`]
module"]
pub type P3DIR = crate::Reg<p3dir::P3DIR_SPEC>;
#[doc = "Port 3 Direction"]
pub mod p3dir;
#[doc = "P3REN (rw) register accessor: Port 3 Resistor Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p3ren::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p3ren::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3ren`]
module"]
pub type P3REN = crate::Reg<p3ren::P3REN_SPEC>;
#[doc = "Port 3 Resistor Enable"]
pub mod p3ren;
#[doc = "P3SEL0 (rw) register accessor: Port 3 Selection0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p3sel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p3sel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3sel0`]
module"]
pub type P3SEL0 = crate::Reg<p3sel0::P3SEL0_SPEC>;
#[doc = "Port 3 Selection0"]
pub mod p3sel0;
#[doc = "P3SEL1 (rw) register accessor: Port 3 Selection1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p3sel1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p3sel1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3sel1`]
module"]
pub type P3SEL1 = crate::Reg<p3sel1::P3SEL1_SPEC>;
#[doc = "Port 3 Selection1"]
pub mod p3sel1;
