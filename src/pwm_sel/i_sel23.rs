#[doc = "Register `I_SEL23` reader"]
pub struct R(crate::R<I_SEL23_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I_SEL23_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I_SEL23_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I_SEL23_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I_SEL23` writer"]
pub struct W(crate::W<I_SEL23_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I_SEL23_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<I_SEL23_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I_SEL23_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL20` reader - IN0 of PWM2 external input control."]
pub type SEL20_R = crate::FieldReader<u8, SEL20_A>;
#[doc = "IN0 of PWM2 external input control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL20_A {
    #[doc = "0: `0`"]
    EXTERNAL_PWM0_PIN = 0,
    #[doc = "1: `1`"]
    EXTERNAL_PWM1_PIN = 1,
    #[doc = "2: `10`"]
    EXTERNAL_PWM2_PIN = 2,
    #[doc = "3: `11`"]
    EXTERNAL_PWM3_PIN = 3,
}
impl From<SEL20_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL20_A) -> Self {
        variant as _
    }
}
impl SEL20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL20_A {
        match self.bits {
            0 => SEL20_A::EXTERNAL_PWM0_PIN,
            1 => SEL20_A::EXTERNAL_PWM1_PIN,
            2 => SEL20_A::EXTERNAL_PWM2_PIN,
            3 => SEL20_A::EXTERNAL_PWM3_PIN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM0_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm0_pin(&self) -> bool {
        *self == SEL20_A::EXTERNAL_PWM0_PIN
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM1_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm1_pin(&self) -> bool {
        *self == SEL20_A::EXTERNAL_PWM1_PIN
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM2_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm2_pin(&self) -> bool {
        *self == SEL20_A::EXTERNAL_PWM2_PIN
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM3_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm3_pin(&self) -> bool {
        *self == SEL20_A::EXTERNAL_PWM3_PIN
    }
}
#[doc = "Field `SEL20` writer - IN0 of PWM2 external input control."]
pub type SEL20_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, I_SEL23_SPEC, u8, SEL20_A, 2, O>;
impl<'a, const O: u8> SEL20_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn external_pwm0_pin(self) -> &'a mut W {
        self.variant(SEL20_A::EXTERNAL_PWM0_PIN)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn external_pwm1_pin(self) -> &'a mut W {
        self.variant(SEL20_A::EXTERNAL_PWM1_PIN)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn external_pwm2_pin(self) -> &'a mut W {
        self.variant(SEL20_A::EXTERNAL_PWM2_PIN)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn external_pwm3_pin(self) -> &'a mut W {
        self.variant(SEL20_A::EXTERNAL_PWM3_PIN)
    }
}
#[doc = "Field `SEL21` reader - IN1 of PWM2 external input control."]
pub type SEL21_R = crate::FieldReader<u8, SEL21_A>;
#[doc = "IN1 of PWM2 external input control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL21_A {
    #[doc = "0: `0`"]
    EXTERNAL_PWM0_PIN = 0,
    #[doc = "1: `1`"]
    EXTERNAL_PWM1_PIN = 1,
    #[doc = "2: `10`"]
    EXTERNAL_PWM2_PIN = 2,
    #[doc = "3: `11`"]
    EXTERNAL_PWM3_PIN = 3,
}
impl From<SEL21_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL21_A) -> Self {
        variant as _
    }
}
impl SEL21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL21_A {
        match self.bits {
            0 => SEL21_A::EXTERNAL_PWM0_PIN,
            1 => SEL21_A::EXTERNAL_PWM1_PIN,
            2 => SEL21_A::EXTERNAL_PWM2_PIN,
            3 => SEL21_A::EXTERNAL_PWM3_PIN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM0_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm0_pin(&self) -> bool {
        *self == SEL21_A::EXTERNAL_PWM0_PIN
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM1_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm1_pin(&self) -> bool {
        *self == SEL21_A::EXTERNAL_PWM1_PIN
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM2_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm2_pin(&self) -> bool {
        *self == SEL21_A::EXTERNAL_PWM2_PIN
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM3_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm3_pin(&self) -> bool {
        *self == SEL21_A::EXTERNAL_PWM3_PIN
    }
}
#[doc = "Field `SEL21` writer - IN1 of PWM2 external input control."]
pub type SEL21_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, I_SEL23_SPEC, u8, SEL21_A, 2, O>;
impl<'a, const O: u8> SEL21_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn external_pwm0_pin(self) -> &'a mut W {
        self.variant(SEL21_A::EXTERNAL_PWM0_PIN)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn external_pwm1_pin(self) -> &'a mut W {
        self.variant(SEL21_A::EXTERNAL_PWM1_PIN)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn external_pwm2_pin(self) -> &'a mut W {
        self.variant(SEL21_A::EXTERNAL_PWM2_PIN)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn external_pwm3_pin(self) -> &'a mut W {
        self.variant(SEL21_A::EXTERNAL_PWM3_PIN)
    }
}
#[doc = "Field `SEL22` reader - IN2 of PWM2 external input control."]
pub type SEL22_R = crate::FieldReader<u8, SEL22_A>;
#[doc = "IN2 of PWM2 external input control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL22_A {
    #[doc = "0: `0`"]
    EXTERNAL_PWM0_PIN = 0,
    #[doc = "1: `1`"]
    EXTERNAL_PWM1_PIN = 1,
    #[doc = "2: `10`"]
    EXTERNAL_PWM2_PIN = 2,
    #[doc = "3: `11`"]
    EXTERNAL_PWM3_PIN = 3,
}
impl From<SEL22_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL22_A) -> Self {
        variant as _
    }
}
impl SEL22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL22_A {
        match self.bits {
            0 => SEL22_A::EXTERNAL_PWM0_PIN,
            1 => SEL22_A::EXTERNAL_PWM1_PIN,
            2 => SEL22_A::EXTERNAL_PWM2_PIN,
            3 => SEL22_A::EXTERNAL_PWM3_PIN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM0_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm0_pin(&self) -> bool {
        *self == SEL22_A::EXTERNAL_PWM0_PIN
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM1_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm1_pin(&self) -> bool {
        *self == SEL22_A::EXTERNAL_PWM1_PIN
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM2_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm2_pin(&self) -> bool {
        *self == SEL22_A::EXTERNAL_PWM2_PIN
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM3_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm3_pin(&self) -> bool {
        *self == SEL22_A::EXTERNAL_PWM3_PIN
    }
}
#[doc = "Field `SEL22` writer - IN2 of PWM2 external input control."]
pub type SEL22_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, I_SEL23_SPEC, u8, SEL22_A, 2, O>;
impl<'a, const O: u8> SEL22_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn external_pwm0_pin(self) -> &'a mut W {
        self.variant(SEL22_A::EXTERNAL_PWM0_PIN)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn external_pwm1_pin(self) -> &'a mut W {
        self.variant(SEL22_A::EXTERNAL_PWM1_PIN)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn external_pwm2_pin(self) -> &'a mut W {
        self.variant(SEL22_A::EXTERNAL_PWM2_PIN)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn external_pwm3_pin(self) -> &'a mut W {
        self.variant(SEL22_A::EXTERNAL_PWM3_PIN)
    }
}
#[doc = "Field `SEL30` reader - IN0 of PWM3 external input control."]
pub type SEL30_R = crate::FieldReader<u8, SEL30_A>;
#[doc = "IN0 of PWM3 external input control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL30_A {
    #[doc = "0: `0`"]
    EXTERNAL_PWM0_PIN = 0,
    #[doc = "1: `1`"]
    EXTERNAL_PWM1_PIN = 1,
    #[doc = "2: `10`"]
    EXTERNAL_PWM2_PIN = 2,
    #[doc = "3: `11`"]
    EXTERNAL_PWM3_PIN = 3,
}
impl From<SEL30_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL30_A) -> Self {
        variant as _
    }
}
impl SEL30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL30_A {
        match self.bits {
            0 => SEL30_A::EXTERNAL_PWM0_PIN,
            1 => SEL30_A::EXTERNAL_PWM1_PIN,
            2 => SEL30_A::EXTERNAL_PWM2_PIN,
            3 => SEL30_A::EXTERNAL_PWM3_PIN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM0_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm0_pin(&self) -> bool {
        *self == SEL30_A::EXTERNAL_PWM0_PIN
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM1_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm1_pin(&self) -> bool {
        *self == SEL30_A::EXTERNAL_PWM1_PIN
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM2_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm2_pin(&self) -> bool {
        *self == SEL30_A::EXTERNAL_PWM2_PIN
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM3_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm3_pin(&self) -> bool {
        *self == SEL30_A::EXTERNAL_PWM3_PIN
    }
}
#[doc = "Field `SEL30` writer - IN0 of PWM3 external input control."]
pub type SEL30_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, I_SEL23_SPEC, u8, SEL30_A, 2, O>;
impl<'a, const O: u8> SEL30_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn external_pwm0_pin(self) -> &'a mut W {
        self.variant(SEL30_A::EXTERNAL_PWM0_PIN)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn external_pwm1_pin(self) -> &'a mut W {
        self.variant(SEL30_A::EXTERNAL_PWM1_PIN)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn external_pwm2_pin(self) -> &'a mut W {
        self.variant(SEL30_A::EXTERNAL_PWM2_PIN)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn external_pwm3_pin(self) -> &'a mut W {
        self.variant(SEL30_A::EXTERNAL_PWM3_PIN)
    }
}
#[doc = "Field `SEL31` reader - IN1 of PWM3 external input control."]
pub type SEL31_R = crate::FieldReader<u8, SEL31_A>;
#[doc = "IN1 of PWM3 external input control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL31_A {
    #[doc = "0: `0`"]
    EXTERNAL_PWM0_PIN = 0,
    #[doc = "1: `1`"]
    EXTERNAL_PWM1_PIN = 1,
    #[doc = "2: `10`"]
    EXTERNAL_PWM2_PIN = 2,
    #[doc = "3: `11`"]
    EXTERNAL_PWM3_PIN = 3,
}
impl From<SEL31_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL31_A) -> Self {
        variant as _
    }
}
impl SEL31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL31_A {
        match self.bits {
            0 => SEL31_A::EXTERNAL_PWM0_PIN,
            1 => SEL31_A::EXTERNAL_PWM1_PIN,
            2 => SEL31_A::EXTERNAL_PWM2_PIN,
            3 => SEL31_A::EXTERNAL_PWM3_PIN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM0_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm0_pin(&self) -> bool {
        *self == SEL31_A::EXTERNAL_PWM0_PIN
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM1_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm1_pin(&self) -> bool {
        *self == SEL31_A::EXTERNAL_PWM1_PIN
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM2_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm2_pin(&self) -> bool {
        *self == SEL31_A::EXTERNAL_PWM2_PIN
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM3_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm3_pin(&self) -> bool {
        *self == SEL31_A::EXTERNAL_PWM3_PIN
    }
}
#[doc = "Field `SEL31` writer - IN1 of PWM3 external input control."]
pub type SEL31_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, I_SEL23_SPEC, u8, SEL31_A, 2, O>;
impl<'a, const O: u8> SEL31_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn external_pwm0_pin(self) -> &'a mut W {
        self.variant(SEL31_A::EXTERNAL_PWM0_PIN)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn external_pwm1_pin(self) -> &'a mut W {
        self.variant(SEL31_A::EXTERNAL_PWM1_PIN)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn external_pwm2_pin(self) -> &'a mut W {
        self.variant(SEL31_A::EXTERNAL_PWM2_PIN)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn external_pwm3_pin(self) -> &'a mut W {
        self.variant(SEL31_A::EXTERNAL_PWM3_PIN)
    }
}
#[doc = "Field `SEL32` reader - IN2 of PWM3 external input control."]
pub type SEL32_R = crate::FieldReader<u8, SEL32_A>;
#[doc = "IN2 of PWM3 external input control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL32_A {
    #[doc = "0: `0`"]
    EXTERNAL_PWM0_PIN = 0,
    #[doc = "1: `1`"]
    EXTERNAL_PWM1_PIN = 1,
    #[doc = "2: `10`"]
    EXTERNAL_PWM2_PIN = 2,
    #[doc = "3: `11`"]
    EXTERNAL_PWM3_PIN = 3,
}
impl From<SEL32_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL32_A) -> Self {
        variant as _
    }
}
impl SEL32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL32_A {
        match self.bits {
            0 => SEL32_A::EXTERNAL_PWM0_PIN,
            1 => SEL32_A::EXTERNAL_PWM1_PIN,
            2 => SEL32_A::EXTERNAL_PWM2_PIN,
            3 => SEL32_A::EXTERNAL_PWM3_PIN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM0_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm0_pin(&self) -> bool {
        *self == SEL32_A::EXTERNAL_PWM0_PIN
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM1_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm1_pin(&self) -> bool {
        *self == SEL32_A::EXTERNAL_PWM1_PIN
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM2_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm2_pin(&self) -> bool {
        *self == SEL32_A::EXTERNAL_PWM2_PIN
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM3_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm3_pin(&self) -> bool {
        *self == SEL32_A::EXTERNAL_PWM3_PIN
    }
}
#[doc = "Field `SEL32` writer - IN2 of PWM3 external input control."]
pub type SEL32_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, I_SEL23_SPEC, u8, SEL32_A, 2, O>;
impl<'a, const O: u8> SEL32_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn external_pwm0_pin(self) -> &'a mut W {
        self.variant(SEL32_A::EXTERNAL_PWM0_PIN)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn external_pwm1_pin(self) -> &'a mut W {
        self.variant(SEL32_A::EXTERNAL_PWM1_PIN)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn external_pwm2_pin(self) -> &'a mut W {
        self.variant(SEL32_A::EXTERNAL_PWM2_PIN)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn external_pwm3_pin(self) -> &'a mut W {
        self.variant(SEL32_A::EXTERNAL_PWM3_PIN)
    }
}
impl R {
    #[doc = "Bits 0:1 - IN0 of PWM2 external input control."]
    #[inline(always)]
    pub fn sel20(&self) -> SEL20_R {
        SEL20_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - IN1 of PWM2 external input control."]
    #[inline(always)]
    pub fn sel21(&self) -> SEL21_R {
        SEL21_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - IN2 of PWM2 external input control."]
    #[inline(always)]
    pub fn sel22(&self) -> SEL22_R {
        SEL22_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:17 - IN0 of PWM3 external input control."]
    #[inline(always)]
    pub fn sel30(&self) -> SEL30_R {
        SEL30_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - IN1 of PWM3 external input control."]
    #[inline(always)]
    pub fn sel31(&self) -> SEL31_R {
        SEL31_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - IN2 of PWM3 external input control."]
    #[inline(always)]
    pub fn sel32(&self) -> SEL32_R {
        SEL32_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - IN0 of PWM2 external input control."]
    #[inline(always)]
    #[must_use]
    pub fn sel20(&mut self) -> SEL20_W<0> {
        SEL20_W::new(self)
    }
    #[doc = "Bits 2:3 - IN1 of PWM2 external input control."]
    #[inline(always)]
    #[must_use]
    pub fn sel21(&mut self) -> SEL21_W<2> {
        SEL21_W::new(self)
    }
    #[doc = "Bits 4:5 - IN2 of PWM2 external input control."]
    #[inline(always)]
    #[must_use]
    pub fn sel22(&mut self) -> SEL22_W<4> {
        SEL22_W::new(self)
    }
    #[doc = "Bits 16:17 - IN0 of PWM3 external input control."]
    #[inline(always)]
    #[must_use]
    pub fn sel30(&mut self) -> SEL30_W<16> {
        SEL30_W::new(self)
    }
    #[doc = "Bits 18:19 - IN1 of PWM3 external input control."]
    #[inline(always)]
    #[must_use]
    pub fn sel31(&mut self) -> SEL31_W<18> {
        SEL31_W::new(self)
    }
    #[doc = "Bits 20:21 - IN2 of PWM3 external input control."]
    #[inline(always)]
    #[must_use]
    pub fn sel32(&mut self) -> SEL32_W<20> {
        SEL32_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input of PWM2 and PWM3 selection register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i_sel23](index.html) module"]
pub struct I_SEL23_SPEC;
impl crate::RegisterSpec for I_SEL23_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i_sel23::R](R) reader structure"]
impl crate::Readable for I_SEL23_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i_sel23::W](W) writer structure"]
impl crate::Writable for I_SEL23_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I_SEL23 to value 0"]
impl crate::Resettable for I_SEL23_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
