#[doc = "Register `UCB0IE_I2C` reader"]
pub type R = crate::R<UCB0IE_I2C_SPEC>;
#[doc = "Register `UCB0IE_I2C` writer"]
pub type W = crate::W<UCB0IE_I2C_SPEC>;
#[doc = "Field `UCRXIE0` reader - I2C Receive Interrupt Enable 0"]
pub type UCRXIE0_R = crate::BitReader;
#[doc = "Field `UCRXIE0` writer - I2C Receive Interrupt Enable 0"]
pub type UCRXIE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCTXIE0` reader - I2C Transmit Interrupt Enable 0"]
pub type UCTXIE0_R = crate::BitReader;
#[doc = "Field `UCTXIE0` writer - I2C Transmit Interrupt Enable 0"]
pub type UCTXIE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSTTIE` reader - I2C START Condition interrupt enable"]
pub type UCSTTIE_R = crate::BitReader;
#[doc = "Field `UCSTTIE` writer - I2C START Condition interrupt enable"]
pub type UCSTTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSTPIE` reader - I2C STOP Condition interrupt enable"]
pub type UCSTPIE_R = crate::BitReader;
#[doc = "Field `UCSTPIE` writer - I2C STOP Condition interrupt enable"]
pub type UCSTPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCALIE` reader - I2C Arbitration Lost interrupt enable"]
pub type UCALIE_R = crate::BitReader;
#[doc = "Field `UCALIE` writer - I2C Arbitration Lost interrupt enable"]
pub type UCALIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCNACKIE` reader - I2C NACK Condition interrupt enable"]
pub type UCNACKIE_R = crate::BitReader;
#[doc = "Field `UCNACKIE` writer - I2C NACK Condition interrupt enable"]
pub type UCNACKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCBCNTIE` reader - I2C Automatic stop assertion interrupt enable"]
pub type UCBCNTIE_R = crate::BitReader;
#[doc = "Field `UCBCNTIE` writer - I2C Automatic stop assertion interrupt enable"]
pub type UCBCNTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCCLTOIE` reader - I2C Clock Low Timeout interrupt enable"]
pub type UCCLTOIE_R = crate::BitReader;
#[doc = "Field `UCCLTOIE` writer - I2C Clock Low Timeout interrupt enable"]
pub type UCCLTOIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCRXIE1` reader - I2C Receive Interrupt Enable 1"]
pub type UCRXIE1_R = crate::BitReader;
#[doc = "Field `UCRXIE1` writer - I2C Receive Interrupt Enable 1"]
pub type UCRXIE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCTXIE1` reader - I2C Transmit Interrupt Enable 1"]
pub type UCTXIE1_R = crate::BitReader;
#[doc = "Field `UCTXIE1` writer - I2C Transmit Interrupt Enable 1"]
pub type UCTXIE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCRXIE2` reader - I2C Receive Interrupt Enable 2"]
pub type UCRXIE2_R = crate::BitReader;
#[doc = "Field `UCRXIE2` writer - I2C Receive Interrupt Enable 2"]
pub type UCRXIE2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCTXIE2` reader - I2C Transmit Interrupt Enable 2"]
pub type UCTXIE2_R = crate::BitReader;
#[doc = "Field `UCTXIE2` writer - I2C Transmit Interrupt Enable 2"]
pub type UCTXIE2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCRXIE3` reader - I2C Receive Interrupt Enable 3"]
pub type UCRXIE3_R = crate::BitReader;
#[doc = "Field `UCRXIE3` writer - I2C Receive Interrupt Enable 3"]
pub type UCRXIE3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCTXIE3` reader - I2C Transmit Interrupt Enable 3"]
pub type UCTXIE3_R = crate::BitReader;
#[doc = "Field `UCTXIE3` writer - I2C Transmit Interrupt Enable 3"]
pub type UCTXIE3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCBIT9IE` reader - I2C Bit 9 Position Interrupt Enable 3"]
pub type UCBIT9IE_R = crate::BitReader;
#[doc = "Field `UCBIT9IE` writer - I2C Bit 9 Position Interrupt Enable 3"]
pub type UCBIT9IE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C Receive Interrupt Enable 0"]
    #[inline(always)]
    pub fn ucrxie0(&self) -> UCRXIE0_R {
        UCRXIE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C Transmit Interrupt Enable 0"]
    #[inline(always)]
    pub fn uctxie0(&self) -> UCTXIE0_R {
        UCTXIE0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C START Condition interrupt enable"]
    #[inline(always)]
    pub fn ucsttie(&self) -> UCSTTIE_R {
        UCSTTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C STOP Condition interrupt enable"]
    #[inline(always)]
    pub fn ucstpie(&self) -> UCSTPIE_R {
        UCSTPIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I2C Arbitration Lost interrupt enable"]
    #[inline(always)]
    pub fn ucalie(&self) -> UCALIE_R {
        UCALIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C NACK Condition interrupt enable"]
    #[inline(always)]
    pub fn ucnackie(&self) -> UCNACKIE_R {
        UCNACKIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C Automatic stop assertion interrupt enable"]
    #[inline(always)]
    pub fn ucbcntie(&self) -> UCBCNTIE_R {
        UCBCNTIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C Clock Low Timeout interrupt enable"]
    #[inline(always)]
    pub fn uccltoie(&self) -> UCCLTOIE_R {
        UCCLTOIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C Receive Interrupt Enable 1"]
    #[inline(always)]
    pub fn ucrxie1(&self) -> UCRXIE1_R {
        UCRXIE1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2C Transmit Interrupt Enable 1"]
    #[inline(always)]
    pub fn uctxie1(&self) -> UCTXIE1_R {
        UCTXIE1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - I2C Receive Interrupt Enable 2"]
    #[inline(always)]
    pub fn ucrxie2(&self) -> UCRXIE2_R {
        UCRXIE2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - I2C Transmit Interrupt Enable 2"]
    #[inline(always)]
    pub fn uctxie2(&self) -> UCTXIE2_R {
        UCTXIE2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - I2C Receive Interrupt Enable 3"]
    #[inline(always)]
    pub fn ucrxie3(&self) -> UCRXIE3_R {
        UCRXIE3_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - I2C Transmit Interrupt Enable 3"]
    #[inline(always)]
    pub fn uctxie3(&self) -> UCTXIE3_R {
        UCTXIE3_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - I2C Bit 9 Position Interrupt Enable 3"]
    #[inline(always)]
    pub fn ucbit9ie(&self) -> UCBIT9IE_R {
        UCBIT9IE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Receive Interrupt Enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn ucrxie0(&mut self) -> UCRXIE0_W<UCB0IE_I2C_SPEC> {
        UCRXIE0_W::new(self, 0)
    }
    #[doc = "Bit 1 - I2C Transmit Interrupt Enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn uctxie0(&mut self) -> UCTXIE0_W<UCB0IE_I2C_SPEC> {
        UCTXIE0_W::new(self, 1)
    }
    #[doc = "Bit 2 - I2C START Condition interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ucsttie(&mut self) -> UCSTTIE_W<UCB0IE_I2C_SPEC> {
        UCSTTIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - I2C STOP Condition interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ucstpie(&mut self) -> UCSTPIE_W<UCB0IE_I2C_SPEC> {
        UCSTPIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - I2C Arbitration Lost interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ucalie(&mut self) -> UCALIE_W<UCB0IE_I2C_SPEC> {
        UCALIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - I2C NACK Condition interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ucnackie(&mut self) -> UCNACKIE_W<UCB0IE_I2C_SPEC> {
        UCNACKIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - I2C Automatic stop assertion interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ucbcntie(&mut self) -> UCBCNTIE_W<UCB0IE_I2C_SPEC> {
        UCBCNTIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - I2C Clock Low Timeout interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn uccltoie(&mut self) -> UCCLTOIE_W<UCB0IE_I2C_SPEC> {
        UCCLTOIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - I2C Receive Interrupt Enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn ucrxie1(&mut self) -> UCRXIE1_W<UCB0IE_I2C_SPEC> {
        UCRXIE1_W::new(self, 8)
    }
    #[doc = "Bit 9 - I2C Transmit Interrupt Enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn uctxie1(&mut self) -> UCTXIE1_W<UCB0IE_I2C_SPEC> {
        UCTXIE1_W::new(self, 9)
    }
    #[doc = "Bit 10 - I2C Receive Interrupt Enable 2"]
    #[inline(always)]
    #[must_use]
    pub fn ucrxie2(&mut self) -> UCRXIE2_W<UCB0IE_I2C_SPEC> {
        UCRXIE2_W::new(self, 10)
    }
    #[doc = "Bit 11 - I2C Transmit Interrupt Enable 2"]
    #[inline(always)]
    #[must_use]
    pub fn uctxie2(&mut self) -> UCTXIE2_W<UCB0IE_I2C_SPEC> {
        UCTXIE2_W::new(self, 11)
    }
    #[doc = "Bit 12 - I2C Receive Interrupt Enable 3"]
    #[inline(always)]
    #[must_use]
    pub fn ucrxie3(&mut self) -> UCRXIE3_W<UCB0IE_I2C_SPEC> {
        UCRXIE3_W::new(self, 12)
    }
    #[doc = "Bit 13 - I2C Transmit Interrupt Enable 3"]
    #[inline(always)]
    #[must_use]
    pub fn uctxie3(&mut self) -> UCTXIE3_W<UCB0IE_I2C_SPEC> {
        UCTXIE3_W::new(self, 13)
    }
    #[doc = "Bit 14 - I2C Bit 9 Position Interrupt Enable 3"]
    #[inline(always)]
    #[must_use]
    pub fn ucbit9ie(&mut self) -> UCBIT9IE_W<UCB0IE_I2C_SPEC> {
        UCBIT9IE_W::new(self, 14)
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
#[doc = "USCI B0 Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0ie_i2c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0ie_i2c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCB0IE_I2C_SPEC;
impl crate::RegisterSpec for UCB0IE_I2C_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb0ie_i2c::R`](R) reader structure"]
impl crate::Readable for UCB0IE_I2C_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucb0ie_i2c::W`](W) writer structure"]
impl crate::Writable for UCB0IE_I2C_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCB0IE_I2C to value 0"]
impl crate::Resettable for UCB0IE_I2C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
