#[doc = "Register `ADCCTL2` reader"]
pub type R = crate::R<ADCCTL2_SPEC>;
#[doc = "Register `ADCCTL2` writer"]
pub type W = crate::W<ADCCTL2_SPEC>;
#[doc = "Field `ADCSR` reader - ADC Sampling Rate"]
pub type ADCSR_R = crate::BitReader;
#[doc = "Field `ADCSR` writer - ADC Sampling Rate"]
pub type ADCSR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCDF` reader - ADC Data Format"]
pub type ADCDF_R = crate::BitReader;
#[doc = "Field `ADCDF` writer - ADC Data Format"]
pub type ADCDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCRES` reader - ADC Resolution"]
pub type ADCRES_R = crate::FieldReader<ADCRES_A>;
#[doc = "ADC Resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCRES_A {
    #[doc = "0: 8 bit"]
    ADCRES_0 = 0,
    #[doc = "1: 10 bit"]
    ADCRES_1 = 1,
    #[doc = "2: Reserved"]
    ADCRES_2 = 2,
    #[doc = "3: Reserved"]
    ADCRES_3 = 3,
}
impl From<ADCRES_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCRES_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADCRES_A {
    type Ux = u8;
}
impl ADCRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADCRES_A {
        match self.bits {
            0 => ADCRES_A::ADCRES_0,
            1 => ADCRES_A::ADCRES_1,
            2 => ADCRES_A::ADCRES_2,
            3 => ADCRES_A::ADCRES_3,
            _ => unreachable!(),
        }
    }
    #[doc = "8 bit"]
    #[inline(always)]
    pub fn is_adcres_0(&self) -> bool {
        *self == ADCRES_A::ADCRES_0
    }
    #[doc = "10 bit"]
    #[inline(always)]
    pub fn is_adcres_1(&self) -> bool {
        *self == ADCRES_A::ADCRES_1
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_adcres_2(&self) -> bool {
        *self == ADCRES_A::ADCRES_2
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_adcres_3(&self) -> bool {
        *self == ADCRES_A::ADCRES_3
    }
}
#[doc = "Field `ADCRES` writer - ADC Resolution"]
pub type ADCRES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ADCRES_A>;
impl<'a, REG> ADCRES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bit"]
    #[inline(always)]
    pub fn adcres_0(self) -> &'a mut crate::W<REG> {
        self.variant(ADCRES_A::ADCRES_0)
    }
    #[doc = "10 bit"]
    #[inline(always)]
    pub fn adcres_1(self) -> &'a mut crate::W<REG> {
        self.variant(ADCRES_A::ADCRES_1)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn adcres_2(self) -> &'a mut crate::W<REG> {
        self.variant(ADCRES_A::ADCRES_2)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn adcres_3(self) -> &'a mut crate::W<REG> {
        self.variant(ADCRES_A::ADCRES_3)
    }
}
#[doc = "Field `ADCPDIV` reader - ADC predivider Bit: 0"]
pub type ADCPDIV_R = crate::FieldReader<ADCPDIV_A>;
#[doc = "ADC predivider Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCPDIV_A {
    #[doc = "0: ADC predivider /1"]
    ADCPDIV_0 = 0,
    #[doc = "1: ADC predivider /2"]
    ADCPDIV_1 = 1,
    #[doc = "2: ADC predivider /64"]
    ADCPDIV_2 = 2,
    #[doc = "3: ADC predivider reserved"]
    ADCPDIV_3 = 3,
}
impl From<ADCPDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCPDIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADCPDIV_A {
    type Ux = u8;
}
impl ADCPDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADCPDIV_A {
        match self.bits {
            0 => ADCPDIV_A::ADCPDIV_0,
            1 => ADCPDIV_A::ADCPDIV_1,
            2 => ADCPDIV_A::ADCPDIV_2,
            3 => ADCPDIV_A::ADCPDIV_3,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC predivider /1"]
    #[inline(always)]
    pub fn is_adcpdiv_0(&self) -> bool {
        *self == ADCPDIV_A::ADCPDIV_0
    }
    #[doc = "ADC predivider /2"]
    #[inline(always)]
    pub fn is_adcpdiv_1(&self) -> bool {
        *self == ADCPDIV_A::ADCPDIV_1
    }
    #[doc = "ADC predivider /64"]
    #[inline(always)]
    pub fn is_adcpdiv_2(&self) -> bool {
        *self == ADCPDIV_A::ADCPDIV_2
    }
    #[doc = "ADC predivider reserved"]
    #[inline(always)]
    pub fn is_adcpdiv_3(&self) -> bool {
        *self == ADCPDIV_A::ADCPDIV_3
    }
}
#[doc = "Field `ADCPDIV` writer - ADC predivider Bit: 0"]
pub type ADCPDIV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ADCPDIV_A>;
impl<'a, REG> ADCPDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC predivider /1"]
    #[inline(always)]
    pub fn adcpdiv_0(self) -> &'a mut crate::W<REG> {
        self.variant(ADCPDIV_A::ADCPDIV_0)
    }
    #[doc = "ADC predivider /2"]
    #[inline(always)]
    pub fn adcpdiv_1(self) -> &'a mut crate::W<REG> {
        self.variant(ADCPDIV_A::ADCPDIV_1)
    }
    #[doc = "ADC predivider /64"]
    #[inline(always)]
    pub fn adcpdiv_2(self) -> &'a mut crate::W<REG> {
        self.variant(ADCPDIV_A::ADCPDIV_2)
    }
    #[doc = "ADC predivider reserved"]
    #[inline(always)]
    pub fn adcpdiv_3(self) -> &'a mut crate::W<REG> {
        self.variant(ADCPDIV_A::ADCPDIV_3)
    }
}
impl R {
    #[doc = "Bit 2 - ADC Sampling Rate"]
    #[inline(always)]
    pub fn adcsr(&self) -> ADCSR_R {
        ADCSR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC Data Format"]
    #[inline(always)]
    pub fn adcdf(&self) -> ADCDF_R {
        ADCDF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - ADC Resolution"]
    #[inline(always)]
    pub fn adcres(&self) -> ADCRES_R {
        ADCRES_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - ADC predivider Bit: 0"]
    #[inline(always)]
    pub fn adcpdiv(&self) -> ADCPDIV_R {
        ADCPDIV_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - ADC Sampling Rate"]
    #[inline(always)]
    #[must_use]
    pub fn adcsr(&mut self) -> ADCSR_W<ADCCTL2_SPEC> {
        ADCSR_W::new(self, 2)
    }
    #[doc = "Bit 3 - ADC Data Format"]
    #[inline(always)]
    #[must_use]
    pub fn adcdf(&mut self) -> ADCDF_W<ADCCTL2_SPEC> {
        ADCDF_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - ADC Resolution"]
    #[inline(always)]
    #[must_use]
    pub fn adcres(&mut self) -> ADCRES_W<ADCCTL2_SPEC> {
        ADCRES_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - ADC predivider Bit: 0"]
    #[inline(always)]
    #[must_use]
    pub fn adcpdiv(&mut self) -> ADCPDIV_W<ADCCTL2_SPEC> {
        ADCPDIV_W::new(self, 8)
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
#[doc = "ADC Control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcctl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcctl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCCTL2_SPEC;
impl crate::RegisterSpec for ADCCTL2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcctl2::R`](R) reader structure"]
impl crate::Readable for ADCCTL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adcctl2::W`](W) writer structure"]
impl crate::Writable for ADCCTL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCCTL2 to value 0"]
impl crate::Resettable for ADCCTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
