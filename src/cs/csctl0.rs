#[doc = "Register `CSCTL0` reader"]
pub struct R(crate::R<CSCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSCTL0` writer"]
pub struct W(crate::W<CSCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSCTL0_SPEC>;
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
impl From<crate::W<CSCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCO0` reader - DCO TAP Bit : 0"]
pub type DCO0_R = crate::BitReader<bool>;
#[doc = "Field `DCO0` writer - DCO TAP Bit : 0"]
pub type DCO0_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL0_SPEC, bool, O>;
#[doc = "Field `DCO1` reader - DCO TAP Bit : 1"]
pub type DCO1_R = crate::BitReader<bool>;
#[doc = "Field `DCO1` writer - DCO TAP Bit : 1"]
pub type DCO1_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL0_SPEC, bool, O>;
#[doc = "Field `DCO2` reader - DCO TAP Bit : 2"]
pub type DCO2_R = crate::BitReader<bool>;
#[doc = "Field `DCO2` writer - DCO TAP Bit : 2"]
pub type DCO2_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL0_SPEC, bool, O>;
#[doc = "Field `DCO3` reader - DCO TAP Bit : 3"]
pub type DCO3_R = crate::BitReader<bool>;
#[doc = "Field `DCO3` writer - DCO TAP Bit : 3"]
pub type DCO3_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL0_SPEC, bool, O>;
#[doc = "Field `DCO4` reader - DCO TAP Bit : 4"]
pub type DCO4_R = crate::BitReader<bool>;
#[doc = "Field `DCO4` writer - DCO TAP Bit : 4"]
pub type DCO4_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL0_SPEC, bool, O>;
#[doc = "Field `DCO5` reader - DCO TAP Bit : 5"]
pub type DCO5_R = crate::BitReader<bool>;
#[doc = "Field `DCO5` writer - DCO TAP Bit : 5"]
pub type DCO5_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL0_SPEC, bool, O>;
#[doc = "Field `DCO6` reader - DCO TAP Bit : 6"]
pub type DCO6_R = crate::BitReader<bool>;
#[doc = "Field `DCO6` writer - DCO TAP Bit : 6"]
pub type DCO6_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL0_SPEC, bool, O>;
#[doc = "Field `DCO7` reader - DCO TAP Bit : 7"]
pub type DCO7_R = crate::BitReader<bool>;
#[doc = "Field `DCO7` writer - DCO TAP Bit : 7"]
pub type DCO7_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL0_SPEC, bool, O>;
#[doc = "Field `DCO8` reader - DCO TAP Bit : 8"]
pub type DCO8_R = crate::BitReader<bool>;
#[doc = "Field `DCO8` writer - DCO TAP Bit : 8"]
pub type DCO8_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL0_SPEC, bool, O>;
#[doc = "Field `MOD0` reader - Modulation Bit Counter Bit : 0"]
pub type MOD0_R = crate::BitReader<bool>;
#[doc = "Field `MOD0` writer - Modulation Bit Counter Bit : 0"]
pub type MOD0_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL0_SPEC, bool, O>;
#[doc = "Field `MOD1` reader - Modulation Bit Counter Bit : 1"]
pub type MOD1_R = crate::BitReader<bool>;
#[doc = "Field `MOD1` writer - Modulation Bit Counter Bit : 1"]
pub type MOD1_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL0_SPEC, bool, O>;
#[doc = "Field `MOD2` reader - Modulation Bit Counter Bit : 2"]
pub type MOD2_R = crate::BitReader<bool>;
#[doc = "Field `MOD2` writer - Modulation Bit Counter Bit : 2"]
pub type MOD2_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL0_SPEC, bool, O>;
#[doc = "Field `MOD3` reader - Modulation Bit Counter Bit : 3"]
pub type MOD3_R = crate::BitReader<bool>;
#[doc = "Field `MOD3` writer - Modulation Bit Counter Bit : 3"]
pub type MOD3_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL0_SPEC, bool, O>;
#[doc = "Field `MOD4` reader - Modulation Bit Counter Bit : 4"]
pub type MOD4_R = crate::BitReader<bool>;
#[doc = "Field `MOD4` writer - Modulation Bit Counter Bit : 4"]
pub type MOD4_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DCO TAP Bit : 0"]
    #[inline(always)]
    pub fn dco0(&self) -> DCO0_R {
        DCO0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DCO TAP Bit : 1"]
    #[inline(always)]
    pub fn dco1(&self) -> DCO1_R {
        DCO1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DCO TAP Bit : 2"]
    #[inline(always)]
    pub fn dco2(&self) -> DCO2_R {
        DCO2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DCO TAP Bit : 3"]
    #[inline(always)]
    pub fn dco3(&self) -> DCO3_R {
        DCO3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DCO TAP Bit : 4"]
    #[inline(always)]
    pub fn dco4(&self) -> DCO4_R {
        DCO4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DCO TAP Bit : 5"]
    #[inline(always)]
    pub fn dco5(&self) -> DCO5_R {
        DCO5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DCO TAP Bit : 6"]
    #[inline(always)]
    pub fn dco6(&self) -> DCO6_R {
        DCO6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DCO TAP Bit : 7"]
    #[inline(always)]
    pub fn dco7(&self) -> DCO7_R {
        DCO7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DCO TAP Bit : 8"]
    #[inline(always)]
    pub fn dco8(&self) -> DCO8_R {
        DCO8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Modulation Bit Counter Bit : 0"]
    #[inline(always)]
    pub fn mod0(&self) -> MOD0_R {
        MOD0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Modulation Bit Counter Bit : 1"]
    #[inline(always)]
    pub fn mod1(&self) -> MOD1_R {
        MOD1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Modulation Bit Counter Bit : 2"]
    #[inline(always)]
    pub fn mod2(&self) -> MOD2_R {
        MOD2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Modulation Bit Counter Bit : 3"]
    #[inline(always)]
    pub fn mod3(&self) -> MOD3_R {
        MOD3_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Modulation Bit Counter Bit : 4"]
    #[inline(always)]
    pub fn mod4(&self) -> MOD4_R {
        MOD4_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCO TAP Bit : 0"]
    #[inline(always)]
    pub fn dco0(&mut self) -> DCO0_W<0> {
        DCO0_W::new(self)
    }
    #[doc = "Bit 1 - DCO TAP Bit : 1"]
    #[inline(always)]
    pub fn dco1(&mut self) -> DCO1_W<1> {
        DCO1_W::new(self)
    }
    #[doc = "Bit 2 - DCO TAP Bit : 2"]
    #[inline(always)]
    pub fn dco2(&mut self) -> DCO2_W<2> {
        DCO2_W::new(self)
    }
    #[doc = "Bit 3 - DCO TAP Bit : 3"]
    #[inline(always)]
    pub fn dco3(&mut self) -> DCO3_W<3> {
        DCO3_W::new(self)
    }
    #[doc = "Bit 4 - DCO TAP Bit : 4"]
    #[inline(always)]
    pub fn dco4(&mut self) -> DCO4_W<4> {
        DCO4_W::new(self)
    }
    #[doc = "Bit 5 - DCO TAP Bit : 5"]
    #[inline(always)]
    pub fn dco5(&mut self) -> DCO5_W<5> {
        DCO5_W::new(self)
    }
    #[doc = "Bit 6 - DCO TAP Bit : 6"]
    #[inline(always)]
    pub fn dco6(&mut self) -> DCO6_W<6> {
        DCO6_W::new(self)
    }
    #[doc = "Bit 7 - DCO TAP Bit : 7"]
    #[inline(always)]
    pub fn dco7(&mut self) -> DCO7_W<7> {
        DCO7_W::new(self)
    }
    #[doc = "Bit 8 - DCO TAP Bit : 8"]
    #[inline(always)]
    pub fn dco8(&mut self) -> DCO8_W<8> {
        DCO8_W::new(self)
    }
    #[doc = "Bit 9 - Modulation Bit Counter Bit : 0"]
    #[inline(always)]
    pub fn mod0(&mut self) -> MOD0_W<9> {
        MOD0_W::new(self)
    }
    #[doc = "Bit 10 - Modulation Bit Counter Bit : 1"]
    #[inline(always)]
    pub fn mod1(&mut self) -> MOD1_W<10> {
        MOD1_W::new(self)
    }
    #[doc = "Bit 11 - Modulation Bit Counter Bit : 2"]
    #[inline(always)]
    pub fn mod2(&mut self) -> MOD2_W<11> {
        MOD2_W::new(self)
    }
    #[doc = "Bit 12 - Modulation Bit Counter Bit : 3"]
    #[inline(always)]
    pub fn mod3(&mut self) -> MOD3_W<12> {
        MOD3_W::new(self)
    }
    #[doc = "Bit 13 - Modulation Bit Counter Bit : 4"]
    #[inline(always)]
    pub fn mod4(&mut self) -> MOD4_W<13> {
        MOD4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CS Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csctl0](index.html) module"]
pub struct CSCTL0_SPEC;
impl crate::RegisterSpec for CSCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [csctl0::R](R) reader structure"]
impl crate::Readable for CSCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csctl0::W](W) writer structure"]
impl crate::Writable for CSCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSCTL0 to value 0"]
impl crate::Resettable for CSCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
