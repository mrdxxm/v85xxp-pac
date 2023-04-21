#[doc = "Register `CLKSEL` reader"]
pub struct R(crate::R<CLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKSEL` writer"]
pub struct W(crate::W<CLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKSEL_SPEC>;
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
impl From<crate::W<CLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKSEL` reader - This register is used to control AHB clock source."]
pub type CLKSEL_R = crate::FieldReader<u8, CLKSEL_A>;
#[doc = "This register is used to control AHB clock source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: `0`"]
    RCH = 0,
    #[doc = "1: `1`"]
    XOH = 1,
    #[doc = "2: `10`"]
    PLLH = 2,
    #[doc = "3: controlled by RTCLK_SEL in PMU_CONTROL register"]
    RTCCLK = 3,
    #[doc = "4: `100`"]
    PLLL = 4,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
impl CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKSEL_A> {
        match self.bits {
            0 => Some(CLKSEL_A::RCH),
            1 => Some(CLKSEL_A::XOH),
            2 => Some(CLKSEL_A::PLLH),
            3 => Some(CLKSEL_A::RTCCLK),
            4 => Some(CLKSEL_A::PLLL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RCH`"]
    #[inline(always)]
    pub fn is_rch(&self) -> bool {
        *self == CLKSEL_A::RCH
    }
    #[doc = "Checks if the value of the field is `XOH`"]
    #[inline(always)]
    pub fn is_xoh(&self) -> bool {
        *self == CLKSEL_A::XOH
    }
    #[doc = "Checks if the value of the field is `PLLH`"]
    #[inline(always)]
    pub fn is_pllh(&self) -> bool {
        *self == CLKSEL_A::PLLH
    }
    #[doc = "Checks if the value of the field is `RTCCLK`"]
    #[inline(always)]
    pub fn is_rtcclk(&self) -> bool {
        *self == CLKSEL_A::RTCCLK
    }
    #[doc = "Checks if the value of the field is `PLLL`"]
    #[inline(always)]
    pub fn is_plll(&self) -> bool {
        *self == CLKSEL_A::PLLL
    }
}
#[doc = "Field `CLKSEL` writer - This register is used to control AHB clock source."]
pub type CLKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKSEL_SPEC, u8, CLKSEL_A, 3, O>;
impl<'a, const O: u8> CLKSEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rch(self) -> &'a mut W {
        self.variant(CLKSEL_A::RCH)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn xoh(self) -> &'a mut W {
        self.variant(CLKSEL_A::XOH)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pllh(self) -> &'a mut W {
        self.variant(CLKSEL_A::PLLH)
    }
    #[doc = "controlled by RTCLK_SEL in PMU_CONTROL register"]
    #[inline(always)]
    pub fn rtcclk(self) -> &'a mut W {
        self.variant(CLKSEL_A::RTCCLK)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn plll(self) -> &'a mut W {
        self.variant(CLKSEL_A::PLLL)
    }
}
impl R {
    #[doc = "Bits 0:2 - This register is used to control AHB clock source."]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - This register is used to control AHB clock source."]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<0> {
        CLKSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock selection register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clksel](index.html) module"]
pub struct CLKSEL_SPEC;
impl crate::RegisterSpec for CLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clksel::R](R) reader structure"]
impl crate::Readable for CLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clksel::W](W) writer structure"]
impl crate::Writable for CLKSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKSEL to value 0"]
impl crate::Resettable for CLKSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
