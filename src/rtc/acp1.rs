#[doc = "Register `ACP1` reader"]
pub struct R(crate::R<ACP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACP1` writer"]
pub struct W(crate::W<ACP1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACP1_SPEC>;
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
impl From<crate::W<ACP1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACP1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P1` reader - The P1 register is used for temperature test."]
pub type P1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `P1` writer - The P1 register is used for temperature test."]
pub type P1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACP1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - The P1 register is used for temperature test."]
    #[inline(always)]
    pub fn p1(&self) -> P1_R {
        P1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The P1 register is used for temperature test."]
    #[inline(always)]
    #[must_use]
    pub fn p1(&mut self) -> P1_W<0> {
        P1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC parameter P1 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acp1](index.html) module"]
pub struct ACP1_SPEC;
impl crate::RegisterSpec for ACP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acp1::R](R) reader structure"]
impl crate::Readable for ACP1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acp1::W](W) writer structure"]
impl crate::Writable for ACP1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACP1 to value 0"]
impl crate::Resettable for ACP1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
