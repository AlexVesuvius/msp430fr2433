#[doc = "Register `ADCIV` reader"]
pub type R = crate::R<ADCIV_SPEC>;
#[doc = "Register `ADCIV` writer"]
pub type W = crate::W<ADCIV_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<ADCIV_SPEC> {
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
#[doc = "ADC Interrupt Vector Word\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adciv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adciv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCIV_SPEC;
impl crate::RegisterSpec for ADCIV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adciv::R`](R) reader structure"]
impl crate::Readable for ADCIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adciv::W`](W) writer structure"]
impl crate::Writable for ADCIV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCIV to value 0"]
impl crate::Resettable for ADCIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
