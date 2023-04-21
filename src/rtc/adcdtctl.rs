#[doc = "Register `ADCDTCTL` reader"]
pub struct R(crate::R<ADCDTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCDTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCDTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCDTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCDTCTL` writer"]
pub struct W(crate::W<ADCDTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCDTCTL_SPEC>;
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
impl From<crate::W<ADCDTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCDTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENDED_IN2ADC_CONVERT` reader - Sig/diff ended data processing mode."]
pub type ENDED_IN2ADC_CONVERT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ENDED_IN2ADC_CONVERT` writer - Sig/diff ended data processing mode."]
pub type ENDED_IN2ADC_CONVERT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADCDTCTL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 30:31 - Sig/diff ended data processing mode."]
    #[inline(always)]
    pub fn ended_in2adc_convert(&self) -> ENDED_IN2ADC_CONVERT_R {
        ENDED_IN2ADC_CONVERT_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Sig/diff ended data processing mode."]
    #[inline(always)]
    #[must_use]
    pub fn ended_in2adc_convert(&mut self) -> ENDED_IN2ADC_CONVERT_W<30> {
        ENDED_IN2ADC_CONVERT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC ADC data control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcdtctl](index.html) module"]
pub struct ADCDTCTL_SPEC;
impl crate::RegisterSpec for ADCDTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcdtctl::R](R) reader structure"]
impl crate::Readable for ADCDTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcdtctl::W](W) writer structure"]
impl crate::Writable for ADCDTCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCDTCTL to value 0x8000_0000"]
impl crate::Resettable for ADCDTCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
