#[doc = "Register `CSCTL4` reader"]
pub type R = crate::R<CSCTL4_SPEC>;
#[doc = "Register `CSCTL4` writer"]
pub type W = crate::W<CSCTL4_SPEC>;
#[doc = "Field `SELMS` reader - MCLK and SMCLK Source Select Bit: 0"]
pub type SELMS_R = crate::FieldReader<SELMS_A>;
#[doc = "MCLK and SMCLK Source Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SELMS_A {
    #[doc = "0: MCLK and SMCLK Source Select 0"]
    SELMS_0 = 0,
    #[doc = "1: MCLK and SMCLK Source Select 1"]
    SELMS_1 = 1,
    #[doc = "2: MCLK and SMCLK Source Select 2"]
    SELMS_2 = 2,
    #[doc = "3: MCLK and SMCLK Source Select 3"]
    SELMS_3 = 3,
    #[doc = "4: MCLK and SMCLK Source Select 4"]
    SELMS_4 = 4,
    #[doc = "5: MCLK and SMCLK Source Select 5"]
    SELMS_5 = 5,
    #[doc = "6: MCLK and SMCLK Source Select 6"]
    SELMS_6 = 6,
    #[doc = "7: MCLK and SMCLK Source Select 7"]
    SELMS_7 = 7,
}
impl From<SELMS_A> for u8 {
    #[inline(always)]
    fn from(variant: SELMS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SELMS_A {
    type Ux = u8;
}
impl SELMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SELMS_A {
        match self.bits {
            0 => SELMS_A::SELMS_0,
            1 => SELMS_A::SELMS_1,
            2 => SELMS_A::SELMS_2,
            3 => SELMS_A::SELMS_3,
            4 => SELMS_A::SELMS_4,
            5 => SELMS_A::SELMS_5,
            6 => SELMS_A::SELMS_6,
            7 => SELMS_A::SELMS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "MCLK and SMCLK Source Select 0"]
    #[inline(always)]
    pub fn is_selms_0(&self) -> bool {
        *self == SELMS_A::SELMS_0
    }
    #[doc = "MCLK and SMCLK Source Select 1"]
    #[inline(always)]
    pub fn is_selms_1(&self) -> bool {
        *self == SELMS_A::SELMS_1
    }
    #[doc = "MCLK and SMCLK Source Select 2"]
    #[inline(always)]
    pub fn is_selms_2(&self) -> bool {
        *self == SELMS_A::SELMS_2
    }
    #[doc = "MCLK and SMCLK Source Select 3"]
    #[inline(always)]
    pub fn is_selms_3(&self) -> bool {
        *self == SELMS_A::SELMS_3
    }
    #[doc = "MCLK and SMCLK Source Select 4"]
    #[inline(always)]
    pub fn is_selms_4(&self) -> bool {
        *self == SELMS_A::SELMS_4
    }
    #[doc = "MCLK and SMCLK Source Select 5"]
    #[inline(always)]
    pub fn is_selms_5(&self) -> bool {
        *self == SELMS_A::SELMS_5
    }
    #[doc = "MCLK and SMCLK Source Select 6"]
    #[inline(always)]
    pub fn is_selms_6(&self) -> bool {
        *self == SELMS_A::SELMS_6
    }
    #[doc = "MCLK and SMCLK Source Select 7"]
    #[inline(always)]
    pub fn is_selms_7(&self) -> bool {
        *self == SELMS_A::SELMS_7
    }
}
#[doc = "Field `SELMS` writer - MCLK and SMCLK Source Select Bit: 0"]
pub type SELMS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, SELMS_A>;
impl<'a, REG> SELMS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MCLK and SMCLK Source Select 0"]
    #[inline(always)]
    pub fn selms_0(self) -> &'a mut crate::W<REG> {
        self.variant(SELMS_A::SELMS_0)
    }
    #[doc = "MCLK and SMCLK Source Select 1"]
    #[inline(always)]
    pub fn selms_1(self) -> &'a mut crate::W<REG> {
        self.variant(SELMS_A::SELMS_1)
    }
    #[doc = "MCLK and SMCLK Source Select 2"]
    #[inline(always)]
    pub fn selms_2(self) -> &'a mut crate::W<REG> {
        self.variant(SELMS_A::SELMS_2)
    }
    #[doc = "MCLK and SMCLK Source Select 3"]
    #[inline(always)]
    pub fn selms_3(self) -> &'a mut crate::W<REG> {
        self.variant(SELMS_A::SELMS_3)
    }
    #[doc = "MCLK and SMCLK Source Select 4"]
    #[inline(always)]
    pub fn selms_4(self) -> &'a mut crate::W<REG> {
        self.variant(SELMS_A::SELMS_4)
    }
    #[doc = "MCLK and SMCLK Source Select 5"]
    #[inline(always)]
    pub fn selms_5(self) -> &'a mut crate::W<REG> {
        self.variant(SELMS_A::SELMS_5)
    }
    #[doc = "MCLK and SMCLK Source Select 6"]
    #[inline(always)]
    pub fn selms_6(self) -> &'a mut crate::W<REG> {
        self.variant(SELMS_A::SELMS_6)
    }
    #[doc = "MCLK and SMCLK Source Select 7"]
    #[inline(always)]
    pub fn selms_7(self) -> &'a mut crate::W<REG> {
        self.variant(SELMS_A::SELMS_7)
    }
}
#[doc = "Field `SELA` reader - ACLK Source Select Bit: 0"]
pub type SELA_R = crate::BitReader;
#[doc = "Field `SELA` writer - ACLK Source Select Bit: 0"]
pub type SELA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - MCLK and SMCLK Source Select Bit: 0"]
    #[inline(always)]
    pub fn selms(&self) -> SELMS_R {
        SELMS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - ACLK Source Select Bit: 0"]
    #[inline(always)]
    pub fn sela(&self) -> SELA_R {
        SELA_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - MCLK and SMCLK Source Select Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn selms(&mut self) -> SELMS_W<CSCTL4_SPEC> {
        SELMS_W::new(self, 0)
    }
    #[doc = "Bit 8 - ACLK Source Select Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn sela(&mut self) -> SELA_W<CSCTL4_SPEC> {
        SELA_W::new(self, 8)
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
#[doc = "CS Control Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csctl4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csctl4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSCTL4_SPEC;
impl crate::RegisterSpec for CSCTL4_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`csctl4::R`](R) reader structure"]
impl crate::Readable for CSCTL4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csctl4::W`](W) writer structure"]
impl crate::Writable for CSCTL4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSCTL4 to value 0"]
impl crate::Resettable for CSCTL4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
