#[doc = "Register `I_SEL01` reader"]
pub struct R(crate::R<I_SEL01_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I_SEL01_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I_SEL01_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I_SEL01_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I_SEL01` writer"]
pub struct W(crate::W<I_SEL01_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I_SEL01_SPEC>;
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
impl From<crate::W<I_SEL01_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I_SEL01_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL00` reader - IN0 of PWM0 external input control."]
pub type SEL00_R = crate::FieldReader<u8, SEL00_A>;
#[doc = "IN0 of PWM0 external input control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL00_A {
    #[doc = "0: `0`"]
    EXTERNAL_PWM0_PIN = 0,
    #[doc = "1: `1`"]
    EXTERNAL_PWM1_PIN = 1,
    #[doc = "2: `10`"]
    EXTERNAL_PWM2_PIN = 2,
    #[doc = "3: `11`"]
    EXTERNAL_PWM3_PIN = 3,
}
impl From<SEL00_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL00_A) -> Self {
        variant as _
    }
}
impl SEL00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL00_A {
        match self.bits {
            0 => SEL00_A::EXTERNAL_PWM0_PIN,
            1 => SEL00_A::EXTERNAL_PWM1_PIN,
            2 => SEL00_A::EXTERNAL_PWM2_PIN,
            3 => SEL00_A::EXTERNAL_PWM3_PIN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM0_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm0_pin(&self) -> bool {
        *self == SEL00_A::EXTERNAL_PWM0_PIN
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM1_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm1_pin(&self) -> bool {
        *self == SEL00_A::EXTERNAL_PWM1_PIN
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM2_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm2_pin(&self) -> bool {
        *self == SEL00_A::EXTERNAL_PWM2_PIN
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM3_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm3_pin(&self) -> bool {
        *self == SEL00_A::EXTERNAL_PWM3_PIN
    }
}
#[doc = "Field `SEL00` writer - IN0 of PWM0 external input control."]
pub type SEL00_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, I_SEL01_SPEC, u8, SEL00_A, 2, O>;
impl<'a, const O: u8> SEL00_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn external_pwm0_pin(self) -> &'a mut W {
        self.variant(SEL00_A::EXTERNAL_PWM0_PIN)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn external_pwm1_pin(self) -> &'a mut W {
        self.variant(SEL00_A::EXTERNAL_PWM1_PIN)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn external_pwm2_pin(self) -> &'a mut W {
        self.variant(SEL00_A::EXTERNAL_PWM2_PIN)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn external_pwm3_pin(self) -> &'a mut W {
        self.variant(SEL00_A::EXTERNAL_PWM3_PIN)
    }
}
#[doc = "Field `SEL01` reader - IN1 of PWM0 external input control."]
pub type SEL01_R = crate::FieldReader<u8, SEL01_A>;
#[doc = "IN1 of PWM0 external input control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL01_A {
    #[doc = "0: `0`"]
    EXTERNAL_PWM0_PIN = 0,
    #[doc = "1: `1`"]
    EXTERNAL_PWM1_PIN = 1,
    #[doc = "2: `10`"]
    EXTERNAL_PWM2_PIN = 2,
    #[doc = "3: `11`"]
    EXTERNAL_PWM3_PIN = 3,
}
impl From<SEL01_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL01_A) -> Self {
        variant as _
    }
}
impl SEL01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL01_A {
        match self.bits {
            0 => SEL01_A::EXTERNAL_PWM0_PIN,
            1 => SEL01_A::EXTERNAL_PWM1_PIN,
            2 => SEL01_A::EXTERNAL_PWM2_PIN,
            3 => SEL01_A::EXTERNAL_PWM3_PIN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM0_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm0_pin(&self) -> bool {
        *self == SEL01_A::EXTERNAL_PWM0_PIN
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM1_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm1_pin(&self) -> bool {
        *self == SEL01_A::EXTERNAL_PWM1_PIN
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM2_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm2_pin(&self) -> bool {
        *self == SEL01_A::EXTERNAL_PWM2_PIN
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM3_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm3_pin(&self) -> bool {
        *self == SEL01_A::EXTERNAL_PWM3_PIN
    }
}
#[doc = "Field `SEL01` writer - IN1 of PWM0 external input control."]
pub type SEL01_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, I_SEL01_SPEC, u8, SEL01_A, 2, O>;
impl<'a, const O: u8> SEL01_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn external_pwm0_pin(self) -> &'a mut W {
        self.variant(SEL01_A::EXTERNAL_PWM0_PIN)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn external_pwm1_pin(self) -> &'a mut W {
        self.variant(SEL01_A::EXTERNAL_PWM1_PIN)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn external_pwm2_pin(self) -> &'a mut W {
        self.variant(SEL01_A::EXTERNAL_PWM2_PIN)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn external_pwm3_pin(self) -> &'a mut W {
        self.variant(SEL01_A::EXTERNAL_PWM3_PIN)
    }
}
#[doc = "Field `SEL02` reader - IN2 of PWM0 external input control."]
pub type SEL02_R = crate::FieldReader<u8, SEL02_A>;
#[doc = "IN2 of PWM0 external input control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL02_A {
    #[doc = "0: `0`"]
    EXTERNAL_PWM0_PIN = 0,
    #[doc = "1: `1`"]
    EXTERNAL_PWM1_PIN = 1,
    #[doc = "2: `10`"]
    EXTERNAL_PWM2_PIN = 2,
    #[doc = "3: `11`"]
    EXTERNAL_PWM3_PIN = 3,
}
impl From<SEL02_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL02_A) -> Self {
        variant as _
    }
}
impl SEL02_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL02_A {
        match self.bits {
            0 => SEL02_A::EXTERNAL_PWM0_PIN,
            1 => SEL02_A::EXTERNAL_PWM1_PIN,
            2 => SEL02_A::EXTERNAL_PWM2_PIN,
            3 => SEL02_A::EXTERNAL_PWM3_PIN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM0_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm0_pin(&self) -> bool {
        *self == SEL02_A::EXTERNAL_PWM0_PIN
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM1_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm1_pin(&self) -> bool {
        *self == SEL02_A::EXTERNAL_PWM1_PIN
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM2_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm2_pin(&self) -> bool {
        *self == SEL02_A::EXTERNAL_PWM2_PIN
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM3_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm3_pin(&self) -> bool {
        *self == SEL02_A::EXTERNAL_PWM3_PIN
    }
}
#[doc = "Field `SEL02` writer - IN2 of PWM0 external input control."]
pub type SEL02_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, I_SEL01_SPEC, u8, SEL02_A, 2, O>;
impl<'a, const O: u8> SEL02_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn external_pwm0_pin(self) -> &'a mut W {
        self.variant(SEL02_A::EXTERNAL_PWM0_PIN)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn external_pwm1_pin(self) -> &'a mut W {
        self.variant(SEL02_A::EXTERNAL_PWM1_PIN)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn external_pwm2_pin(self) -> &'a mut W {
        self.variant(SEL02_A::EXTERNAL_PWM2_PIN)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn external_pwm3_pin(self) -> &'a mut W {
        self.variant(SEL02_A::EXTERNAL_PWM3_PIN)
    }
}
#[doc = "Field `SEL10` reader - IN0 of PWM1 external input control."]
pub type SEL10_R = crate::FieldReader<u8, SEL10_A>;
#[doc = "IN0 of PWM1 external input control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL10_A {
    #[doc = "0: `0`"]
    EXTERNAL_PWM0_PIN = 0,
    #[doc = "1: `1`"]
    EXTERNAL_PWM1_PIN = 1,
    #[doc = "2: `10`"]
    EXTERNAL_PWM2_PIN = 2,
    #[doc = "3: `11`"]
    EXTERNAL_PWM3_PIN = 3,
}
impl From<SEL10_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL10_A) -> Self {
        variant as _
    }
}
impl SEL10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL10_A {
        match self.bits {
            0 => SEL10_A::EXTERNAL_PWM0_PIN,
            1 => SEL10_A::EXTERNAL_PWM1_PIN,
            2 => SEL10_A::EXTERNAL_PWM2_PIN,
            3 => SEL10_A::EXTERNAL_PWM3_PIN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM0_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm0_pin(&self) -> bool {
        *self == SEL10_A::EXTERNAL_PWM0_PIN
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM1_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm1_pin(&self) -> bool {
        *self == SEL10_A::EXTERNAL_PWM1_PIN
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM2_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm2_pin(&self) -> bool {
        *self == SEL10_A::EXTERNAL_PWM2_PIN
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM3_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm3_pin(&self) -> bool {
        *self == SEL10_A::EXTERNAL_PWM3_PIN
    }
}
#[doc = "Field `SEL10` writer - IN0 of PWM1 external input control."]
pub type SEL10_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, I_SEL01_SPEC, u8, SEL10_A, 2, O>;
impl<'a, const O: u8> SEL10_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn external_pwm0_pin(self) -> &'a mut W {
        self.variant(SEL10_A::EXTERNAL_PWM0_PIN)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn external_pwm1_pin(self) -> &'a mut W {
        self.variant(SEL10_A::EXTERNAL_PWM1_PIN)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn external_pwm2_pin(self) -> &'a mut W {
        self.variant(SEL10_A::EXTERNAL_PWM2_PIN)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn external_pwm3_pin(self) -> &'a mut W {
        self.variant(SEL10_A::EXTERNAL_PWM3_PIN)
    }
}
#[doc = "Field `SEL11` reader - IN1 of PWM1 external input control."]
pub type SEL11_R = crate::FieldReader<u8, SEL11_A>;
#[doc = "IN1 of PWM1 external input control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL11_A {
    #[doc = "0: `0`"]
    EXTERNAL_PWM0_PIN = 0,
    #[doc = "1: `1`"]
    EXTERNAL_PWM1_PIN = 1,
    #[doc = "2: `10`"]
    EXTERNAL_PWM2_PIN = 2,
    #[doc = "3: `11`"]
    EXTERNAL_PWM3_PIN = 3,
}
impl From<SEL11_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL11_A) -> Self {
        variant as _
    }
}
impl SEL11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL11_A {
        match self.bits {
            0 => SEL11_A::EXTERNAL_PWM0_PIN,
            1 => SEL11_A::EXTERNAL_PWM1_PIN,
            2 => SEL11_A::EXTERNAL_PWM2_PIN,
            3 => SEL11_A::EXTERNAL_PWM3_PIN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM0_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm0_pin(&self) -> bool {
        *self == SEL11_A::EXTERNAL_PWM0_PIN
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM1_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm1_pin(&self) -> bool {
        *self == SEL11_A::EXTERNAL_PWM1_PIN
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM2_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm2_pin(&self) -> bool {
        *self == SEL11_A::EXTERNAL_PWM2_PIN
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM3_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm3_pin(&self) -> bool {
        *self == SEL11_A::EXTERNAL_PWM3_PIN
    }
}
#[doc = "Field `SEL11` writer - IN1 of PWM1 external input control."]
pub type SEL11_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, I_SEL01_SPEC, u8, SEL11_A, 2, O>;
impl<'a, const O: u8> SEL11_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn external_pwm0_pin(self) -> &'a mut W {
        self.variant(SEL11_A::EXTERNAL_PWM0_PIN)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn external_pwm1_pin(self) -> &'a mut W {
        self.variant(SEL11_A::EXTERNAL_PWM1_PIN)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn external_pwm2_pin(self) -> &'a mut W {
        self.variant(SEL11_A::EXTERNAL_PWM2_PIN)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn external_pwm3_pin(self) -> &'a mut W {
        self.variant(SEL11_A::EXTERNAL_PWM3_PIN)
    }
}
#[doc = "Field `SEL12` reader - IN2 of PWM1 external input control."]
pub type SEL12_R = crate::FieldReader<u8, SEL12_A>;
#[doc = "IN2 of PWM1 external input control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL12_A {
    #[doc = "0: `0`"]
    EXTERNAL_PWM0_PIN = 0,
    #[doc = "1: `1`"]
    EXTERNAL_PWM1_PIN = 1,
    #[doc = "2: `10`"]
    EXTERNAL_PWM2_PIN = 2,
    #[doc = "3: `11`"]
    EXTERNAL_PWM3_PIN = 3,
}
impl From<SEL12_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL12_A) -> Self {
        variant as _
    }
}
impl SEL12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL12_A {
        match self.bits {
            0 => SEL12_A::EXTERNAL_PWM0_PIN,
            1 => SEL12_A::EXTERNAL_PWM1_PIN,
            2 => SEL12_A::EXTERNAL_PWM2_PIN,
            3 => SEL12_A::EXTERNAL_PWM3_PIN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM0_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm0_pin(&self) -> bool {
        *self == SEL12_A::EXTERNAL_PWM0_PIN
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM1_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm1_pin(&self) -> bool {
        *self == SEL12_A::EXTERNAL_PWM1_PIN
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM2_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm2_pin(&self) -> bool {
        *self == SEL12_A::EXTERNAL_PWM2_PIN
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_PWM3_PIN`"]
    #[inline(always)]
    pub fn is_external_pwm3_pin(&self) -> bool {
        *self == SEL12_A::EXTERNAL_PWM3_PIN
    }
}
#[doc = "Field `SEL12` writer - IN2 of PWM1 external input control."]
pub type SEL12_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, I_SEL01_SPEC, u8, SEL12_A, 2, O>;
impl<'a, const O: u8> SEL12_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn external_pwm0_pin(self) -> &'a mut W {
        self.variant(SEL12_A::EXTERNAL_PWM0_PIN)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn external_pwm1_pin(self) -> &'a mut W {
        self.variant(SEL12_A::EXTERNAL_PWM1_PIN)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn external_pwm2_pin(self) -> &'a mut W {
        self.variant(SEL12_A::EXTERNAL_PWM2_PIN)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn external_pwm3_pin(self) -> &'a mut W {
        self.variant(SEL12_A::EXTERNAL_PWM3_PIN)
    }
}
impl R {
    #[doc = "Bits 0:1 - IN0 of PWM0 external input control."]
    #[inline(always)]
    pub fn sel00(&self) -> SEL00_R {
        SEL00_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - IN1 of PWM0 external input control."]
    #[inline(always)]
    pub fn sel01(&self) -> SEL01_R {
        SEL01_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - IN2 of PWM0 external input control."]
    #[inline(always)]
    pub fn sel02(&self) -> SEL02_R {
        SEL02_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:17 - IN0 of PWM1 external input control."]
    #[inline(always)]
    pub fn sel10(&self) -> SEL10_R {
        SEL10_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - IN1 of PWM1 external input control."]
    #[inline(always)]
    pub fn sel11(&self) -> SEL11_R {
        SEL11_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - IN2 of PWM1 external input control."]
    #[inline(always)]
    pub fn sel12(&self) -> SEL12_R {
        SEL12_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - IN0 of PWM0 external input control."]
    #[inline(always)]
    #[must_use]
    pub fn sel00(&mut self) -> SEL00_W<0> {
        SEL00_W::new(self)
    }
    #[doc = "Bits 2:3 - IN1 of PWM0 external input control."]
    #[inline(always)]
    #[must_use]
    pub fn sel01(&mut self) -> SEL01_W<2> {
        SEL01_W::new(self)
    }
    #[doc = "Bits 4:5 - IN2 of PWM0 external input control."]
    #[inline(always)]
    #[must_use]
    pub fn sel02(&mut self) -> SEL02_W<4> {
        SEL02_W::new(self)
    }
    #[doc = "Bits 16:17 - IN0 of PWM1 external input control."]
    #[inline(always)]
    #[must_use]
    pub fn sel10(&mut self) -> SEL10_W<16> {
        SEL10_W::new(self)
    }
    #[doc = "Bits 18:19 - IN1 of PWM1 external input control."]
    #[inline(always)]
    #[must_use]
    pub fn sel11(&mut self) -> SEL11_W<18> {
        SEL11_W::new(self)
    }
    #[doc = "Bits 20:21 - IN2 of PWM1 external input control."]
    #[inline(always)]
    #[must_use]
    pub fn sel12(&mut self) -> SEL12_W<20> {
        SEL12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input of PWM0 and PWM1 selection register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i_sel01](index.html) module"]
pub struct I_SEL01_SPEC;
impl crate::RegisterSpec for I_SEL01_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i_sel01::R](R) reader structure"]
impl crate::Readable for I_SEL01_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i_sel01::W](W) writer structure"]
impl crate::Writable for I_SEL01_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I_SEL01 to value 0"]
impl crate::Resettable for I_SEL01_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
