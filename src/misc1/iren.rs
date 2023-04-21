#[doc = "Register `IREN` reader"]
pub struct R(crate::R<IREN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IREN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IREN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IREN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IREN` writer"]
pub struct W(crate::W<IREN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IREN_SPEC>;
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
impl From<crate::W<IREN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IREN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IREN` reader - IR enable control register."]
pub type IREN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IREN` writer - IR enable control register."]
pub type IREN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IREN_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - IR enable control register."]
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - IR enable control register."]
    #[inline(always)]
    #[must_use]
    pub fn iren(&mut self) -> IREN_W<0> {
        IREN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IR enable control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iren](index.html) module"]
pub struct IREN_SPEC;
impl crate::RegisterSpec for IREN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iren::R](R) reader structure"]
impl crate::Readable for IREN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iren::W](W) writer structure"]
impl crate::Writable for IREN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IREN to value 0"]
impl crate::Resettable for IREN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
