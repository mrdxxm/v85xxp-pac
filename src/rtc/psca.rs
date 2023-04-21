#[doc = "Register `PSCA` reader"]
pub struct R(crate::R<PSCA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSCA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSCA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSCA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSCA` writer"]
pub struct W(crate::W<PSCA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSCA_SPEC>;
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
impl From<crate::W<PSCA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSCA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSCA` reader - This register is used to control the RTC clock pre-scaler."]
pub type PSCA_R = crate::FieldReader<u8, PSCA_A>;
#[doc = "This register is used to control the RTC clock pre-scaler.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSCA_A {
    #[doc = "0: `0`"]
    NO_PRE_SCALER = 0,
    #[doc = "1: `1`"]
    _0_25_PRE_SCALER = 1,
}
impl From<PSCA_A> for u8 {
    #[inline(always)]
    fn from(variant: PSCA_A) -> Self {
        variant as _
    }
}
impl PSCA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PSCA_A> {
        match self.bits {
            0 => Some(PSCA_A::NO_PRE_SCALER),
            1 => Some(PSCA_A::_0_25_PRE_SCALER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PRE_SCALER`"]
    #[inline(always)]
    pub fn is_no_pre_scaler(&self) -> bool {
        *self == PSCA_A::NO_PRE_SCALER
    }
    #[doc = "Checks if the value of the field is `_0_25_PRE_SCALER`"]
    #[inline(always)]
    pub fn is_0_25_pre_scaler(&self) -> bool {
        *self == PSCA_A::_0_25_PRE_SCALER
    }
}
#[doc = "Field `PSCA` writer - This register is used to control the RTC clock pre-scaler."]
pub type PSCA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PSCA_SPEC, u8, PSCA_A, 2, O>;
impl<'a, const O: u8> PSCA_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pre_scaler(self) -> &'a mut W {
        self.variant(PSCA_A::NO_PRE_SCALER)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _0_25_pre_scaler(self) -> &'a mut W {
        self.variant(PSCA_A::_0_25_PRE_SCALER)
    }
}
impl R {
    #[doc = "Bits 0:1 - This register is used to control the RTC clock pre-scaler."]
    #[inline(always)]
    pub fn psca(&self) -> PSCA_R {
        PSCA_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - This register is used to control the RTC clock pre-scaler."]
    #[inline(always)]
    #[must_use]
    pub fn psca(&mut self) -> PSCA_W<0> {
        PSCA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC clock pre-scaler control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psca](index.html) module"]
pub struct PSCA_SPEC;
impl crate::RegisterSpec for PSCA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psca::R](R) reader structure"]
impl crate::Readable for PSCA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psca::W](W) writer structure"]
impl crate::Writable for PSCA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSCA to value 0"]
impl crate::Resettable for PSCA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
