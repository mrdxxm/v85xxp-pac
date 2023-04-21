#[doc = "Register `ACTI` reader"]
pub struct R(crate::R<ACTI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACTI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACTI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACTI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACTI` writer"]
pub struct W(crate::W<ACTI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACTI_SPEC>;
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
impl From<crate::W<ACTI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACTI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACTI` reader - Auto-calibration Ti control register."]
pub type ACTI_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ACTI` writer - Auto-calibration Ti control register."]
pub type ACTI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACTI_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Auto-calibration Ti control register."]
    #[inline(always)]
    pub fn acti(&self) -> ACTI_R {
        ACTI_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Auto-calibration Ti control register."]
    #[inline(always)]
    #[must_use]
    pub fn acti(&mut self) -> ACTI_W<0> {
        ACTI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC auto-calibration center temperature control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acti](index.html) module"]
pub struct ACTI_SPEC;
impl crate::RegisterSpec for ACTI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acti::R](R) reader structure"]
impl crate::Readable for ACTI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acti::W](W) writer structure"]
impl crate::Writable for ACTI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACTI to value 0x1800"]
impl crate::Resettable for ACTI_SPEC {
    const RESET_VALUE: Self::Ux = 0x1800;
}
