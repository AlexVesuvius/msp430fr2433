#[doc = "Register `ADCIFG` reader"]
pub struct R(crate::R<ADCIFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCIFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCIFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCIFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCIFG` writer"]
pub struct W(crate::W<ADCIFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCIFG_SPEC>;
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
impl From<crate::W<ADCIFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCIFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCIFG0` reader - ADC Interrupt Flag"]
pub type ADCIFG0_R = crate::BitReader<bool>;
#[doc = "Field `ADCIFG0` writer - ADC Interrupt Flag"]
pub type ADCIFG0_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCIFG_SPEC, bool, O>;
#[doc = "Field `ADCINIFG` reader - ADC Interrupt Flag for the inside of window of the Window comparator"]
pub type ADCINIFG_R = crate::BitReader<bool>;
#[doc = "Field `ADCINIFG` writer - ADC Interrupt Flag for the inside of window of the Window comparator"]
pub type ADCINIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCIFG_SPEC, bool, O>;
#[doc = "Field `ADCLOIFG` reader - ADC Interrupt Flag for lower threshold of the Window comparator"]
pub type ADCLOIFG_R = crate::BitReader<bool>;
#[doc = "Field `ADCLOIFG` writer - ADC Interrupt Flag for lower threshold of the Window comparator"]
pub type ADCLOIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCIFG_SPEC, bool, O>;
#[doc = "Field `ADCHIIFG` reader - ADC Interrupt Flag for upper threshold of the Window comparator"]
pub type ADCHIIFG_R = crate::BitReader<bool>;
#[doc = "Field `ADCHIIFG` writer - ADC Interrupt Flag for upper threshold of the Window comparator"]
pub type ADCHIIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCIFG_SPEC, bool, O>;
#[doc = "Field `ADCOVIFG` reader - ADC ADCMEM overflow Interrupt Flag"]
pub type ADCOVIFG_R = crate::BitReader<bool>;
#[doc = "Field `ADCOVIFG` writer - ADC ADCMEM overflow Interrupt Flag"]
pub type ADCOVIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCIFG_SPEC, bool, O>;
#[doc = "Field `ADCTOVIFG` reader - ADC conversion-time-overflow Interrupt Flag"]
pub type ADCTOVIFG_R = crate::BitReader<bool>;
#[doc = "Field `ADCTOVIFG` writer - ADC conversion-time-overflow Interrupt Flag"]
pub type ADCTOVIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCIFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ADC Interrupt Flag"]
    #[inline(always)]
    pub fn adcifg0(&self) -> ADCIFG0_R {
        ADCIFG0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC Interrupt Flag for the inside of window of the Window comparator"]
    #[inline(always)]
    pub fn adcinifg(&self) -> ADCINIFG_R {
        ADCINIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC Interrupt Flag for lower threshold of the Window comparator"]
    #[inline(always)]
    pub fn adcloifg(&self) -> ADCLOIFG_R {
        ADCLOIFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC Interrupt Flag for upper threshold of the Window comparator"]
    #[inline(always)]
    pub fn adchiifg(&self) -> ADCHIIFG_R {
        ADCHIIFG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC ADCMEM overflow Interrupt Flag"]
    #[inline(always)]
    pub fn adcovifg(&self) -> ADCOVIFG_R {
        ADCOVIFG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC conversion-time-overflow Interrupt Flag"]
    #[inline(always)]
    pub fn adctovifg(&self) -> ADCTOVIFG_R {
        ADCTOVIFG_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Interrupt Flag"]
    #[inline(always)]
    pub fn adcifg0(&mut self) -> ADCIFG0_W<0> {
        ADCIFG0_W::new(self)
    }
    #[doc = "Bit 1 - ADC Interrupt Flag for the inside of window of the Window comparator"]
    #[inline(always)]
    pub fn adcinifg(&mut self) -> ADCINIFG_W<1> {
        ADCINIFG_W::new(self)
    }
    #[doc = "Bit 2 - ADC Interrupt Flag for lower threshold of the Window comparator"]
    #[inline(always)]
    pub fn adcloifg(&mut self) -> ADCLOIFG_W<2> {
        ADCLOIFG_W::new(self)
    }
    #[doc = "Bit 3 - ADC Interrupt Flag for upper threshold of the Window comparator"]
    #[inline(always)]
    pub fn adchiifg(&mut self) -> ADCHIIFG_W<3> {
        ADCHIIFG_W::new(self)
    }
    #[doc = "Bit 4 - ADC ADCMEM overflow Interrupt Flag"]
    #[inline(always)]
    pub fn adcovifg(&mut self) -> ADCOVIFG_W<4> {
        ADCOVIFG_W::new(self)
    }
    #[doc = "Bit 5 - ADC conversion-time-overflow Interrupt Flag"]
    #[inline(always)]
    pub fn adctovifg(&mut self) -> ADCTOVIFG_W<5> {
        ADCTOVIFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcifg](index.html) module"]
pub struct ADCIFG_SPEC;
impl crate::RegisterSpec for ADCIFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adcifg::R](R) reader structure"]
impl crate::Readable for ADCIFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcifg::W](W) writer structure"]
impl crate::Writable for ADCIFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCIFG to value 0"]
impl crate::Resettable for ADCIFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
