#[doc = "Register `PTRO` reader"]
pub struct R(crate::R<PTRO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTRO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTRO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTRO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTRO` writer"]
pub struct W(crate::W<PTRO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTRO_SPEC>;
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
impl From<crate::W<PTRO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTRO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PTRO` reader - This is the PTRO register of CRYPT controller."]
pub type PTRO_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PTRO` writer - This is the PTRO register of CRYPT controller."]
pub type PTRO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTRO_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - This is the PTRO register of CRYPT controller."]
    #[inline(always)]
    pub fn ptro(&self) -> PTRO_R {
        PTRO_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This is the PTRO register of CRYPT controller."]
    #[inline(always)]
    #[must_use]
    pub fn ptro(&mut self) -> PTRO_W<0> {
        PTRO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRYPT pointer O.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptro](index.html) module"]
pub struct PTRO_SPEC;
impl crate::RegisterSpec for PTRO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptro::R](R) reader structure"]
impl crate::Readable for PTRO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptro::W](W) writer structure"]
impl crate::Writable for PTRO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTRO to value 0"]
impl crate::Resettable for PTRO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
