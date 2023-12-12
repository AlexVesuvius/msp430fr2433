#[doc = "Register `GCCTL1` reader"]
pub type R = crate::R<GCCTL1_SPEC>;
#[doc = "Register `GCCTL1` writer"]
pub type W = crate::W<GCCTL1_SPEC>;
#[doc = "Field `CBDIFG` reader - FRAM correctable bit error flag"]
pub type CBDIFG_R = crate::BitReader;
#[doc = "Field `CBDIFG` writer - FRAM correctable bit error flag"]
pub type CBDIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UBDIFG` reader - FRAM uncorrectable bit error flag"]
pub type UBDIFG_R = crate::BitReader;
#[doc = "Field `UBDIFG` writer - FRAM uncorrectable bit error flag"]
pub type UBDIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACCTEIFG` reader - Access time error flag"]
pub type ACCTEIFG_R = crate::BitReader;
#[doc = "Field `ACCTEIFG` writer - Access time error flag"]
pub type ACCTEIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - FRAM correctable bit error flag"]
    #[inline(always)]
    pub fn cbdifg(&self) -> CBDIFG_R {
        CBDIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FRAM uncorrectable bit error flag"]
    #[inline(always)]
    pub fn ubdifg(&self) -> UBDIFG_R {
        UBDIFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Access time error flag"]
    #[inline(always)]
    pub fn accteifg(&self) -> ACCTEIFG_R {
        ACCTEIFG_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - FRAM correctable bit error flag"]
    #[inline(always)]
    #[must_use]
    pub fn cbdifg(&mut self) -> CBDIFG_W<GCCTL1_SPEC> {
        CBDIFG_W::new(self, 1)
    }
    #[doc = "Bit 2 - FRAM uncorrectable bit error flag"]
    #[inline(always)]
    #[must_use]
    pub fn ubdifg(&mut self) -> UBDIFG_W<GCCTL1_SPEC> {
        UBDIFG_W::new(self, 2)
    }
    #[doc = "Bit 3 - Access time error flag"]
    #[inline(always)]
    #[must_use]
    pub fn accteifg(&mut self) -> ACCTEIFG_W<GCCTL1_SPEC> {
        ACCTEIFG_W::new(self, 3)
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
#[doc = "General Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GCCTL1_SPEC;
impl crate::RegisterSpec for GCCTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`gcctl1::R`](R) reader structure"]
impl crate::Readable for GCCTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gcctl1::W`](W) writer structure"]
impl crate::Writable for GCCTL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GCCTL1 to value 0"]
impl crate::Resettable for GCCTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
