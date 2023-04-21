#[doc = "Register `MIN` reader"]
pub struct R(crate::R<MIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIN` writer"]
pub struct W(crate::W<MIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIN_SPEC>;
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
impl From<crate::W<MIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MIN` reader - RTC_MIN, 0~59."]
pub type MIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MIN` writer - RTC_MIN, 0~59."]
pub type MIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MIN_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - RTC_MIN, 0~59."]
    #[inline(always)]
    pub fn min(&self) -> MIN_R {
        MIN_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - RTC_MIN, 0~59."]
    #[inline(always)]
    #[must_use]
    pub fn min(&mut self) -> MIN_W<0> {
        MIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC minute register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [min](index.html) module"]
pub struct MIN_SPEC;
impl crate::RegisterSpec for MIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [min::R](R) reader structure"]
impl crate::Readable for MIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [min::W](W) writer structure"]
impl crate::Writable for MIN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
