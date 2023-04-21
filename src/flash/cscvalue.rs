#[doc = "Register `CSCVALUE` reader"]
pub struct R(crate::R<CSCVALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSCVALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSCVALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSCVALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSCVALUE` writer"]
pub struct W(crate::W<CSCVALUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSCVALUE_SPEC>;
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
impl From<crate::W<CSCVALUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSCVALUE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSCVALUE` reader - Checksum compare value register."]
pub type CSCVALUE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CSCVALUE` writer - Checksum compare value register."]
pub type CSCVALUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSCVALUE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Checksum compare value register."]
    #[inline(always)]
    pub fn cscvalue(&self) -> CSCVALUE_R {
        CSCVALUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Checksum compare value register."]
    #[inline(always)]
    #[must_use]
    pub fn cscvalue(&mut self) -> CSCVALUE_W<0> {
        CSCVALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH Checksum compare value register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cscvalue](index.html) module"]
pub struct CSCVALUE_SPEC;
impl crate::RegisterSpec for CSCVALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cscvalue::R](R) reader structure"]
impl crate::Readable for CSCVALUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cscvalue::W](W) writer structure"]
impl crate::Writable for CSCVALUE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSCVALUE to value 0"]
impl crate::Resettable for CSCVALUE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
