#[doc = "Register `SYSCFG0` reader"]
pub struct R(crate::R<SYSCFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCFG0` writer"]
pub struct W(crate::W<SYSCFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCFG0_SPEC>;
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
impl From<crate::W<SYSCFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PFWP` reader - Program FRAM Write Protection"]
pub type PFWP_R = crate::BitReader<bool>;
#[doc = "Field `PFWP` writer - Program FRAM Write Protection"]
pub type PFWP_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG0_SPEC, bool, O>;
#[doc = "Field `DFWP` reader - Data FRAM Write Protection"]
pub type DFWP_R = crate::BitReader<bool>;
#[doc = "Field `DFWP` writer - Data FRAM Write Protection"]
pub type DFWP_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Program FRAM Write Protection"]
    #[inline(always)]
    pub fn pfwp(&self) -> PFWP_R {
        PFWP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data FRAM Write Protection"]
    #[inline(always)]
    pub fn dfwp(&self) -> DFWP_R {
        DFWP_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Program FRAM Write Protection"]
    #[inline(always)]
    pub fn pfwp(&mut self) -> PFWP_W<0> {
        PFWP_W::new(self)
    }
    #[doc = "Bit 1 - Data FRAM Write Protection"]
    #[inline(always)]
    pub fn dfwp(&mut self) -> DFWP_W<1> {
        DFWP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Configuration 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg0](index.html) module"]
pub struct SYSCFG0_SPEC;
impl crate::RegisterSpec for SYSCFG0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [syscfg0::R](R) reader structure"]
impl crate::Readable for SYSCFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syscfg0::W](W) writer structure"]
impl crate::Writable for SYSCFG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSCFG0 to value 0"]
impl crate::Resettable for SYSCFG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
