#[doc = "Register `REG5` reader"]
pub struct R(crate::R<REG5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG5` writer"]
pub struct W(crate::W<REG5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG5_SPEC>;
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
impl From<crate::W<REG5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP1IT` reader - Bias current selection of CMP1."]
pub type CMP1IT_R = crate::FieldReader<u8, CMP1IT_A>;
#[doc = "Bias current selection of CMP1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMP1IT_A {
    #[doc = "0: `0`"]
    _20N_A = 0,
    #[doc = "1: `1`"]
    _100N_A = 1,
    #[doc = "2: `10`"]
    _500N_A = 2,
}
impl From<CMP1IT_A> for u8 {
    #[inline(always)]
    fn from(variant: CMP1IT_A) -> Self {
        variant as _
    }
}
impl CMP1IT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMP1IT_A> {
        match self.bits {
            0 => Some(CMP1IT_A::_20N_A),
            1 => Some(CMP1IT_A::_100N_A),
            2 => Some(CMP1IT_A::_500N_A),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_20N_A`"]
    #[inline(always)]
    pub fn is_20n_a(&self) -> bool {
        *self == CMP1IT_A::_20N_A
    }
    #[doc = "Checks if the value of the field is `_100N_A`"]
    #[inline(always)]
    pub fn is_100n_a(&self) -> bool {
        *self == CMP1IT_A::_100N_A
    }
    #[doc = "Checks if the value of the field is `_500N_A`"]
    #[inline(always)]
    pub fn is_500n_a(&self) -> bool {
        *self == CMP1IT_A::_500N_A
    }
}
#[doc = "Field `CMP1IT` writer - Bias current selection of CMP1."]
pub type CMP1IT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG5_SPEC, u8, CMP1IT_A, 2, O>;
impl<'a, const O: u8> CMP1IT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _20n_a(self) -> &'a mut W {
        self.variant(CMP1IT_A::_20N_A)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _100n_a(self) -> &'a mut W {
        self.variant(CMP1IT_A::_100N_A)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _500n_a(self) -> &'a mut W {
        self.variant(CMP1IT_A::_500N_A)
    }
}
#[doc = "Field `CMP2IT` reader - Bias current selection of CMP2"]
pub type CMP2IT_R = crate::FieldReader<u8, CMP2IT_A>;
#[doc = "Bias current selection of CMP2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMP2IT_A {
    #[doc = "0: `0`"]
    _20N_A = 0,
    #[doc = "1: `1`"]
    _100N_A = 1,
    #[doc = "2: `10`"]
    _500N_A = 2,
}
impl From<CMP2IT_A> for u8 {
    #[inline(always)]
    fn from(variant: CMP2IT_A) -> Self {
        variant as _
    }
}
impl CMP2IT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMP2IT_A> {
        match self.bits {
            0 => Some(CMP2IT_A::_20N_A),
            1 => Some(CMP2IT_A::_100N_A),
            2 => Some(CMP2IT_A::_500N_A),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_20N_A`"]
    #[inline(always)]
    pub fn is_20n_a(&self) -> bool {
        *self == CMP2IT_A::_20N_A
    }
    #[doc = "Checks if the value of the field is `_100N_A`"]
    #[inline(always)]
    pub fn is_100n_a(&self) -> bool {
        *self == CMP2IT_A::_100N_A
    }
    #[doc = "Checks if the value of the field is `_500N_A`"]
    #[inline(always)]
    pub fn is_500n_a(&self) -> bool {
        *self == CMP2IT_A::_500N_A
    }
}
#[doc = "Field `CMP2IT` writer - Bias current selection of CMP2"]
pub type CMP2IT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG5_SPEC, u8, CMP2IT_A, 2, O>;
impl<'a, const O: u8> CMP2IT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _20n_a(self) -> &'a mut W {
        self.variant(CMP2IT_A::_20N_A)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _100n_a(self) -> &'a mut W {
        self.variant(CMP2IT_A::_100N_A)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _500n_a(self) -> &'a mut W {
        self.variant(CMP2IT_A::_500N_A)
    }
}
#[doc = "Field `AVCCLVDETPD` reader - Power down low voltage detector."]
pub type AVCCLVDETPD_R = crate::BitReader<bool>;
#[doc = "Field `AVCCLVDETPD` writer - Power down low voltage detector."]
pub type AVCCLVDETPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG5_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Bias current selection of CMP1."]
    #[inline(always)]
    pub fn cmp1it(&self) -> CMP1IT_R {
        CMP1IT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Bias current selection of CMP2"]
    #[inline(always)]
    pub fn cmp2it(&self) -> CMP2IT_R {
        CMP2IT_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 6 - Power down low voltage detector."]
    #[inline(always)]
    pub fn avcclvdetpd(&self) -> AVCCLVDETPD_R {
        AVCCLVDETPD_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Bias current selection of CMP1."]
    #[inline(always)]
    #[must_use]
    pub fn cmp1it(&mut self) -> CMP1IT_W<0> {
        CMP1IT_W::new(self)
    }
    #[doc = "Bits 2:3 - Bias current selection of CMP2"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2it(&mut self) -> CMP2IT_W<2> {
        CMP2IT_W::new(self)
    }
    #[doc = "Bit 6 - Power down low voltage detector."]
    #[inline(always)]
    #[must_use]
    pub fn avcclvdetpd(&mut self) -> AVCCLVDETPD_W<6> {
        AVCCLVDETPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog register 5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg5](index.html) module"]
pub struct REG5_SPEC;
impl crate::RegisterSpec for REG5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg5::R](R) reader structure"]
impl crate::Readable for REG5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg5::W](W) writer structure"]
impl crate::Writable for REG5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REG5 to value 0"]
impl crate::Resettable for REG5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
