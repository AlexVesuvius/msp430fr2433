#[doc = "Register `SYSCFG2` reader"]
pub struct R(crate::R<SYSCFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCFG2` writer"]
pub struct W(crate::W<SYSCFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCFG2_SPEC>;
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
impl From<crate::W<SYSCFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCPCTL0` reader - ADC input A0 pin select"]
pub type ADCPCTL0_R = crate::BitReader<bool>;
#[doc = "Field `ADCPCTL0` writer - ADC input A0 pin select"]
pub type ADCPCTL0_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG2_SPEC, bool, O>;
#[doc = "Field `ADCPCTL1` reader - ADC input A1 pin select"]
pub type ADCPCTL1_R = crate::BitReader<bool>;
#[doc = "Field `ADCPCTL1` writer - ADC input A1 pin select"]
pub type ADCPCTL1_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG2_SPEC, bool, O>;
#[doc = "Field `ADCPCTL2` reader - ADC input A2 pin select"]
pub type ADCPCTL2_R = crate::BitReader<bool>;
#[doc = "Field `ADCPCTL2` writer - ADC input A2 pin select"]
pub type ADCPCTL2_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG2_SPEC, bool, O>;
#[doc = "Field `ADCPCTL3` reader - ADC input A3 pin select"]
pub type ADCPCTL3_R = crate::BitReader<bool>;
#[doc = "Field `ADCPCTL3` writer - ADC input A3 pin select"]
pub type ADCPCTL3_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG2_SPEC, bool, O>;
#[doc = "Field `ADCPCTL4` reader - ADC input A4 pin select"]
pub type ADCPCTL4_R = crate::BitReader<bool>;
#[doc = "Field `ADCPCTL4` writer - ADC input A4 pin select"]
pub type ADCPCTL4_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG2_SPEC, bool, O>;
#[doc = "Field `ADCPCTL5` reader - ADC input A5 pin select"]
pub type ADCPCTL5_R = crate::BitReader<bool>;
#[doc = "Field `ADCPCTL5` writer - ADC input A5 pin select"]
pub type ADCPCTL5_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG2_SPEC, bool, O>;
#[doc = "Field `ADCPCTL6` reader - ADC input A6 pin select"]
pub type ADCPCTL6_R = crate::BitReader<bool>;
#[doc = "Field `ADCPCTL6` writer - ADC input A6 pin select"]
pub type ADCPCTL6_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG2_SPEC, bool, O>;
#[doc = "Field `ADCPCTL7` reader - ADC input A7 pin select"]
pub type ADCPCTL7_R = crate::BitReader<bool>;
#[doc = "Field `ADCPCTL7` writer - ADC input A7 pin select"]
pub type ADCPCTL7_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ADC input A0 pin select"]
    #[inline(always)]
    pub fn adcpctl0(&self) -> ADCPCTL0_R {
        ADCPCTL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC input A1 pin select"]
    #[inline(always)]
    pub fn adcpctl1(&self) -> ADCPCTL1_R {
        ADCPCTL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC input A2 pin select"]
    #[inline(always)]
    pub fn adcpctl2(&self) -> ADCPCTL2_R {
        ADCPCTL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC input A3 pin select"]
    #[inline(always)]
    pub fn adcpctl3(&self) -> ADCPCTL3_R {
        ADCPCTL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC input A4 pin select"]
    #[inline(always)]
    pub fn adcpctl4(&self) -> ADCPCTL4_R {
        ADCPCTL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC input A5 pin select"]
    #[inline(always)]
    pub fn adcpctl5(&self) -> ADCPCTL5_R {
        ADCPCTL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC input A6 pin select"]
    #[inline(always)]
    pub fn adcpctl6(&self) -> ADCPCTL6_R {
        ADCPCTL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC input A7 pin select"]
    #[inline(always)]
    pub fn adcpctl7(&self) -> ADCPCTL7_R {
        ADCPCTL7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC input A0 pin select"]
    #[inline(always)]
    pub fn adcpctl0(&mut self) -> ADCPCTL0_W<0> {
        ADCPCTL0_W::new(self)
    }
    #[doc = "Bit 1 - ADC input A1 pin select"]
    #[inline(always)]
    pub fn adcpctl1(&mut self) -> ADCPCTL1_W<1> {
        ADCPCTL1_W::new(self)
    }
    #[doc = "Bit 2 - ADC input A2 pin select"]
    #[inline(always)]
    pub fn adcpctl2(&mut self) -> ADCPCTL2_W<2> {
        ADCPCTL2_W::new(self)
    }
    #[doc = "Bit 3 - ADC input A3 pin select"]
    #[inline(always)]
    pub fn adcpctl3(&mut self) -> ADCPCTL3_W<3> {
        ADCPCTL3_W::new(self)
    }
    #[doc = "Bit 4 - ADC input A4 pin select"]
    #[inline(always)]
    pub fn adcpctl4(&mut self) -> ADCPCTL4_W<4> {
        ADCPCTL4_W::new(self)
    }
    #[doc = "Bit 5 - ADC input A5 pin select"]
    #[inline(always)]
    pub fn adcpctl5(&mut self) -> ADCPCTL5_W<5> {
        ADCPCTL5_W::new(self)
    }
    #[doc = "Bit 6 - ADC input A6 pin select"]
    #[inline(always)]
    pub fn adcpctl6(&mut self) -> ADCPCTL6_W<6> {
        ADCPCTL6_W::new(self)
    }
    #[doc = "Bit 7 - ADC input A7 pin select"]
    #[inline(always)]
    pub fn adcpctl7(&mut self) -> ADCPCTL7_W<7> {
        ADCPCTL7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Configuration 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg2](index.html) module"]
pub struct SYSCFG2_SPEC;
impl crate::RegisterSpec for SYSCFG2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [syscfg2::R](R) reader structure"]
impl crate::Readable for SYSCFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syscfg2::W](W) writer structure"]
impl crate::Writable for SYSCFG2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSCFG2 to value 0"]
impl crate::Resettable for SYSCFG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
