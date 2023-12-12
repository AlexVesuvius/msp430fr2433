#[doc = "Register `SYSCFG2` reader"]
pub type R = crate::R<SYSCFG2_SPEC>;
#[doc = "Register `SYSCFG2` writer"]
pub type W = crate::W<SYSCFG2_SPEC>;
#[doc = "Field `ADCPCTL0` reader - ADC input A0 pin select"]
pub type ADCPCTL0_R = crate::BitReader;
#[doc = "Field `ADCPCTL0` writer - ADC input A0 pin select"]
pub type ADCPCTL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCPCTL1` reader - ADC input A1 pin select"]
pub type ADCPCTL1_R = crate::BitReader;
#[doc = "Field `ADCPCTL1` writer - ADC input A1 pin select"]
pub type ADCPCTL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCPCTL2` reader - ADC input A2 pin select"]
pub type ADCPCTL2_R = crate::BitReader;
#[doc = "Field `ADCPCTL2` writer - ADC input A2 pin select"]
pub type ADCPCTL2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCPCTL3` reader - ADC input A3 pin select"]
pub type ADCPCTL3_R = crate::BitReader;
#[doc = "Field `ADCPCTL3` writer - ADC input A3 pin select"]
pub type ADCPCTL3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCPCTL4` reader - ADC input A4 pin select"]
pub type ADCPCTL4_R = crate::BitReader;
#[doc = "Field `ADCPCTL4` writer - ADC input A4 pin select"]
pub type ADCPCTL4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCPCTL5` reader - ADC input A5 pin select"]
pub type ADCPCTL5_R = crate::BitReader;
#[doc = "Field `ADCPCTL5` writer - ADC input A5 pin select"]
pub type ADCPCTL5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCPCTL6` reader - ADC input A6 pin select"]
pub type ADCPCTL6_R = crate::BitReader;
#[doc = "Field `ADCPCTL6` writer - ADC input A6 pin select"]
pub type ADCPCTL6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCPCTL7` reader - ADC input A7 pin select"]
pub type ADCPCTL7_R = crate::BitReader;
#[doc = "Field `ADCPCTL7` writer - ADC input A7 pin select"]
pub type ADCPCTL7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC input A0 pin select"]
    #[inline(always)]
    pub fn adcpctl0(&self) -> ADCPCTL0_R {
        ADCPCTL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC input A1 pin select"]
    #[inline(always)]
    pub fn adcpctl1(&self) -> ADCPCTL1_R {
        ADCPCTL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC input A2 pin select"]
    #[inline(always)]
    pub fn adcpctl2(&self) -> ADCPCTL2_R {
        ADCPCTL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC input A3 pin select"]
    #[inline(always)]
    pub fn adcpctl3(&self) -> ADCPCTL3_R {
        ADCPCTL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC input A4 pin select"]
    #[inline(always)]
    pub fn adcpctl4(&self) -> ADCPCTL4_R {
        ADCPCTL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC input A5 pin select"]
    #[inline(always)]
    pub fn adcpctl5(&self) -> ADCPCTL5_R {
        ADCPCTL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC input A6 pin select"]
    #[inline(always)]
    pub fn adcpctl6(&self) -> ADCPCTL6_R {
        ADCPCTL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC input A7 pin select"]
    #[inline(always)]
    pub fn adcpctl7(&self) -> ADCPCTL7_R {
        ADCPCTL7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC input A0 pin select"]
    #[inline(always)]
    #[must_use]
    pub fn adcpctl0(&mut self) -> ADCPCTL0_W<SYSCFG2_SPEC> {
        ADCPCTL0_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC input A1 pin select"]
    #[inline(always)]
    #[must_use]
    pub fn adcpctl1(&mut self) -> ADCPCTL1_W<SYSCFG2_SPEC> {
        ADCPCTL1_W::new(self, 1)
    }
    #[doc = "Bit 2 - ADC input A2 pin select"]
    #[inline(always)]
    #[must_use]
    pub fn adcpctl2(&mut self) -> ADCPCTL2_W<SYSCFG2_SPEC> {
        ADCPCTL2_W::new(self, 2)
    }
    #[doc = "Bit 3 - ADC input A3 pin select"]
    #[inline(always)]
    #[must_use]
    pub fn adcpctl3(&mut self) -> ADCPCTL3_W<SYSCFG2_SPEC> {
        ADCPCTL3_W::new(self, 3)
    }
    #[doc = "Bit 4 - ADC input A4 pin select"]
    #[inline(always)]
    #[must_use]
    pub fn adcpctl4(&mut self) -> ADCPCTL4_W<SYSCFG2_SPEC> {
        ADCPCTL4_W::new(self, 4)
    }
    #[doc = "Bit 5 - ADC input A5 pin select"]
    #[inline(always)]
    #[must_use]
    pub fn adcpctl5(&mut self) -> ADCPCTL5_W<SYSCFG2_SPEC> {
        ADCPCTL5_W::new(self, 5)
    }
    #[doc = "Bit 6 - ADC input A6 pin select"]
    #[inline(always)]
    #[must_use]
    pub fn adcpctl6(&mut self) -> ADCPCTL6_W<SYSCFG2_SPEC> {
        ADCPCTL6_W::new(self, 6)
    }
    #[doc = "Bit 7 - ADC input A7 pin select"]
    #[inline(always)]
    #[must_use]
    pub fn adcpctl7(&mut self) -> ADCPCTL7_W<SYSCFG2_SPEC> {
        ADCPCTL7_W::new(self, 7)
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
#[doc = "System Configuration 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCFG2_SPEC;
impl crate::RegisterSpec for SYSCFG2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`syscfg2::R`](R) reader structure"]
impl crate::Readable for SYSCFG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`syscfg2::W`](W) writer structure"]
impl crate::Writable for SYSCFG2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSCFG2 to value 0"]
impl crate::Resettable for SYSCFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
