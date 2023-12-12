#[doc = "Register `P3SEL1` reader"]
pub type R = crate::R<P3SEL1_SPEC>;
#[doc = "Register `P3SEL1` writer"]
pub type W = crate::W<P3SEL1_SPEC>;
#[doc = "Field `P3SEL1_0` reader - P3SEL1_0"]
pub type P3SEL1_0_R = crate::BitReader;
#[doc = "Field `P3SEL1_0` writer - P3SEL1_0"]
pub type P3SEL1_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3SEL1_1` reader - P3SEL1_1"]
pub type P3SEL1_1_R = crate::BitReader;
#[doc = "Field `P3SEL1_1` writer - P3SEL1_1"]
pub type P3SEL1_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3SEL1_2` reader - P3SEL1_2"]
pub type P3SEL1_2_R = crate::BitReader;
#[doc = "Field `P3SEL1_2` writer - P3SEL1_2"]
pub type P3SEL1_2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3SEL1_3` reader - P3SEL1_3"]
pub type P3SEL1_3_R = crate::BitReader;
#[doc = "Field `P3SEL1_3` writer - P3SEL1_3"]
pub type P3SEL1_3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3SEL1_4` reader - P3SEL1_4"]
pub type P3SEL1_4_R = crate::BitReader;
#[doc = "Field `P3SEL1_4` writer - P3SEL1_4"]
pub type P3SEL1_4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3SEL1_5` reader - P3SEL1_5"]
pub type P3SEL1_5_R = crate::BitReader;
#[doc = "Field `P3SEL1_5` writer - P3SEL1_5"]
pub type P3SEL1_5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3SEL1_6` reader - P3SEL1_6"]
pub type P3SEL1_6_R = crate::BitReader;
#[doc = "Field `P3SEL1_6` writer - P3SEL1_6"]
pub type P3SEL1_6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3SEL1_7` reader - P3SEL1_7"]
pub type P3SEL1_7_R = crate::BitReader;
#[doc = "Field `P3SEL1_7` writer - P3SEL1_7"]
pub type P3SEL1_7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - P3SEL1_0"]
    #[inline(always)]
    pub fn p3sel1_0(&self) -> P3SEL1_0_R {
        P3SEL1_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P3SEL1_1"]
    #[inline(always)]
    pub fn p3sel1_1(&self) -> P3SEL1_1_R {
        P3SEL1_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P3SEL1_2"]
    #[inline(always)]
    pub fn p3sel1_2(&self) -> P3SEL1_2_R {
        P3SEL1_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P3SEL1_3"]
    #[inline(always)]
    pub fn p3sel1_3(&self) -> P3SEL1_3_R {
        P3SEL1_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P3SEL1_4"]
    #[inline(always)]
    pub fn p3sel1_4(&self) -> P3SEL1_4_R {
        P3SEL1_4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P3SEL1_5"]
    #[inline(always)]
    pub fn p3sel1_5(&self) -> P3SEL1_5_R {
        P3SEL1_5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P3SEL1_6"]
    #[inline(always)]
    pub fn p3sel1_6(&self) -> P3SEL1_6_R {
        P3SEL1_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P3SEL1_7"]
    #[inline(always)]
    pub fn p3sel1_7(&self) -> P3SEL1_7_R {
        P3SEL1_7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P3SEL1_0"]
    #[inline(always)]
    #[must_use]
    pub fn p3sel1_0(&mut self) -> P3SEL1_0_W<P3SEL1_SPEC> {
        P3SEL1_0_W::new(self, 0)
    }
    #[doc = "Bit 1 - P3SEL1_1"]
    #[inline(always)]
    #[must_use]
    pub fn p3sel1_1(&mut self) -> P3SEL1_1_W<P3SEL1_SPEC> {
        P3SEL1_1_W::new(self, 1)
    }
    #[doc = "Bit 2 - P3SEL1_2"]
    #[inline(always)]
    #[must_use]
    pub fn p3sel1_2(&mut self) -> P3SEL1_2_W<P3SEL1_SPEC> {
        P3SEL1_2_W::new(self, 2)
    }
    #[doc = "Bit 3 - P3SEL1_3"]
    #[inline(always)]
    #[must_use]
    pub fn p3sel1_3(&mut self) -> P3SEL1_3_W<P3SEL1_SPEC> {
        P3SEL1_3_W::new(self, 3)
    }
    #[doc = "Bit 4 - P3SEL1_4"]
    #[inline(always)]
    #[must_use]
    pub fn p3sel1_4(&mut self) -> P3SEL1_4_W<P3SEL1_SPEC> {
        P3SEL1_4_W::new(self, 4)
    }
    #[doc = "Bit 5 - P3SEL1_5"]
    #[inline(always)]
    #[must_use]
    pub fn p3sel1_5(&mut self) -> P3SEL1_5_W<P3SEL1_SPEC> {
        P3SEL1_5_W::new(self, 5)
    }
    #[doc = "Bit 6 - P3SEL1_6"]
    #[inline(always)]
    #[must_use]
    pub fn p3sel1_6(&mut self) -> P3SEL1_6_W<P3SEL1_SPEC> {
        P3SEL1_6_W::new(self, 6)
    }
    #[doc = "Bit 7 - P3SEL1_7"]
    #[inline(always)]
    #[must_use]
    pub fn p3sel1_7(&mut self) -> P3SEL1_7_W<P3SEL1_SPEC> {
        P3SEL1_7_W::new(self, 7)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port 3 Selection1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p3sel1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p3sel1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P3SEL1_SPEC;
impl crate::RegisterSpec for P3SEL1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p3sel1::R`](R) reader structure"]
impl crate::Readable for P3SEL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p3sel1::W`](W) writer structure"]
impl crate::Writable for P3SEL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P3SEL1 to value 0"]
impl crate::Resettable for P3SEL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
