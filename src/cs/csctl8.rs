#[doc = "Register `CSCTL8` reader"]
pub struct R(crate::R<CSCTL8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSCTL8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSCTL8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSCTL8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSCTL8` writer"]
pub struct W(crate::W<CSCTL8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSCTL8_SPEC>;
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
impl From<crate::W<CSCTL8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSCTL8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACLKREQEN` reader - ACLK Clock Request Enable"]
pub type ACLKREQEN_R = crate::BitReader<bool>;
#[doc = "Field `ACLKREQEN` writer - ACLK Clock Request Enable"]
pub type ACLKREQEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL8_SPEC, bool, O>;
#[doc = "Field `MCLKREQEN` reader - MCLK Clock Request Enable"]
pub type MCLKREQEN_R = crate::BitReader<bool>;
#[doc = "Field `MCLKREQEN` writer - MCLK Clock Request Enable"]
pub type MCLKREQEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL8_SPEC, bool, O>;
#[doc = "Field `SMCLKREQEN` reader - SMCLK Clock Request Enable"]
pub type SMCLKREQEN_R = crate::BitReader<bool>;
#[doc = "Field `SMCLKREQEN` writer - SMCLK Clock Request Enable"]
pub type SMCLKREQEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL8_SPEC, bool, O>;
#[doc = "Field `MODOSCREQEN` reader - MODOSC Clock Request Enable"]
pub type MODOSCREQEN_R = crate::BitReader<bool>;
#[doc = "Field `MODOSCREQEN` writer - MODOSC Clock Request Enable"]
pub type MODOSCREQEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL8_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ACLK Clock Request Enable"]
    #[inline(always)]
    pub fn aclkreqen(&self) -> ACLKREQEN_R {
        ACLKREQEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MCLK Clock Request Enable"]
    #[inline(always)]
    pub fn mclkreqen(&self) -> MCLKREQEN_R {
        MCLKREQEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SMCLK Clock Request Enable"]
    #[inline(always)]
    pub fn smclkreqen(&self) -> SMCLKREQEN_R {
        SMCLKREQEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MODOSC Clock Request Enable"]
    #[inline(always)]
    pub fn modoscreqen(&self) -> MODOSCREQEN_R {
        MODOSCREQEN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ACLK Clock Request Enable"]
    #[inline(always)]
    pub fn aclkreqen(&mut self) -> ACLKREQEN_W<0> {
        ACLKREQEN_W::new(self)
    }
    #[doc = "Bit 1 - MCLK Clock Request Enable"]
    #[inline(always)]
    pub fn mclkreqen(&mut self) -> MCLKREQEN_W<1> {
        MCLKREQEN_W::new(self)
    }
    #[doc = "Bit 2 - SMCLK Clock Request Enable"]
    #[inline(always)]
    pub fn smclkreqen(&mut self) -> SMCLKREQEN_W<2> {
        SMCLKREQEN_W::new(self)
    }
    #[doc = "Bit 3 - MODOSC Clock Request Enable"]
    #[inline(always)]
    pub fn modoscreqen(&mut self) -> MODOSCREQEN_W<3> {
        MODOSCREQEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CS Control Register 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csctl8](index.html) module"]
pub struct CSCTL8_SPEC;
impl crate::RegisterSpec for CSCTL8_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [csctl8::R](R) reader structure"]
impl crate::Readable for CSCTL8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csctl8::W](W) writer structure"]
impl crate::Writable for CSCTL8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSCTL8 to value 0"]
impl crate::Resettable for CSCTL8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
