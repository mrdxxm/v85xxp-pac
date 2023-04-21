#[doc = "Register `REG2` reader"]
pub struct R(crate::R<REG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG2` writer"]
pub struct W(crate::W<REG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG2_SPEC>;
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
impl From<crate::W<REG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP1SEL` reader - Signal source selection of comparator 1."]
pub type CMP1SEL_R = crate::FieldReader<u8, CMP1SEL_A>;
#[doc = "Signal source selection of comparator 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMP1SEL_A {
    #[doc = "0: `0`"]
    CMP1_P_TO_REF = 0,
    #[doc = "1: `1`"]
    CMP1_N_TO_REF = 1,
    #[doc = "2: `10`"]
    CMP1_P_TO_N = 2,
}
impl From<CMP1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CMP1SEL_A) -> Self {
        variant as _
    }
}
impl CMP1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMP1SEL_A> {
        match self.bits {
            0 => Some(CMP1SEL_A::CMP1_P_TO_REF),
            1 => Some(CMP1SEL_A::CMP1_N_TO_REF),
            2 => Some(CMP1SEL_A::CMP1_P_TO_N),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CMP1_P_TO_REF`"]
    #[inline(always)]
    pub fn is_cmp1_p_to_ref(&self) -> bool {
        *self == CMP1SEL_A::CMP1_P_TO_REF
    }
    #[doc = "Checks if the value of the field is `CMP1_N_TO_REF`"]
    #[inline(always)]
    pub fn is_cmp1_n_to_ref(&self) -> bool {
        *self == CMP1SEL_A::CMP1_N_TO_REF
    }
    #[doc = "Checks if the value of the field is `CMP1_P_TO_N`"]
    #[inline(always)]
    pub fn is_cmp1_p_to_n(&self) -> bool {
        *self == CMP1SEL_A::CMP1_P_TO_N
    }
}
#[doc = "Field `CMP1SEL` writer - Signal source selection of comparator 1."]
pub type CMP1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG2_SPEC, u8, CMP1SEL_A, 2, O>;
impl<'a, const O: u8> CMP1SEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn cmp1_p_to_ref(self) -> &'a mut W {
        self.variant(CMP1SEL_A::CMP1_P_TO_REF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn cmp1_n_to_ref(self) -> &'a mut W {
        self.variant(CMP1SEL_A::CMP1_N_TO_REF)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn cmp1_p_to_n(self) -> &'a mut W {
        self.variant(CMP1SEL_A::CMP1_P_TO_N)
    }
}
#[doc = "Field `CMP2SEL` reader - Signal source selection of comparator 2."]
pub type CMP2SEL_R = crate::FieldReader<u8, CMP2SEL_A>;
#[doc = "Signal source selection of comparator 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMP2SEL_A {
    #[doc = "0: `0`"]
    CMP2_P_TO_REF = 0,
    #[doc = "1: `1`"]
    CMP2_N_TO_REF = 1,
    #[doc = "2: `10`"]
    CMP2_P_TO_N = 2,
}
impl From<CMP2SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CMP2SEL_A) -> Self {
        variant as _
    }
}
impl CMP2SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMP2SEL_A> {
        match self.bits {
            0 => Some(CMP2SEL_A::CMP2_P_TO_REF),
            1 => Some(CMP2SEL_A::CMP2_N_TO_REF),
            2 => Some(CMP2SEL_A::CMP2_P_TO_N),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CMP2_P_TO_REF`"]
    #[inline(always)]
    pub fn is_cmp2_p_to_ref(&self) -> bool {
        *self == CMP2SEL_A::CMP2_P_TO_REF
    }
    #[doc = "Checks if the value of the field is `CMP2_N_TO_REF`"]
    #[inline(always)]
    pub fn is_cmp2_n_to_ref(&self) -> bool {
        *self == CMP2SEL_A::CMP2_N_TO_REF
    }
    #[doc = "Checks if the value of the field is `CMP2_P_TO_N`"]
    #[inline(always)]
    pub fn is_cmp2_p_to_n(&self) -> bool {
        *self == CMP2SEL_A::CMP2_P_TO_N
    }
}
#[doc = "Field `CMP2SEL` writer - Signal source selection of comparator 2."]
pub type CMP2SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG2_SPEC, u8, CMP2SEL_A, 2, O>;
impl<'a, const O: u8> CMP2SEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn cmp2_p_to_ref(self) -> &'a mut W {
        self.variant(CMP2SEL_A::CMP2_P_TO_REF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn cmp2_n_to_ref(self) -> &'a mut W {
        self.variant(CMP2SEL_A::CMP2_N_TO_REF)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn cmp2_p_to_n(self) -> &'a mut W {
        self.variant(CMP2SEL_A::CMP2_P_TO_N)
    }
}
#[doc = "Field `CMP1REFSEL` reader - REF selection of CMP1."]
pub type CMP1REFSEL_R = crate::BitReader<bool>;
#[doc = "Field `CMP1REFSEL` writer - REF selection of CMP1."]
pub type CMP1REFSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG2_SPEC, bool, O>;
#[doc = "Field `CMP2REFSEL` reader - REF selection of CMP2."]
pub type CMP2REFSEL_R = crate::BitReader<bool>;
#[doc = "Field `CMP2REFSEL` writer - REF selection of CMP2."]
pub type CMP2REFSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Signal source selection of comparator 1."]
    #[inline(always)]
    pub fn cmp1sel(&self) -> CMP1SEL_R {
        CMP1SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Signal source selection of comparator 2."]
    #[inline(always)]
    pub fn cmp2sel(&self) -> CMP2SEL_R {
        CMP2SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - REF selection of CMP1."]
    #[inline(always)]
    pub fn cmp1refsel(&self) -> CMP1REFSEL_R {
        CMP1REFSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - REF selection of CMP2."]
    #[inline(always)]
    pub fn cmp2refsel(&self) -> CMP2REFSEL_R {
        CMP2REFSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Signal source selection of comparator 1."]
    #[inline(always)]
    #[must_use]
    pub fn cmp1sel(&mut self) -> CMP1SEL_W<0> {
        CMP1SEL_W::new(self)
    }
    #[doc = "Bits 2:3 - Signal source selection of comparator 2."]
    #[inline(always)]
    #[must_use]
    pub fn cmp2sel(&mut self) -> CMP2SEL_W<2> {
        CMP2SEL_W::new(self)
    }
    #[doc = "Bit 4 - REF selection of CMP1."]
    #[inline(always)]
    #[must_use]
    pub fn cmp1refsel(&mut self) -> CMP1REFSEL_W<4> {
        CMP1REFSEL_W::new(self)
    }
    #[doc = "Bit 5 - REF selection of CMP2."]
    #[inline(always)]
    #[must_use]
    pub fn cmp2refsel(&mut self) -> CMP2REFSEL_W<5> {
        CMP2REFSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog register 2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg2](index.html) module"]
pub struct REG2_SPEC;
impl crate::RegisterSpec for REG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg2::R](R) reader structure"]
impl crate::Readable for REG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg2::W](W) writer structure"]
impl crate::Writable for REG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REG2 to value 0"]
impl crate::Resettable for REG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
