#[doc = "Register `UCA0IV` reader"]
pub type R = crate::R<UCA0IV_SPEC>;
#[doc = "Register `UCA0IV` writer"]
pub type W = crate::W<UCA0IV_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<UCA0IV_SPEC> {
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USCI A0 Interrupt Vector Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca0iv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca0iv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCA0IV_SPEC;
impl crate::RegisterSpec for UCA0IV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uca0iv::R`](R) reader structure"]
impl crate::Readable for UCA0IV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uca0iv::W`](W) writer structure"]
impl crate::Writable for UCA0IV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCA0IV to value 0"]
impl crate::Resettable for UCA0IV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
