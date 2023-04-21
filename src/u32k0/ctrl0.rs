#[doc = "Register `CTRL0` reader"]
pub struct R(crate::R<CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL0` writer"]
pub struct W(crate::W<CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL0_SPEC>;
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
impl From<crate::W<CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - UART 32K controller enable register."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - UART 32K controller enable register."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL0_SPEC, bool, O>;
#[doc = "Field `ACOFF` reader - Auto-calibration off control register."]
pub type ACOFF_R = crate::BitReader<bool>;
#[doc = "Field `ACOFF` writer - Auto-calibration off control register."]
pub type ACOFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL0_SPEC, bool, O>;
#[doc = "Field `MSB` reader - UART receive order control register."]
pub type MSB_R = crate::BitReader<bool>;
#[doc = "Field `MSB` writer - UART receive order control register."]
pub type MSB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL0_SPEC, bool, O>;
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
pub type PMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL0_SPEC, u8, PMODE_A, 3, O>;
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
#[doc = "Field `DEBSEL` reader - De-bounce control register."]
pub type DEBSEL_R = crate::FieldReader<u8, DEBSEL_A>;
#[doc = "De-bounce control register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DEBSEL_A {
    #[doc = "0: `0`"]
    NONE = 0,
    #[doc = "1: `1`"]
    TWO_RTCCLK = 1,
    #[doc = "2: `10`"]
    THREE_RTCCLK = 2,
    #[doc = "3: `11`"]
    FOUR_RTCCLK = 3,
}
impl From<DEBSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DEBSEL_A) -> Self {
        variant as _
    }
}
impl DEBSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEBSEL_A {
        match self.bits {
            0 => DEBSEL_A::NONE,
            1 => DEBSEL_A::TWO_RTCCLK,
            2 => DEBSEL_A::THREE_RTCCLK,
            3 => DEBSEL_A::FOUR_RTCCLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == DEBSEL_A::NONE
    }
    #[doc = "Checks if the value of the field is `TWO_RTCCLK`"]
    #[inline(always)]
    pub fn is_two_rtcclk(&self) -> bool {
        *self == DEBSEL_A::TWO_RTCCLK
    }
    #[doc = "Checks if the value of the field is `THREE_RTCCLK`"]
    #[inline(always)]
    pub fn is_three_rtcclk(&self) -> bool {
        *self == DEBSEL_A::THREE_RTCCLK
    }
    #[doc = "Checks if the value of the field is `FOUR_RTCCLK`"]
    #[inline(always)]
    pub fn is_four_rtcclk(&self) -> bool {
        *self == DEBSEL_A::FOUR_RTCCLK
    }
}
#[doc = "Field `DEBSEL` writer - De-bounce control register."]
pub type DEBSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTRL0_SPEC, u8, DEBSEL_A, 2, O>;
impl<'a, const O: u8> DEBSEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(DEBSEL_A::NONE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn two_rtcclk(self) -> &'a mut W {
        self.variant(DEBSEL_A::TWO_RTCCLK)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn three_rtcclk(self) -> &'a mut W {
        self.variant(DEBSEL_A::THREE_RTCCLK)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn four_rtcclk(self) -> &'a mut W {
        self.variant(DEBSEL_A::FOUR_RTCCLK)
    }
}
#[doc = "Field `WKUMODE` reader - Wake-up mode control register."]
pub type WKUMODE_R = crate::BitReader<bool>;
#[doc = "Field `WKUMODE` writer - Wake-up mode control register."]
pub type WKUMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - UART 32K controller enable register."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Auto-calibration off control register."]
    #[inline(always)]
    pub fn acoff(&self) -> ACOFF_R {
        ACOFF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART receive order control register."]
    #[inline(always)]
    pub fn msb(&self) -> MSB_R {
        MSB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Parity mode control register."]
    #[inline(always)]
    pub fn pmode(&self) -> PMODE_R {
        PMODE_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - De-bounce control register."]
    #[inline(always)]
    pub fn debsel(&self) -> DEBSEL_R {
        DEBSEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Wake-up mode control register."]
    #[inline(always)]
    pub fn wkumode(&self) -> WKUMODE_R {
        WKUMODE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART 32K controller enable register."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Auto-calibration off control register."]
    #[inline(always)]
    #[must_use]
    pub fn acoff(&mut self) -> ACOFF_W<1> {
        ACOFF_W::new(self)
    }
    #[doc = "Bit 2 - UART receive order control register."]
    #[inline(always)]
    #[must_use]
    pub fn msb(&mut self) -> MSB_W<2> {
        MSB_W::new(self)
    }
    #[doc = "Bits 3:5 - Parity mode control register."]
    #[inline(always)]
    #[must_use]
    pub fn pmode(&mut self) -> PMODE_W<3> {
        PMODE_W::new(self)
    }
    #[doc = "Bits 6:7 - De-bounce control register."]
    #[inline(always)]
    #[must_use]
    pub fn debsel(&mut self) -> DEBSEL_W<6> {
        DEBSEL_W::new(self)
    }
    #[doc = "Bit 8 - Wake-up mode control register."]
    #[inline(always)]
    #[must_use]
    pub fn wkumode(&mut self) -> WKUMODE_W<8> {
        WKUMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART 32K x control register 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl0](index.html) module"]
pub struct CTRL0_SPEC;
impl crate::RegisterSpec for CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl0::R](R) reader structure"]
impl crate::Readable for CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl0::W](W) writer structure"]
impl crate::Writable for CTRL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL0 to value 0"]
impl crate::Resettable for CTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
