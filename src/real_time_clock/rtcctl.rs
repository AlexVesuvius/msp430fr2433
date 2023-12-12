#[doc = "Register `RTCCTL` reader"]
pub type R = crate::R<RTCCTL_SPEC>;
#[doc = "Register `RTCCTL` writer"]
pub type W = crate::W<RTCCTL_SPEC>;
#[doc = "Field `RTCIF` reader - Low-Power-Counter Interrupt Flag"]
pub type RTCIF_R = crate::BitReader;
#[doc = "Field `RTCIF` writer - Low-Power-Counter Interrupt Flag"]
pub type RTCIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCIE` reader - Low-Power-Counter Interrupt Enable"]
pub type RTCIE_R = crate::BitReader;
#[doc = "Field `RTCIE` writer - Low-Power-Counter Interrupt Enable"]
pub type RTCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCSR` reader - Low-Power-Counter Software Reset"]
pub type RTCSR_R = crate::BitReader;
#[doc = "Field `RTCSR` writer - Low-Power-Counter Software Reset"]
pub type RTCSR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCPS` reader - Low-Power-Counter Clock Pre-divider Select Bit: 0"]
pub type RTCPS_R = crate::FieldReader<RTCPS_A>;
#[doc = "Low-Power-Counter Clock Pre-divider Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTCPS_A {
    #[doc = "0: Low-Power-Counter Clock Pre-divider Select: 0"]
    RTCPS_0 = 0,
    #[doc = "1: Low-Power-Counter Clock Pre-divider Select: 1"]
    RTCPS_1 = 1,
    #[doc = "2: Low-Power-Counter Clock Pre-divider Select: 2"]
    RTCPS_2 = 2,
    #[doc = "3: Low-Power-Counter Clock Pre-divider Select: 3"]
    RTCPS_3 = 3,
    #[doc = "4: Low-Power-Counter Clock Pre-divider Select: 4"]
    RTCPS_4 = 4,
    #[doc = "5: Low-Power-Counter Clock Pre-divider Select: 5"]
    RTCPS_5 = 5,
    #[doc = "6: Low-Power-Counter Clock Pre-divider Select: 6"]
    RTCPS_6 = 6,
    #[doc = "7: Low-Power-Counter Clock Pre-divider Select: 7"]
    RTCPS_7 = 7,
}
impl From<RTCPS_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCPS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RTCPS_A {
    type Ux = u8;
}
impl RTCPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTCPS_A {
        match self.bits {
            0 => RTCPS_A::RTCPS_0,
            1 => RTCPS_A::RTCPS_1,
            2 => RTCPS_A::RTCPS_2,
            3 => RTCPS_A::RTCPS_3,
            4 => RTCPS_A::RTCPS_4,
            5 => RTCPS_A::RTCPS_5,
            6 => RTCPS_A::RTCPS_6,
            7 => RTCPS_A::RTCPS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 0"]
    #[inline(always)]
    pub fn is_rtcps_0(&self) -> bool {
        *self == RTCPS_A::RTCPS_0
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 1"]
    #[inline(always)]
    pub fn is_rtcps_1(&self) -> bool {
        *self == RTCPS_A::RTCPS_1
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 2"]
    #[inline(always)]
    pub fn is_rtcps_2(&self) -> bool {
        *self == RTCPS_A::RTCPS_2
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 3"]
    #[inline(always)]
    pub fn is_rtcps_3(&self) -> bool {
        *self == RTCPS_A::RTCPS_3
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 4"]
    #[inline(always)]
    pub fn is_rtcps_4(&self) -> bool {
        *self == RTCPS_A::RTCPS_4
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 5"]
    #[inline(always)]
    pub fn is_rtcps_5(&self) -> bool {
        *self == RTCPS_A::RTCPS_5
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 6"]
    #[inline(always)]
    pub fn is_rtcps_6(&self) -> bool {
        *self == RTCPS_A::RTCPS_6
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 7"]
    #[inline(always)]
    pub fn is_rtcps_7(&self) -> bool {
        *self == RTCPS_A::RTCPS_7
    }
}
#[doc = "Field `RTCPS` writer - Low-Power-Counter Clock Pre-divider Select Bit: 0"]
pub type RTCPS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, RTCPS_A>;
impl<'a, REG> RTCPS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 0"]
    #[inline(always)]
    pub fn rtcps_0(self) -> &'a mut crate::W<REG> {
        self.variant(RTCPS_A::RTCPS_0)
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 1"]
    #[inline(always)]
    pub fn rtcps_1(self) -> &'a mut crate::W<REG> {
        self.variant(RTCPS_A::RTCPS_1)
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 2"]
    #[inline(always)]
    pub fn rtcps_2(self) -> &'a mut crate::W<REG> {
        self.variant(RTCPS_A::RTCPS_2)
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 3"]
    #[inline(always)]
    pub fn rtcps_3(self) -> &'a mut crate::W<REG> {
        self.variant(RTCPS_A::RTCPS_3)
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 4"]
    #[inline(always)]
    pub fn rtcps_4(self) -> &'a mut crate::W<REG> {
        self.variant(RTCPS_A::RTCPS_4)
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 5"]
    #[inline(always)]
    pub fn rtcps_5(self) -> &'a mut crate::W<REG> {
        self.variant(RTCPS_A::RTCPS_5)
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 6"]
    #[inline(always)]
    pub fn rtcps_6(self) -> &'a mut crate::W<REG> {
        self.variant(RTCPS_A::RTCPS_6)
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 7"]
    #[inline(always)]
    pub fn rtcps_7(self) -> &'a mut crate::W<REG> {
        self.variant(RTCPS_A::RTCPS_7)
    }
}
#[doc = "Field `RTCSS` reader - Low-Power-Counter Clock Source Select Bit: 0"]
pub type RTCSS_R = crate::FieldReader<RTCSS_A>;
#[doc = "Low-Power-Counter Clock Source Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTCSS_A {
    #[doc = "0: Low-Power-Counter Clock Source Select: 0"]
    RTCSS_0 = 0,
    #[doc = "1: Low-Power-Counter Clock Source Select: 1"]
    RTCSS_1 = 1,
    #[doc = "2: Low-Power-Counter Clock Source Select: 2"]
    RTCSS_2 = 2,
    #[doc = "3: Low-Power-Counter Clock Source Select: 3"]
    RTCSS_3 = 3,
}
impl From<RTCSS_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCSS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RTCSS_A {
    type Ux = u8;
}
impl RTCSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTCSS_A {
        match self.bits {
            0 => RTCSS_A::RTCSS_0,
            1 => RTCSS_A::RTCSS_1,
            2 => RTCSS_A::RTCSS_2,
            3 => RTCSS_A::RTCSS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Low-Power-Counter Clock Source Select: 0"]
    #[inline(always)]
    pub fn is_rtcss_0(&self) -> bool {
        *self == RTCSS_A::RTCSS_0
    }
    #[doc = "Low-Power-Counter Clock Source Select: 1"]
    #[inline(always)]
    pub fn is_rtcss_1(&self) -> bool {
        *self == RTCSS_A::RTCSS_1
    }
    #[doc = "Low-Power-Counter Clock Source Select: 2"]
    #[inline(always)]
    pub fn is_rtcss_2(&self) -> bool {
        *self == RTCSS_A::RTCSS_2
    }
    #[doc = "Low-Power-Counter Clock Source Select: 3"]
    #[inline(always)]
    pub fn is_rtcss_3(&self) -> bool {
        *self == RTCSS_A::RTCSS_3
    }
}
#[doc = "Field `RTCSS` writer - Low-Power-Counter Clock Source Select Bit: 0"]
pub type RTCSS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, RTCSS_A>;
impl<'a, REG> RTCSS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low-Power-Counter Clock Source Select: 0"]
    #[inline(always)]
    pub fn rtcss_0(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSS_A::RTCSS_0)
    }
    #[doc = "Low-Power-Counter Clock Source Select: 1"]
    #[inline(always)]
    pub fn rtcss_1(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSS_A::RTCSS_1)
    }
    #[doc = "Low-Power-Counter Clock Source Select: 2"]
    #[inline(always)]
    pub fn rtcss_2(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSS_A::RTCSS_2)
    }
    #[doc = "Low-Power-Counter Clock Source Select: 3"]
    #[inline(always)]
    pub fn rtcss_3(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSS_A::RTCSS_3)
    }
}
impl R {
    #[doc = "Bit 0 - Low-Power-Counter Interrupt Flag"]
    #[inline(always)]
    pub fn rtcif(&self) -> RTCIF_R {
        RTCIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low-Power-Counter Interrupt Enable"]
    #[inline(always)]
    pub fn rtcie(&self) -> RTCIE_R {
        RTCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - Low-Power-Counter Software Reset"]
    #[inline(always)]
    pub fn rtcsr(&self) -> RTCSR_R {
        RTCSR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Low-Power-Counter Clock Pre-divider Select Bit: 0"]
    #[inline(always)]
    pub fn rtcps(&self) -> RTCPS_R {
        RTCPS_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:13 - Low-Power-Counter Clock Source Select Bit: 0"]
    #[inline(always)]
    pub fn rtcss(&self) -> RTCSS_R {
        RTCSS_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Low-Power-Counter Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rtcif(&mut self) -> RTCIF_W<RTCCTL_SPEC> {
        RTCIF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Low-Power-Counter Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcie(&mut self) -> RTCIE_W<RTCCTL_SPEC> {
        RTCIE_W::new(self, 1)
    }
    #[doc = "Bit 6 - Low-Power-Counter Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rtcsr(&mut self) -> RTCSR_W<RTCCTL_SPEC> {
        RTCSR_W::new(self, 6)
    }
    #[doc = "Bits 8:10 - Low-Power-Counter Clock Pre-divider Select Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn rtcps(&mut self) -> RTCPS_W<RTCCTL_SPEC> {
        RTCPS_W::new(self, 8)
    }
    #[doc = "Bits 12:13 - Low-Power-Counter Clock Source Select Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn rtcss(&mut self) -> RTCSS_W<RTCCTL_SPEC> {
        RTCSS_W::new(self, 12)
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
#[doc = "RTC control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTCCTL_SPEC;
impl crate::RegisterSpec for RTCCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtcctl::R`](R) reader structure"]
impl crate::Readable for RTCCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtcctl::W`](W) writer structure"]
impl crate::Writable for RTCCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCCTL to value 0"]
impl crate::Resettable for RTCCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
