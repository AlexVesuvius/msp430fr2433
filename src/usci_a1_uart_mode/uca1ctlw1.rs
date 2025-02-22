#[doc = "Register `UCA1CTLW1` reader"]
pub type R = crate::R<UCA1CTLW1_SPEC>;
#[doc = "Register `UCA1CTLW1` writer"]
pub type W = crate::W<UCA1CTLW1_SPEC>;
#[doc = "Field `UCGLIT` reader - USCI Deglitch Time Bit 1"]
pub type UCGLIT_R = crate::FieldReader<UCGLIT_A>;
#[doc = "USCI Deglitch Time Bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UCGLIT_A {
    #[doc = "0: USCI Deglitch time: 0"]
    UCGLIT_0 = 0,
    #[doc = "1: USCI Deglitch time: 1"]
    UCGLIT_1 = 1,
    #[doc = "2: USCI Deglitch time: 2"]
    UCGLIT_2 = 2,
    #[doc = "3: USCI Deglitch time: 3"]
    UCGLIT_3 = 3,
}
impl From<UCGLIT_A> for u8 {
    #[inline(always)]
    fn from(variant: UCGLIT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UCGLIT_A {
    type Ux = u8;
}
impl UCGLIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UCGLIT_A {
        match self.bits {
            0 => UCGLIT_A::UCGLIT_0,
            1 => UCGLIT_A::UCGLIT_1,
            2 => UCGLIT_A::UCGLIT_2,
            3 => UCGLIT_A::UCGLIT_3,
            _ => unreachable!(),
        }
    }
    #[doc = "USCI Deglitch time: 0"]
    #[inline(always)]
    pub fn is_ucglit_0(&self) -> bool {
        *self == UCGLIT_A::UCGLIT_0
    }
    #[doc = "USCI Deglitch time: 1"]
    #[inline(always)]
    pub fn is_ucglit_1(&self) -> bool {
        *self == UCGLIT_A::UCGLIT_1
    }
    #[doc = "USCI Deglitch time: 2"]
    #[inline(always)]
    pub fn is_ucglit_2(&self) -> bool {
        *self == UCGLIT_A::UCGLIT_2
    }
    #[doc = "USCI Deglitch time: 3"]
    #[inline(always)]
    pub fn is_ucglit_3(&self) -> bool {
        *self == UCGLIT_A::UCGLIT_3
    }
}
#[doc = "Field `UCGLIT` writer - USCI Deglitch Time Bit 1"]
pub type UCGLIT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, UCGLIT_A>;
impl<'a, REG> UCGLIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USCI Deglitch time: 0"]
    #[inline(always)]
    pub fn ucglit_0(self) -> &'a mut crate::W<REG> {
        self.variant(UCGLIT_A::UCGLIT_0)
    }
    #[doc = "USCI Deglitch time: 1"]
    #[inline(always)]
    pub fn ucglit_1(self) -> &'a mut crate::W<REG> {
        self.variant(UCGLIT_A::UCGLIT_1)
    }
    #[doc = "USCI Deglitch time: 2"]
    #[inline(always)]
    pub fn ucglit_2(self) -> &'a mut crate::W<REG> {
        self.variant(UCGLIT_A::UCGLIT_2)
    }
    #[doc = "USCI Deglitch time: 3"]
    #[inline(always)]
    pub fn ucglit_3(self) -> &'a mut crate::W<REG> {
        self.variant(UCGLIT_A::UCGLIT_3)
    }
}
impl R {
    #[doc = "Bits 0:1 - USCI Deglitch Time Bit 1"]
    #[inline(always)]
    pub fn ucglit(&self) -> UCGLIT_R {
        UCGLIT_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - USCI Deglitch Time Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ucglit(&mut self) -> UCGLIT_W<UCA1CTLW1_SPEC> {
        UCGLIT_W::new(self, 0)
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
#[doc = "USCI A1 Control Word Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uca1ctlw1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uca1ctlw1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCA1CTLW1_SPEC;
impl crate::RegisterSpec for UCA1CTLW1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uca1ctlw1::R`](R) reader structure"]
impl crate::Readable for UCA1CTLW1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uca1ctlw1::W`](W) writer structure"]
impl crate::Writable for UCA1CTLW1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCA1CTLW1 to value 0"]
impl crate::Resettable for UCA1CTLW1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
