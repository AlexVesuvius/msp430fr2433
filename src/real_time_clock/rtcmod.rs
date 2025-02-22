#[doc = "Register `RTCMOD` reader"]
pub type R = crate::R<RTCMOD_SPEC>;
#[doc = "Register `RTCMOD` writer"]
pub type W = crate::W<RTCMOD_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RTCMOD_SPEC> {
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
#[doc = "RTC moduloRegister\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcmod::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcmod::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTCMOD_SPEC;
impl crate::RegisterSpec for RTCMOD_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtcmod::R`](R) reader structure"]
impl crate::Readable for RTCMOD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtcmod::W`](W) writer structure"]
impl crate::Writable for RTCMOD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCMOD to value 0"]
impl crate::Resettable for RTCMOD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
