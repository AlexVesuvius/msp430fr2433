#[doc = "Register `GCCTL1` reader"]
pub struct R(crate::R<GCCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCCTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GCCTL1` writer"]
pub struct W(crate::W<GCCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCCTL1_SPEC>;
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
impl From<crate::W<GCCTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CBDIFG` reader - FRAM correctable bit error flag"]
pub type CBDIFG_R = crate::BitReader<bool>;
#[doc = "Field `CBDIFG` writer - FRAM correctable bit error flag"]
pub type CBDIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, GCCTL1_SPEC, bool, O>;
#[doc = "Field `UBDIFG` reader - FRAM uncorrectable bit error flag"]
pub type UBDIFG_R = crate::BitReader<bool>;
#[doc = "Field `UBDIFG` writer - FRAM uncorrectable bit error flag"]
pub type UBDIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, GCCTL1_SPEC, bool, O>;
#[doc = "Field `ACCTEIFG` reader - Access time error flag"]
pub type ACCTEIFG_R = crate::BitReader<bool>;
#[doc = "Field `ACCTEIFG` writer - Access time error flag"]
pub type ACCTEIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, GCCTL1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - FRAM correctable bit error flag"]
    #[inline(always)]
    pub fn cbdifg(&self) -> CBDIFG_R {
        CBDIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FRAM uncorrectable bit error flag"]
    #[inline(always)]
    pub fn ubdifg(&self) -> UBDIFG_R {
        UBDIFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Access time error flag"]
    #[inline(always)]
    pub fn accteifg(&self) -> ACCTEIFG_R {
        ACCTEIFG_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - FRAM correctable bit error flag"]
    #[inline(always)]
    pub fn cbdifg(&mut self) -> CBDIFG_W<1> {
        CBDIFG_W::new(self)
    }
    #[doc = "Bit 2 - FRAM uncorrectable bit error flag"]
    #[inline(always)]
    pub fn ubdifg(&mut self) -> UBDIFG_W<2> {
        UBDIFG_W::new(self)
    }
    #[doc = "Bit 3 - Access time error flag"]
    #[inline(always)]
    pub fn accteifg(&mut self) -> ACCTEIFG_W<3> {
        ACCTEIFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcctl1](index.html) module"]
pub struct GCCTL1_SPEC;
impl crate::RegisterSpec for GCCTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [gcctl1::R](R) reader structure"]
impl crate::Readable for GCCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gcctl1::W](W) writer structure"]
impl crate::Writable for GCCTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GCCTL1 to value 0"]
impl crate::Resettable for GCCTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
