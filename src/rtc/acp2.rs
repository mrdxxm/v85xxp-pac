#[doc = "Register `ACP2` reader"]
pub struct R(crate::R<ACP2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACP2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACP2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACP2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACP2` writer"]
pub struct W(crate::W<ACP2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACP2_SPEC>;
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
impl From<crate::W<ACP2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACP2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P2` reader - The P2 register is used for temperature test."]
pub type P2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `P2` writer - The P2 register is used for temperature test."]
pub type P2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACP2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - The P2 register is used for temperature test."]
    #[inline(always)]
    pub fn p2(&self) -> P2_R {
        P2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The P2 register is used for temperature test."]
    #[inline(always)]
    #[must_use]
    pub fn p2(&mut self) -> P2_W<0> {
        P2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC parameter P2 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acp2](index.html) module"]
pub struct ACP2_SPEC;
impl crate::RegisterSpec for ACP2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acp2::R](R) reader structure"]
impl crate::Readable for ACP2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acp2::W](W) writer structure"]
impl crate::Writable for ACP2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACP2 to value 0"]
impl crate::Resettable for ACP2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
