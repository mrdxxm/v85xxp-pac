#[doc = "Register `ADCCTRL2` reader"]
pub struct R(crate::R<ADCCTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCCTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCCTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCCTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCCTRL2` writer"]
pub struct W(crate::W<ADCCTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCCTRL2_SPEC>;
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
impl From<crate::W<ADCCTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCCTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_EN` reader - ADC Enabe."]
pub type ADC_EN_R = crate::BitReader<bool>;
#[doc = "Field `ADC_EN` writer - ADC Enabe."]
pub type ADC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCTRL2_SPEC, bool, O>;
#[doc = "Field `RESET` reader - RESET in ADC interface, driven by CPU"]
pub type RESET_R = crate::BitReader<bool>;
#[doc = "Field `RESET` writer - RESET in ADC interface, driven by CPU"]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCTRL2_SPEC, bool, O>;
#[doc = "Field `ADCCR` reader - ADCCR in ADC interface"]
pub type ADCCR_R = crate::BitReader<bool>;
#[doc = "Field `ADCCR` writer - ADCCR in ADC interface"]
pub type ADCCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCTRL2_SPEC, bool, O>;
#[doc = "Field `BUSY` reader - BUSY in ADC interface"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` writer - BUSY in ADC interface"]
pub type BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCTRL2_SPEC, bool, O>;
#[doc = "Field `ADC_EN_TRG_CAL` reader - 1: Writing 1 to ADC_EN will trigger automatic CAL_EN flow."]
pub type ADC_EN_TRG_CAL_R = crate::BitReader<bool>;
#[doc = "Field `ADC_EN_TRG_CAL` writer - 1: Writing 1 to ADC_EN will trigger automatic CAL_EN flow."]
pub type ADC_EN_TRG_CAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCTRL2_SPEC, bool, O>;
#[doc = "Field `ADC_CAL_DONE` reader - Indicate automatic CAL_EN flow done"]
pub type ADC_CAL_DONE_R = crate::BitReader<bool>;
#[doc = "Field `ADC_CAL_DONE` writer - Indicate automatic CAL_EN flow done"]
pub type ADC_CAL_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCTRL2_SPEC, bool, O>;
#[doc = "Field `CAL_ERR_CLR` reader - CAL_ERR_CLR in ADC interface"]
pub type CAL_ERR_CLR_R = crate::BitReader<bool>;
#[doc = "Field `CAL_ERR_CLR` writer - CAL_ERR_CLR in ADC interface"]
pub type CAL_ERR_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCTRL2_SPEC, bool, O>;
#[doc = "Field `CONV_ERR_CLR` reader - CONV_ERR_CLR in ADC interface"]
pub type CONV_ERR_CLR_R = crate::BitReader<bool>;
#[doc = "Field `CONV_ERR_CLR` writer - CONV_ERR_CLR in ADC interface"]
pub type CONV_ERR_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCTRL2_SPEC, bool, O>;
#[doc = "Field `CAL_ERR` reader - CAL_ERR in ADC interface"]
pub type CAL_ERR_R = crate::BitReader<bool>;
#[doc = "Field `CAL_ERR` writer - CAL_ERR in ADC interface"]
pub type CAL_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCTRL2_SPEC, bool, O>;
#[doc = "Field `CONV_ERR` reader - CONV_ERR in ADC interface"]
pub type CONV_ERR_R = crate::BitReader<bool>;
#[doc = "Field `CONV_ERR` writer - CONV_ERR in ADC interface"]
pub type CONV_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCTRL2_SPEC, bool, O>;
#[doc = "Field `SCAN_CHx` reader - BITy Control CHx auto-scan enable. y=31~16, x=15~0."]
pub type SCAN_CHX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SCAN_CHx` writer - BITy Control CHx auto-scan enable. y=31~16, x=15~0."]
pub type SCAN_CHX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCCTRL2_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - ADC Enabe."]
    #[inline(always)]
    pub fn adc_en(&self) -> ADC_EN_R {
        ADC_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RESET in ADC interface, driven by CPU"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - ADCCR in ADC interface"]
    #[inline(always)]
    pub fn adccr(&self) -> ADCCR_R {
        ADCCR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - BUSY in ADC interface"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: Writing 1 to ADC_EN will trigger automatic CAL_EN flow."]
    #[inline(always)]
    pub fn adc_en_trg_cal(&self) -> ADC_EN_TRG_CAL_R {
        ADC_EN_TRG_CAL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicate automatic CAL_EN flow done"]
    #[inline(always)]
    pub fn adc_cal_done(&self) -> ADC_CAL_DONE_R {
        ADC_CAL_DONE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CAL_ERR_CLR in ADC interface"]
    #[inline(always)]
    pub fn cal_err_clr(&self) -> CAL_ERR_CLR_R {
        CAL_ERR_CLR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CONV_ERR_CLR in ADC interface"]
    #[inline(always)]
    pub fn conv_err_clr(&self) -> CONV_ERR_CLR_R {
        CONV_ERR_CLR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CAL_ERR in ADC interface"]
    #[inline(always)]
    pub fn cal_err(&self) -> CAL_ERR_R {
        CAL_ERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CONV_ERR in ADC interface"]
    #[inline(always)]
    pub fn conv_err(&self) -> CONV_ERR_R {
        CONV_ERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:31 - BITy Control CHx auto-scan enable. y=31~16, x=15~0."]
    #[inline(always)]
    pub fn scan_chx(&self) -> SCAN_CHX_R {
        SCAN_CHX_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Enabe."]
    #[inline(always)]
    #[must_use]
    pub fn adc_en(&mut self) -> ADC_EN_W<0> {
        ADC_EN_W::new(self)
    }
    #[doc = "Bit 1 - RESET in ADC interface, driven by CPU"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<1> {
        RESET_W::new(self)
    }
    #[doc = "Bit 3 - ADCCR in ADC interface"]
    #[inline(always)]
    #[must_use]
    pub fn adccr(&mut self) -> ADCCR_W<3> {
        ADCCR_W::new(self)
    }
    #[doc = "Bit 5 - BUSY in ADC interface"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<5> {
        BUSY_W::new(self)
    }
    #[doc = "Bit 6 - 1: Writing 1 to ADC_EN will trigger automatic CAL_EN flow."]
    #[inline(always)]
    #[must_use]
    pub fn adc_en_trg_cal(&mut self) -> ADC_EN_TRG_CAL_W<6> {
        ADC_EN_TRG_CAL_W::new(self)
    }
    #[doc = "Bit 7 - Indicate automatic CAL_EN flow done"]
    #[inline(always)]
    #[must_use]
    pub fn adc_cal_done(&mut self) -> ADC_CAL_DONE_W<7> {
        ADC_CAL_DONE_W::new(self)
    }
    #[doc = "Bit 8 - CAL_ERR_CLR in ADC interface"]
    #[inline(always)]
    #[must_use]
    pub fn cal_err_clr(&mut self) -> CAL_ERR_CLR_W<8> {
        CAL_ERR_CLR_W::new(self)
    }
    #[doc = "Bit 9 - CONV_ERR_CLR in ADC interface"]
    #[inline(always)]
    #[must_use]
    pub fn conv_err_clr(&mut self) -> CONV_ERR_CLR_W<9> {
        CONV_ERR_CLR_W::new(self)
    }
    #[doc = "Bit 10 - CAL_ERR in ADC interface"]
    #[inline(always)]
    #[must_use]
    pub fn cal_err(&mut self) -> CAL_ERR_W<10> {
        CAL_ERR_W::new(self)
    }
    #[doc = "Bit 11 - CONV_ERR in ADC interface"]
    #[inline(always)]
    #[must_use]
    pub fn conv_err(&mut self) -> CONV_ERR_W<11> {
        CONV_ERR_W::new(self)
    }
    #[doc = "Bits 16:31 - BITy Control CHx auto-scan enable. y=31~16, x=15~0."]
    #[inline(always)]
    #[must_use]
    pub fn scan_chx(&mut self) -> SCAN_CHX_W<16> {
        SCAN_CHX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ANA_ADCCTRL2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcctrl2](index.html) module"]
pub struct ADCCTRL2_SPEC;
impl crate::RegisterSpec for ADCCTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcctrl2::R](R) reader structure"]
impl crate::Readable for ADCCTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcctrl2::W](W) writer structure"]
impl crate::Writable for ADCCTRL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCCTRL2 to value 0"]
impl crate::Resettable for ADCCTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
