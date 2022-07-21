#[doc = "Register `PMMIFG` reader"]
pub struct R(crate::R<PMMIFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMMIFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMMIFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMMIFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMMIFG` writer"]
pub struct W(crate::W<PMMIFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMMIFG_SPEC>;
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
impl From<crate::W<PMMIFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMMIFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMMBORIFG` reader - PMM Software BOR interrupt flag"]
pub type PMMBORIFG_R = crate::BitReader<bool>;
#[doc = "Field `PMMBORIFG` writer - PMM Software BOR interrupt flag"]
pub type PMMBORIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMIFG_SPEC, bool, O>;
#[doc = "Field `PMMRSTIFG` reader - PMM RESET pin interrupt flag"]
pub type PMMRSTIFG_R = crate::BitReader<bool>;
#[doc = "Field `PMMRSTIFG` writer - PMM RESET pin interrupt flag"]
pub type PMMRSTIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMIFG_SPEC, bool, O>;
#[doc = "Field `PMMPORIFG` reader - PMM Software POR interrupt flag"]
pub type PMMPORIFG_R = crate::BitReader<bool>;
#[doc = "Field `PMMPORIFG` writer - PMM Software POR interrupt flag"]
pub type PMMPORIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMIFG_SPEC, bool, O>;
#[doc = "Field `SVSHIFG` reader - SVS low side interrupt flag"]
pub type SVSHIFG_R = crate::BitReader<bool>;
#[doc = "Field `SVSHIFG` writer - SVS low side interrupt flag"]
pub type SVSHIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMIFG_SPEC, bool, O>;
#[doc = "Field `PMMLPM5IFG` reader - LPM5 indication Flag"]
pub type PMMLPM5IFG_R = crate::BitReader<bool>;
#[doc = "Field `PMMLPM5IFG` writer - LPM5 indication Flag"]
pub type PMMLPM5IFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMIFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 8 - PMM Software BOR interrupt flag"]
    #[inline(always)]
    pub fn pmmborifg(&self) -> PMMBORIFG_R {
        PMMBORIFG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PMM RESET pin interrupt flag"]
    #[inline(always)]
    pub fn pmmrstifg(&self) -> PMMRSTIFG_R {
        PMMRSTIFG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PMM Software POR interrupt flag"]
    #[inline(always)]
    pub fn pmmporifg(&self) -> PMMPORIFG_R {
        PMMPORIFG_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - SVS low side interrupt flag"]
    #[inline(always)]
    pub fn svshifg(&self) -> SVSHIFG_R {
        SVSHIFG_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - LPM5 indication Flag"]
    #[inline(always)]
    pub fn pmmlpm5ifg(&self) -> PMMLPM5IFG_R {
        PMMLPM5IFG_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - PMM Software BOR interrupt flag"]
    #[inline(always)]
    pub fn pmmborifg(&mut self) -> PMMBORIFG_W<8> {
        PMMBORIFG_W::new(self)
    }
    #[doc = "Bit 9 - PMM RESET pin interrupt flag"]
    #[inline(always)]
    pub fn pmmrstifg(&mut self) -> PMMRSTIFG_W<9> {
        PMMRSTIFG_W::new(self)
    }
    #[doc = "Bit 10 - PMM Software POR interrupt flag"]
    #[inline(always)]
    pub fn pmmporifg(&mut self) -> PMMPORIFG_W<10> {
        PMMPORIFG_W::new(self)
    }
    #[doc = "Bit 13 - SVS low side interrupt flag"]
    #[inline(always)]
    pub fn svshifg(&mut self) -> SVSHIFG_W<13> {
        SVSHIFG_W::new(self)
    }
    #[doc = "Bit 15 - LPM5 indication Flag"]
    #[inline(always)]
    pub fn pmmlpm5ifg(&mut self) -> PMMLPM5IFG_W<15> {
        PMMLPM5IFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMM Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmmifg](index.html) module"]
pub struct PMMIFG_SPEC;
impl crate::RegisterSpec for PMMIFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pmmifg::R](R) reader structure"]
impl crate::Readable for PMMIFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmmifg::W](W) writer structure"]
impl crate::Writable for PMMIFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMMIFG to value 0"]
impl crate::Resettable for PMMIFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
