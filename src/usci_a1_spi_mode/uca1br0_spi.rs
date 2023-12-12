#[doc = "Register `UCA1BR0_SPI` reader"]
pub type R = crate::R<UCA1BR0_SPI_SPEC>;
#[doc = "Register `UCA1BR0_SPI` writer"]
pub type W = crate::W<UCA1BR0_SPI_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<UCA1BR0_SPI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
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
#[doc = "USCI A1 Baud Rate 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca1br0_spi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca1br0_spi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCA1BR0_SPI_SPEC;
impl crate::RegisterSpec for UCA1BR0_SPI_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uca1br0_spi::R`](R) reader structure"]
impl crate::Readable for UCA1BR0_SPI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uca1br0_spi::W`](W) writer structure"]
impl crate::Writable for UCA1BR0_SPI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCA1BR0_SPI to value 0"]
impl crate::Resettable for UCA1BR0_SPI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
