#[doc = "Register `UCA1IFG_SPI` reader"]
pub type R = crate::R<UCA1IFG_SPI_SPEC>;
#[doc = "Register `UCA1IFG_SPI` writer"]
pub type W = crate::W<UCA1IFG_SPI_SPEC>;
#[doc = "Field `UCRXIFG` reader - SPI Receive Interrupt Flag"]
pub type UCRXIFG_R = crate::BitReader;
#[doc = "Field `UCRXIFG` writer - SPI Receive Interrupt Flag"]
pub type UCRXIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCTXIFG` reader - SPI Transmit Interrupt Flag"]
pub type UCTXIFG_R = crate::BitReader;
#[doc = "Field `UCTXIFG` writer - SPI Transmit Interrupt Flag"]
pub type UCTXIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPI Receive Interrupt Flag"]
    #[inline(always)]
    pub fn ucrxifg(&self) -> UCRXIFG_R {
        UCRXIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI Transmit Interrupt Flag"]
    #[inline(always)]
    pub fn uctxifg(&self) -> UCTXIFG_R {
        UCTXIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI Receive Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ucrxifg(&mut self) -> UCRXIFG_W<UCA1IFG_SPI_SPEC> {
        UCRXIFG_W::new(self, 0)
    }
    #[doc = "Bit 1 - SPI Transmit Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn uctxifg(&mut self) -> UCTXIFG_W<UCA1IFG_SPI_SPEC> {
        UCTXIFG_W::new(self, 1)
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
#[doc = "USCI A1 Interrupt Flags Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca1ifg_spi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca1ifg_spi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCA1IFG_SPI_SPEC;
impl crate::RegisterSpec for UCA1IFG_SPI_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca1ifg_spi::R`](R) reader structure"]
impl crate::Readable for UCA1IFG_SPI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uca1ifg_spi::W`](W) writer structure"]
impl crate::Writable for UCA1IFG_SPI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCA1IFG_SPI to value 0"]
impl crate::Resettable for UCA1IFG_SPI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
