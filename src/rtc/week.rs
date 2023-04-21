#[doc = "Register `WEEK` reader"]
pub struct R(crate::R<WEEK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WEEK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WEEK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WEEK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WEEK` writer"]
pub struct W(crate::W<WEEK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WEEK_SPEC>;
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
impl From<crate::W<WEEK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WEEK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WEEK` reader - RTC_WEEK, 0~6."]
pub type WEEK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WEEK` writer - RTC_WEEK, 0~6."]
pub type WEEK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WEEK_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - RTC_WEEK, 0~6."]
    #[inline(always)]
    pub fn week(&self) -> WEEK_R {
        WEEK_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - RTC_WEEK, 0~6."]
    #[inline(always)]
    #[must_use]
    pub fn week(&mut self) -> WEEK_W<0> {
        WEEK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC week register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [week](index.html) module"]
pub struct WEEK_SPEC;
impl crate::RegisterSpec for WEEK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [week::R](R) reader structure"]
impl crate::Readable for WEEK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [week::W](W) writer structure"]
impl crate::Writable for WEEK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
