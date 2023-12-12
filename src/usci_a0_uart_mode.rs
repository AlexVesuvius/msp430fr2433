#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    uca0ctl1: UCA0CTL1,
    uca0ctl0: UCA0CTL0,
    uca0ctlw1: UCA0CTLW1,
    _reserved3: [u8; 0x02],
    uca0br0: UCA0BR0,
    uca0br1: UCA0BR1,
    uca0mctlw: UCA0MCTLW,
    uca0statw: UCA0STATW,
    _reserved7: [u8; 0x01],
    uca0rxbuf: UCA0RXBUF,
    uca0txbuf: UCA0TXBUF,
    uca0abctl: UCA0ABCTL,
    _reserved10: [u8; 0x01],
    uca0irtctl: UCA0IRTCTL,
    uca0irrctl: UCA0IRRCTL,
    _reserved12: [u8; 0x0a],
    uca0iv: UCA0IV,
}
impl RegisterBlock {
    #[doc = "0x00 - USCI A0 Control Register 1"]
    #[inline(always)]
    pub const fn uca0ctl1(&self) -> &UCA0CTL1 {
        &self.uca0ctl1
    }
    #[doc = "0x01 - USCI A0 Control Register 0"]
    #[inline(always)]
    pub const fn uca0ctl0(&self) -> &UCA0CTL0 {
        &self.uca0ctl0
    }
    #[doc = "0x02 - USCI A0 Control Word Register 1"]
    #[inline(always)]
    pub const fn uca0ctlw1(&self) -> &UCA0CTLW1 {
        &self.uca0ctlw1
    }
    #[doc = "0x06 - USCI A0 Baud Rate 0"]
    #[inline(always)]
    pub const fn uca0br0(&self) -> &UCA0BR0 {
        &self.uca0br0
    }
    #[doc = "0x07 - USCI A0 Baud Rate 1"]
    #[inline(always)]
    pub const fn uca0br1(&self) -> &UCA0BR1 {
        &self.uca0br1
    }
    #[doc = "0x08 - USCI A0 Modulation Control"]
    #[inline(always)]
    pub const fn uca0mctlw(&self) -> &UCA0MCTLW {
        &self.uca0mctlw
    }
    #[doc = "0x0a - USCI A0 Status Register"]
    #[inline(always)]
    pub const fn uca0statw(&self) -> &UCA0STATW {
        &self.uca0statw
    }
    #[doc = "0x0c - USCI A0 Receive Buffer"]
    #[inline(always)]
    pub const fn uca0rxbuf(&self) -> &UCA0RXBUF {
        &self.uca0rxbuf
    }
    #[doc = "0x0e - USCI A0 Transmit Buffer"]
    #[inline(always)]
    pub const fn uca0txbuf(&self) -> &UCA0TXBUF {
        &self.uca0txbuf
    }
    #[doc = "0x10 - USCI A0 LIN Control"]
    #[inline(always)]
    pub const fn uca0abctl(&self) -> &UCA0ABCTL {
        &self.uca0abctl
    }
    #[doc = "0x12 - USCI A0 IrDA Transmit Control"]
    #[inline(always)]
    pub const fn uca0irtctl(&self) -> &UCA0IRTCTL {
        &self.uca0irtctl
    }
    #[doc = "0x13 - USCI A0 IrDA Receive Control"]
    #[inline(always)]
    pub const fn uca0irrctl(&self) -> &UCA0IRRCTL {
        &self.uca0irrctl
    }
    #[doc = "0x1e - USCI A0 Interrupt Vector Register"]
    #[inline(always)]
    pub const fn uca0iv(&self) -> &UCA0IV {
        &self.uca0iv
    }
}
#[doc = "UCA0CTL1 (rw) register accessor: USCI A0 Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca0ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca0ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0ctl1`]
module"]
pub type UCA0CTL1 = crate::Reg<uca0ctl1::UCA0CTL1_SPEC>;
#[doc = "USCI A0 Control Register 1"]
pub mod uca0ctl1;
#[doc = "UCA0CTL0 (rw) register accessor: USCI A0 Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca0ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca0ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0ctl0`]
module"]
pub type UCA0CTL0 = crate::Reg<uca0ctl0::UCA0CTL0_SPEC>;
#[doc = "USCI A0 Control Register 0"]
pub mod uca0ctl0;
#[doc = "UCA0BR0 (rw) register accessor: USCI A0 Baud Rate 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca0br0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca0br0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0br0`]
module"]
pub type UCA0BR0 = crate::Reg<uca0br0::UCA0BR0_SPEC>;
#[doc = "USCI A0 Baud Rate 0"]
pub mod uca0br0;
#[doc = "UCA0BR1 (rw) register accessor: USCI A0 Baud Rate 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca0br1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca0br1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0br1`]
module"]
pub type UCA0BR1 = crate::Reg<uca0br1::UCA0BR1_SPEC>;
#[doc = "USCI A0 Baud Rate 1"]
pub mod uca0br1;
#[doc = "UCA0STATW (rw) register accessor: USCI A0 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca0statw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca0statw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0statw`]
module"]
pub type UCA0STATW = crate::Reg<uca0statw::UCA0STATW_SPEC>;
#[doc = "USCI A0 Status Register"]
pub mod uca0statw;
#[doc = "UCA0ABCTL (rw) register accessor: USCI A0 LIN Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca0abctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca0abctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0abctl`]
module"]
pub type UCA0ABCTL = crate::Reg<uca0abctl::UCA0ABCTL_SPEC>;
#[doc = "USCI A0 LIN Control"]
pub mod uca0abctl;
#[doc = "UCA0IRTCTL (rw) register accessor: USCI A0 IrDA Transmit Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca0irtctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca0irtctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0irtctl`]
module"]
pub type UCA0IRTCTL = crate::Reg<uca0irtctl::UCA0IRTCTL_SPEC>;
#[doc = "USCI A0 IrDA Transmit Control"]
pub mod uca0irtctl;
#[doc = "UCA0IRRCTL (rw) register accessor: USCI A0 IrDA Receive Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca0irrctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca0irrctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0irrctl`]
module"]
pub type UCA0IRRCTL = crate::Reg<uca0irrctl::UCA0IRRCTL_SPEC>;
#[doc = "USCI A0 IrDA Receive Control"]
pub mod uca0irrctl;
#[doc = "UCA0CTLW1 (rw) register accessor: USCI A0 Control Word Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca0ctlw1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca0ctlw1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0ctlw1`]
module"]
pub type UCA0CTLW1 = crate::Reg<uca0ctlw1::UCA0CTLW1_SPEC>;
#[doc = "USCI A0 Control Word Register 1"]
pub mod uca0ctlw1;
#[doc = "UCA0MCTLW (rw) register accessor: USCI A0 Modulation Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca0mctlw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca0mctlw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0mctlw`]
module"]
pub type UCA0MCTLW = crate::Reg<uca0mctlw::UCA0MCTLW_SPEC>;
#[doc = "USCI A0 Modulation Control"]
pub mod uca0mctlw;
#[doc = "UCA0RXBUF (rw) register accessor: USCI A0 Receive Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca0rxbuf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca0rxbuf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0rxbuf`]
module"]
pub type UCA0RXBUF = crate::Reg<uca0rxbuf::UCA0RXBUF_SPEC>;
#[doc = "USCI A0 Receive Buffer"]
pub mod uca0rxbuf;
#[doc = "UCA0TXBUF (rw) register accessor: USCI A0 Transmit Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca0txbuf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca0txbuf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0txbuf`]
module"]
pub type UCA0TXBUF = crate::Reg<uca0txbuf::UCA0TXBUF_SPEC>;
#[doc = "USCI A0 Transmit Buffer"]
pub mod uca0txbuf;
#[doc = "UCA0IV (rw) register accessor: USCI A0 Interrupt Vector Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca0iv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca0iv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca0iv`]
module"]
pub type UCA0IV = crate::Reg<uca0iv::UCA0IV_SPEC>;
#[doc = "USCI A0 Interrupt Vector Register"]
pub mod uca0iv;
