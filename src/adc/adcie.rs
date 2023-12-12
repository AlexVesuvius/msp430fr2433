#[doc = "Register `ADCIE` reader"]
pub type R = crate::R<ADCIE_SPEC>;
#[doc = "Register `ADCIE` writer"]
pub type W = crate::W<ADCIE_SPEC>;
#[doc = "Field `ADCIE0` reader - ADC Interrupt enable"]
pub type ADCIE0_R = crate::BitReader;
#[doc = "Field `ADCIE0` writer - ADC Interrupt enable"]
pub type ADCIE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCINIE` reader - ADC Interrupt enable for the inside of window of the Window comparator"]
pub type ADCINIE_R = crate::BitReader;
#[doc = "Field `ADCINIE` writer - ADC Interrupt enable for the inside of window of the Window comparator"]
pub type ADCINIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCLOIE` reader - ADC Interrupt enable for lower threshold of the Window comparator"]
pub type ADCLOIE_R = crate::BitReader;
#[doc = "Field `ADCLOIE` writer - ADC Interrupt enable for lower threshold of the Window comparator"]
pub type ADCLOIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCHIIE` reader - ADC Interrupt enable for upper threshold of the Window comparator"]
pub type ADCHIIE_R = crate::BitReader;
#[doc = "Field `ADCHIIE` writer - ADC Interrupt enable for upper threshold of the Window comparator"]
pub type ADCHIIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCOVIE` reader - ADC ADCMEM overflow Interrupt enable"]
pub type ADCOVIE_R = crate::BitReader;
#[doc = "Field `ADCOVIE` writer - ADC ADCMEM overflow Interrupt enable"]
pub type ADCOVIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCTOVIE` reader - ADC conversion-time-overflow Interrupt enable"]
pub type ADCTOVIE_R = crate::BitReader;
#[doc = "Field `ADCTOVIE` writer - ADC conversion-time-overflow Interrupt enable"]
pub type ADCTOVIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC Interrupt enable"]
    #[inline(always)]
    pub fn adcie0(&self) -> ADCIE0_R {
        ADCIE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC Interrupt enable for the inside of window of the Window comparator"]
    #[inline(always)]
    pub fn adcinie(&self) -> ADCINIE_R {
        ADCINIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC Interrupt enable for lower threshold of the Window comparator"]
    #[inline(always)]
    pub fn adcloie(&self) -> ADCLOIE_R {
        ADCLOIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC Interrupt enable for upper threshold of the Window comparator"]
    #[inline(always)]
    pub fn adchiie(&self) -> ADCHIIE_R {
        ADCHIIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC ADCMEM overflow Interrupt enable"]
    #[inline(always)]
    pub fn adcovie(&self) -> ADCOVIE_R {
        ADCOVIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC conversion-time-overflow Interrupt enable"]
    #[inline(always)]
    pub fn adctovie(&self) -> ADCTOVIE_R {
        ADCTOVIE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn adcie0(&mut self) -> ADCIE0_W<ADCIE_SPEC> {
        ADCIE0_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC Interrupt enable for the inside of window of the Window comparator"]
    #[inline(always)]
    #[must_use]
    pub fn adcinie(&mut self) -> ADCINIE_W<ADCIE_SPEC> {
        ADCINIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - ADC Interrupt enable for lower threshold of the Window comparator"]
    #[inline(always)]
    #[must_use]
    pub fn adcloie(&mut self) -> ADCLOIE_W<ADCIE_SPEC> {
        ADCLOIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - ADC Interrupt enable for upper threshold of the Window comparator"]
    #[inline(always)]
    #[must_use]
    pub fn adchiie(&mut self) -> ADCHIIE_W<ADCIE_SPEC> {
        ADCHIIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - ADC ADCMEM overflow Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn adcovie(&mut self) -> ADCOVIE_W<ADCIE_SPEC> {
        ADCOVIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - ADC conversion-time-overflow Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn adctovie(&mut self) -> ADCTOVIE_W<ADCIE_SPEC> {
        ADCTOVIE_W::new(self, 5)
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
#[doc = "ADC Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCIE_SPEC;
impl crate::RegisterSpec for ADCIE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcie::R`](R) reader structure"]
impl crate::Readable for ADCIE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adcie::W`](W) writer structure"]
impl crate::Writable for ADCIE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCIE to value 0"]
impl crate::Resettable for ADCIE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
