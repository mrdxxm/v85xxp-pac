#[doc = "Register `O_SEL` reader"]
pub struct R(crate::R<O_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<O_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<O_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<O_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `O_SEL` writer"]
pub struct W(crate::W<O_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<O_SEL_SPEC>;
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
impl From<crate::W<O_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<O_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL0` reader - Output selection register for external output PWM0."]
pub type SEL0_R = crate::FieldReader<u8, SEL0_A>;
#[doc = "Output selection register for external output PWM0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL0_A {
    #[doc = "0: `0`"]
    PWM0_OUT0 = 0,
    #[doc = "1: `1`"]
    PWM0_OUT1 = 1,
    #[doc = "2: `10`"]
    PWM0_OUT2 = 2,
    #[doc = "4: `100`"]
    PWM1_OUT0 = 4,
    #[doc = "5: `101`"]
    PWM1_OUT1 = 5,
    #[doc = "6: `110`"]
    PWM1_OUT2 = 6,
    #[doc = "8: `1000`"]
    PWM2_OUT0 = 8,
    #[doc = "9: `1001`"]
    PWM2_OUT1 = 9,
    #[doc = "10: `1010`"]
    PWM2_OUT2 = 10,
    #[doc = "12: `1100`"]
    PWM3_OUT0 = 12,
    #[doc = "13: `1101`"]
    PWM3_OUT1 = 13,
    #[doc = "14: `1110`"]
    PWM3_OUT2 = 14,
}
impl From<SEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL0_A) -> Self {
        variant as _
    }
}
impl SEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEL0_A> {
        match self.bits {
            0 => Some(SEL0_A::PWM0_OUT0),
            1 => Some(SEL0_A::PWM0_OUT1),
            2 => Some(SEL0_A::PWM0_OUT2),
            4 => Some(SEL0_A::PWM1_OUT0),
            5 => Some(SEL0_A::PWM1_OUT1),
            6 => Some(SEL0_A::PWM1_OUT2),
            8 => Some(SEL0_A::PWM2_OUT0),
            9 => Some(SEL0_A::PWM2_OUT1),
            10 => Some(SEL0_A::PWM2_OUT2),
            12 => Some(SEL0_A::PWM3_OUT0),
            13 => Some(SEL0_A::PWM3_OUT1),
            14 => Some(SEL0_A::PWM3_OUT2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PWM0_OUT0`"]
    #[inline(always)]
    pub fn is_pwm0_out0(&self) -> bool {
        *self == SEL0_A::PWM0_OUT0
    }
    #[doc = "Checks if the value of the field is `PWM0_OUT1`"]
    #[inline(always)]
    pub fn is_pwm0_out1(&self) -> bool {
        *self == SEL0_A::PWM0_OUT1
    }
    #[doc = "Checks if the value of the field is `PWM0_OUT2`"]
    #[inline(always)]
    pub fn is_pwm0_out2(&self) -> bool {
        *self == SEL0_A::PWM0_OUT2
    }
    #[doc = "Checks if the value of the field is `PWM1_OUT0`"]
    #[inline(always)]
    pub fn is_pwm1_out0(&self) -> bool {
        *self == SEL0_A::PWM1_OUT0
    }
    #[doc = "Checks if the value of the field is `PWM1_OUT1`"]
    #[inline(always)]
    pub fn is_pwm1_out1(&self) -> bool {
        *self == SEL0_A::PWM1_OUT1
    }
    #[doc = "Checks if the value of the field is `PWM1_OUT2`"]
    #[inline(always)]
    pub fn is_pwm1_out2(&self) -> bool {
        *self == SEL0_A::PWM1_OUT2
    }
    #[doc = "Checks if the value of the field is `PWM2_OUT0`"]
    #[inline(always)]
    pub fn is_pwm2_out0(&self) -> bool {
        *self == SEL0_A::PWM2_OUT0
    }
    #[doc = "Checks if the value of the field is `PWM2_OUT1`"]
    #[inline(always)]
    pub fn is_pwm2_out1(&self) -> bool {
        *self == SEL0_A::PWM2_OUT1
    }
    #[doc = "Checks if the value of the field is `PWM2_OUT2`"]
    #[inline(always)]
    pub fn is_pwm2_out2(&self) -> bool {
        *self == SEL0_A::PWM2_OUT2
    }
    #[doc = "Checks if the value of the field is `PWM3_OUT0`"]
    #[inline(always)]
    pub fn is_pwm3_out0(&self) -> bool {
        *self == SEL0_A::PWM3_OUT0
    }
    #[doc = "Checks if the value of the field is `PWM3_OUT1`"]
    #[inline(always)]
    pub fn is_pwm3_out1(&self) -> bool {
        *self == SEL0_A::PWM3_OUT1
    }
    #[doc = "Checks if the value of the field is `PWM3_OUT2`"]
    #[inline(always)]
    pub fn is_pwm3_out2(&self) -> bool {
        *self == SEL0_A::PWM3_OUT2
    }
}
#[doc = "Field `SEL0` writer - Output selection register for external output PWM0."]
pub type SEL0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, O_SEL_SPEC, u8, SEL0_A, 4, O>;
impl<'a, const O: u8> SEL0_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pwm0_out0(self) -> &'a mut W {
        self.variant(SEL0_A::PWM0_OUT0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pwm0_out1(self) -> &'a mut W {
        self.variant(SEL0_A::PWM0_OUT1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pwm0_out2(self) -> &'a mut W {
        self.variant(SEL0_A::PWM0_OUT2)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn pwm1_out0(self) -> &'a mut W {
        self.variant(SEL0_A::PWM1_OUT0)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn pwm1_out1(self) -> &'a mut W {
        self.variant(SEL0_A::PWM1_OUT1)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn pwm1_out2(self) -> &'a mut W {
        self.variant(SEL0_A::PWM1_OUT2)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm2_out0(self) -> &'a mut W {
        self.variant(SEL0_A::PWM2_OUT0)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn pwm2_out1(self) -> &'a mut W {
        self.variant(SEL0_A::PWM2_OUT1)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn pwm2_out2(self) -> &'a mut W {
        self.variant(SEL0_A::PWM2_OUT2)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn pwm3_out0(self) -> &'a mut W {
        self.variant(SEL0_A::PWM3_OUT0)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn pwm3_out1(self) -> &'a mut W {
        self.variant(SEL0_A::PWM3_OUT1)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pwm3_out2(self) -> &'a mut W {
        self.variant(SEL0_A::PWM3_OUT2)
    }
}
#[doc = "Field `SEL1` reader - Output selection register for external output PWM1."]
pub type SEL1_R = crate::FieldReader<u8, SEL1_A>;
#[doc = "Output selection register for external output PWM1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL1_A {
    #[doc = "0: `0`"]
    PWM0_OUT0 = 0,
    #[doc = "1: `1`"]
    PWM0_OUT1 = 1,
    #[doc = "2: `10`"]
    PWM0_OUT2 = 2,
    #[doc = "4: `100`"]
    PWM1_OUT0 = 4,
    #[doc = "5: `101`"]
    PWM1_OUT1 = 5,
    #[doc = "6: `110`"]
    PWM1_OUT2 = 6,
    #[doc = "8: `1000`"]
    PWM2_OUT0 = 8,
    #[doc = "9: `1001`"]
    PWM2_OUT1 = 9,
    #[doc = "10: `1010`"]
    PWM2_OUT2 = 10,
    #[doc = "12: `1100`"]
    PWM3_OUT0 = 12,
    #[doc = "13: `1101`"]
    PWM3_OUT1 = 13,
    #[doc = "14: `1110`"]
    PWM3_OUT2 = 14,
}
impl From<SEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL1_A) -> Self {
        variant as _
    }
}
impl SEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEL1_A> {
        match self.bits {
            0 => Some(SEL1_A::PWM0_OUT0),
            1 => Some(SEL1_A::PWM0_OUT1),
            2 => Some(SEL1_A::PWM0_OUT2),
            4 => Some(SEL1_A::PWM1_OUT0),
            5 => Some(SEL1_A::PWM1_OUT1),
            6 => Some(SEL1_A::PWM1_OUT2),
            8 => Some(SEL1_A::PWM2_OUT0),
            9 => Some(SEL1_A::PWM2_OUT1),
            10 => Some(SEL1_A::PWM2_OUT2),
            12 => Some(SEL1_A::PWM3_OUT0),
            13 => Some(SEL1_A::PWM3_OUT1),
            14 => Some(SEL1_A::PWM3_OUT2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PWM0_OUT0`"]
    #[inline(always)]
    pub fn is_pwm0_out0(&self) -> bool {
        *self == SEL1_A::PWM0_OUT0
    }
    #[doc = "Checks if the value of the field is `PWM0_OUT1`"]
    #[inline(always)]
    pub fn is_pwm0_out1(&self) -> bool {
        *self == SEL1_A::PWM0_OUT1
    }
    #[doc = "Checks if the value of the field is `PWM0_OUT2`"]
    #[inline(always)]
    pub fn is_pwm0_out2(&self) -> bool {
        *self == SEL1_A::PWM0_OUT2
    }
    #[doc = "Checks if the value of the field is `PWM1_OUT0`"]
    #[inline(always)]
    pub fn is_pwm1_out0(&self) -> bool {
        *self == SEL1_A::PWM1_OUT0
    }
    #[doc = "Checks if the value of the field is `PWM1_OUT1`"]
    #[inline(always)]
    pub fn is_pwm1_out1(&self) -> bool {
        *self == SEL1_A::PWM1_OUT1
    }
    #[doc = "Checks if the value of the field is `PWM1_OUT2`"]
    #[inline(always)]
    pub fn is_pwm1_out2(&self) -> bool {
        *self == SEL1_A::PWM1_OUT2
    }
    #[doc = "Checks if the value of the field is `PWM2_OUT0`"]
    #[inline(always)]
    pub fn is_pwm2_out0(&self) -> bool {
        *self == SEL1_A::PWM2_OUT0
    }
    #[doc = "Checks if the value of the field is `PWM2_OUT1`"]
    #[inline(always)]
    pub fn is_pwm2_out1(&self) -> bool {
        *self == SEL1_A::PWM2_OUT1
    }
    #[doc = "Checks if the value of the field is `PWM2_OUT2`"]
    #[inline(always)]
    pub fn is_pwm2_out2(&self) -> bool {
        *self == SEL1_A::PWM2_OUT2
    }
    #[doc = "Checks if the value of the field is `PWM3_OUT0`"]
    #[inline(always)]
    pub fn is_pwm3_out0(&self) -> bool {
        *self == SEL1_A::PWM3_OUT0
    }
    #[doc = "Checks if the value of the field is `PWM3_OUT1`"]
    #[inline(always)]
    pub fn is_pwm3_out1(&self) -> bool {
        *self == SEL1_A::PWM3_OUT1
    }
    #[doc = "Checks if the value of the field is `PWM3_OUT2`"]
    #[inline(always)]
    pub fn is_pwm3_out2(&self) -> bool {
        *self == SEL1_A::PWM3_OUT2
    }
}
#[doc = "Field `SEL1` writer - Output selection register for external output PWM1."]
pub type SEL1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, O_SEL_SPEC, u8, SEL1_A, 4, O>;
impl<'a, const O: u8> SEL1_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pwm0_out0(self) -> &'a mut W {
        self.variant(SEL1_A::PWM0_OUT0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pwm0_out1(self) -> &'a mut W {
        self.variant(SEL1_A::PWM0_OUT1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pwm0_out2(self) -> &'a mut W {
        self.variant(SEL1_A::PWM0_OUT2)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn pwm1_out0(self) -> &'a mut W {
        self.variant(SEL1_A::PWM1_OUT0)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn pwm1_out1(self) -> &'a mut W {
        self.variant(SEL1_A::PWM1_OUT1)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn pwm1_out2(self) -> &'a mut W {
        self.variant(SEL1_A::PWM1_OUT2)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm2_out0(self) -> &'a mut W {
        self.variant(SEL1_A::PWM2_OUT0)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn pwm2_out1(self) -> &'a mut W {
        self.variant(SEL1_A::PWM2_OUT1)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn pwm2_out2(self) -> &'a mut W {
        self.variant(SEL1_A::PWM2_OUT2)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn pwm3_out0(self) -> &'a mut W {
        self.variant(SEL1_A::PWM3_OUT0)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn pwm3_out1(self) -> &'a mut W {
        self.variant(SEL1_A::PWM3_OUT1)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pwm3_out2(self) -> &'a mut W {
        self.variant(SEL1_A::PWM3_OUT2)
    }
}
#[doc = "Field `SEL2` reader - Output selection register for external output PWM2."]
pub type SEL2_R = crate::FieldReader<u8, SEL2_A>;
#[doc = "Output selection register for external output PWM2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL2_A {
    #[doc = "0: `0`"]
    PWM0_OUT0 = 0,
    #[doc = "1: `1`"]
    PWM0_OUT1 = 1,
    #[doc = "2: `10`"]
    PWM0_OUT2 = 2,
    #[doc = "4: `100`"]
    PWM1_OUT0 = 4,
    #[doc = "5: `101`"]
    PWM1_OUT1 = 5,
    #[doc = "6: `110`"]
    PWM1_OUT2 = 6,
    #[doc = "8: `1000`"]
    PWM2_OUT0 = 8,
    #[doc = "9: `1001`"]
    PWM2_OUT1 = 9,
    #[doc = "10: `1010`"]
    PWM2_OUT2 = 10,
    #[doc = "12: `1100`"]
    PWM3_OUT0 = 12,
    #[doc = "13: `1101`"]
    PWM3_OUT1 = 13,
    #[doc = "14: `1110`"]
    PWM3_OUT2 = 14,
}
impl From<SEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL2_A) -> Self {
        variant as _
    }
}
impl SEL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEL2_A> {
        match self.bits {
            0 => Some(SEL2_A::PWM0_OUT0),
            1 => Some(SEL2_A::PWM0_OUT1),
            2 => Some(SEL2_A::PWM0_OUT2),
            4 => Some(SEL2_A::PWM1_OUT0),
            5 => Some(SEL2_A::PWM1_OUT1),
            6 => Some(SEL2_A::PWM1_OUT2),
            8 => Some(SEL2_A::PWM2_OUT0),
            9 => Some(SEL2_A::PWM2_OUT1),
            10 => Some(SEL2_A::PWM2_OUT2),
            12 => Some(SEL2_A::PWM3_OUT0),
            13 => Some(SEL2_A::PWM3_OUT1),
            14 => Some(SEL2_A::PWM3_OUT2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PWM0_OUT0`"]
    #[inline(always)]
    pub fn is_pwm0_out0(&self) -> bool {
        *self == SEL2_A::PWM0_OUT0
    }
    #[doc = "Checks if the value of the field is `PWM0_OUT1`"]
    #[inline(always)]
    pub fn is_pwm0_out1(&self) -> bool {
        *self == SEL2_A::PWM0_OUT1
    }
    #[doc = "Checks if the value of the field is `PWM0_OUT2`"]
    #[inline(always)]
    pub fn is_pwm0_out2(&self) -> bool {
        *self == SEL2_A::PWM0_OUT2
    }
    #[doc = "Checks if the value of the field is `PWM1_OUT0`"]
    #[inline(always)]
    pub fn is_pwm1_out0(&self) -> bool {
        *self == SEL2_A::PWM1_OUT0
    }
    #[doc = "Checks if the value of the field is `PWM1_OUT1`"]
    #[inline(always)]
    pub fn is_pwm1_out1(&self) -> bool {
        *self == SEL2_A::PWM1_OUT1
    }
    #[doc = "Checks if the value of the field is `PWM1_OUT2`"]
    #[inline(always)]
    pub fn is_pwm1_out2(&self) -> bool {
        *self == SEL2_A::PWM1_OUT2
    }
    #[doc = "Checks if the value of the field is `PWM2_OUT0`"]
    #[inline(always)]
    pub fn is_pwm2_out0(&self) -> bool {
        *self == SEL2_A::PWM2_OUT0
    }
    #[doc = "Checks if the value of the field is `PWM2_OUT1`"]
    #[inline(always)]
    pub fn is_pwm2_out1(&self) -> bool {
        *self == SEL2_A::PWM2_OUT1
    }
    #[doc = "Checks if the value of the field is `PWM2_OUT2`"]
    #[inline(always)]
    pub fn is_pwm2_out2(&self) -> bool {
        *self == SEL2_A::PWM2_OUT2
    }
    #[doc = "Checks if the value of the field is `PWM3_OUT0`"]
    #[inline(always)]
    pub fn is_pwm3_out0(&self) -> bool {
        *self == SEL2_A::PWM3_OUT0
    }
    #[doc = "Checks if the value of the field is `PWM3_OUT1`"]
    #[inline(always)]
    pub fn is_pwm3_out1(&self) -> bool {
        *self == SEL2_A::PWM3_OUT1
    }
    #[doc = "Checks if the value of the field is `PWM3_OUT2`"]
    #[inline(always)]
    pub fn is_pwm3_out2(&self) -> bool {
        *self == SEL2_A::PWM3_OUT2
    }
}
#[doc = "Field `SEL2` writer - Output selection register for external output PWM2."]
pub type SEL2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, O_SEL_SPEC, u8, SEL2_A, 4, O>;
impl<'a, const O: u8> SEL2_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pwm0_out0(self) -> &'a mut W {
        self.variant(SEL2_A::PWM0_OUT0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pwm0_out1(self) -> &'a mut W {
        self.variant(SEL2_A::PWM0_OUT1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pwm0_out2(self) -> &'a mut W {
        self.variant(SEL2_A::PWM0_OUT2)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn pwm1_out0(self) -> &'a mut W {
        self.variant(SEL2_A::PWM1_OUT0)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn pwm1_out1(self) -> &'a mut W {
        self.variant(SEL2_A::PWM1_OUT1)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn pwm1_out2(self) -> &'a mut W {
        self.variant(SEL2_A::PWM1_OUT2)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm2_out0(self) -> &'a mut W {
        self.variant(SEL2_A::PWM2_OUT0)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn pwm2_out1(self) -> &'a mut W {
        self.variant(SEL2_A::PWM2_OUT1)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn pwm2_out2(self) -> &'a mut W {
        self.variant(SEL2_A::PWM2_OUT2)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn pwm3_out0(self) -> &'a mut W {
        self.variant(SEL2_A::PWM3_OUT0)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn pwm3_out1(self) -> &'a mut W {
        self.variant(SEL2_A::PWM3_OUT1)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pwm3_out2(self) -> &'a mut W {
        self.variant(SEL2_A::PWM3_OUT2)
    }
}
#[doc = "Field `SEL3` reader - Output selection register for external output PWM3."]
pub type SEL3_R = crate::FieldReader<u8, SEL3_A>;
#[doc = "Output selection register for external output PWM3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL3_A {
    #[doc = "0: `0`"]
    PWM0_OUT0 = 0,
    #[doc = "1: `1`"]
    PWM0_OUT1 = 1,
    #[doc = "2: `10`"]
    PWM0_OUT2 = 2,
    #[doc = "4: `100`"]
    PWM1_OUT0 = 4,
    #[doc = "5: `101`"]
    PWM1_OUT1 = 5,
    #[doc = "6: `110`"]
    PWM1_OUT2 = 6,
    #[doc = "8: `1000`"]
    PWM2_OUT0 = 8,
    #[doc = "9: `1001`"]
    PWM2_OUT1 = 9,
    #[doc = "10: `1010`"]
    PWM2_OUT2 = 10,
    #[doc = "12: `1100`"]
    PWM3_OUT0 = 12,
    #[doc = "13: `1101`"]
    PWM3_OUT1 = 13,
    #[doc = "14: `1110`"]
    PWM3_OUT2 = 14,
}
impl From<SEL3_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL3_A) -> Self {
        variant as _
    }
}
impl SEL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEL3_A> {
        match self.bits {
            0 => Some(SEL3_A::PWM0_OUT0),
            1 => Some(SEL3_A::PWM0_OUT1),
            2 => Some(SEL3_A::PWM0_OUT2),
            4 => Some(SEL3_A::PWM1_OUT0),
            5 => Some(SEL3_A::PWM1_OUT1),
            6 => Some(SEL3_A::PWM1_OUT2),
            8 => Some(SEL3_A::PWM2_OUT0),
            9 => Some(SEL3_A::PWM2_OUT1),
            10 => Some(SEL3_A::PWM2_OUT2),
            12 => Some(SEL3_A::PWM3_OUT0),
            13 => Some(SEL3_A::PWM3_OUT1),
            14 => Some(SEL3_A::PWM3_OUT2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PWM0_OUT0`"]
    #[inline(always)]
    pub fn is_pwm0_out0(&self) -> bool {
        *self == SEL3_A::PWM0_OUT0
    }
    #[doc = "Checks if the value of the field is `PWM0_OUT1`"]
    #[inline(always)]
    pub fn is_pwm0_out1(&self) -> bool {
        *self == SEL3_A::PWM0_OUT1
    }
    #[doc = "Checks if the value of the field is `PWM0_OUT2`"]
    #[inline(always)]
    pub fn is_pwm0_out2(&self) -> bool {
        *self == SEL3_A::PWM0_OUT2
    }
    #[doc = "Checks if the value of the field is `PWM1_OUT0`"]
    #[inline(always)]
    pub fn is_pwm1_out0(&self) -> bool {
        *self == SEL3_A::PWM1_OUT0
    }
    #[doc = "Checks if the value of the field is `PWM1_OUT1`"]
    #[inline(always)]
    pub fn is_pwm1_out1(&self) -> bool {
        *self == SEL3_A::PWM1_OUT1
    }
    #[doc = "Checks if the value of the field is `PWM1_OUT2`"]
    #[inline(always)]
    pub fn is_pwm1_out2(&self) -> bool {
        *self == SEL3_A::PWM1_OUT2
    }
    #[doc = "Checks if the value of the field is `PWM2_OUT0`"]
    #[inline(always)]
    pub fn is_pwm2_out0(&self) -> bool {
        *self == SEL3_A::PWM2_OUT0
    }
    #[doc = "Checks if the value of the field is `PWM2_OUT1`"]
    #[inline(always)]
    pub fn is_pwm2_out1(&self) -> bool {
        *self == SEL3_A::PWM2_OUT1
    }
    #[doc = "Checks if the value of the field is `PWM2_OUT2`"]
    #[inline(always)]
    pub fn is_pwm2_out2(&self) -> bool {
        *self == SEL3_A::PWM2_OUT2
    }
    #[doc = "Checks if the value of the field is `PWM3_OUT0`"]
    #[inline(always)]
    pub fn is_pwm3_out0(&self) -> bool {
        *self == SEL3_A::PWM3_OUT0
    }
    #[doc = "Checks if the value of the field is `PWM3_OUT1`"]
    #[inline(always)]
    pub fn is_pwm3_out1(&self) -> bool {
        *self == SEL3_A::PWM3_OUT1
    }
    #[doc = "Checks if the value of the field is `PWM3_OUT2`"]
    #[inline(always)]
    pub fn is_pwm3_out2(&self) -> bool {
        *self == SEL3_A::PWM3_OUT2
    }
}
#[doc = "Field `SEL3` writer - Output selection register for external output PWM3."]
pub type SEL3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, O_SEL_SPEC, u8, SEL3_A, 4, O>;
impl<'a, const O: u8> SEL3_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pwm0_out0(self) -> &'a mut W {
        self.variant(SEL3_A::PWM0_OUT0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pwm0_out1(self) -> &'a mut W {
        self.variant(SEL3_A::PWM0_OUT1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pwm0_out2(self) -> &'a mut W {
        self.variant(SEL3_A::PWM0_OUT2)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn pwm1_out0(self) -> &'a mut W {
        self.variant(SEL3_A::PWM1_OUT0)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn pwm1_out1(self) -> &'a mut W {
        self.variant(SEL3_A::PWM1_OUT1)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn pwm1_out2(self) -> &'a mut W {
        self.variant(SEL3_A::PWM1_OUT2)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm2_out0(self) -> &'a mut W {
        self.variant(SEL3_A::PWM2_OUT0)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn pwm2_out1(self) -> &'a mut W {
        self.variant(SEL3_A::PWM2_OUT1)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn pwm2_out2(self) -> &'a mut W {
        self.variant(SEL3_A::PWM2_OUT2)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn pwm3_out0(self) -> &'a mut W {
        self.variant(SEL3_A::PWM3_OUT0)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn pwm3_out1(self) -> &'a mut W {
        self.variant(SEL3_A::PWM3_OUT1)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pwm3_out2(self) -> &'a mut W {
        self.variant(SEL3_A::PWM3_OUT2)
    }
}
impl R {
    #[doc = "Bits 0:3 - Output selection register for external output PWM0."]
    #[inline(always)]
    pub fn sel0(&self) -> SEL0_R {
        SEL0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Output selection register for external output PWM1."]
    #[inline(always)]
    pub fn sel1(&self) -> SEL1_R {
        SEL1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Output selection register for external output PWM2."]
    #[inline(always)]
    pub fn sel2(&self) -> SEL2_R {
        SEL2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Output selection register for external output PWM3."]
    #[inline(always)]
    pub fn sel3(&self) -> SEL3_R {
        SEL3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Output selection register for external output PWM0."]
    #[inline(always)]
    #[must_use]
    pub fn sel0(&mut self) -> SEL0_W<0> {
        SEL0_W::new(self)
    }
    #[doc = "Bits 4:7 - Output selection register for external output PWM1."]
    #[inline(always)]
    #[must_use]
    pub fn sel1(&mut self) -> SEL1_W<4> {
        SEL1_W::new(self)
    }
    #[doc = "Bits 8:11 - Output selection register for external output PWM2."]
    #[inline(always)]
    #[must_use]
    pub fn sel2(&mut self) -> SEL2_W<8> {
        SEL2_W::new(self)
    }
    #[doc = "Bits 12:15 - Output selection register for external output PWM3."]
    #[inline(always)]
    #[must_use]
    pub fn sel3(&mut self) -> SEL3_W<12> {
        SEL3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM output selection register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [o_sel](index.html) module"]
pub struct O_SEL_SPEC;
impl crate::RegisterSpec for O_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [o_sel::R](R) reader structure"]
impl crate::Readable for O_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [o_sel::W](W) writer structure"]
impl crate::Writable for O_SEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets O_SEL to value 0"]
impl crate::Resettable for O_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
