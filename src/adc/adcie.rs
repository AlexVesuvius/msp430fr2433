#[doc = "Register `ADCIE` reader"]
pub struct R(crate::R<ADCIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCIE` writer"]
pub struct W(crate::W<ADCIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCIE_SPEC>;
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
impl From<crate::W<ADCIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCIE0` reader - ADC Interrupt enable"]
pub type ADCIE0_R = crate::BitReader<bool>;
#[doc = "Field `ADCIE0` writer - ADC Interrupt enable"]
pub type ADCIE0_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCIE_SPEC, bool, O>;
#[doc = "Field `ADCINIE` reader - ADC Interrupt enable for the inside of window of the Window comparator"]
pub type ADCINIE_R = crate::BitReader<bool>;
#[doc = "Field `ADCINIE` writer - ADC Interrupt enable for the inside of window of the Window comparator"]
pub type ADCINIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCIE_SPEC, bool, O>;
#[doc = "Field `ADCLOIE` reader - ADC Interrupt enable for lower threshold of the Window comparator"]
pub type ADCLOIE_R = crate::BitReader<bool>;
#[doc = "Field `ADCLOIE` writer - ADC Interrupt enable for lower threshold of the Window comparator"]
pub type ADCLOIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCIE_SPEC, bool, O>;
#[doc = "Field `ADCHIIE` reader - ADC Interrupt enable for upper threshold of the Window comparator"]
pub type ADCHIIE_R = crate::BitReader<bool>;
#[doc = "Field `ADCHIIE` writer - ADC Interrupt enable for upper threshold of the Window comparator"]
pub type ADCHIIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCIE_SPEC, bool, O>;
#[doc = "Field `ADCOVIE` reader - ADC ADCMEM overflow Interrupt enable"]
pub type ADCOVIE_R = crate::BitReader<bool>;
#[doc = "Field `ADCOVIE` writer - ADC ADCMEM overflow Interrupt enable"]
pub type ADCOVIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCIE_SPEC, bool, O>;
#[doc = "Field `ADCTOVIE` reader - ADC conversion-time-overflow Interrupt enable"]
pub type ADCTOVIE_R = crate::BitReader<bool>;
#[doc = "Field `ADCTOVIE` writer - ADC conversion-time-overflow Interrupt enable"]
pub type ADCTOVIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCIE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ADC Interrupt enable"]
    #[inline(always)]
    pub fn adcie0(&self) -> ADCIE0_R {
        ADCIE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC Interrupt enable for the inside of window of the Window comparator"]
    #[inline(always)]
    pub fn adcinie(&self) -> ADCINIE_R {
        ADCINIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC Interrupt enable for lower threshold of the Window comparator"]
    #[inline(always)]
    pub fn adcloie(&self) -> ADCLOIE_R {
        ADCLOIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC Interrupt enable for upper threshold of the Window comparator"]
    #[inline(always)]
    pub fn adchiie(&self) -> ADCHIIE_R {
        ADCHIIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC ADCMEM overflow Interrupt enable"]
    #[inline(always)]
    pub fn adcovie(&self) -> ADCOVIE_R {
        ADCOVIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC conversion-time-overflow Interrupt enable"]
    #[inline(always)]
    pub fn adctovie(&self) -> ADCTOVIE_R {
        ADCTOVIE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Interrupt enable"]
    #[inline(always)]
    pub fn adcie0(&mut self) -> ADCIE0_W<0> {
        ADCIE0_W::new(self)
    }
    #[doc = "Bit 1 - ADC Interrupt enable for the inside of window of the Window comparator"]
    #[inline(always)]
    pub fn adcinie(&mut self) -> ADCINIE_W<1> {
        ADCINIE_W::new(self)
    }
    #[doc = "Bit 2 - ADC Interrupt enable for lower threshold of the Window comparator"]
    #[inline(always)]
    pub fn adcloie(&mut self) -> ADCLOIE_W<2> {
        ADCLOIE_W::new(self)
    }
    #[doc = "Bit 3 - ADC Interrupt enable for upper threshold of the Window comparator"]
    #[inline(always)]
    pub fn adchiie(&mut self) -> ADCHIIE_W<3> {
        ADCHIIE_W::new(self)
    }
    #[doc = "Bit 4 - ADC ADCMEM overflow Interrupt enable"]
    #[inline(always)]
    pub fn adcovie(&mut self) -> ADCOVIE_W<4> {
        ADCOVIE_W::new(self)
    }
    #[doc = "Bit 5 - ADC conversion-time-overflow Interrupt enable"]
    #[inline(always)]
    pub fn adctovie(&mut self) -> ADCTOVIE_W<5> {
        ADCTOVIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcie](index.html) module"]
pub struct ADCIE_SPEC;
impl crate::RegisterSpec for ADCIE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adcie::R](R) reader structure"]
impl crate::Readable for ADCIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcie::W](W) writer structure"]
impl crate::Writable for ADCIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCIE to value 0"]
impl crate::Resettable for ADCIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
