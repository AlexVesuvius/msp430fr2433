#[doc = "Register `UCB0STAT_I2C` reader"]
pub type R = crate::R<UCB0STAT_I2C_SPEC>;
#[doc = "Register `UCB0STAT_I2C` writer"]
pub type W = crate::W<UCB0STAT_I2C_SPEC>;
#[doc = "Field `UCBBUSY` reader - Bus Busy Flag"]
pub type UCBBUSY_R = crate::BitReader;
#[doc = "Field `UCBBUSY` writer - Bus Busy Flag"]
pub type UCBBUSY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCGC` reader - General Call address received Flag"]
pub type UCGC_R = crate::BitReader;
#[doc = "Field `UCGC` writer - General Call address received Flag"]
pub type UCGC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSCLLOW` reader - SCL low"]
pub type UCSCLLOW_R = crate::BitReader;
#[doc = "Field `UCSCLLOW` writer - SCL low"]
pub type UCSCLLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - Bus Busy Flag"]
    #[inline(always)]
    pub fn ucbbusy(&self) -> UCBBUSY_R {
        UCBBUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - General Call address received Flag"]
    #[inline(always)]
    pub fn ucgc(&self) -> UCGC_R {
        UCGC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCL low"]
    #[inline(always)]
    pub fn ucscllow(&self) -> UCSCLLOW_R {
        UCSCLLOW_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Bus Busy Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ucbbusy(&mut self) -> UCBBUSY_W<UCB0STAT_I2C_SPEC> {
        UCBBUSY_W::new(self, 4)
    }
    #[doc = "Bit 5 - General Call address received Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ucgc(&mut self) -> UCGC_W<UCB0STAT_I2C_SPEC> {
        UCGC_W::new(self, 5)
    }
    #[doc = "Bit 6 - SCL low"]
    #[inline(always)]
    #[must_use]
    pub fn ucscllow(&mut self) -> UCSCLLOW_W<UCB0STAT_I2C_SPEC> {
        UCSCLLOW_W::new(self, 6)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USCI B0 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0stat_i2c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0stat_i2c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCB0STAT_I2C_SPEC;
impl crate::RegisterSpec for UCB0STAT_I2C_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucb0stat_i2c::R`](R) reader structure"]
impl crate::Readable for UCB0STAT_I2C_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucb0stat_i2c::W`](W) writer structure"]
impl crate::Writable for UCB0STAT_I2C_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCB0STAT_I2C to value 0"]
impl crate::Resettable for UCB0STAT_I2C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
