#[doc = "Register `CSCTL7` reader"]
pub type R = crate::R<CSCTL7_SPEC>;
#[doc = "Register `CSCTL7` writer"]
pub type W = crate::W<CSCTL7_SPEC>;
#[doc = "Field `DCOFFG` reader - DCO fault flag"]
pub type DCOFFG_R = crate::BitReader;
#[doc = "Field `DCOFFG` writer - DCO fault flag"]
pub type DCOFFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XT1OFFG` reader - XT1 Low Frequency Oscillator Fault Flag"]
pub type XT1OFFG_R = crate::BitReader;
#[doc = "Field `XT1OFFG` writer - XT1 Low Frequency Oscillator Fault Flag"]
pub type XT1OFFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLLULIFG` reader - FLL unlock interrupt flag"]
pub type FLLULIFG_R = crate::BitReader;
#[doc = "Field `FLLULIFG` writer - FLL unlock interrupt flag"]
pub type FLLULIFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENSTFCNT1` reader - Enable start counter for XT1"]
pub type ENSTFCNT1_R = crate::BitReader;
#[doc = "Field `ENSTFCNT1` writer - Enable start counter for XT1"]
pub type ENSTFCNT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLLUNLOCK` reader - FLL unlock condition Bit: 0"]
pub type FLLUNLOCK_R = crate::FieldReader<FLLUNLOCK_A>;
#[doc = "FLL unlock condition Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLLUNLOCK_A {
    #[doc = "0: FLL unlock condition: 0"]
    FLLUNLOCK_0 = 0,
    #[doc = "1: FLL unlock condition: 1"]
    FLLUNLOCK_1 = 1,
    #[doc = "2: FLL unlock condition: 2"]
    FLLUNLOCK_2 = 2,
    #[doc = "3: FLL unlock condition: 3"]
    FLLUNLOCK_3 = 3,
}
impl From<FLLUNLOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: FLLUNLOCK_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FLLUNLOCK_A {
    type Ux = u8;
}
impl FLLUNLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FLLUNLOCK_A {
        match self.bits {
            0 => FLLUNLOCK_A::FLLUNLOCK_0,
            1 => FLLUNLOCK_A::FLLUNLOCK_1,
            2 => FLLUNLOCK_A::FLLUNLOCK_2,
            3 => FLLUNLOCK_A::FLLUNLOCK_3,
            _ => unreachable!(),
        }
    }
    #[doc = "FLL unlock condition: 0"]
    #[inline(always)]
    pub fn is_fllunlock_0(&self) -> bool {
        *self == FLLUNLOCK_A::FLLUNLOCK_0
    }
    #[doc = "FLL unlock condition: 1"]
    #[inline(always)]
    pub fn is_fllunlock_1(&self) -> bool {
        *self == FLLUNLOCK_A::FLLUNLOCK_1
    }
    #[doc = "FLL unlock condition: 2"]
    #[inline(always)]
    pub fn is_fllunlock_2(&self) -> bool {
        *self == FLLUNLOCK_A::FLLUNLOCK_2
    }
    #[doc = "FLL unlock condition: 3"]
    #[inline(always)]
    pub fn is_fllunlock_3(&self) -> bool {
        *self == FLLUNLOCK_A::FLLUNLOCK_3
    }
}
#[doc = "Field `FLLUNLOCK` writer - FLL unlock condition Bit: 0"]
pub type FLLUNLOCK_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, FLLUNLOCK_A>;
impl<'a, REG> FLLUNLOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "FLL unlock condition: 0"]
    #[inline(always)]
    pub fn fllunlock_0(self) -> &'a mut crate::W<REG> {
        self.variant(FLLUNLOCK_A::FLLUNLOCK_0)
    }
    #[doc = "FLL unlock condition: 1"]
    #[inline(always)]
    pub fn fllunlock_1(self) -> &'a mut crate::W<REG> {
        self.variant(FLLUNLOCK_A::FLLUNLOCK_1)
    }
    #[doc = "FLL unlock condition: 2"]
    #[inline(always)]
    pub fn fllunlock_2(self) -> &'a mut crate::W<REG> {
        self.variant(FLLUNLOCK_A::FLLUNLOCK_2)
    }
    #[doc = "FLL unlock condition: 3"]
    #[inline(always)]
    pub fn fllunlock_3(self) -> &'a mut crate::W<REG> {
        self.variant(FLLUNLOCK_A::FLLUNLOCK_3)
    }
}
#[doc = "Field `FLLUNLOCKHIS` reader - Unlock history Bit: 0"]
pub type FLLUNLOCKHIS_R = crate::FieldReader<FLLUNLOCKHIS_A>;
#[doc = "Unlock history Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLLUNLOCKHIS_A {
    #[doc = "0: Unlock history: 0"]
    FLLUNLOCKHIS_0 = 0,
    #[doc = "1: Unlock history: 1"]
    FLLUNLOCKHIS_1 = 1,
    #[doc = "2: Unlock history: 2"]
    FLLUNLOCKHIS_2 = 2,
    #[doc = "3: Unlock history: 3"]
    FLLUNLOCKHIS_3 = 3,
}
impl From<FLLUNLOCKHIS_A> for u8 {
    #[inline(always)]
    fn from(variant: FLLUNLOCKHIS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FLLUNLOCKHIS_A {
    type Ux = u8;
}
impl FLLUNLOCKHIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FLLUNLOCKHIS_A {
        match self.bits {
            0 => FLLUNLOCKHIS_A::FLLUNLOCKHIS_0,
            1 => FLLUNLOCKHIS_A::FLLUNLOCKHIS_1,
            2 => FLLUNLOCKHIS_A::FLLUNLOCKHIS_2,
            3 => FLLUNLOCKHIS_A::FLLUNLOCKHIS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Unlock history: 0"]
    #[inline(always)]
    pub fn is_fllunlockhis_0(&self) -> bool {
        *self == FLLUNLOCKHIS_A::FLLUNLOCKHIS_0
    }
    #[doc = "Unlock history: 1"]
    #[inline(always)]
    pub fn is_fllunlockhis_1(&self) -> bool {
        *self == FLLUNLOCKHIS_A::FLLUNLOCKHIS_1
    }
    #[doc = "Unlock history: 2"]
    #[inline(always)]
    pub fn is_fllunlockhis_2(&self) -> bool {
        *self == FLLUNLOCKHIS_A::FLLUNLOCKHIS_2
    }
    #[doc = "Unlock history: 3"]
    #[inline(always)]
    pub fn is_fllunlockhis_3(&self) -> bool {
        *self == FLLUNLOCKHIS_A::FLLUNLOCKHIS_3
    }
}
#[doc = "Field `FLLUNLOCKHIS` writer - Unlock history Bit: 0"]
pub type FLLUNLOCKHIS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, FLLUNLOCKHIS_A>;
impl<'a, REG> FLLUNLOCKHIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Unlock history: 0"]
    #[inline(always)]
    pub fn fllunlockhis_0(self) -> &'a mut crate::W<REG> {
        self.variant(FLLUNLOCKHIS_A::FLLUNLOCKHIS_0)
    }
    #[doc = "Unlock history: 1"]
    #[inline(always)]
    pub fn fllunlockhis_1(self) -> &'a mut crate::W<REG> {
        self.variant(FLLUNLOCKHIS_A::FLLUNLOCKHIS_1)
    }
    #[doc = "Unlock history: 2"]
    #[inline(always)]
    pub fn fllunlockhis_2(self) -> &'a mut crate::W<REG> {
        self.variant(FLLUNLOCKHIS_A::FLLUNLOCKHIS_2)
    }
    #[doc = "Unlock history: 3"]
    #[inline(always)]
    pub fn fllunlockhis_3(self) -> &'a mut crate::W<REG> {
        self.variant(FLLUNLOCKHIS_A::FLLUNLOCKHIS_3)
    }
}
#[doc = "Field `FLLULPUC` reader - FLL unlock PUC enable"]
pub type FLLULPUC_R = crate::BitReader;
#[doc = "Field `FLLULPUC` writer - FLL unlock PUC enable"]
pub type FLLULPUC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLLWARNEN` reader - Warning enable"]
pub type FLLWARNEN_R = crate::BitReader;
#[doc = "Field `FLLWARNEN` writer - Warning enable"]
pub type FLLWARNEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DCO fault flag"]
    #[inline(always)]
    pub fn dcoffg(&self) -> DCOFFG_R {
        DCOFFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - XT1 Low Frequency Oscillator Fault Flag"]
    #[inline(always)]
    pub fn xt1offg(&self) -> XT1OFFG_R {
        XT1OFFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - FLL unlock interrupt flag"]
    #[inline(always)]
    pub fn fllulifg(&self) -> FLLULIFG_R {
        FLLULIFG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable start counter for XT1"]
    #[inline(always)]
    pub fn enstfcnt1(&self) -> ENSTFCNT1_R {
        ENSTFCNT1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - FLL unlock condition Bit: 0"]
    #[inline(always)]
    pub fn fllunlock(&self) -> FLLUNLOCK_R {
        FLLUNLOCK_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Unlock history Bit: 0"]
    #[inline(always)]
    pub fn fllunlockhis(&self) -> FLLUNLOCKHIS_R {
        FLLUNLOCKHIS_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - FLL unlock PUC enable"]
    #[inline(always)]
    pub fn fllulpuc(&self) -> FLLULPUC_R {
        FLLULPUC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Warning enable"]
    #[inline(always)]
    pub fn fllwarnen(&self) -> FLLWARNEN_R {
        FLLWARNEN_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCO fault flag"]
    #[inline(always)]
    #[must_use]
    pub fn dcoffg(&mut self) -> DCOFFG_W<CSCTL7_SPEC> {
        DCOFFG_W::new(self, 0)
    }
    #[doc = "Bit 1 - XT1 Low Frequency Oscillator Fault Flag"]
    #[inline(always)]
    #[must_use]
    pub fn xt1offg(&mut self) -> XT1OFFG_W<CSCTL7_SPEC> {
        XT1OFFG_W::new(self, 1)
    }
    #[doc = "Bit 4 - FLL unlock interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn fllulifg(&mut self) -> FLLULIFG_W<CSCTL7_SPEC> {
        FLLULIFG_W::new(self, 4)
    }
    #[doc = "Bit 6 - Enable start counter for XT1"]
    #[inline(always)]
    #[must_use]
    pub fn enstfcnt1(&mut self) -> ENSTFCNT1_W<CSCTL7_SPEC> {
        ENSTFCNT1_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - FLL unlock condition Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn fllunlock(&mut self) -> FLLUNLOCK_W<CSCTL7_SPEC> {
        FLLUNLOCK_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Unlock history Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn fllunlockhis(&mut self) -> FLLUNLOCKHIS_W<CSCTL7_SPEC> {
        FLLUNLOCKHIS_W::new(self, 10)
    }
    #[doc = "Bit 12 - FLL unlock PUC enable"]
    #[inline(always)]
    #[must_use]
    pub fn fllulpuc(&mut self) -> FLLULPUC_W<CSCTL7_SPEC> {
        FLLULPUC_W::new(self, 12)
    }
    #[doc = "Bit 13 - Warning enable"]
    #[inline(always)]
    #[must_use]
    pub fn fllwarnen(&mut self) -> FLLWARNEN_W<CSCTL7_SPEC> {
        FLLWARNEN_W::new(self, 13)
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
#[doc = "CS Control Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csctl7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csctl7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSCTL7_SPEC;
impl crate::RegisterSpec for CSCTL7_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`csctl7::R`](R) reader structure"]
impl crate::Readable for CSCTL7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csctl7::W`](W) writer structure"]
impl crate::Writable for CSCTL7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSCTL7 to value 0"]
impl crate::Resettable for CSCTL7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
