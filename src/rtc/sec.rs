#[doc = "Register `SEC` reader"]
pub struct R(crate::R<SEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEC` writer"]
pub struct W(crate::W<SEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEC_SPEC>;
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
impl From<crate::W<SEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC` reader - RTC_SEC, 0~59."]
pub type SEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEC` writer - RTC_SEC, 0~59."]
pub type SEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEC_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - RTC_SEC, 0~59."]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - RTC_SEC, 0~59."]
    #[inline(always)]
    #[must_use]
    pub fn sec(&mut self) -> SEC_W<0> {
        SEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC second register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec](index.html) module"]
pub struct SEC_SPEC;
impl crate::RegisterSpec for SEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec::R](R) reader structure"]
impl crate::Readable for SEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sec::W](W) writer structure"]
impl crate::Writable for SEC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
