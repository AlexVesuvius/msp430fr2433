#[doc = "Register `SYSCFG0` reader"]
pub type R = crate::R<SYSCFG0_SPEC>;
#[doc = "Register `SYSCFG0` writer"]
pub type W = crate::W<SYSCFG0_SPEC>;
#[doc = "Field `PFWP` reader - Program FRAM Write Protection"]
pub type PFWP_R = crate::BitReader;
#[doc = "Field `PFWP` writer - Program FRAM Write Protection"]
pub type PFWP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFWP` reader - Data FRAM Write Protection"]
pub type DFWP_R = crate::BitReader;
#[doc = "Field `DFWP` writer - Data FRAM Write Protection"]
pub type DFWP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Program FRAM Write Protection"]
    #[inline(always)]
    pub fn pfwp(&self) -> PFWP_R {
        PFWP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data FRAM Write Protection"]
    #[inline(always)]
    pub fn dfwp(&self) -> DFWP_R {
        DFWP_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Program FRAM Write Protection"]
    #[inline(always)]
    #[must_use]
    pub fn pfwp(&mut self) -> PFWP_W<SYSCFG0_SPEC> {
        PFWP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Data FRAM Write Protection"]
    #[inline(always)]
    #[must_use]
    pub fn dfwp(&mut self) -> DFWP_W<SYSCFG0_SPEC> {
        DFWP_W::new(self, 1)
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
#[doc = "System Configuration 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCFG0_SPEC;
impl crate::RegisterSpec for SYSCFG0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`syscfg0::R`](R) reader structure"]
impl crate::Readable for SYSCFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`syscfg0::W`](W) writer structure"]
impl crate::Writable for SYSCFG0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSCFG0 to value 0"]
impl crate::Resettable for SYSCFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
