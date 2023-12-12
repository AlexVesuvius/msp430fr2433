#[doc = "Register `MAC32L` reader"]
pub type R = crate::R<MAC32L_SPEC>;
#[doc = "Register `MAC32L` writer"]
pub type W = crate::W<MAC32L_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<MAC32L_SPEC> {
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
#[doc = "32-bit operand 1 - multiply accumulate - low word\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac32l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac32l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAC32L_SPEC;
impl crate::RegisterSpec for MAC32L_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mac32l::R`](R) reader structure"]
impl crate::Readable for MAC32L_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mac32l::W`](W) writer structure"]
impl crate::Writable for MAC32L_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAC32L to value 0"]
impl crate::Resettable for MAC32L_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
