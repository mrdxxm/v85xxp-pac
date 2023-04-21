#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSMODE` reader - This register is used to control the checksum mode."]
pub type CSMODE_R = crate::FieldReader<u8, CSMODE_A>;
#[doc = "This register is used to control the checksum mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSMODE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ALWAYS_ON = 1,
    #[doc = "2: `10`"]
    TIMER2_OVERFLOW = 2,
    #[doc = "3: `11`"]
    RISING_EDGE_OF_RTC_SECOND_PULSE = 3,
}
impl From<CSMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CSMODE_A) -> Self {
        variant as _
    }
}
impl CSMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSMODE_A {
        match self.bits {
            0 => CSMODE_A::DISABLE,
            1 => CSMODE_A::ALWAYS_ON,
            2 => CSMODE_A::TIMER2_OVERFLOW,
            3 => CSMODE_A::RISING_EDGE_OF_RTC_SECOND_PULSE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CSMODE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ALWAYS_ON`"]
    #[inline(always)]
    pub fn is_always_on(&self) -> bool {
        *self == CSMODE_A::ALWAYS_ON
    }
    #[doc = "Checks if the value of the field is `TIMER2_OVERFLOW`"]
    #[inline(always)]
    pub fn is_timer2_overflow(&self) -> bool {
        *self == CSMODE_A::TIMER2_OVERFLOW
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE_OF_RTC_SECOND_PULSE`"]
    #[inline(always)]
    pub fn is_rising_edge_of_rtc_second_pulse(&self) -> bool {
        *self == CSMODE_A::RISING_EDGE_OF_RTC_SECOND_PULSE
    }
}
#[doc = "Field `CSMODE` writer - This register is used to control the checksum mode."]
pub type CSMODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, CSMODE_A, 2, O>;
impl<'a, const O: u8> CSMODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CSMODE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn always_on(self) -> &'a mut W {
        self.variant(CSMODE_A::ALWAYS_ON)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn timer2_overflow(self) -> &'a mut W {
        self.variant(CSMODE_A::TIMER2_OVERFLOW)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn rising_edge_of_rtc_second_pulse(self) -> &'a mut W {
        self.variant(CSMODE_A::RISING_EDGE_OF_RTC_SECOND_PULSE)
    }
}
#[doc = "Field `CSINTEN` reader - This register is used to control the interrupt enable of checksum error."]
pub type CSINTEN_R = crate::BitReader<bool>;
#[doc = "Field `CSINTEN` writer - This register is used to control the interrupt enable of checksum error."]
pub type CSINTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - This register is used to control the checksum mode."]
    #[inline(always)]
    pub fn csmode(&self) -> CSMODE_R {
        CSMODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - This register is used to control the interrupt enable of checksum error."]
    #[inline(always)]
    pub fn csinten(&self) -> CSINTEN_R {
        CSINTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - This register is used to control the checksum mode."]
    #[inline(always)]
    #[must_use]
    pub fn csmode(&mut self) -> CSMODE_W<0> {
        CSMODE_W::new(self)
    }
    #[doc = "Bit 2 - This register is used to control the interrupt enable of checksum error."]
    #[inline(always)]
    #[must_use]
    pub fn csinten(&mut self) -> CSINTEN_W<2> {
        CSINTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
