#[doc = "Register `BAUDDIVL` reader"]
pub struct R(crate::R<BAUDDIVL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BAUDDIVL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BAUDDIVL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BAUDDIVL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BAUDDIVL` writer"]
pub struct W(crate::W<BAUDDIVL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BAUDDIVL_SPEC>;
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
impl From<crate::W<BAUDDIVL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BAUDDIVL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BAUDDIVL` reader - Low byte of baud-rate divider."]
pub type BAUDDIVL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BAUDDIVL` writer - Low byte of baud-rate divider."]
pub type BAUDDIVL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BAUDDIVL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Low byte of baud-rate divider."]
    #[inline(always)]
    pub fn bauddivl(&self) -> BAUDDIVL_R {
        BAUDDIVL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low byte of baud-rate divider."]
    #[inline(always)]
    #[must_use]
    pub fn bauddivl(&mut self) -> BAUDDIVL_W<0> {
        BAUDDIVL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISO7816 baud-rate low byte register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bauddivl](index.html) module"]
pub struct BAUDDIVL_SPEC;
impl crate::RegisterSpec for BAUDDIVL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bauddivl::R](R) reader structure"]
impl crate::Readable for BAUDDIVL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bauddivl::W](W) writer structure"]
impl crate::Writable for BAUDDIVL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BAUDDIVL to value 0"]
impl crate::Resettable for BAUDDIVL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
