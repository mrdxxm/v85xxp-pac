#[doc = "Register `ADCCTRL0` reader"]
pub struct R(crate::R<ADCCTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCCTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCCTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCCTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCCTRL0` writer"]
pub struct W(crate::W<ADCCTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCCTRL0_SPEC>;
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
impl From<crate::W<ADCCTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCCTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKSRCSEL` reader - ADC clock source selection."]
pub type CLKSRCSEL_R = crate::BitReader<CLKSRCSEL_A>;
#[doc = "ADC clock source selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKSRCSEL_A {
    #[doc = "0: `0`"]
    RCH = 0,
    #[doc = "1: `1`"]
    PLLL = 1,
}
impl From<CLKSRCSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CLKSRCSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKSRCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKSRCSEL_A {
        match self.bits {
            false => CLKSRCSEL_A::RCH,
            true => CLKSRCSEL_A::PLLL,
        }
    }
    #[doc = "Checks if the value of the field is `RCH`"]
    #[inline(always)]
    pub fn is_rch(&self) -> bool {
        *self == CLKSRCSEL_A::RCH
    }
    #[doc = "Checks if the value of the field is `PLLL`"]
    #[inline(always)]
    pub fn is_plll(&self) -> bool {
        *self == CLKSRCSEL_A::PLLL
    }
}
#[doc = "Field `CLKSRCSEL` writer - ADC clock source selection."]
pub type CLKSRCSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCTRL0_SPEC, CLKSRCSEL_A, O>;
impl<'a, const O: u8> CLKSRCSEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rch(self) -> &'a mut W {
        self.variant(CLKSRCSEL_A::RCH)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn plll(self) -> &'a mut W {
        self.variant(CLKSRCSEL_A::PLLL)
    }
}
#[doc = "Field `AEN` reader - Auto ADC conversion enable control register."]
pub type AEN_R = crate::FieldReader<u8, AEN_A>;
#[doc = "Auto ADC conversion enable control register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AEN_A {
    #[doc = "0: `0`"]
    OFF = 0,
    #[doc = "1: Auto ADC will be triggered by RTC_INTSTS\\[0\\]
that is determined by RTC_ITV and RTC_SITV."]
    RTC_ITV_AND_RTC_SITV = 1,
    #[doc = "2: `10`"]
    RTC_WKUSEC = 2,
    #[doc = "3: `11`"]
    RTC_ALARM = 3,
    #[doc = "4: `100`"]
    TIMER0_OVERFLOW = 4,
    #[doc = "5: `101`"]
    TIMER1_OVERFLOW = 5,
    #[doc = "6: `110`"]
    TIMER2_OVERFLOW = 6,
    #[doc = "7: `111`"]
    TIMER3_OVERFLOW = 7,
}
impl From<AEN_A> for u8 {
    #[inline(always)]
    fn from(variant: AEN_A) -> Self {
        variant as _
    }
}
impl AEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AEN_A {
        match self.bits {
            0 => AEN_A::OFF,
            1 => AEN_A::RTC_ITV_AND_RTC_SITV,
            2 => AEN_A::RTC_WKUSEC,
            3 => AEN_A::RTC_ALARM,
            4 => AEN_A::TIMER0_OVERFLOW,
            5 => AEN_A::TIMER1_OVERFLOW,
            6 => AEN_A::TIMER2_OVERFLOW,
            7 => AEN_A::TIMER3_OVERFLOW,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == AEN_A::OFF
    }
    #[doc = "Checks if the value of the field is `RTC_ITV_AND_RTC_SITV`"]
    #[inline(always)]
    pub fn is_rtc_itv_and_rtc_sitv(&self) -> bool {
        *self == AEN_A::RTC_ITV_AND_RTC_SITV
    }
    #[doc = "Checks if the value of the field is `RTC_WKUSEC`"]
    #[inline(always)]
    pub fn is_rtc_wkusec(&self) -> bool {
        *self == AEN_A::RTC_WKUSEC
    }
    #[doc = "Checks if the value of the field is `RTC_ALARM`"]
    #[inline(always)]
    pub fn is_rtc_alarm(&self) -> bool {
        *self == AEN_A::RTC_ALARM
    }
    #[doc = "Checks if the value of the field is `TIMER0_OVERFLOW`"]
    #[inline(always)]
    pub fn is_timer0_overflow(&self) -> bool {
        *self == AEN_A::TIMER0_OVERFLOW
    }
    #[doc = "Checks if the value of the field is `TIMER1_OVERFLOW`"]
    #[inline(always)]
    pub fn is_timer1_overflow(&self) -> bool {
        *self == AEN_A::TIMER1_OVERFLOW
    }
    #[doc = "Checks if the value of the field is `TIMER2_OVERFLOW`"]
    #[inline(always)]
    pub fn is_timer2_overflow(&self) -> bool {
        *self == AEN_A::TIMER2_OVERFLOW
    }
    #[doc = "Checks if the value of the field is `TIMER3_OVERFLOW`"]
    #[inline(always)]
    pub fn is_timer3_overflow(&self) -> bool {
        *self == AEN_A::TIMER3_OVERFLOW
    }
}
#[doc = "Field `AEN` writer - Auto ADC conversion enable control register."]
pub type AEN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ADCCTRL0_SPEC, u8, AEN_A, 3, O>;
impl<'a, const O: u8> AEN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(AEN_A::OFF)
    }
    #[doc = "Auto ADC will be triggered by RTC_INTSTS\\[0\\]
that is determined by RTC_ITV and RTC_SITV."]
    #[inline(always)]
    pub fn rtc_itv_and_rtc_sitv(self) -> &'a mut W {
        self.variant(AEN_A::RTC_ITV_AND_RTC_SITV)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn rtc_wkusec(self) -> &'a mut W {
        self.variant(AEN_A::RTC_WKUSEC)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn rtc_alarm(self) -> &'a mut W {
        self.variant(AEN_A::RTC_ALARM)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn timer0_overflow(self) -> &'a mut W {
        self.variant(AEN_A::TIMER0_OVERFLOW)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn timer1_overflow(self) -> &'a mut W {
        self.variant(AEN_A::TIMER1_OVERFLOW)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn timer2_overflow(self) -> &'a mut W {
        self.variant(AEN_A::TIMER2_OVERFLOW)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn timer3_overflow(self) -> &'a mut W {
        self.variant(AEN_A::TIMER3_OVERFLOW)
    }
}
#[doc = "Field `STOP` reader - Force stop current ADC conversion process."]
pub type STOP_R = crate::BitReader<bool>;
#[doc = "Field `STOP` writer - Force stop current ADC conversion process."]
pub type STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCTRL0_SPEC, bool, O>;
#[doc = "Field `MTRIG` reader - Manual ADC trigger."]
pub type MTRIG_R = crate::BitReader<bool>;
#[doc = "Field `MTRIG` writer - Manual ADC trigger."]
pub type MTRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCTRL0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 12 - ADC clock source selection."]
    #[inline(always)]
    pub fn clksrcsel(&self) -> CLKSRCSEL_R {
        CLKSRCSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Auto ADC conversion enable control register."]
    #[inline(always)]
    pub fn aen(&self) -> AEN_R {
        AEN_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Force stop current ADC conversion process."]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 31 - Manual ADC trigger."]
    #[inline(always)]
    pub fn mtrig(&self) -> MTRIG_R {
        MTRIG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - ADC clock source selection."]
    #[inline(always)]
    #[must_use]
    pub fn clksrcsel(&mut self) -> CLKSRCSEL_W<12> {
        CLKSRCSEL_W::new(self)
    }
    #[doc = "Bits 16:18 - Auto ADC conversion enable control register."]
    #[inline(always)]
    #[must_use]
    pub fn aen(&mut self) -> AEN_W<16> {
        AEN_W::new(self)
    }
    #[doc = "Bit 19 - Force stop current ADC conversion process."]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<19> {
        STOP_W::new(self)
    }
    #[doc = "Bit 31 - Manual ADC trigger."]
    #[inline(always)]
    #[must_use]
    pub fn mtrig(&mut self) -> MTRIG_W<31> {
        MTRIG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcctrl0](index.html) module"]
pub struct ADCCTRL0_SPEC;
impl crate::RegisterSpec for ADCCTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcctrl0::R](R) reader structure"]
impl crate::Readable for ADCCTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcctrl0::W](W) writer structure"]
impl crate::Writable for ADCCTRL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCCTRL0 to value 0"]
impl crate::Resettable for ADCCTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
