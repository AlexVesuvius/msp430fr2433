#[doc = "Register `FRCTL0` reader"]
pub type R = crate::R<FRCTL0_SPEC>;
#[doc = "Register `FRCTL0` writer"]
pub type W = crate::W<FRCTL0_SPEC>;
#[doc = "Field `NWAITS` reader - FRAM Wait state control Bit: 0"]
pub type NWAITS_R = crate::FieldReader<NWAITS_A>;
#[doc = "FRAM Wait state control Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NWAITS_A {
    #[doc = "0: FRAM Wait state control: 0"]
    NWAITS_0 = 0,
    #[doc = "1: FRAM Wait state control: 1"]
    NWAITS_1 = 1,
    #[doc = "2: FRAM Wait state control: 2"]
    NWAITS_2 = 2,
    #[doc = "3: FRAM Wait state control: 3"]
    NWAITS_3 = 3,
    #[doc = "4: FRAM Wait state control: 4"]
    NWAITS_4 = 4,
    #[doc = "5: FRAM Wait state control: 5"]
    NWAITS_5 = 5,
    #[doc = "6: FRAM Wait state control: 6"]
    NWAITS_6 = 6,
    #[doc = "7: FRAM Wait state control: 7"]
    NWAITS_7 = 7,
}
impl From<NWAITS_A> for u8 {
    #[inline(always)]
    fn from(variant: NWAITS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NWAITS_A {
    type Ux = u8;
}
impl NWAITS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NWAITS_A {
        match self.bits {
            0 => NWAITS_A::NWAITS_0,
            1 => NWAITS_A::NWAITS_1,
            2 => NWAITS_A::NWAITS_2,
            3 => NWAITS_A::NWAITS_3,
            4 => NWAITS_A::NWAITS_4,
            5 => NWAITS_A::NWAITS_5,
            6 => NWAITS_A::NWAITS_6,
            7 => NWAITS_A::NWAITS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "FRAM Wait state control: 0"]
    #[inline(always)]
    pub fn is_nwaits_0(&self) -> bool {
        *self == NWAITS_A::NWAITS_0
    }
    #[doc = "FRAM Wait state control: 1"]
    #[inline(always)]
    pub fn is_nwaits_1(&self) -> bool {
        *self == NWAITS_A::NWAITS_1
    }
    #[doc = "FRAM Wait state control: 2"]
    #[inline(always)]
    pub fn is_nwaits_2(&self) -> bool {
        *self == NWAITS_A::NWAITS_2
    }
    #[doc = "FRAM Wait state control: 3"]
    #[inline(always)]
    pub fn is_nwaits_3(&self) -> bool {
        *self == NWAITS_A::NWAITS_3
    }
    #[doc = "FRAM Wait state control: 4"]
    #[inline(always)]
    pub fn is_nwaits_4(&self) -> bool {
        *self == NWAITS_A::NWAITS_4
    }
    #[doc = "FRAM Wait state control: 5"]
    #[inline(always)]
    pub fn is_nwaits_5(&self) -> bool {
        *self == NWAITS_A::NWAITS_5
    }
    #[doc = "FRAM Wait state control: 6"]
    #[inline(always)]
    pub fn is_nwaits_6(&self) -> bool {
        *self == NWAITS_A::NWAITS_6
    }
    #[doc = "FRAM Wait state control: 7"]
    #[inline(always)]
    pub fn is_nwaits_7(&self) -> bool {
        *self == NWAITS_A::NWAITS_7
    }
}
#[doc = "Field `NWAITS` writer - FRAM Wait state control Bit: 0"]
pub type NWAITS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, NWAITS_A>;
impl<'a, REG> NWAITS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "FRAM Wait state control: 0"]
    #[inline(always)]
    pub fn nwaits_0(self) -> &'a mut crate::W<REG> {
        self.variant(NWAITS_A::NWAITS_0)
    }
    #[doc = "FRAM Wait state control: 1"]
    #[inline(always)]
    pub fn nwaits_1(self) -> &'a mut crate::W<REG> {
        self.variant(NWAITS_A::NWAITS_1)
    }
    #[doc = "FRAM Wait state control: 2"]
    #[inline(always)]
    pub fn nwaits_2(self) -> &'a mut crate::W<REG> {
        self.variant(NWAITS_A::NWAITS_2)
    }
    #[doc = "FRAM Wait state control: 3"]
    #[inline(always)]
    pub fn nwaits_3(self) -> &'a mut crate::W<REG> {
        self.variant(NWAITS_A::NWAITS_3)
    }
    #[doc = "FRAM Wait state control: 4"]
    #[inline(always)]
    pub fn nwaits_4(self) -> &'a mut crate::W<REG> {
        self.variant(NWAITS_A::NWAITS_4)
    }
    #[doc = "FRAM Wait state control: 5"]
    #[inline(always)]
    pub fn nwaits_5(self) -> &'a mut crate::W<REG> {
        self.variant(NWAITS_A::NWAITS_5)
    }
    #[doc = "FRAM Wait state control: 6"]
    #[inline(always)]
    pub fn nwaits_6(self) -> &'a mut crate::W<REG> {
        self.variant(NWAITS_A::NWAITS_6)
    }
    #[doc = "FRAM Wait state control: 7"]
    #[inline(always)]
    pub fn nwaits_7(self) -> &'a mut crate::W<REG> {
        self.variant(NWAITS_A::NWAITS_7)
    }
}
impl R {
    #[doc = "Bits 4:6 - FRAM Wait state control Bit: 0"]
    #[inline(always)]
    pub fn nwaits(&self) -> NWAITS_R {
        NWAITS_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - FRAM Wait state control Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn nwaits(&mut self) -> NWAITS_W<FRCTL0_SPEC> {
        NWAITS_W::new(self, 4)
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
#[doc = "FRAM Controller Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRCTL0_SPEC;
impl crate::RegisterSpec for FRCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`frctl0::R`](R) reader structure"]
impl crate::Readable for FRCTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`frctl0::W`](W) writer structure"]
impl crate::Writable for FRCTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRCTL0 to value 0"]
impl crate::Resettable for FRCTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
