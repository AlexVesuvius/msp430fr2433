#[doc = "Register `P1SEL0` reader"]
pub type R = crate::R<P1SEL0_SPEC>;
#[doc = "Register `P1SEL0` writer"]
pub type W = crate::W<P1SEL0_SPEC>;
#[doc = "Field `P1SEL0_0` reader - P1SEL0_0"]
pub type P1SEL0_0_R = crate::BitReader;
#[doc = "Field `P1SEL0_0` writer - P1SEL0_0"]
pub type P1SEL0_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1SEL0_1` reader - P1SEL0_1"]
pub type P1SEL0_1_R = crate::BitReader;
#[doc = "Field `P1SEL0_1` writer - P1SEL0_1"]
pub type P1SEL0_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1SEL0_2` reader - P1SEL0_2"]
pub type P1SEL0_2_R = crate::BitReader;
#[doc = "Field `P1SEL0_2` writer - P1SEL0_2"]
pub type P1SEL0_2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1SEL0_3` reader - P1SEL0_3"]
pub type P1SEL0_3_R = crate::BitReader;
#[doc = "Field `P1SEL0_3` writer - P1SEL0_3"]
pub type P1SEL0_3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1SEL0_4` reader - P1SEL0_4"]
pub type P1SEL0_4_R = crate::BitReader;
#[doc = "Field `P1SEL0_4` writer - P1SEL0_4"]
pub type P1SEL0_4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1SEL0_5` reader - P1SEL0_5"]
pub type P1SEL0_5_R = crate::BitReader;
#[doc = "Field `P1SEL0_5` writer - P1SEL0_5"]
pub type P1SEL0_5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1SEL0_6` reader - P1SEL0_6"]
pub type P1SEL0_6_R = crate::BitReader;
#[doc = "Field `P1SEL0_6` writer - P1SEL0_6"]
pub type P1SEL0_6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1SEL0_7` reader - P1SEL0_7"]
pub type P1SEL0_7_R = crate::BitReader;
#[doc = "Field `P1SEL0_7` writer - P1SEL0_7"]
pub type P1SEL0_7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P1SEL0_0"]
    #[inline(always)]
    pub fn p1sel0_0(&self) -> P1SEL0_0_R {
        P1SEL0_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P1SEL0_1"]
    #[inline(always)]
    pub fn p1sel0_1(&self) -> P1SEL0_1_R {
        P1SEL0_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P1SEL0_2"]
    #[inline(always)]
    pub fn p1sel0_2(&self) -> P1SEL0_2_R {
        P1SEL0_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P1SEL0_3"]
    #[inline(always)]
    pub fn p1sel0_3(&self) -> P1SEL0_3_R {
        P1SEL0_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P1SEL0_4"]
    #[inline(always)]
    pub fn p1sel0_4(&self) -> P1SEL0_4_R {
        P1SEL0_4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P1SEL0_5"]
    #[inline(always)]
    pub fn p1sel0_5(&self) -> P1SEL0_5_R {
        P1SEL0_5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P1SEL0_6"]
    #[inline(always)]
    pub fn p1sel0_6(&self) -> P1SEL0_6_R {
        P1SEL0_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P1SEL0_7"]
    #[inline(always)]
    pub fn p1sel0_7(&self) -> P1SEL0_7_R {
        P1SEL0_7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P1SEL0_0"]
    #[inline(always)]
    #[must_use]
    pub fn p1sel0_0(&mut self) -> P1SEL0_0_W<P1SEL0_SPEC> {
        P1SEL0_0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P1SEL0_1"]
    #[inline(always)]
    #[must_use]
    pub fn p1sel0_1(&mut self) -> P1SEL0_1_W<P1SEL0_SPEC> {
        P1SEL0_1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P1SEL0_2"]
    #[inline(always)]
    #[must_use]
    pub fn p1sel0_2(&mut self) -> P1SEL0_2_W<P1SEL0_SPEC> {
        P1SEL0_2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P1SEL0_3"]
    #[inline(always)]
    #[must_use]
    pub fn p1sel0_3(&mut self) -> P1SEL0_3_W<P1SEL0_SPEC> {
        P1SEL0_3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P1SEL0_4"]
    #[inline(always)]
    #[must_use]
    pub fn p1sel0_4(&mut self) -> P1SEL0_4_W<P1SEL0_SPEC> {
        P1SEL0_4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P1SEL0_5"]
    #[inline(always)]
    #[must_use]
    pub fn p1sel0_5(&mut self) -> P1SEL0_5_W<P1SEL0_SPEC> {
        P1SEL0_5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P1SEL0_6"]
    #[inline(always)]
    #[must_use]
    pub fn p1sel0_6(&mut self) -> P1SEL0_6_W<P1SEL0_SPEC> {
        P1SEL0_6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P1SEL0_7"]
    #[inline(always)]
    #[must_use]
    pub fn p1sel0_7(&mut self) -> P1SEL0_7_W<P1SEL0_SPEC> {
        P1SEL0_7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 1 Selection 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p1sel0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p1sel0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1SEL0_SPEC;
impl crate::RegisterSpec for P1SEL0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p1sel0::R`](R) reader structure"]
impl crate::Readable for P1SEL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p1sel0::W`](W) writer structure"]
impl crate::Writable for P1SEL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P1SEL0 to value 0"]
impl crate::Resettable for P1SEL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
