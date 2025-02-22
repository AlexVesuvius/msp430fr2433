#[doc = "Register `ADCMCTL0` reader"]
pub type R = crate::R<ADCMCTL0_SPEC>;
#[doc = "Register `ADCMCTL0` writer"]
pub type W = crate::W<ADCMCTL0_SPEC>;
#[doc = "Field `ADCINCH` reader - ADC Input Channel Select Bit 0"]
pub type ADCINCH_R = crate::FieldReader<ADCINCH_A>;
#[doc = "ADC Input Channel Select Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCINCH_A {
    #[doc = "0: ADC Input Channel 0"]
    ADCINCH_0 = 0,
    #[doc = "1: ADC Input Channel 1"]
    ADCINCH_1 = 1,
    #[doc = "2: ADC Input Channel 2"]
    ADCINCH_2 = 2,
    #[doc = "3: ADC Input Channel 3"]
    ADCINCH_3 = 3,
    #[doc = "4: ADC Input Channel 4"]
    ADCINCH_4 = 4,
    #[doc = "5: ADC Input Channel 5"]
    ADCINCH_5 = 5,
    #[doc = "6: ADC Input Channel 6"]
    ADCINCH_6 = 6,
    #[doc = "7: ADC Input Channel 7"]
    ADCINCH_7 = 7,
    #[doc = "8: ADC Input Channel 8"]
    ADCINCH_8 = 8,
    #[doc = "9: ADC Input Channel 9"]
    ADCINCH_9 = 9,
    #[doc = "10: ADC Input Channel 10"]
    ADCINCH_10 = 10,
    #[doc = "11: ADC Input Channel 11"]
    ADCINCH_11 = 11,
    #[doc = "12: ADC Input Channel 12"]
    ADCINCH_12 = 12,
    #[doc = "13: ADC Input Channel 13"]
    ADCINCH_13 = 13,
    #[doc = "14: ADC Input Channel 14"]
    ADCINCH_14 = 14,
    #[doc = "15: ADC Input Channel 15"]
    ADCINCH_15 = 15,
}
impl From<ADCINCH_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCINCH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADCINCH_A {
    type Ux = u8;
}
impl ADCINCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADCINCH_A {
        match self.bits {
            0 => ADCINCH_A::ADCINCH_0,
            1 => ADCINCH_A::ADCINCH_1,
            2 => ADCINCH_A::ADCINCH_2,
            3 => ADCINCH_A::ADCINCH_3,
            4 => ADCINCH_A::ADCINCH_4,
            5 => ADCINCH_A::ADCINCH_5,
            6 => ADCINCH_A::ADCINCH_6,
            7 => ADCINCH_A::ADCINCH_7,
            8 => ADCINCH_A::ADCINCH_8,
            9 => ADCINCH_A::ADCINCH_9,
            10 => ADCINCH_A::ADCINCH_10,
            11 => ADCINCH_A::ADCINCH_11,
            12 => ADCINCH_A::ADCINCH_12,
            13 => ADCINCH_A::ADCINCH_13,
            14 => ADCINCH_A::ADCINCH_14,
            15 => ADCINCH_A::ADCINCH_15,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC Input Channel 0"]
    #[inline(always)]
    pub fn is_adcinch_0(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_0
    }
    #[doc = "ADC Input Channel 1"]
    #[inline(always)]
    pub fn is_adcinch_1(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_1
    }
    #[doc = "ADC Input Channel 2"]
    #[inline(always)]
    pub fn is_adcinch_2(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_2
    }
    #[doc = "ADC Input Channel 3"]
    #[inline(always)]
    pub fn is_adcinch_3(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_3
    }
    #[doc = "ADC Input Channel 4"]
    #[inline(always)]
    pub fn is_adcinch_4(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_4
    }
    #[doc = "ADC Input Channel 5"]
    #[inline(always)]
    pub fn is_adcinch_5(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_5
    }
    #[doc = "ADC Input Channel 6"]
    #[inline(always)]
    pub fn is_adcinch_6(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_6
    }
    #[doc = "ADC Input Channel 7"]
    #[inline(always)]
    pub fn is_adcinch_7(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_7
    }
    #[doc = "ADC Input Channel 8"]
    #[inline(always)]
    pub fn is_adcinch_8(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_8
    }
    #[doc = "ADC Input Channel 9"]
    #[inline(always)]
    pub fn is_adcinch_9(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_9
    }
    #[doc = "ADC Input Channel 10"]
    #[inline(always)]
    pub fn is_adcinch_10(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_10
    }
    #[doc = "ADC Input Channel 11"]
    #[inline(always)]
    pub fn is_adcinch_11(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_11
    }
    #[doc = "ADC Input Channel 12"]
    #[inline(always)]
    pub fn is_adcinch_12(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_12
    }
    #[doc = "ADC Input Channel 13"]
    #[inline(always)]
    pub fn is_adcinch_13(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_13
    }
    #[doc = "ADC Input Channel 14"]
    #[inline(always)]
    pub fn is_adcinch_14(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_14
    }
    #[doc = "ADC Input Channel 15"]
    #[inline(always)]
    pub fn is_adcinch_15(&self) -> bool {
        *self == ADCINCH_A::ADCINCH_15
    }
}
#[doc = "Field `ADCINCH` writer - ADC Input Channel Select Bit 0"]
pub type ADCINCH_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, ADCINCH_A>;
impl<'a, REG> ADCINCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC Input Channel 0"]
    #[inline(always)]
    pub fn adcinch_0(self) -> &'a mut crate::W<REG> {
        self.variant(ADCINCH_A::ADCINCH_0)
    }
    #[doc = "ADC Input Channel 1"]
    #[inline(always)]
    pub fn adcinch_1(self) -> &'a mut crate::W<REG> {
        self.variant(ADCINCH_A::ADCINCH_1)
    }
    #[doc = "ADC Input Channel 2"]
    #[inline(always)]
    pub fn adcinch_2(self) -> &'a mut crate::W<REG> {
        self.variant(ADCINCH_A::ADCINCH_2)
    }
    #[doc = "ADC Input Channel 3"]
    #[inline(always)]
    pub fn adcinch_3(self) -> &'a mut crate::W<REG> {
        self.variant(ADCINCH_A::ADCINCH_3)
    }
    #[doc = "ADC Input Channel 4"]
    #[inline(always)]
    pub fn adcinch_4(self) -> &'a mut crate::W<REG> {
        self.variant(ADCINCH_A::ADCINCH_4)
    }
    #[doc = "ADC Input Channel 5"]
    #[inline(always)]
    pub fn adcinch_5(self) -> &'a mut crate::W<REG> {
        self.variant(ADCINCH_A::ADCINCH_5)
    }
    #[doc = "ADC Input Channel 6"]
    #[inline(always)]
    pub fn adcinch_6(self) -> &'a mut crate::W<REG> {
        self.variant(ADCINCH_A::ADCINCH_6)
    }
    #[doc = "ADC Input Channel 7"]
    #[inline(always)]
    pub fn adcinch_7(self) -> &'a mut crate::W<REG> {
        self.variant(ADCINCH_A::ADCINCH_7)
    }
    #[doc = "ADC Input Channel 8"]
    #[inline(always)]
    pub fn adcinch_8(self) -> &'a mut crate::W<REG> {
        self.variant(ADCINCH_A::ADCINCH_8)
    }
    #[doc = "ADC Input Channel 9"]
    #[inline(always)]
    pub fn adcinch_9(self) -> &'a mut crate::W<REG> {
        self.variant(ADCINCH_A::ADCINCH_9)
    }
    #[doc = "ADC Input Channel 10"]
    #[inline(always)]
    pub fn adcinch_10(self) -> &'a mut crate::W<REG> {
        self.variant(ADCINCH_A::ADCINCH_10)
    }
    #[doc = "ADC Input Channel 11"]
    #[inline(always)]
    pub fn adcinch_11(self) -> &'a mut crate::W<REG> {
        self.variant(ADCINCH_A::ADCINCH_11)
    }
    #[doc = "ADC Input Channel 12"]
    #[inline(always)]
    pub fn adcinch_12(self) -> &'a mut crate::W<REG> {
        self.variant(ADCINCH_A::ADCINCH_12)
    }
    #[doc = "ADC Input Channel 13"]
    #[inline(always)]
    pub fn adcinch_13(self) -> &'a mut crate::W<REG> {
        self.variant(ADCINCH_A::ADCINCH_13)
    }
    #[doc = "ADC Input Channel 14"]
    #[inline(always)]
    pub fn adcinch_14(self) -> &'a mut crate::W<REG> {
        self.variant(ADCINCH_A::ADCINCH_14)
    }
    #[doc = "ADC Input Channel 15"]
    #[inline(always)]
    pub fn adcinch_15(self) -> &'a mut crate::W<REG> {
        self.variant(ADCINCH_A::ADCINCH_15)
    }
}
#[doc = "Field `ADCSREF` reader - ADC Select Reference Bit 0"]
pub type ADCSREF_R = crate::FieldReader<ADCSREF_A>;
#[doc = "ADC Select Reference Bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCSREF_A {
    #[doc = "0: ADC Select Reference 0"]
    ADCSREF_0 = 0,
    #[doc = "1: ADC Select Reference 1"]
    ADCSREF_1 = 1,
    #[doc = "2: ADC Select Reference 2"]
    ADCSREF_2 = 2,
    #[doc = "3: ADC Select Reference 3"]
    ADCSREF_3 = 3,
    #[doc = "4: ADC Select Reference 4"]
    ADCSREF_4 = 4,
    #[doc = "5: ADC Select Reference 5"]
    ADCSREF_5 = 5,
    #[doc = "6: ADC Select Reference 6"]
    ADCSREF_6 = 6,
    #[doc = "7: ADC Select Reference 7"]
    ADCSREF_7 = 7,
}
impl From<ADCSREF_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCSREF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADCSREF_A {
    type Ux = u8;
}
impl ADCSREF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADCSREF_A {
        match self.bits {
            0 => ADCSREF_A::ADCSREF_0,
            1 => ADCSREF_A::ADCSREF_1,
            2 => ADCSREF_A::ADCSREF_2,
            3 => ADCSREF_A::ADCSREF_3,
            4 => ADCSREF_A::ADCSREF_4,
            5 => ADCSREF_A::ADCSREF_5,
            6 => ADCSREF_A::ADCSREF_6,
            7 => ADCSREF_A::ADCSREF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC Select Reference 0"]
    #[inline(always)]
    pub fn is_adcsref_0(&self) -> bool {
        *self == ADCSREF_A::ADCSREF_0
    }
    #[doc = "ADC Select Reference 1"]
    #[inline(always)]
    pub fn is_adcsref_1(&self) -> bool {
        *self == ADCSREF_A::ADCSREF_1
    }
    #[doc = "ADC Select Reference 2"]
    #[inline(always)]
    pub fn is_adcsref_2(&self) -> bool {
        *self == ADCSREF_A::ADCSREF_2
    }
    #[doc = "ADC Select Reference 3"]
    #[inline(always)]
    pub fn is_adcsref_3(&self) -> bool {
        *self == ADCSREF_A::ADCSREF_3
    }
    #[doc = "ADC Select Reference 4"]
    #[inline(always)]
    pub fn is_adcsref_4(&self) -> bool {
        *self == ADCSREF_A::ADCSREF_4
    }
    #[doc = "ADC Select Reference 5"]
    #[inline(always)]
    pub fn is_adcsref_5(&self) -> bool {
        *self == ADCSREF_A::ADCSREF_5
    }
    #[doc = "ADC Select Reference 6"]
    #[inline(always)]
    pub fn is_adcsref_6(&self) -> bool {
        *self == ADCSREF_A::ADCSREF_6
    }
    #[doc = "ADC Select Reference 7"]
    #[inline(always)]
    pub fn is_adcsref_7(&self) -> bool {
        *self == ADCSREF_A::ADCSREF_7
    }
}
#[doc = "Field `ADCSREF` writer - ADC Select Reference Bit 0"]
pub type ADCSREF_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, ADCSREF_A>;
impl<'a, REG> ADCSREF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC Select Reference 0"]
    #[inline(always)]
    pub fn adcsref_0(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSREF_A::ADCSREF_0)
    }
    #[doc = "ADC Select Reference 1"]
    #[inline(always)]
    pub fn adcsref_1(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSREF_A::ADCSREF_1)
    }
    #[doc = "ADC Select Reference 2"]
    #[inline(always)]
    pub fn adcsref_2(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSREF_A::ADCSREF_2)
    }
    #[doc = "ADC Select Reference 3"]
    #[inline(always)]
    pub fn adcsref_3(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSREF_A::ADCSREF_3)
    }
    #[doc = "ADC Select Reference 4"]
    #[inline(always)]
    pub fn adcsref_4(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSREF_A::ADCSREF_4)
    }
    #[doc = "ADC Select Reference 5"]
    #[inline(always)]
    pub fn adcsref_5(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSREF_A::ADCSREF_5)
    }
    #[doc = "ADC Select Reference 6"]
    #[inline(always)]
    pub fn adcsref_6(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSREF_A::ADCSREF_6)
    }
    #[doc = "ADC Select Reference 7"]
    #[inline(always)]
    pub fn adcsref_7(self) -> &'a mut crate::W<REG> {
        self.variant(ADCSREF_A::ADCSREF_7)
    }
}
impl R {
    #[doc = "Bits 0:3 - ADC Input Channel Select Bit 0"]
    #[inline(always)]
    pub fn adcinch(&self) -> ADCINCH_R {
        ADCINCH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - ADC Select Reference Bit 0"]
    #[inline(always)]
    pub fn adcsref(&self) -> ADCSREF_R {
        ADCSREF_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADC Input Channel Select Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn adcinch(&mut self) -> ADCINCH_W<ADCMCTL0_SPEC> {
        ADCINCH_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - ADC Select Reference Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn adcsref(&mut self) -> ADCSREF_W<ADCMCTL0_SPEC> {
        ADCSREF_W::new(self, 4)
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
#[doc = "ADC Memory Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcmctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcmctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCMCTL0_SPEC;
impl crate::RegisterSpec for ADCMCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcmctl0::R`](R) reader structure"]
impl crate::Readable for ADCMCTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adcmctl0::W`](W) writer structure"]
impl crate::Writable for ADCMCTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMCTL0 to value 0"]
impl crate::Resettable for ADCMCTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
