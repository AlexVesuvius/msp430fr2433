#[doc = "Register `GCCTL0` reader"]
pub struct R(crate::R<GCCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GCCTL0` writer"]
pub struct W(crate::W<GCCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCCTL0_SPEC>;
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
impl From<crate::W<GCCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRLPMPWR` reader - FRAM Enable FRAM auto power up after LPM"]
pub type FRLPMPWR_R = crate::BitReader<bool>;
#[doc = "Field `FRLPMPWR` writer - FRAM Enable FRAM auto power up after LPM"]
pub type FRLPMPWR_W<'a, const O: u8> = crate::BitWriter<'a, u16, GCCTL0_SPEC, bool, O>;
#[doc = "Field `FRPWR` reader - FRAM Power Control"]
pub type FRPWR_R = crate::BitReader<bool>;
#[doc = "Field `FRPWR` writer - FRAM Power Control"]
pub type FRPWR_W<'a, const O: u8> = crate::BitWriter<'a, u16, GCCTL0_SPEC, bool, O>;
#[doc = "Field `ACCTEIE` reader - RESERVED"]
pub type ACCTEIE_R = crate::BitReader<bool>;
#[doc = "Field `ACCTEIE` writer - RESERVED"]
pub type ACCTEIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, GCCTL0_SPEC, bool, O>;
#[doc = "Field `CBDIE` reader - Enable NMI event if correctable bit error detected"]
pub type CBDIE_R = crate::BitReader<bool>;
#[doc = "Field `CBDIE` writer - Enable NMI event if correctable bit error detected"]
pub type CBDIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, GCCTL0_SPEC, bool, O>;
#[doc = "Field `UBDIE` reader - Enable NMI event if uncorrectable bit error detected"]
pub type UBDIE_R = crate::BitReader<bool>;
#[doc = "Field `UBDIE` writer - Enable NMI event if uncorrectable bit error detected"]
pub type UBDIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, GCCTL0_SPEC, bool, O>;
#[doc = "Field `UBDRSTEN` reader - Enable Power Up Clear (PUC) reset if FRAM uncorrectable bit error detected"]
pub type UBDRSTEN_R = crate::BitReader<bool>;
#[doc = "Field `UBDRSTEN` writer - Enable Power Up Clear (PUC) reset if FRAM uncorrectable bit error detected"]
pub type UBDRSTEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, GCCTL0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - FRAM Enable FRAM auto power up after LPM"]
    #[inline(always)]
    pub fn frlpmpwr(&self) -> FRLPMPWR_R {
        FRLPMPWR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FRAM Power Control"]
    #[inline(always)]
    pub fn frpwr(&self) -> FRPWR_R {
        FRPWR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RESERVED"]
    #[inline(always)]
    pub fn accteie(&self) -> ACCTEIE_R {
        ACCTEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable NMI event if correctable bit error detected"]
    #[inline(always)]
    pub fn cbdie(&self) -> CBDIE_R {
        CBDIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable NMI event if uncorrectable bit error detected"]
    #[inline(always)]
    pub fn ubdie(&self) -> UBDIE_R {
        UBDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Power Up Clear (PUC) reset if FRAM uncorrectable bit error detected"]
    #[inline(always)]
    pub fn ubdrsten(&self) -> UBDRSTEN_R {
        UBDRSTEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - FRAM Enable FRAM auto power up after LPM"]
    #[inline(always)]
    pub fn frlpmpwr(&mut self) -> FRLPMPWR_W<1> {
        FRLPMPWR_W::new(self)
    }
    #[doc = "Bit 2 - FRAM Power Control"]
    #[inline(always)]
    pub fn frpwr(&mut self) -> FRPWR_W<2> {
        FRPWR_W::new(self)
    }
    #[doc = "Bit 3 - RESERVED"]
    #[inline(always)]
    pub fn accteie(&mut self) -> ACCTEIE_W<3> {
        ACCTEIE_W::new(self)
    }
    #[doc = "Bit 5 - Enable NMI event if correctable bit error detected"]
    #[inline(always)]
    pub fn cbdie(&mut self) -> CBDIE_W<5> {
        CBDIE_W::new(self)
    }
    #[doc = "Bit 6 - Enable NMI event if uncorrectable bit error detected"]
    #[inline(always)]
    pub fn ubdie(&mut self) -> UBDIE_W<6> {
        UBDIE_W::new(self)
    }
    #[doc = "Bit 7 - Enable Power Up Clear (PUC) reset if FRAM uncorrectable bit error detected"]
    #[inline(always)]
    pub fn ubdrsten(&mut self) -> UBDRSTEN_W<7> {
        UBDRSTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcctl0](index.html) module"]
pub struct GCCTL0_SPEC;
impl crate::RegisterSpec for GCCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [gcctl0::R](R) reader structure"]
impl crate::Readable for GCCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gcctl0::W](W) writer structure"]
impl crate::Writable for GCCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GCCTL0 to value 0"]
impl crate::Resettable for GCCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
