#[doc = "Register `ADCHI` reader"]
pub type R = crate::R<ADCHI_SPEC>;
#[doc = "Register `ADCHI` writer"]
pub type W = crate::W<ADCHI_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<ADCHI_SPEC> {
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
#[doc = "ADC Window Comparator High Threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adchi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adchi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCHI_SPEC;
impl crate::RegisterSpec for ADCHI_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adchi::R`](R) reader structure"]
impl crate::Readable for ADCHI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adchi::W`](W) writer structure"]
impl crate::Writable for ADCHI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCHI to value 0"]
impl crate::Resettable for ADCHI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
