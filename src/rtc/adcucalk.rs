#[doc = "Register `ADCUCALK` reader"]
pub struct R(crate::R<ADCUCALK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCUCALK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCUCALK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCUCALK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCUCALK` writer"]
pub struct W(crate::W<ADCUCALK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCUCALK_SPEC>;
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
impl From<crate::W<ADCUCALK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCUCALK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCAL_K1` reader - Ucal K1"]
pub type UCAL_K1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `UCAL_K1` writer - Ucal K1"]
pub type UCAL_K1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCUCALK_SPEC, u16, u16, 16, O>;
#[doc = "Field `UCAL_K3` reader - Ucal K3"]
pub type UCAL_K3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `UCAL_K3` writer - Ucal K3"]
pub type UCAL_K3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCUCALK_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Ucal K1"]
    #[inline(always)]
    pub fn ucal_k1(&self) -> UCAL_K1_R {
        UCAL_K1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Ucal K3"]
    #[inline(always)]
    pub fn ucal_k3(&self) -> UCAL_K3_R {
        UCAL_K3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Ucal K1"]
    #[inline(always)]
    #[must_use]
    pub fn ucal_k1(&mut self) -> UCAL_K1_W<0> {
        UCAL_K1_W::new(self)
    }
    #[doc = "Bits 16:31 - Ucal K3"]
    #[inline(always)]
    #[must_use]
    pub fn ucal_k3(&mut self) -> UCAL_K3_W<16> {
        UCAL_K3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC ADC Ucal K coefficients\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcucalk](index.html) module"]
pub struct ADCUCALK_SPEC;
impl crate::RegisterSpec for ADCUCALK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcucalk::R](R) reader structure"]
impl crate::Readable for ADCUCALK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcucalk::W](W) writer structure"]
impl crate::Writable for ADCUCALK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCUCALK to value 0x546a_546a"]
impl crate::Resettable for ADCUCALK_SPEC {
    const RESET_VALUE: Self::Ux = 0x546a_546a;
}
