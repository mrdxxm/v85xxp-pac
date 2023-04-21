#[doc = "Register `CTRL2` reader"]
pub struct R(crate::R<CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL2` writer"]
pub struct W(crate::W<CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL2_SPEC>;
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
impl From<crate::W<CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSB` reader - LSB/MSB transmit order control register."]
pub type MSB_R = crate::BitReader<MSB_A>;
#[doc = "LSB/MSB transmit order control register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSB_A {
    #[doc = "0: `0`"]
    LSB = 0,
    #[doc = "1: `1`"]
    MSB = 1,
}
impl From<MSB_A> for bool {
    #[inline(always)]
    fn from(variant: MSB_A) -> Self {
        variant as u8 != 0
    }
}
impl MSB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSB_A {
        match self.bits {
            false => MSB_A::LSB,
            true => MSB_A::MSB,
        }
    }
    #[doc = "Checks if the value of the field is `LSB`"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == MSB_A::LSB
    }
    #[doc = "Checks if the value of the field is `MSB`"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == MSB_A::MSB
    }
}
#[doc = "Field `MSB` writer - LSB/MSB transmit order control register."]
pub type MSB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL2_SPEC, MSB_A, O>;
impl<'a, const O: u8> MSB_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut W {
        self.variant(MSB_A::LSB)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn msb(self) -> &'a mut W {
        self.variant(MSB_A::MSB)
    }
}
#[doc = "Field `PMODE` reader - Parity mode control register."]
pub type PMODE_R = crate::FieldReader<u8, PMODE_A>;
#[doc = "Parity mode control register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PMODE_A {
    #[doc = "0: `0`"]
    PARITY_NONE = 0,
    #[doc = "1: `1`"]
    PARITY_EVEN = 1,
    #[doc = "3: `11`"]
    PARITY_ODD = 3,
    #[doc = "5: `101`"]
    PARITY_L = 5,
    #[doc = "7: `111`"]
    PARITY_H = 7,
}
impl From<PMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PMODE_A) -> Self {
        variant as _
    }
}
impl PMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PMODE_A> {
        match self.bits {
            0 => Some(PMODE_A::PARITY_NONE),
            1 => Some(PMODE_A::PARITY_EVEN),
            3 => Some(PMODE_A::PARITY_ODD),
            5 => Some(PMODE_A::PARITY_L),
            7 => Some(PMODE_A::PARITY_H),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PARITY_NONE`"]
    #[inline(always)]
    pub fn is_parity_none(&self) -> bool {
        *self == PMODE_A::PARITY_NONE
    }
    #[doc = "Checks if the value of the field is `PARITY_EVEN`"]
    #[inline(always)]
    pub fn is_parity_even(&self) -> bool {
        *self == PMODE_A::PARITY_EVEN
    }
    #[doc = "Checks if the value of the field is `PARITY_ODD`"]
    #[inline(always)]
    pub fn is_parity_odd(&self) -> bool {
        *self == PMODE_A::PARITY_ODD
    }
    #[doc = "Checks if the value of the field is `PARITY_L`"]
    #[inline(always)]
    pub fn is_parity_l(&self) -> bool {
        *self == PMODE_A::PARITY_L
    }
    #[doc = "Checks if the value of the field is `PARITY_H`"]
    #[inline(always)]
    pub fn is_parity_h(&self) -> bool {
        *self == PMODE_A::PARITY_H
    }
}
#[doc = "Field `PMODE` writer - Parity mode control register."]
pub type PMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL2_SPEC, u8, PMODE_A, 3, O>;
impl<'a, const O: u8> PMODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn parity_none(self) -> &'a mut W {
        self.variant(PMODE_A::PARITY_NONE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn parity_even(self) -> &'a mut W {
        self.variant(PMODE_A::PARITY_EVEN)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn parity_odd(self) -> &'a mut W {
        self.variant(PMODE_A::PARITY_ODD)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn parity_l(self) -> &'a mut W {
        self.variant(PMODE_A::PARITY_L)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn parity_h(self) -> &'a mut W {
        self.variant(PMODE_A::PARITY_H)
    }
}
impl R {
    #[doc = "Bit 0 - LSB/MSB transmit order control register."]
    #[inline(always)]
    pub fn msb(&self) -> MSB_R {
        MSB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Parity mode control register."]
    #[inline(always)]
    pub fn pmode(&self) -> PMODE_R {
        PMODE_R::new(((self.bits >> 1) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LSB/MSB transmit order control register."]
    #[inline(always)]
    #[must_use]
    pub fn msb(&mut self) -> MSB_W<0> {
        MSB_W::new(self)
    }
    #[doc = "Bits 1:3 - Parity mode control register."]
    #[inline(always)]
    #[must_use]
    pub fn pmode(&mut self) -> PMODE_W<1> {
        PMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART control register 2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl2](index.html) module"]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl2::R](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl2::W](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
