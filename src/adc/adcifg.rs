#[doc = "Register `ADCIFG` reader"]
pub type R = crate::R<ADCIFG_SPEC>;
#[doc = "Register `ADCIFG` writer"]
pub type W = crate::W<ADCIFG_SPEC>;
#[doc = "Field `ADCIFG0` reader - ADC Interrupt Flag"]
pub type ADCIFG0_R = crate::BitReader;
#[doc = "Field `ADCIFG0` writer - ADC Interrupt Flag"]
pub type ADCIFG0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCINIFG` reader - ADC Interrupt Flag for the inside of window of the Window comparator"]
pub type ADCINIFG_R = crate::BitReader;
#[doc = "Field `ADCINIFG` writer - ADC Interrupt Flag for the inside of window of the Window comparator"]
pub type ADCINIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCLOIFG` reader - ADC Interrupt Flag for lower threshold of the Window comparator"]
pub type ADCLOIFG_R = crate::BitReader;
#[doc = "Field `ADCLOIFG` writer - ADC Interrupt Flag for lower threshold of the Window comparator"]
pub type ADCLOIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCHIIFG` reader - ADC Interrupt Flag for upper threshold of the Window comparator"]
pub type ADCHIIFG_R = crate::BitReader;
#[doc = "Field `ADCHIIFG` writer - ADC Interrupt Flag for upper threshold of the Window comparator"]
pub type ADCHIIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCOVIFG` reader - ADC ADCMEM overflow Interrupt Flag"]
pub type ADCOVIFG_R = crate::BitReader;
#[doc = "Field `ADCOVIFG` writer - ADC ADCMEM overflow Interrupt Flag"]
pub type ADCOVIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCTOVIFG` reader - ADC conversion-time-overflow Interrupt Flag"]
pub type ADCTOVIFG_R = crate::BitReader;
#[doc = "Field `ADCTOVIFG` writer - ADC conversion-time-overflow Interrupt Flag"]
pub type ADCTOVIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC Interrupt Flag"]
    #[inline(always)]
    pub fn adcifg0(&self) -> ADCIFG0_R {
        ADCIFG0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC Interrupt Flag for the inside of window of the Window comparator"]
    #[inline(always)]
    pub fn adcinifg(&self) -> ADCINIFG_R {
        ADCINIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC Interrupt Flag for lower threshold of the Window comparator"]
    #[inline(always)]
    pub fn adcloifg(&self) -> ADCLOIFG_R {
        ADCLOIFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC Interrupt Flag for upper threshold of the Window comparator"]
    #[inline(always)]
    pub fn adchiifg(&self) -> ADCHIIFG_R {
        ADCHIIFG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC ADCMEM overflow Interrupt Flag"]
    #[inline(always)]
    pub fn adcovifg(&self) -> ADCOVIFG_R {
        ADCOVIFG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC conversion-time-overflow Interrupt Flag"]
    #[inline(always)]
    pub fn adctovifg(&self) -> ADCTOVIFG_R {
        ADCTOVIFG_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adcifg0(&mut self) -> ADCIFG0_W<ADCIFG_SPEC> {
        ADCIFG0_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC Interrupt Flag for the inside of window of the Window comparator"]
    #[inline(always)]
    #[must_use]
    pub fn adcinifg(&mut self) -> ADCINIFG_W<ADCIFG_SPEC> {
        ADCINIFG_W::new(self, 1)
    }
    #[doc = "Bit 2 - ADC Interrupt Flag for lower threshold of the Window comparator"]
    #[inline(always)]
    #[must_use]
    pub fn adcloifg(&mut self) -> ADCLOIFG_W<ADCIFG_SPEC> {
        ADCLOIFG_W::new(self, 2)
    }
    #[doc = "Bit 3 - ADC Interrupt Flag for upper threshold of the Window comparator"]
    #[inline(always)]
    #[must_use]
    pub fn adchiifg(&mut self) -> ADCHIIFG_W<ADCIFG_SPEC> {
        ADCHIIFG_W::new(self, 3)
    }
    #[doc = "Bit 4 - ADC ADCMEM overflow Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adcovifg(&mut self) -> ADCOVIFG_W<ADCIFG_SPEC> {
        ADCOVIFG_W::new(self, 4)
    }
    #[doc = "Bit 5 - ADC conversion-time-overflow Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn adctovifg(&mut self) -> ADCTOVIFG_W<ADCIFG_SPEC> {
        ADCTOVIFG_W::new(self, 5)
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
#[doc = "ADC Interrupt Flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcifg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcifg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCIFG_SPEC;
impl crate::RegisterSpec for ADCIFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcifg::R`](R) reader structure"]
impl crate::Readable for ADCIFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adcifg::W`](W) writer structure"]
impl crate::Writable for ADCIFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCIFG to value 0"]
impl crate::Resettable for ADCIFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
