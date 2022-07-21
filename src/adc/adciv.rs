#[doc = "Register `ADCIV` reader"]
pub struct R(crate::R<ADCIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCIV` writer"]
pub struct W(crate::W<ADCIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ADCIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCIV_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Interrupt Vector Word\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adciv](index.html) module"]
pub struct ADCIV_SPEC;
impl crate::RegisterSpec for ADCIV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adciv::R](R) reader structure"]
impl crate::Readable for ADCIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adciv::W](W) writer structure"]
impl crate::Writable for ADCIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCIV to value 0"]
impl crate::Resettable for ADCIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
