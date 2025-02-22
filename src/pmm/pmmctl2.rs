#[doc = "Register `PMMCTL2` reader"]
pub type R = crate::R<PMMCTL2_SPEC>;
#[doc = "Register `PMMCTL2` writer"]
pub type W = crate::W<PMMCTL2_SPEC>;
#[doc = "Field `INTREFEN` reader - Internal Reference Enable"]
pub type INTREFEN_R = crate::BitReader;
#[doc = "Field `INTREFEN` writer - Internal Reference Enable"]
pub type INTREFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTREFEN` reader - External Reference output Enable"]
pub type EXTREFEN_R = crate::BitReader;
#[doc = "Field `EXTREFEN` writer - External Reference output Enable"]
pub type EXTREFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSENSOREN` reader - Temperature Sensor Enable"]
pub type TSENSOREN_R = crate::BitReader;
#[doc = "Field `TSENSOREN` writer - Temperature Sensor Enable"]
pub type TSENSOREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFGENACT` reader - REF Reference generator active"]
pub type REFGENACT_R = crate::BitReader;
#[doc = "Field `REFGENACT` writer - REF Reference generator active"]
pub type REFGENACT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFBGACT` reader - REF Reference bandgap active"]
pub type REFBGACT_R = crate::BitReader;
#[doc = "Field `REFBGACT` writer - REF Reference bandgap active"]
pub type REFBGACT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BGMODE` reader - REF Bandgap mode"]
pub type BGMODE_R = crate::BitReader;
#[doc = "Field `BGMODE` writer - REF Bandgap mode"]
pub type BGMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFGENRDY` reader - REF Reference generator ready"]
pub type REFGENRDY_R = crate::BitReader;
#[doc = "Field `REFGENRDY` writer - REF Reference generator ready"]
pub type REFGENRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFBGRDY` reader - REF Reference bandgap ready"]
pub type REFBGRDY_R = crate::BitReader;
#[doc = "Field `REFBGRDY` writer - REF Reference bandgap ready"]
pub type REFBGRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Internal Reference Enable"]
    #[inline(always)]
    pub fn intrefen(&self) -> INTREFEN_R {
        INTREFEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Reference output Enable"]
    #[inline(always)]
    pub fn extrefen(&self) -> EXTREFEN_R {
        EXTREFEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Temperature Sensor Enable"]
    #[inline(always)]
    pub fn tsensoren(&self) -> TSENSOREN_R {
        TSENSOREN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - REF Reference generator active"]
    #[inline(always)]
    pub fn refgenact(&self) -> REFGENACT_R {
        REFGENACT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - REF Reference bandgap active"]
    #[inline(always)]
    pub fn refbgact(&self) -> REFBGACT_R {
        REFBGACT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - REF Bandgap mode"]
    #[inline(always)]
    pub fn bgmode(&self) -> BGMODE_R {
        BGMODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - REF Reference generator ready"]
    #[inline(always)]
    pub fn refgenrdy(&self) -> REFGENRDY_R {
        REFGENRDY_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - REF Reference bandgap ready"]
    #[inline(always)]
    pub fn refbgrdy(&self) -> REFBGRDY_R {
        REFBGRDY_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Reference Enable"]
    #[inline(always)]
    #[must_use]
    pub fn intrefen(&mut self) -> INTREFEN_W<PMMCTL2_SPEC> {
        INTREFEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - External Reference output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extrefen(&mut self) -> EXTREFEN_W<PMMCTL2_SPEC> {
        EXTREFEN_W::new(self, 1)
    }
    #[doc = "Bit 3 - Temperature Sensor Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsensoren(&mut self) -> TSENSOREN_W<PMMCTL2_SPEC> {
        TSENSOREN_W::new(self, 3)
    }
    #[doc = "Bit 8 - REF Reference generator active"]
    #[inline(always)]
    #[must_use]
    pub fn refgenact(&mut self) -> REFGENACT_W<PMMCTL2_SPEC> {
        REFGENACT_W::new(self, 8)
    }
    #[doc = "Bit 9 - REF Reference bandgap active"]
    #[inline(always)]
    #[must_use]
    pub fn refbgact(&mut self) -> REFBGACT_W<PMMCTL2_SPEC> {
        REFBGACT_W::new(self, 9)
    }
    #[doc = "Bit 11 - REF Bandgap mode"]
    #[inline(always)]
    #[must_use]
    pub fn bgmode(&mut self) -> BGMODE_W<PMMCTL2_SPEC> {
        BGMODE_W::new(self, 11)
    }
    #[doc = "Bit 12 - REF Reference generator ready"]
    #[inline(always)]
    #[must_use]
    pub fn refgenrdy(&mut self) -> REFGENRDY_W<PMMCTL2_SPEC> {
        REFGENRDY_W::new(self, 12)
    }
    #[doc = "Bit 13 - REF Reference bandgap ready"]
    #[inline(always)]
    #[must_use]
    pub fn refbgrdy(&mut self) -> REFBGRDY_W<PMMCTL2_SPEC> {
        REFBGRDY_W::new(self, 13)
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
#[doc = "PMM Control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmmctl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmmctl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMMCTL2_SPEC;
impl crate::RegisterSpec for PMMCTL2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pmmctl2::R`](R) reader structure"]
impl crate::Readable for PMMCTL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmmctl2::W`](W) writer structure"]
impl crate::Writable for PMMCTL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PMMCTL2 to value 0"]
impl crate::Resettable for PMMCTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
