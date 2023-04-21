#[doc = "Register `ADCMACTL` reader"]
pub struct R(crate::R<ADCMACTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCMACTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCMACTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCMACTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCMACTL` writer"]
pub struct W(crate::W<ADCMACTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCMACTL_SPEC>;
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
impl From<crate::W<ADCMACTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCMACTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AVERAGE_CHx` reader - Moving average enable for a channel. BITx control Channelx. x=0~15."]
pub type AVERAGE_CHX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `AVERAGE_CHx` writer - Moving average enable for a channel. BITx control Channelx. x=0~15."]
pub type AVERAGE_CHX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADCMACTL_SPEC, u16, u16, 16, O>;
#[doc = "Field `AVERAGE_SAMPLE` reader - Moving average sample count."]
pub type AVERAGE_SAMPLE_R = crate::FieldReader<u8, AVERAGE_SAMPLE_A>;
#[doc = "Moving average sample count.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AVERAGE_SAMPLE_A {
    #[doc = "0: `0`"]
    _2_SAMPLES = 0,
    #[doc = "1: `1`"]
    _4_SAMPLES = 1,
    #[doc = "2: `10`"]
    _8_SAMPLES = 2,
    #[doc = "3: `11`"]
    _16_SAMPLES = 3,
    #[doc = "4: `100`"]
    _32_SAMPLES = 4,
    #[doc = "5: `101`"]
    _64_SAMPLES = 5,
}
impl From<AVERAGE_SAMPLE_A> for u8 {
    #[inline(always)]
    fn from(variant: AVERAGE_SAMPLE_A) -> Self {
        variant as _
    }
}
impl AVERAGE_SAMPLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AVERAGE_SAMPLE_A> {
        match self.bits {
            0 => Some(AVERAGE_SAMPLE_A::_2_SAMPLES),
            1 => Some(AVERAGE_SAMPLE_A::_4_SAMPLES),
            2 => Some(AVERAGE_SAMPLE_A::_8_SAMPLES),
            3 => Some(AVERAGE_SAMPLE_A::_16_SAMPLES),
            4 => Some(AVERAGE_SAMPLE_A::_32_SAMPLES),
            5 => Some(AVERAGE_SAMPLE_A::_64_SAMPLES),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_2_SAMPLES`"]
    #[inline(always)]
    pub fn is_2_samples(&self) -> bool {
        *self == AVERAGE_SAMPLE_A::_2_SAMPLES
    }
    #[doc = "Checks if the value of the field is `_4_SAMPLES`"]
    #[inline(always)]
    pub fn is_4_samples(&self) -> bool {
        *self == AVERAGE_SAMPLE_A::_4_SAMPLES
    }
    #[doc = "Checks if the value of the field is `_8_SAMPLES`"]
    #[inline(always)]
    pub fn is_8_samples(&self) -> bool {
        *self == AVERAGE_SAMPLE_A::_8_SAMPLES
    }
    #[doc = "Checks if the value of the field is `_16_SAMPLES`"]
    #[inline(always)]
    pub fn is_16_samples(&self) -> bool {
        *self == AVERAGE_SAMPLE_A::_16_SAMPLES
    }
    #[doc = "Checks if the value of the field is `_32_SAMPLES`"]
    #[inline(always)]
    pub fn is_32_samples(&self) -> bool {
        *self == AVERAGE_SAMPLE_A::_32_SAMPLES
    }
    #[doc = "Checks if the value of the field is `_64_SAMPLES`"]
    #[inline(always)]
    pub fn is_64_samples(&self) -> bool {
        *self == AVERAGE_SAMPLE_A::_64_SAMPLES
    }
}
#[doc = "Field `AVERAGE_SAMPLE` writer - Moving average sample count."]
pub type AVERAGE_SAMPLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADCMACTL_SPEC, u8, AVERAGE_SAMPLE_A, 3, O>;
impl<'a, const O: u8> AVERAGE_SAMPLE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _2_samples(self) -> &'a mut W {
        self.variant(AVERAGE_SAMPLE_A::_2_SAMPLES)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _4_samples(self) -> &'a mut W {
        self.variant(AVERAGE_SAMPLE_A::_4_SAMPLES)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _8_samples(self) -> &'a mut W {
        self.variant(AVERAGE_SAMPLE_A::_8_SAMPLES)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn _16_samples(self) -> &'a mut W {
        self.variant(AVERAGE_SAMPLE_A::_16_SAMPLES)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn _32_samples(self) -> &'a mut W {
        self.variant(AVERAGE_SAMPLE_A::_32_SAMPLES)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn _64_samples(self) -> &'a mut W {
        self.variant(AVERAGE_SAMPLE_A::_64_SAMPLES)
    }
}
#[doc = "Field `SKIP_SAMPLE` reader - CIC output skip control register."]
pub type SKIP_SAMPLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SKIP_SAMPLE` writer - CIC output skip control register."]
pub type SKIP_SAMPLE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCMACTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `ADCSREF_CAL` reader - ADC ADCSREF used in calibration"]
pub type ADCSREF_CAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADCSREF_CAL` writer - ADC ADCSREF used in calibration"]
pub type ADCSREF_CAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCMACTL_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:15 - Moving average enable for a channel. BITx control Channelx. x=0~15."]
    #[inline(always)]
    pub fn average_chx(&self) -> AVERAGE_CHX_R {
        AVERAGE_CHX_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Moving average sample count."]
    #[inline(always)]
    pub fn average_sample(&self) -> AVERAGE_SAMPLE_R {
        AVERAGE_SAMPLE_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:23 - CIC output skip control register."]
    #[inline(always)]
    pub fn skip_sample(&self) -> SKIP_SAMPLE_R {
        SKIP_SAMPLE_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - ADC ADCSREF used in calibration"]
    #[inline(always)]
    pub fn adcsref_cal(&self) -> ADCSREF_CAL_R {
        ADCSREF_CAL_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Moving average enable for a channel. BITx control Channelx. x=0~15."]
    #[inline(always)]
    #[must_use]
    pub fn average_chx(&mut self) -> AVERAGE_CHX_W<0> {
        AVERAGE_CHX_W::new(self)
    }
    #[doc = "Bits 16:18 - Moving average sample count."]
    #[inline(always)]
    #[must_use]
    pub fn average_sample(&mut self) -> AVERAGE_SAMPLE_W<16> {
        AVERAGE_SAMPLE_W::new(self)
    }
    #[doc = "Bits 20:23 - CIC output skip control register."]
    #[inline(always)]
    #[must_use]
    pub fn skip_sample(&mut self) -> SKIP_SAMPLE_W<20> {
        SKIP_SAMPLE_W::new(self)
    }
    #[doc = "Bits 24:26 - ADC ADCSREF used in calibration"]
    #[inline(always)]
    #[must_use]
    pub fn adcsref_cal(&mut self) -> ADCSREF_CAL_W<24> {
        ADCSREF_CAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC ADC control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmactl](index.html) module"]
pub struct ADCMACTL_SPEC;
impl crate::RegisterSpec for ADCMACTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcmactl::R](R) reader structure"]
impl crate::Readable for ADCMACTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcmactl::W](W) writer structure"]
impl crate::Writable for ADCMACTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCMACTL to value 0x7000_0000"]
impl crate::Resettable for ADCMACTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x7000_0000;
}
