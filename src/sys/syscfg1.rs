#[doc = "Register `SYSCFG1` reader"]
pub struct R(crate::R<SYSCFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCFG1` writer"]
pub struct W(crate::W<SYSCFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCFG1_SPEC>;
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
impl From<crate::W<SYSCFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IREN` reader - Infrared enable"]
pub type IREN_R = crate::BitReader<bool>;
#[doc = "Field `IREN` writer - Infrared enable"]
pub type IREN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG1_SPEC, bool, O>;
#[doc = "Field `IRPSEL` reader - Infrared polarity select"]
pub type IRPSEL_R = crate::BitReader<bool>;
#[doc = "Field `IRPSEL` writer - Infrared polarity select"]
pub type IRPSEL_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG1_SPEC, bool, O>;
#[doc = "Field `IRMSEL` reader - Infrared mode select"]
pub type IRMSEL_R = crate::BitReader<bool>;
#[doc = "Field `IRMSEL` writer - Infrared mode select"]
pub type IRMSEL_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG1_SPEC, bool, O>;
#[doc = "Field `IRDSSEL` reader - Infrared data source select"]
pub type IRDSSEL_R = crate::BitReader<bool>;
#[doc = "Field `IRDSSEL` writer - Infrared data source select"]
pub type IRDSSEL_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG1_SPEC, bool, O>;
#[doc = "Field `IRDATA` reader - Infrared enable"]
pub type IRDATA_R = crate::BitReader<bool>;
#[doc = "Field `IRDATA` writer - Infrared enable"]
pub type IRDATA_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Infrared enable"]
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Infrared polarity select"]
    #[inline(always)]
    pub fn irpsel(&self) -> IRPSEL_R {
        IRPSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Infrared mode select"]
    #[inline(always)]
    pub fn irmsel(&self) -> IRMSEL_R {
        IRMSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Infrared data source select"]
    #[inline(always)]
    pub fn irdssel(&self) -> IRDSSEL_R {
        IRDSSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Infrared enable"]
    #[inline(always)]
    pub fn irdata(&self) -> IRDATA_R {
        IRDATA_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Infrared enable"]
    #[inline(always)]
    pub fn iren(&mut self) -> IREN_W<0> {
        IREN_W::new(self)
    }
    #[doc = "Bit 1 - Infrared polarity select"]
    #[inline(always)]
    pub fn irpsel(&mut self) -> IRPSEL_W<1> {
        IRPSEL_W::new(self)
    }
    #[doc = "Bit 2 - Infrared mode select"]
    #[inline(always)]
    pub fn irmsel(&mut self) -> IRMSEL_W<2> {
        IRMSEL_W::new(self)
    }
    #[doc = "Bit 3 - Infrared data source select"]
    #[inline(always)]
    pub fn irdssel(&mut self) -> IRDSSEL_W<3> {
        IRDSSEL_W::new(self)
    }
    #[doc = "Bit 4 - Infrared enable"]
    #[inline(always)]
    pub fn irdata(&mut self) -> IRDATA_W<4> {
        IRDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Configuration 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg1](index.html) module"]
pub struct SYSCFG1_SPEC;
impl crate::RegisterSpec for SYSCFG1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [syscfg1::R](R) reader structure"]
impl crate::Readable for SYSCFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syscfg1::W](W) writer structure"]
impl crate::Writable for SYSCFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSCFG1 to value 0"]
impl crate::Resettable for SYSCFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
