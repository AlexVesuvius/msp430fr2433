#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ucb0ctl1: UCB0CTL1,
    ucb0ctl0: UCB0CTL0,
    ucb0ctlw1: UCB0CTLW1,
    _reserved3: [u8; 0x02],
    ucb0br0: UCB0BR0,
    ucb0br1: UCB0BR1,
    ucb0stat_i2c: UCB0STAT_I2C,
    ucb0bcnt_i2c: UCB0BCNT_I2C,
    ucb0tbcnt: UCB0TBCNT,
    ucb0rxbuf: UCB0RXBUF,
    ucb0txbuf: UCB0TXBUF,
    _reserved10: [u8; 0x04],
    ucb0i2coa0: UCB0I2COA0,
    ucb0i2coa1: UCB0I2COA1,
    ucb0i2coa2: UCB0I2COA2,
    ucb0i2coa3: UCB0I2COA3,
    ucb0addrx: UCB0ADDRX,
    ucb0addmask: UCB0ADDMASK,
    ucb0i2csa: UCB0I2CSA,
    _reserved17: [u8; 0x08],
    _reserved_17_ucb0: [u8; 0x02],
    _reserved_18_ucb0: [u8; 0x02],
    ucb0iv: UCB0IV,
}
impl RegisterBlock {
    #[doc = "0x00 - USCI B0 Control Register 1"]
    #[inline(always)]
    pub const fn ucb0ctl1(&self) -> &UCB0CTL1 {
        &self.ucb0ctl1
    }
    #[doc = "0x01 - USCI B0 Control Register 0"]
    #[inline(always)]
    pub const fn ucb0ctl0(&self) -> &UCB0CTL0 {
        &self.ucb0ctl0
    }
    #[doc = "0x02 - USCI B0 Control Word Register 1"]
    #[inline(always)]
    pub const fn ucb0ctlw1(&self) -> &UCB0CTLW1 {
        &self.ucb0ctlw1
    }
    #[doc = "0x06 - USCI B0 Baud Rate 0"]
    #[inline(always)]
    pub const fn ucb0br0(&self) -> &UCB0BR0 {
        &self.ucb0br0
    }
    #[doc = "0x07 - USCI B0 Baud Rate 1"]
    #[inline(always)]
    pub const fn ucb0br1(&self) -> &UCB0BR1 {
        &self.ucb0br1
    }
    #[doc = "0x08 - USCI B0 Status Register"]
    #[inline(always)]
    pub const fn ucb0stat_i2c(&self) -> &UCB0STAT_I2C {
        &self.ucb0stat_i2c
    }
    #[doc = "0x09 - USCI B0 Byte Counter Register"]
    #[inline(always)]
    pub const fn ucb0bcnt_i2c(&self) -> &UCB0BCNT_I2C {
        &self.ucb0bcnt_i2c
    }
    #[doc = "0x0a - USCI B0 Byte Counter Threshold Register"]
    #[inline(always)]
    pub const fn ucb0tbcnt(&self) -> &UCB0TBCNT {
        &self.ucb0tbcnt
    }
    #[doc = "0x0c - USCI B0 Receive Buffer"]
    #[inline(always)]
    pub const fn ucb0rxbuf(&self) -> &UCB0RXBUF {
        &self.ucb0rxbuf
    }
    #[doc = "0x0e - USCI B0 Transmit Buffer"]
    #[inline(always)]
    pub const fn ucb0txbuf(&self) -> &UCB0TXBUF {
        &self.ucb0txbuf
    }
    #[doc = "0x14 - USCI B0 I2C Own Address 0"]
    #[inline(always)]
    pub const fn ucb0i2coa0(&self) -> &UCB0I2COA0 {
        &self.ucb0i2coa0
    }
    #[doc = "0x16 - USCI B0 I2C Own Address 1"]
    #[inline(always)]
    pub const fn ucb0i2coa1(&self) -> &UCB0I2COA1 {
        &self.ucb0i2coa1
    }
    #[doc = "0x18 - USCI B0 I2C Own Address 2"]
    #[inline(always)]
    pub const fn ucb0i2coa2(&self) -> &UCB0I2COA2 {
        &self.ucb0i2coa2
    }
    #[doc = "0x1a - USCI B0 I2C Own Address 3"]
    #[inline(always)]
    pub const fn ucb0i2coa3(&self) -> &UCB0I2COA3 {
        &self.ucb0i2coa3
    }
    #[doc = "0x1c - USCI B0 Received Address Register"]
    #[inline(always)]
    pub const fn ucb0addrx(&self) -> &UCB0ADDRX {
        &self.ucb0addrx
    }
    #[doc = "0x1e - USCI B0 Address Mask Register"]
    #[inline(always)]
    pub const fn ucb0addmask(&self) -> &UCB0ADDMASK {
        &self.ucb0addmask
    }
    #[doc = "0x20 - USCI B0 I2C Slave Address"]
    #[inline(always)]
    pub const fn ucb0i2csa(&self) -> &UCB0I2CSA {
        &self.ucb0i2csa
    }
    #[doc = "0x2a - USCI B0 Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ucb0ie_i2c(&self) -> &UCB0IE_I2C {
        unsafe { &*(self as *const Self).cast::<u8>().add(42).cast() }
    }
    #[doc = "0x2a - USCI B0 Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ucb0ie(&self) -> &UCB0IE {
        unsafe { &*(self as *const Self).cast::<u8>().add(42).cast() }
    }
    #[doc = "0x2c - USCI B0 Interrupt Flags Register"]
    #[inline(always)]
    pub const fn ucb0ifg_i2c(&self) -> &UCB0IFG_I2C {
        unsafe { &*(self as *const Self).cast::<u8>().add(44).cast() }
    }
    #[doc = "0x2c - USCI B0 Interrupt Flags Register"]
    #[inline(always)]
    pub const fn ucb0ifg(&self) -> &UCB0IFG {
        unsafe { &*(self as *const Self).cast::<u8>().add(44).cast() }
    }
    #[doc = "0x2e - USCI B0 Interrupt Vector Register"]
    #[inline(always)]
    pub const fn ucb0iv(&self) -> &UCB0IV {
        &self.ucb0iv
    }
}
#[doc = "UCB0CTL1 (rw) register accessor: USCI B0 Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ctl1`]
module"]
pub type UCB0CTL1 = crate::Reg<ucb0ctl1::UCB0CTL1_SPEC>;
#[doc = "USCI B0 Control Register 1"]
pub mod ucb0ctl1;
#[doc = "UCB0CTL0 (rw) register accessor: USCI B0 Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ctl0`]
module"]
pub type UCB0CTL0 = crate::Reg<ucb0ctl0::UCB0CTL0_SPEC>;
#[doc = "USCI B0 Control Register 0"]
pub mod ucb0ctl0;
#[doc = "UCB0BR0 (rw) register accessor: USCI B0 Baud Rate 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0br0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0br0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0br0`]
module"]
pub type UCB0BR0 = crate::Reg<ucb0br0::UCB0BR0_SPEC>;
#[doc = "USCI B0 Baud Rate 0"]
pub mod ucb0br0;
#[doc = "UCB0BR1 (rw) register accessor: USCI B0 Baud Rate 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0br1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0br1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0br1`]
module"]
pub type UCB0BR1 = crate::Reg<ucb0br1::UCB0BR1_SPEC>;
#[doc = "USCI B0 Baud Rate 1"]
pub mod ucb0br1;
#[doc = "UCB0STAT_I2C (rw) register accessor: USCI B0 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0stat_i2c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0stat_i2c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0stat_i2c`]
module"]
pub type UCB0STAT_I2C = crate::Reg<ucb0stat_i2c::UCB0STAT_I2C_SPEC>;
#[doc = "USCI B0 Status Register"]
pub mod ucb0stat_i2c;
#[doc = "UCB0BCNT_I2C (rw) register accessor: USCI B0 Byte Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0bcnt_i2c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0bcnt_i2c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0bcnt_i2c`]
module"]
pub type UCB0BCNT_I2C = crate::Reg<ucb0bcnt_i2c::UCB0BCNT_I2C_SPEC>;
#[doc = "USCI B0 Byte Counter Register"]
pub mod ucb0bcnt_i2c;
#[doc = "UCB0CTLW1 (rw) register accessor: USCI B0 Control Word Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0ctlw1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0ctlw1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ctlw1`]
module"]
pub type UCB0CTLW1 = crate::Reg<ucb0ctlw1::UCB0CTLW1_SPEC>;
#[doc = "USCI B0 Control Word Register 1"]
pub mod ucb0ctlw1;
#[doc = "UCB0TBCNT (rw) register accessor: USCI B0 Byte Counter Threshold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0tbcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0tbcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0tbcnt`]
module"]
pub type UCB0TBCNT = crate::Reg<ucb0tbcnt::UCB0TBCNT_SPEC>;
#[doc = "USCI B0 Byte Counter Threshold Register"]
pub mod ucb0tbcnt;
#[doc = "UCB0RXBUF (rw) register accessor: USCI B0 Receive Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0rxbuf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0rxbuf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0rxbuf`]
module"]
pub type UCB0RXBUF = crate::Reg<ucb0rxbuf::UCB0RXBUF_SPEC>;
#[doc = "USCI B0 Receive Buffer"]
pub mod ucb0rxbuf;
#[doc = "UCB0TXBUF (rw) register accessor: USCI B0 Transmit Buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0txbuf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0txbuf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0txbuf`]
module"]
pub type UCB0TXBUF = crate::Reg<ucb0txbuf::UCB0TXBUF_SPEC>;
#[doc = "USCI B0 Transmit Buffer"]
pub mod ucb0txbuf;
#[doc = "UCB0I2COA0 (rw) register accessor: USCI B0 I2C Own Address 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0i2coa0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0i2coa0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0i2coa0`]
module"]
pub type UCB0I2COA0 = crate::Reg<ucb0i2coa0::UCB0I2COA0_SPEC>;
#[doc = "USCI B0 I2C Own Address 0"]
pub mod ucb0i2coa0;
#[doc = "UCB0I2COA1 (rw) register accessor: USCI B0 I2C Own Address 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0i2coa1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0i2coa1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0i2coa1`]
module"]
pub type UCB0I2COA1 = crate::Reg<ucb0i2coa1::UCB0I2COA1_SPEC>;
#[doc = "USCI B0 I2C Own Address 1"]
pub mod ucb0i2coa1;
#[doc = "UCB0I2COA2 (rw) register accessor: USCI B0 I2C Own Address 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0i2coa2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0i2coa2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0i2coa2`]
module"]
pub type UCB0I2COA2 = crate::Reg<ucb0i2coa2::UCB0I2COA2_SPEC>;
#[doc = "USCI B0 I2C Own Address 2"]
pub mod ucb0i2coa2;
#[doc = "UCB0I2COA3 (rw) register accessor: USCI B0 I2C Own Address 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0i2coa3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0i2coa3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0i2coa3`]
module"]
pub type UCB0I2COA3 = crate::Reg<ucb0i2coa3::UCB0I2COA3_SPEC>;
#[doc = "USCI B0 I2C Own Address 3"]
pub mod ucb0i2coa3;
#[doc = "UCB0ADDRX (rw) register accessor: USCI B0 Received Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0addrx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0addrx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0addrx`]
module"]
pub type UCB0ADDRX = crate::Reg<ucb0addrx::UCB0ADDRX_SPEC>;
#[doc = "USCI B0 Received Address Register"]
pub mod ucb0addrx;
#[doc = "UCB0ADDMASK (rw) register accessor: USCI B0 Address Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0addmask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0addmask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0addmask`]
module"]
pub type UCB0ADDMASK = crate::Reg<ucb0addmask::UCB0ADDMASK_SPEC>;
#[doc = "USCI B0 Address Mask Register"]
pub mod ucb0addmask;
#[doc = "UCB0I2CSA (rw) register accessor: USCI B0 I2C Slave Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0i2csa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0i2csa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0i2csa`]
module"]
pub type UCB0I2CSA = crate::Reg<ucb0i2csa::UCB0I2CSA_SPEC>;
#[doc = "USCI B0 I2C Slave Address"]
pub mod ucb0i2csa;
#[doc = "UCB0IE (rw) register accessor: USCI B0 Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0ie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ie`]
module"]
pub type UCB0IE = crate::Reg<ucb0ie::UCB0IE_SPEC>;
#[doc = "USCI B0 Interrupt Enable Register"]
pub mod ucb0ie;
#[doc = "UCB0IE_I2C (rw) register accessor: USCI B0 Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0ie_i2c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0ie_i2c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ie_i2c`]
module"]
pub type UCB0IE_I2C = crate::Reg<ucb0ie_i2c::UCB0IE_I2C_SPEC>;
#[doc = "USCI B0 Interrupt Enable Register"]
pub mod ucb0ie_i2c;
#[doc = "UCB0IFG (rw) register accessor: USCI B0 Interrupt Flags Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0ifg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0ifg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ifg`]
module"]
pub type UCB0IFG = crate::Reg<ucb0ifg::UCB0IFG_SPEC>;
#[doc = "USCI B0 Interrupt Flags Register"]
pub mod ucb0ifg;
#[doc = "UCB0IFG_I2C (rw) register accessor: USCI B0 Interrupt Flags Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0ifg_i2c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0ifg_i2c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0ifg_i2c`]
module"]
pub type UCB0IFG_I2C = crate::Reg<ucb0ifg_i2c::UCB0IFG_I2C_SPEC>;
#[doc = "USCI B0 Interrupt Flags Register"]
pub mod ucb0ifg_i2c;
#[doc = "UCB0IV (rw) register accessor: USCI B0 Interrupt Vector Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0iv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0iv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb0iv`]
module"]
pub type UCB0IV = crate::Reg<ucb0iv::UCB0IV_SPEC>;
#[doc = "USCI B0 Interrupt Vector Register"]
pub mod ucb0iv;
