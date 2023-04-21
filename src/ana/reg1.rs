#[doc = "Register `REG1` reader"]
pub struct R(crate::R<REG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG1` writer"]
pub struct W(crate::W<REG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG1_SPEC>;
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
impl From<crate::W<REG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCSEL` reader - ADC channel select."]
pub type ADCSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADCMODESEL` reader - ADC mode select."]
pub type ADCMODESEL_R = crate::BitReader<bool>;
#[doc = "Field `ADCMODESEL` writer - ADC mode select."]
pub type ADCMODESEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - ADC channel select."]
    #[inline(always)]
    pub fn adcsel(&self) -> ADCSEL_R {
        ADCSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - ADC mode select."]
    #[inline(always)]
    pub fn adcmodesel(&self) -> ADCMODESEL_R {
        ADCMODESEL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - ADC mode select."]
    #[inline(always)]
    #[must_use]
    pub fn adcmodesel(&mut self) -> ADCMODESEL_W<7> {
        ADCMODESEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg1](index.html) module"]
pub struct REG1_SPEC;
impl crate::RegisterSpec for REG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg1::R](R) reader structure"]
impl crate::Readable for REG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg1::W](W) writer structure"]
impl crate::Writable for REG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REG1 to value 0"]
impl crate::Resettable for REG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
