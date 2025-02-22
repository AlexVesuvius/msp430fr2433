#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    uca1ctl1: UCA1CTL1,
    uca1ctl0: UCA1CTL0,
    uca1ctlw1: UCA1CTLW1,
    _reserved3: [u8; 0x02],
    uca1br0: UCA1BR0,
    uca1br1: UCA1BR1,
    uca1mctlw: UCA1MCTLW,
    uca1statw: UCA1STATW,
    _reserved7: [u8; 0x01],
    uca1rxbuf: UCA1RXBUF,
    uca1txbuf: UCA1TXBUF,
    uca1abctl: UCA1ABCTL,
    _reserved10: [u8; 0x01],
    uca1irtctl: UCA1IRTCTL,
    uca1irrctl: UCA1IRRCTL,
    _reserved12: [u8; 0x0a],
    uca1iv: UCA1IV,
}
impl RegisterBlock {
    #[doc = "0x00 - USCI A1 Control Register 1"]
    #[inline(always)]
    pub const fn uca1ctl1(&self) -> &UCA1CTL1 {
        &self.uca1ctl1
    }
    #[doc = "0x01 - USCI A1 Control Register 0"]
    #[inline(always)]
    pub const fn uca1ctl0(&self) -> &UCA1CTL0 {
        &self.uca1ctl0
    }
    #[doc = "0x02 - USCI A1 Control Word Register 1"]
    #[inline(always)]
    pub const fn uca1ctlw1(&self) -> &UCA1CTLW1 {
        &self.uca1ctlw1
    }
    #[doc = "0x06 - USCI A1 Baud Rate 0"]
    #[inline(always)]
    pub const fn uca1br0(&self) -> &UCA1BR0 {
        &self.uca1br0
    }
    #[doc = "0x07 - USCI A1 Baud Rate 1"]
    #[inline(always)]
    pub const fn uca1br1(&self) -> &UCA1BR1 {
        &self.uca1br1
    }
    #[doc = "0x08 - USCI A1 Modulation Control"]
    #[inline(always)]
    pub const fn uca1mctlw(&self) -> &UCA1MCTLW {
        &self.uca1mctlw
    }
    #[doc = "0x0a - USCI A1 Status Register"]
    #[inline(always)]
    pub const fn uca1statw(&self) -> &UCA1STATW {
        &self.uca1statw
    }
    #[doc = "0x0c - USCI A1 Receive Buffer"]
    #[inline(always)]
    pub const fn uca1rxbuf(&self) -> &UCA1RXBUF {
        &self.uca1rxbuf
    }
    #[doc = "0x0e - USCI A1 Transmit Buffer"]
    #[inline(always)]
    pub const fn uca1txbuf(&self) -> &UCA1TXBUF {
        &self.uca1txbuf
    }
    #[doc = "0x10 - USCI A1 LIN Control"]
    #[inline(always)]
    pub const fn uca1abctl(&self) -> &UCA1ABCTL {
        &self.uca1abctl
    }
    #[doc = "0x12 - USCI A1 IrDA Transmit Control"]
    #[inline(always)]
    pub const fn uca1irtctl(&self) -> &UCA1IRTCTL {
        &self.uca1irtctl
    }
    #[doc = "0x13 - USCI A1 IrDA Receive Control"]
    #[inline(always)]
    pub const fn uca1irrctl(&self) -> &UCA1IRRCTL {
        &self.uca1irrctl
    }
    #[doc = "0x1e - USCI A1 Interrupt Vector Register"]
    #[inline(always)]
    pub const fn uca1iv(&self) -> &UCA1IV {
        &self.uca1iv
    }
}
#[doc = "UCA1CTL1 (rw) register accessor: USCI A1 Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca1ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca1ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1ctl1`]
module"]
pub type UCA1CTL1 = crate::Reg<uca1ctl1::UCA1CTL1_SPEC>;
#[doc = "USCI A1 Control Register 1"]
pub mod uca1ctl1;
#[doc = "UCA1CTL0 (rw) register accessor: USCI A1 Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca1ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca1ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1ctl0`]
module"]
pub type UCA1CTL0 = crate::Reg<uca1ctl0::UCA1CTL0_SPEC>;
#[doc = "USCI A1 Control Register 0"]
pub mod uca1ctl0;
#[doc = "UCA1BR0 (rw) register accessor: USCI A1 Baud Rate 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca1br0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca1br0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1br0`]
module"]
pub type UCA1BR0 = crate::Reg<uca1br0::UCA1BR0_SPEC>;
#[doc = "USCI A1 Baud Rate 0"]
pub mod uca1br0;
#[doc = "UCA1BR1 (rw) register accessor: USCI A1 Baud Rate 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca1br1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca1br1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1br1`]
module"]
pub type UCA1BR1 = crate::Reg<uca1br1::UCA1BR1_SPEC>;
#[doc = "USCI A1 Baud Rate 1"]
pub mod uca1br1;
#[doc = "UCA1STATW (rw) register accessor: USCI A1 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca1statw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca1statw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1statw`]
module"]
pub type UCA1STATW = crate::Reg<uca1statw::UCA1STATW_SPEC>;
#[doc = "USCI A1 Status Register"]
pub mod uca1statw;
#[doc = "UCA1ABCTL (rw) register accessor: USCI A1 LIN Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca1abctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca1abctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1abctl`]
module"]
pub type UCA1ABCTL = crate::Reg<uca1abctl::UCA1ABCTL_SPEC>;
#[doc = "USCI A1 LIN Control"]
pub mod uca1abctl;
#[doc = "UCA1IRTCTL (rw) register accessor: USCI A1 IrDA Transmit Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca1irtctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca1irtctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1irtctl`]
module"]
pub type UCA1IRTCTL = crate::Reg<uca1irtctl::UCA1IRTCTL_SPEC>;
#[doc = "USCI A1 IrDA Transmit Control"]
pub mod uca1irtctl;
#[doc = "UCA1IRRCTL (rw) register accessor: USCI A1 IrDA Receive Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca1irrctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca1irrctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1irrctl`]
module"]
pub type UCA1IRRCTL = crate::Reg<uca1irrctl::UCA1IRRCTL_SPEC>;
#[doc = "USCI A1 IrDA Receive Control"]
pub mod uca1irrctl;
#[doc = "UCA1CTLW1 (rw) register accessor: USCI A1 Control Word Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca1ctlw1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca1ctlw1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1ctlw1`]
module"]
pub type UCA1CTLW1 = crate::Reg<uca1ctlw1::UCA1CTLW1_SPEC>;
#[doc = "USCI A1 Control Word Register 1"]
pub mod uca1ctlw1;
#[doc = "UCA1MCTLW (rw) register accessor: USCI A1 Modulation Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca1mctlw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca1mctlw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1mctlw`]
module"]
pub type UCA1MCTLW = crate::Reg<uca1mctlw::UCA1MCTLW_SPEC>;
#[doc = "USCI A1 Modulation Control"]
pub mod uca1mctlw;
#[doc = "UCA1RXBUF (rw) register accessor: USCI A1 Receive Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca1rxbuf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca1rxbuf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1rxbuf`]
module"]
pub type UCA1RXBUF = crate::Reg<uca1rxbuf::UCA1RXBUF_SPEC>;
#[doc = "USCI A1 Receive Buffer"]
pub mod uca1rxbuf;
#[doc = "UCA1TXBUF (rw) register accessor: USCI A1 Transmit Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca1txbuf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca1txbuf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1txbuf`]
module"]
pub type UCA1TXBUF = crate::Reg<uca1txbuf::UCA1TXBUF_SPEC>;
#[doc = "USCI A1 Transmit Buffer"]
pub mod uca1txbuf;
#[doc = "UCA1IV (rw) register accessor: USCI A1 Interrupt Vector Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca1iv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca1iv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uca1iv`]
module"]
pub type UCA1IV = crate::Reg<uca1iv::UCA1IV_SPEC>;
#[doc = "USCI A1 Interrupt Vector Register"]
pub mod uca1iv;
