#[doc = "Register `ICEPROT` reader"]
pub struct R(crate::R<ICEPROT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICEPROT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICEPROT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICEPROT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICEPROT` writer"]
pub struct W(crate::W<ICEPROT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICEPROT_SPEC>;
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
impl From<crate::W<ICEPROT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICEPROT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICEPROT` reader - This bit is used to indicate if the ICE protection mode is set or not."]
pub type ICEPROT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ICEPROT` writer - This bit is used to indicate if the ICE protection mode is set or not."]
pub type ICEPROT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ICEPROT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - This bit is used to indicate if the ICE protection mode is set or not."]
    #[inline(always)]
    pub fn iceprot(&self) -> ICEPROT_R {
        ICEPROT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This bit is used to indicate if the ICE protection mode is set or not."]
    #[inline(always)]
    #[must_use]
    pub fn iceprot(&mut self) -> ICEPROT_W<0> {
        ICEPROT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ICE protect register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iceprot](index.html) module"]
pub struct ICEPROT_SPEC;
impl crate::RegisterSpec for ICEPROT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iceprot::R](R) reader structure"]
impl crate::Readable for ICEPROT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iceprot::W](W) writer structure"]
impl crate::Writable for ICEPROT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICEPROT to value 0"]
impl crate::Resettable for ICEPROT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
