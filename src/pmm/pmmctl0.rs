#[doc = "Register `PMMCTL0` reader"]
pub type R = crate::R<PMMCTL0_SPEC>;
#[doc = "Register `PMMCTL0` writer"]
pub type W = crate::W<PMMCTL0_SPEC>;
#[doc = "Field `PMMSWBOR` reader - PMM Software BOR"]
pub type PMMSWBOR_R = crate::BitReader;
#[doc = "Field `PMMSWBOR` writer - PMM Software BOR"]
pub type PMMSWBOR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMMSWPOR` reader - PMM Software POR"]
pub type PMMSWPOR_R = crate::BitReader;
#[doc = "Field `PMMSWPOR` writer - PMM Software POR"]
pub type PMMSWPOR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMMREGOFF` reader - PMM Turn Regulator off"]
pub type PMMREGOFF_R = crate::BitReader;
#[doc = "Field `PMMREGOFF` writer - PMM Turn Regulator off"]
pub type PMMREGOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVSHE` reader - SVS high side enable"]
pub type SVSHE_R = crate::BitReader;
#[doc = "Field `SVSHE` writer - SVS high side enable"]
pub type SVSHE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - PMM Software BOR"]
    #[inline(always)]
    pub fn pmmswbor(&self) -> PMMSWBOR_R {
        PMMSWBOR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PMM Software POR"]
    #[inline(always)]
    pub fn pmmswpor(&self) -> PMMSWPOR_R {
        PMMSWPOR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PMM Turn Regulator off"]
    #[inline(always)]
    pub fn pmmregoff(&self) -> PMMREGOFF_R {
        PMMREGOFF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - SVS high side enable"]
    #[inline(always)]
    pub fn svshe(&self) -> SVSHE_R {
        SVSHE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - PMM Software BOR"]
    #[inline(always)]
    #[must_use]
    pub fn pmmswbor(&mut self) -> PMMSWBOR_W<PMMCTL0_SPEC> {
        PMMSWBOR_W::new(self, 2)
    }
    #[doc = "Bit 3 - PMM Software POR"]
    #[inline(always)]
    #[must_use]
    pub fn pmmswpor(&mut self) -> PMMSWPOR_W<PMMCTL0_SPEC> {
        PMMSWPOR_W::new(self, 3)
    }
    #[doc = "Bit 4 - PMM Turn Regulator off"]
    #[inline(always)]
    #[must_use]
    pub fn pmmregoff(&mut self) -> PMMREGOFF_W<PMMCTL0_SPEC> {
        PMMREGOFF_W::new(self, 4)
    }
    #[doc = "Bit 6 - SVS high side enable"]
    #[inline(always)]
    #[must_use]
    pub fn svshe(&mut self) -> SVSHE_W<PMMCTL0_SPEC> {
        SVSHE_W::new(self, 6)
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
#[doc = "PMM Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmmctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmmctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMMCTL0_SPEC;
impl crate::RegisterSpec for PMMCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pmmctl0::R`](R) reader structure"]
impl crate::Readable for PMMCTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmmctl0::W`](W) writer structure"]
impl crate::Writable for PMMCTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PMMCTL0 to value 0"]
impl crate::Resettable for PMMCTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
