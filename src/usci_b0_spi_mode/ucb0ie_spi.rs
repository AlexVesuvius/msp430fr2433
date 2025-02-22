#[doc = "Register `UCB0IE_SPI` reader"]
pub type R = crate::R<UCB0IE_SPI_SPEC>;
#[doc = "Register `UCB0IE_SPI` writer"]
pub type W = crate::W<UCB0IE_SPI_SPEC>;
#[doc = "Field `UCRXIE` reader - USCI Receive Interrupt Enable"]
pub type UCRXIE_R = crate::BitReader;
#[doc = "Field `UCRXIE` writer - USCI Receive Interrupt Enable"]
pub type UCRXIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCTXIE` reader - USCI Transmit Interrupt Enable"]
pub type UCTXIE_R = crate::BitReader;
#[doc = "Field `UCTXIE` writer - USCI Transmit Interrupt Enable"]
pub type UCTXIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USCI Receive Interrupt Enable"]
    #[inline(always)]
    pub fn ucrxie(&self) -> UCRXIE_R {
        UCRXIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USCI Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn uctxie(&self) -> UCTXIE_R {
        UCTXIE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USCI Receive Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ucrxie(&mut self) -> UCRXIE_W<UCB0IE_SPI_SPEC> {
        UCRXIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - USCI Transmit Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uctxie(&mut self) -> UCTXIE_W<UCB0IE_SPI_SPEC> {
        UCTXIE_W::new(self, 1)
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
#[doc = "USCI B0 Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0ie_spi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0ie_spi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCB0IE_SPI_SPEC;
impl crate::RegisterSpec for UCB0IE_SPI_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb0ie_spi::R`](R) reader structure"]
impl crate::Readable for UCB0IE_SPI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucb0ie_spi::W`](W) writer structure"]
impl crate::Writable for UCB0IE_SPI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCB0IE_SPI to value 0"]
impl crate::Resettable for UCB0IE_SPI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
