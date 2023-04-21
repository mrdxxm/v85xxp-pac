#[doc = "Register `PTRA` reader"]
pub struct R(crate::R<PTRA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTRA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTRA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTRA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTRA` writer"]
pub struct W(crate::W<PTRA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTRA_SPEC>;
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
impl From<crate::W<PTRA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTRA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PTRA` reader - This is the PTRA register of CRYPT controller."]
pub type PTRA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PTRA` writer - This is the PTRA register of CRYPT controller."]
pub type PTRA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTRA_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - This is the PTRA register of CRYPT controller."]
    #[inline(always)]
    pub fn ptra(&self) -> PTRA_R {
        PTRA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This is the PTRA register of CRYPT controller."]
    #[inline(always)]
    #[must_use]
    pub fn ptra(&mut self) -> PTRA_W<0> {
        PTRA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRYPT pointer A.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptra](index.html) module"]
pub struct PTRA_SPEC;
impl crate::RegisterSpec for PTRA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptra::R](R) reader structure"]
impl crate::Readable for PTRA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptra::W](W) writer structure"]
impl crate::Writable for PTRA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTRA to value 0"]
impl crate::Resettable for PTRA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
