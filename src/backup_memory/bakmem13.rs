#[doc = "Register `BAKMEM13` reader"]
pub type R = crate::R<BAKMEM13_SPEC>;
#[doc = "Register `BAKMEM13` writer"]
pub type W = crate::W<BAKMEM13_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<BAKMEM13_SPEC> {
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
#[doc = "Battery Backup Memory 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bakmem13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bakmem13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BAKMEM13_SPEC;
impl crate::RegisterSpec for BAKMEM13_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`bakmem13::R`](R) reader structure"]
impl crate::Readable for BAKMEM13_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bakmem13::W`](W) writer structure"]
impl crate::Writable for BAKMEM13_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BAKMEM13 to value 0"]
impl crate::Resettable for BAKMEM13_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
