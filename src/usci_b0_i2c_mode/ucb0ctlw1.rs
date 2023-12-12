#[doc = "Register `UCB0CTLW1` reader"]
pub type R = crate::R<UCB0CTLW1_SPEC>;
#[doc = "Register `UCB0CTLW1` writer"]
pub type W = crate::W<UCB0CTLW1_SPEC>;
#[doc = "Field `UCGLIT` reader - USCI Deglitch time Bit: 1"]
pub type UCGLIT_R = crate::FieldReader<UCGLIT_A>;
#[doc = "USCI Deglitch time Bit: 1\n\nValue on reset: 0"]
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
#[doc = "Field `UCGLIT` writer - USCI Deglitch time Bit: 1"]
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
#[doc = "Field `UCASTP` reader - USCI Automatic Stop condition generation Bit: 1"]
pub type UCASTP_R = crate::FieldReader<UCASTP_A>;
#[doc = "USCI Automatic Stop condition generation Bit: 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UCASTP_A {
    #[doc = "0: USCI Automatic Stop condition generation: 0"]
    UCASTP_0 = 0,
    #[doc = "1: USCI Automatic Stop condition generation: 1"]
    UCASTP_1 = 1,
    #[doc = "2: USCI Automatic Stop condition generation: 2"]
    UCASTP_2 = 2,
    #[doc = "3: USCI Automatic Stop condition generation: 3"]
    UCASTP_3 = 3,
}
impl From<UCASTP_A> for u8 {
    #[inline(always)]
    fn from(variant: UCASTP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UCASTP_A {
    type Ux = u8;
}
impl UCASTP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UCASTP_A {
        match self.bits {
            0 => UCASTP_A::UCASTP_0,
            1 => UCASTP_A::UCASTP_1,
            2 => UCASTP_A::UCASTP_2,
            3 => UCASTP_A::UCASTP_3,
            _ => unreachable!(),
        }
    }
    #[doc = "USCI Automatic Stop condition generation: 0"]
    #[inline(always)]
    pub fn is_ucastp_0(&self) -> bool {
        *self == UCASTP_A::UCASTP_0
    }
    #[doc = "USCI Automatic Stop condition generation: 1"]
    #[inline(always)]
    pub fn is_ucastp_1(&self) -> bool {
        *self == UCASTP_A::UCASTP_1
    }
    #[doc = "USCI Automatic Stop condition generation: 2"]
    #[inline(always)]
    pub fn is_ucastp_2(&self) -> bool {
        *self == UCASTP_A::UCASTP_2
    }
    #[doc = "USCI Automatic Stop condition generation: 3"]
    #[inline(always)]
    pub fn is_ucastp_3(&self) -> bool {
        *self == UCASTP_A::UCASTP_3
    }
}
#[doc = "Field `UCASTP` writer - USCI Automatic Stop condition generation Bit: 1"]
pub type UCASTP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, UCASTP_A>;
impl<'a, REG> UCASTP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USCI Automatic Stop condition generation: 0"]
    #[inline(always)]
    pub fn ucastp_0(self) -> &'a mut crate::W<REG> {
        self.variant(UCASTP_A::UCASTP_0)
    }
    #[doc = "USCI Automatic Stop condition generation: 1"]
    #[inline(always)]
    pub fn ucastp_1(self) -> &'a mut crate::W<REG> {
        self.variant(UCASTP_A::UCASTP_1)
    }
    #[doc = "USCI Automatic Stop condition generation: 2"]
    #[inline(always)]
    pub fn ucastp_2(self) -> &'a mut crate::W<REG> {
        self.variant(UCASTP_A::UCASTP_2)
    }
    #[doc = "USCI Automatic Stop condition generation: 3"]
    #[inline(always)]
    pub fn ucastp_3(self) -> &'a mut crate::W<REG> {
        self.variant(UCASTP_A::UCASTP_3)
    }
}
#[doc = "Field `UCSWACK` reader - USCI Software controlled ACK"]
pub type UCSWACK_R = crate::BitReader;
#[doc = "Field `UCSWACK` writer - USCI Software controlled ACK"]
pub type UCSWACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCSTPNACK` reader - USCI Acknowledge Stop last byte"]
pub type UCSTPNACK_R = crate::BitReader;
#[doc = "Field `UCSTPNACK` writer - USCI Acknowledge Stop last byte"]
pub type UCSTPNACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCCLTO` reader - USCI Clock low timeout Bit: 1"]
pub type UCCLTO_R = crate::FieldReader<UCCLTO_A>;
#[doc = "USCI Clock low timeout Bit: 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UCCLTO_A {
    #[doc = "0: USCI Clock low timeout: 0"]
    UCCLTO_0 = 0,
    #[doc = "1: USCI Clock low timeout: 1"]
    UCCLTO_1 = 1,
    #[doc = "2: USCI Clock low timeout: 2"]
    UCCLTO_2 = 2,
    #[doc = "3: USCI Clock low timeout: 3"]
    UCCLTO_3 = 3,
}
impl From<UCCLTO_A> for u8 {
    #[inline(always)]
    fn from(variant: UCCLTO_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UCCLTO_A {
    type Ux = u8;
}
impl UCCLTO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UCCLTO_A {
        match self.bits {
            0 => UCCLTO_A::UCCLTO_0,
            1 => UCCLTO_A::UCCLTO_1,
            2 => UCCLTO_A::UCCLTO_2,
            3 => UCCLTO_A::UCCLTO_3,
            _ => unreachable!(),
        }
    }
    #[doc = "USCI Clock low timeout: 0"]
    #[inline(always)]
    pub fn is_ucclto_0(&self) -> bool {
        *self == UCCLTO_A::UCCLTO_0
    }
    #[doc = "USCI Clock low timeout: 1"]
    #[inline(always)]
    pub fn is_ucclto_1(&self) -> bool {
        *self == UCCLTO_A::UCCLTO_1
    }
    #[doc = "USCI Clock low timeout: 2"]
    #[inline(always)]
    pub fn is_ucclto_2(&self) -> bool {
        *self == UCCLTO_A::UCCLTO_2
    }
    #[doc = "USCI Clock low timeout: 3"]
    #[inline(always)]
    pub fn is_ucclto_3(&self) -> bool {
        *self == UCCLTO_A::UCCLTO_3
    }
}
#[doc = "Field `UCCLTO` writer - USCI Clock low timeout Bit: 1"]
pub type UCCLTO_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, UCCLTO_A>;
impl<'a, REG> UCCLTO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USCI Clock low timeout: 0"]
    #[inline(always)]
    pub fn ucclto_0(self) -> &'a mut crate::W<REG> {
        self.variant(UCCLTO_A::UCCLTO_0)
    }
    #[doc = "USCI Clock low timeout: 1"]
    #[inline(always)]
    pub fn ucclto_1(self) -> &'a mut crate::W<REG> {
        self.variant(UCCLTO_A::UCCLTO_1)
    }
    #[doc = "USCI Clock low timeout: 2"]
    #[inline(always)]
    pub fn ucclto_2(self) -> &'a mut crate::W<REG> {
        self.variant(UCCLTO_A::UCCLTO_2)
    }
    #[doc = "USCI Clock low timeout: 3"]
    #[inline(always)]
    pub fn ucclto_3(self) -> &'a mut crate::W<REG> {
        self.variant(UCCLTO_A::UCCLTO_3)
    }
}
#[doc = "Field `UCETXINT` reader - USCI Early UCTXIFG0"]
pub type UCETXINT_R = crate::BitReader;
#[doc = "Field `UCETXINT` writer - USCI Early UCTXIFG0"]
pub type UCETXINT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - USCI Deglitch time Bit: 1"]
    #[inline(always)]
    pub fn ucglit(&self) -> UCGLIT_R {
        UCGLIT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - USCI Automatic Stop condition generation Bit: 1"]
    #[inline(always)]
    pub fn ucastp(&self) -> UCASTP_R {
        UCASTP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - USCI Software controlled ACK"]
    #[inline(always)]
    pub fn ucswack(&self) -> UCSWACK_R {
        UCSWACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USCI Acknowledge Stop last byte"]
    #[inline(always)]
    pub fn ucstpnack(&self) -> UCSTPNACK_R {
        UCSTPNACK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - USCI Clock low timeout Bit: 1"]
    #[inline(always)]
    pub fn ucclto(&self) -> UCCLTO_R {
        UCCLTO_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - USCI Early UCTXIFG0"]
    #[inline(always)]
    pub fn ucetxint(&self) -> UCETXINT_R {
        UCETXINT_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - USCI Deglitch time Bit: 1"]
    #[inline(always)]
    #[must_use]
    pub fn ucglit(&mut self) -> UCGLIT_W<UCB0CTLW1_SPEC> {
        UCGLIT_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - USCI Automatic Stop condition generation Bit: 1"]
    #[inline(always)]
    #[must_use]
    pub fn ucastp(&mut self) -> UCASTP_W<UCB0CTLW1_SPEC> {
        UCASTP_W::new(self, 2)
    }
    #[doc = "Bit 4 - USCI Software controlled ACK"]
    #[inline(always)]
    #[must_use]
    pub fn ucswack(&mut self) -> UCSWACK_W<UCB0CTLW1_SPEC> {
        UCSWACK_W::new(self, 4)
    }
    #[doc = "Bit 5 - USCI Acknowledge Stop last byte"]
    #[inline(always)]
    #[must_use]
    pub fn ucstpnack(&mut self) -> UCSTPNACK_W<UCB0CTLW1_SPEC> {
        UCSTPNACK_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - USCI Clock low timeout Bit: 1"]
    #[inline(always)]
    #[must_use]
    pub fn ucclto(&mut self) -> UCCLTO_W<UCB0CTLW1_SPEC> {
        UCCLTO_W::new(self, 6)
    }
    #[doc = "Bit 8 - USCI Early UCTXIFG0"]
    #[inline(always)]
    #[must_use]
    pub fn ucetxint(&mut self) -> UCETXINT_W<UCB0CTLW1_SPEC> {
        UCETXINT_W::new(self, 8)
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
#[doc = "USCI B0 Control Word Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucb0ctlw1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucb0ctlw1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCB0CTLW1_SPEC;
impl crate::RegisterSpec for UCB0CTLW1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb0ctlw1::R`](R) reader structure"]
impl crate::Readable for UCB0CTLW1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucb0ctlw1::W`](W) writer structure"]
impl crate::Writable for UCB0CTLW1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCB0CTLW1 to value 0"]
impl crate::Resettable for UCB0CTLW1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
