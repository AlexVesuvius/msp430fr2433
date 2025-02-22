#[doc = "Register `UCB0ADDRX` reader"]
pub type R = crate::R<UCB0ADDRX_SPEC>;
#[doc = "Register `UCB0ADDRX` writer"]
pub type W = crate::W<UCB0ADDRX_SPEC>;
#[doc = "Field `UCADDRX0` reader - I2C Receive Address Bit 0"]
pub type UCADDRX0_R = crate::BitReader;
#[doc = "Field `UCADDRX0` writer - I2C Receive Address Bit 0"]
pub type UCADDRX0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCADDRX1` reader - I2C Receive Address Bit 1"]
pub type UCADDRX1_R = crate::BitReader;
#[doc = "Field `UCADDRX1` writer - I2C Receive Address Bit 1"]
pub type UCADDRX1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCADDRX2` reader - I2C Receive Address Bit 2"]
pub type UCADDRX2_R = crate::BitReader;
#[doc = "Field `UCADDRX2` writer - I2C Receive Address Bit 2"]
pub type UCADDRX2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCADDRX3` reader - I2C Receive Address Bit 3"]
pub type UCADDRX3_R = crate::BitReader;
#[doc = "Field `UCADDRX3` writer - I2C Receive Address Bit 3"]
pub type UCADDRX3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCADDRX4` reader - I2C Receive Address Bit 4"]
pub type UCADDRX4_R = crate::BitReader;
#[doc = "Field `UCADDRX4` writer - I2C Receive Address Bit 4"]
pub type UCADDRX4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCADDRX5` reader - I2C Receive Address Bit 5"]
pub type UCADDRX5_R = crate::BitReader;
#[doc = "Field `UCADDRX5` writer - I2C Receive Address Bit 5"]
pub type UCADDRX5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCADDRX6` reader - I2C Receive Address Bit 6"]
pub type UCADDRX6_R = crate::BitReader;
#[doc = "Field `UCADDRX6` writer - I2C Receive Address Bit 6"]
pub type UCADDRX6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCADDRX7` reader - I2C Receive Address Bit 7"]
pub type UCADDRX7_R = crate::BitReader;
#[doc = "Field `UCADDRX7` writer - I2C Receive Address Bit 7"]
pub type UCADDRX7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCADDRX8` reader - I2C Receive Address Bit 8"]
pub type UCADDRX8_R = crate::BitReader;
#[doc = "Field `UCADDRX8` writer - I2C Receive Address Bit 8"]
pub type UCADDRX8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCADDRX9` reader - I2C Receive Address Bit 9"]
pub type UCADDRX9_R = crate::BitReader;
#[doc = "Field `UCADDRX9` writer - I2C Receive Address Bit 9"]
pub type UCADDRX9_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C Receive Address Bit 0"]
    #[inline(always)]
    pub fn ucaddrx0(&self) -> UCADDRX0_R {
        UCADDRX0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C Receive Address Bit 1"]
    #[inline(always)]
    pub fn ucaddrx1(&self) -> UCADDRX1_R {
        UCADDRX1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C Receive Address Bit 2"]
    #[inline(always)]
    pub fn ucaddrx2(&self) -> UCADDRX2_R {
        UCADDRX2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C Receive Address Bit 3"]
    #[inline(always)]
    pub fn ucaddrx3(&self) -> UCADDRX3_R {
        UCADDRX3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I2C Receive Address Bit 4"]
    #[inline(always)]
    pub fn ucaddrx4(&self) -> UCADDRX4_R {
        UCADDRX4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C Receive Address Bit 5"]
    #[inline(always)]
    pub fn ucaddrx5(&self) -> UCADDRX5_R {
        UCADDRX5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C Receive Address Bit 6"]
    #[inline(always)]
    pub fn ucaddrx6(&self) -> UCADDRX6_R {
        UCADDRX6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C Receive Address Bit 7"]
    #[inline(always)]
    pub fn ucaddrx7(&self) -> UCADDRX7_R {
        UCADDRX7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C Receive Address Bit 8"]
    #[inline(always)]
    pub fn ucaddrx8(&self) -> UCADDRX8_R {
        UCADDRX8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2C Receive Address Bit 9"]
    #[inline(always)]
    pub fn ucaddrx9(&self) -> UCADDRX9_R {
        UCADDRX9_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Receive Address Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ucaddrx0(&mut self) -> UCADDRX0_W<UCB0ADDRX_SPEC> {
        UCADDRX0_W::new(self, 0)
    }
    #[doc = "Bit 1 - I2C Receive Address Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ucaddrx1(&mut self) -> UCADDRX1_W<UCB0ADDRX_SPEC> {
        UCADDRX1_W::new(self, 1)
    }
    #[doc = "Bit 2 - I2C Receive Address Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ucaddrx2(&mut self) -> UCADDRX2_W<UCB0ADDRX_SPEC> {
        UCADDRX2_W::new(self, 2)
    }
    #[doc = "Bit 3 - I2C Receive Address Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn ucaddrx3(&mut self) -> UCADDRX3_W<UCB0ADDRX_SPEC> {
        UCADDRX3_W::new(self, 3)
    }
    #[doc = "Bit 4 - I2C Receive Address Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn ucaddrx4(&mut self) -> UCADDRX4_W<UCB0ADDRX_SPEC> {
        UCADDRX4_W::new(self, 4)
    }
    #[doc = "Bit 5 - I2C Receive Address Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn ucaddrx5(&mut self) -> UCADDRX5_W<UCB0ADDRX_SPEC> {
        UCADDRX5_W::new(self, 5)
    }
    #[doc = "Bit 6 - I2C Receive Address Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn ucaddrx6(&mut self) -> UCADDRX6_W<UCB0ADDRX_SPEC> {
        UCADDRX6_W::new(self, 6)
    }
    #[doc = "Bit 7 - I2C Receive Address Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn ucaddrx7(&mut self) -> UCADDRX7_W<UCB0ADDRX_SPEC> {
        UCADDRX7_W::new(self, 7)
    }
    #[doc = "Bit 8 - I2C Receive Address Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn ucaddrx8(&mut self) -> UCADDRX8_W<UCB0ADDRX_SPEC> {
        UCADDRX8_W::new(self, 8)
    }
    #[doc = "Bit 9 - I2C Receive Address Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn ucaddrx9(&mut self) -> UCADDRX9_W<UCB0ADDRX_SPEC> {
        UCADDRX9_W::new(self, 9)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USCI B0 Received Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0addrx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0addrx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCB0ADDRX_SPEC;
impl crate::RegisterSpec for UCB0ADDRX_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb0addrx::R`](R) reader structure"]
impl crate::Readable for UCB0ADDRX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucb0addrx::W`](W) writer structure"]
impl crate::Writable for UCB0ADDRX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCB0ADDRX to value 0"]
impl crate::Resettable for UCB0ADDRX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
