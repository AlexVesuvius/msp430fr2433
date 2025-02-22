#[doc = "Register `SYSCFG1` reader"]
pub type R = crate::R<SYSCFG1_SPEC>;
#[doc = "Register `SYSCFG1` writer"]
pub type W = crate::W<SYSCFG1_SPEC>;
#[doc = "Field `IREN` reader - Infrared enable"]
pub type IREN_R = crate::BitReader;
#[doc = "Field `IREN` writer - Infrared enable"]
pub type IREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRPSEL` reader - Infrared polarity select"]
pub type IRPSEL_R = crate::BitReader;
#[doc = "Field `IRPSEL` writer - Infrared polarity select"]
pub type IRPSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRMSEL` reader - Infrared mode select"]
pub type IRMSEL_R = crate::BitReader;
#[doc = "Field `IRMSEL` writer - Infrared mode select"]
pub type IRMSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRDSSEL` reader - Infrared data source select"]
pub type IRDSSEL_R = crate::BitReader;
#[doc = "Field `IRDSSEL` writer - Infrared data source select"]
pub type IRDSSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRDATA` reader - Infrared enable"]
pub type IRDATA_R = crate::BitReader;
#[doc = "Field `IRDATA` writer - Infrared enable"]
pub type IRDATA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Infrared enable"]
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Infrared polarity select"]
    #[inline(always)]
    pub fn irpsel(&self) -> IRPSEL_R {
        IRPSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Infrared mode select"]
    #[inline(always)]
    pub fn irmsel(&self) -> IRMSEL_R {
        IRMSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Infrared data source select"]
    #[inline(always)]
    pub fn irdssel(&self) -> IRDSSEL_R {
        IRDSSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Infrared enable"]
    #[inline(always)]
    pub fn irdata(&self) -> IRDATA_R {
        IRDATA_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Infrared enable"]
    #[inline(always)]
    #[must_use]
    pub fn iren(&mut self) -> IREN_W<SYSCFG1_SPEC> {
        IREN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Infrared polarity select"]
    #[inline(always)]
    #[must_use]
    pub fn irpsel(&mut self) -> IRPSEL_W<SYSCFG1_SPEC> {
        IRPSEL_W::new(self, 1)
    }
    #[doc = "Bit 2 - Infrared mode select"]
    #[inline(always)]
    #[must_use]
    pub fn irmsel(&mut self) -> IRMSEL_W<SYSCFG1_SPEC> {
        IRMSEL_W::new(self, 2)
    }
    #[doc = "Bit 3 - Infrared data source select"]
    #[inline(always)]
    #[must_use]
    pub fn irdssel(&mut self) -> IRDSSEL_W<SYSCFG1_SPEC> {
        IRDSSEL_W::new(self, 3)
    }
    #[doc = "Bit 4 - Infrared enable"]
    #[inline(always)]
    #[must_use]
    pub fn irdata(&mut self) -> IRDATA_W<SYSCFG1_SPEC> {
        IRDATA_W::new(self, 4)
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
#[doc = "System Configuration 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCFG1_SPEC;
impl crate::RegisterSpec for SYSCFG1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`syscfg1::R`](R) reader structure"]
impl crate::Readable for SYSCFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`syscfg1::W`](W) writer structure"]
impl crate::Writable for SYSCFG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSCFG1 to value 0"]
impl crate::Resettable for SYSCFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
