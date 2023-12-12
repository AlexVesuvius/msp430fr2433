#[doc = "Register `CSCTL6` reader"]
pub type R = crate::R<CSCTL6_SPEC>;
#[doc = "Register `CSCTL6` writer"]
pub type W = crate::W<CSCTL6_SPEC>;
#[doc = "Field `XT1AUTOOFF` reader - XT1 automatic off enable"]
pub type XT1AUTOOFF_R = crate::BitReader;
#[doc = "Field `XT1AUTOOFF` writer - XT1 automatic off enable"]
pub type XT1AUTOOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XT1AGCOFF` reader - XT1 Automatic Gain Control (AGC) disable"]
pub type XT1AGCOFF_R = crate::BitReader;
#[doc = "Field `XT1AGCOFF` writer - XT1 Automatic Gain Control (AGC) disable"]
pub type XT1AGCOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XT1BYPASS` reader - XT1 bypass mode : 0: internal 1:sourced from external pin"]
pub type XT1BYPASS_R = crate::BitReader;
#[doc = "Field `XT1BYPASS` writer - XT1 bypass mode : 0: internal 1:sourced from external pin"]
pub type XT1BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTS` reader - 1: Selects high-freq. oscillator"]
pub type XTS_R = crate::BitReader;
#[doc = "Field `XTS` writer - 1: Selects high-freq. oscillator"]
pub type XTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XT1DRIVE` reader - XT1 Drive Level mode Bit 0"]
pub type XT1DRIVE_R = crate::FieldReader<XT1DRIVE_A>;
#[doc = "XT1 Drive Level mode Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum XT1DRIVE_A {
    #[doc = "0: XT1 Drive Level mode: 0"]
    XT1DRIVE_0 = 0,
    #[doc = "1: XT1 Drive Level mode: 1"]
    XT1DRIVE_1 = 1,
    #[doc = "2: XT1 Drive Level mode: 2"]
    XT1DRIVE_2 = 2,
    #[doc = "3: XT1 Drive Level mode: 3"]
    XT1DRIVE_3 = 3,
}
impl From<XT1DRIVE_A> for u8 {
    #[inline(always)]
    fn from(variant: XT1DRIVE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for XT1DRIVE_A {
    type Ux = u8;
}
impl XT1DRIVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> XT1DRIVE_A {
        match self.bits {
            0 => XT1DRIVE_A::XT1DRIVE_0,
            1 => XT1DRIVE_A::XT1DRIVE_1,
            2 => XT1DRIVE_A::XT1DRIVE_2,
            3 => XT1DRIVE_A::XT1DRIVE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "XT1 Drive Level mode: 0"]
    #[inline(always)]
    pub fn is_xt1drive_0(&self) -> bool {
        *self == XT1DRIVE_A::XT1DRIVE_0
    }
    #[doc = "XT1 Drive Level mode: 1"]
    #[inline(always)]
    pub fn is_xt1drive_1(&self) -> bool {
        *self == XT1DRIVE_A::XT1DRIVE_1
    }
    #[doc = "XT1 Drive Level mode: 2"]
    #[inline(always)]
    pub fn is_xt1drive_2(&self) -> bool {
        *self == XT1DRIVE_A::XT1DRIVE_2
    }
    #[doc = "XT1 Drive Level mode: 3"]
    #[inline(always)]
    pub fn is_xt1drive_3(&self) -> bool {
        *self == XT1DRIVE_A::XT1DRIVE_3
    }
}
#[doc = "Field `XT1DRIVE` writer - XT1 Drive Level mode Bit 0"]
pub type XT1DRIVE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, XT1DRIVE_A>;
impl<'a, REG> XT1DRIVE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "XT1 Drive Level mode: 0"]
    #[inline(always)]
    pub fn xt1drive_0(self) -> &'a mut crate::W<REG> {
        self.variant(XT1DRIVE_A::XT1DRIVE_0)
    }
    #[doc = "XT1 Drive Level mode: 1"]
    #[inline(always)]
    pub fn xt1drive_1(self) -> &'a mut crate::W<REG> {
        self.variant(XT1DRIVE_A::XT1DRIVE_1)
    }
    #[doc = "XT1 Drive Level mode: 2"]
    #[inline(always)]
    pub fn xt1drive_2(self) -> &'a mut crate::W<REG> {
        self.variant(XT1DRIVE_A::XT1DRIVE_2)
    }
    #[doc = "XT1 Drive Level mode: 3"]
    #[inline(always)]
    pub fn xt1drive_3(self) -> &'a mut crate::W<REG> {
        self.variant(XT1DRIVE_A::XT1DRIVE_3)
    }
}
impl R {
    #[doc = "Bit 0 - XT1 automatic off enable"]
    #[inline(always)]
    pub fn xt1autooff(&self) -> XT1AUTOOFF_R {
        XT1AUTOOFF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - XT1 Automatic Gain Control (AGC) disable"]
    #[inline(always)]
    pub fn xt1agcoff(&self) -> XT1AGCOFF_R {
        XT1AGCOFF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - XT1 bypass mode : 0: internal 1:sourced from external pin"]
    #[inline(always)]
    pub fn xt1bypass(&self) -> XT1BYPASS_R {
        XT1BYPASS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: Selects high-freq. oscillator"]
    #[inline(always)]
    pub fn xts(&self) -> XTS_R {
        XTS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - XT1 Drive Level mode Bit 0"]
    #[inline(always)]
    pub fn xt1drive(&self) -> XT1DRIVE_R {
        XT1DRIVE_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - XT1 automatic off enable"]
    #[inline(always)]
    #[must_use]
    pub fn xt1autooff(&mut self) -> XT1AUTOOFF_W<CSCTL6_SPEC> {
        XT1AUTOOFF_W::new(self, 0)
    }
    #[doc = "Bit 1 - XT1 Automatic Gain Control (AGC) disable"]
    #[inline(always)]
    #[must_use]
    pub fn xt1agcoff(&mut self) -> XT1AGCOFF_W<CSCTL6_SPEC> {
        XT1AGCOFF_W::new(self, 1)
    }
    #[doc = "Bit 4 - XT1 bypass mode : 0: internal 1:sourced from external pin"]
    #[inline(always)]
    #[must_use]
    pub fn xt1bypass(&mut self) -> XT1BYPASS_W<CSCTL6_SPEC> {
        XT1BYPASS_W::new(self, 4)
    }
    #[doc = "Bit 5 - 1: Selects high-freq. oscillator"]
    #[inline(always)]
    #[must_use]
    pub fn xts(&mut self) -> XTS_W<CSCTL6_SPEC> {
        XTS_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - XT1 Drive Level mode Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn xt1drive(&mut self) -> XT1DRIVE_W<CSCTL6_SPEC> {
        XT1DRIVE_W::new(self, 6)
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
#[doc = "CS Control Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csctl6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csctl6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSCTL6_SPEC;
impl crate::RegisterSpec for CSCTL6_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`csctl6::R`](R) reader structure"]
impl crate::Readable for CSCTL6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csctl6::W`](W) writer structure"]
impl crate::Writable for CSCTL6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSCTL6 to value 0"]
impl crate::Resettable for CSCTL6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
