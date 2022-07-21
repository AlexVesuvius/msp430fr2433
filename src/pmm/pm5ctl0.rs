#[doc = "Register `PM5CTL0` reader"]
pub struct R(crate::R<PM5CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PM5CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PM5CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PM5CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PM5CTL0` writer"]
pub struct W(crate::W<PM5CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PM5CTL0_SPEC>;
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
impl From<crate::W<PM5CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PM5CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCKLPM5` reader - Lock I/O pin configuration upon entry/exit to/from LPM5"]
pub type LOCKLPM5_R = crate::BitReader<bool>;
#[doc = "Field `LOCKLPM5` writer - Lock I/O pin configuration upon entry/exit to/from LPM5"]
pub type LOCKLPM5_W<'a, const O: u8> = crate::BitWriter<'a, u16, PM5CTL0_SPEC, bool, O>;
#[doc = "Field `LPM5SW` reader - LPMx.5 switch dis/connected"]
pub type LPM5SW_R = crate::BitReader<bool>;
#[doc = "Field `LPM5SW` writer - LPMx.5 switch dis/connected"]
pub type LPM5SW_W<'a, const O: u8> = crate::BitWriter<'a, u16, PM5CTL0_SPEC, bool, O>;
#[doc = "Field `LPM5SM` reader - Manual mode for LPM3.5 switch"]
pub type LPM5SM_R = crate::BitReader<bool>;
#[doc = "Field `LPM5SM` writer - Manual mode for LPM3.5 switch"]
pub type LPM5SM_W<'a, const O: u8> = crate::BitWriter<'a, u16, PM5CTL0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Lock I/O pin configuration upon entry/exit to/from LPM5"]
    #[inline(always)]
    pub fn locklpm5(&self) -> LOCKLPM5_R {
        LOCKLPM5_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - LPMx.5 switch dis/connected"]
    #[inline(always)]
    pub fn lpm5sw(&self) -> LPM5SW_R {
        LPM5SW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Manual mode for LPM3.5 switch"]
    #[inline(always)]
    pub fn lpm5sm(&self) -> LPM5SM_R {
        LPM5SM_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lock I/O pin configuration upon entry/exit to/from LPM5"]
    #[inline(always)]
    pub fn locklpm5(&mut self) -> LOCKLPM5_W<0> {
        LOCKLPM5_W::new(self)
    }
    #[doc = "Bit 4 - LPMx.5 switch dis/connected"]
    #[inline(always)]
    pub fn lpm5sw(&mut self) -> LPM5SW_W<4> {
        LPM5SW_W::new(self)
    }
    #[doc = "Bit 5 - Manual mode for LPM3.5 switch"]
    #[inline(always)]
    pub fn lpm5sm(&mut self) -> LPM5SM_W<5> {
        LPM5SM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMM Power Mode 5 Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pm5ctl0](index.html) module"]
pub struct PM5CTL0_SPEC;
impl crate::RegisterSpec for PM5CTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pm5ctl0::R](R) reader structure"]
impl crate::Readable for PM5CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pm5ctl0::W](W) writer structure"]
impl crate::Writable for PM5CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PM5CTL0 to value 0"]
impl crate::Resettable for PM5CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
