#[doc = "Register `GCCTL0` reader"]
pub type R = crate::R<GCCTL0_SPEC>;
#[doc = "Register `GCCTL0` writer"]
pub type W = crate::W<GCCTL0_SPEC>;
#[doc = "Field `FRLPMPWR` reader - FRAM Enable FRAM auto power up after LPM"]
pub type FRLPMPWR_R = crate::BitReader;
#[doc = "Field `FRLPMPWR` writer - FRAM Enable FRAM auto power up after LPM"]
pub type FRLPMPWR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRPWR` reader - FRAM Power Control"]
pub type FRPWR_R = crate::BitReader;
#[doc = "Field `FRPWR` writer - FRAM Power Control"]
pub type FRPWR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACCTEIE` reader - RESERVED"]
pub type ACCTEIE_R = crate::BitReader;
#[doc = "Field `ACCTEIE` writer - RESERVED"]
pub type ACCTEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBDIE` reader - Enable NMI event if correctable bit error detected"]
pub type CBDIE_R = crate::BitReader;
#[doc = "Field `CBDIE` writer - Enable NMI event if correctable bit error detected"]
pub type CBDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UBDIE` reader - Enable NMI event if uncorrectable bit error detected"]
pub type UBDIE_R = crate::BitReader;
#[doc = "Field `UBDIE` writer - Enable NMI event if uncorrectable bit error detected"]
pub type UBDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UBDRSTEN` reader - Enable Power Up Clear (PUC) reset if FRAM uncorrectable bit error detected"]
pub type UBDRSTEN_R = crate::BitReader;
#[doc = "Field `UBDRSTEN` writer - Enable Power Up Clear (PUC) reset if FRAM uncorrectable bit error detected"]
pub type UBDRSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[must_use]
    pub fn frlpmpwr(&mut self) -> FRLPMPWR_W<GCCTL0_SPEC> {
        FRLPMPWR_W::new(self, 1)
    }
    #[doc = "Bit 2 - FRAM Power Control"]
    #[inline(always)]
    #[must_use]
    pub fn frpwr(&mut self) -> FRPWR_W<GCCTL0_SPEC> {
        FRPWR_W::new(self, 2)
    }
    #[doc = "Bit 3 - RESERVED"]
    #[inline(always)]
    #[must_use]
    pub fn accteie(&mut self) -> ACCTEIE_W<GCCTL0_SPEC> {
        ACCTEIE_W::new(self, 3)
    }
    #[doc = "Bit 5 - Enable NMI event if correctable bit error detected"]
    #[inline(always)]
    #[must_use]
    pub fn cbdie(&mut self) -> CBDIE_W<GCCTL0_SPEC> {
        CBDIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable NMI event if uncorrectable bit error detected"]
    #[inline(always)]
    #[must_use]
    pub fn ubdie(&mut self) -> UBDIE_W<GCCTL0_SPEC> {
        UBDIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Power Up Clear (PUC) reset if FRAM uncorrectable bit error detected"]
    #[inline(always)]
    #[must_use]
    pub fn ubdrsten(&mut self) -> UBDRSTEN_W<GCCTL0_SPEC> {
        UBDRSTEN_W::new(self, 7)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "General Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GCCTL0_SPEC;
impl crate::RegisterSpec for GCCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`gcctl0::R`](R) reader structure"]
impl crate::Readable for GCCTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gcctl0::W`](W) writer structure"]
impl crate::Writable for GCCTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GCCTL0 to value 0"]
impl crate::Resettable for GCCTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
