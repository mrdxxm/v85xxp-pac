#[doc = "Register `HOUR` reader"]
pub struct R(crate::R<HOUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOUR` writer"]
pub struct W(crate::W<HOUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOUR_SPEC>;
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
impl From<crate::W<HOUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOUR` reader - RTC_HOUR, 0~23."]
pub type HOUR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOUR` writer - RTC_HOUR, 0~23."]
pub type HOUR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HOUR_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - RTC_HOUR, 0~23."]
    #[inline(always)]
    pub fn hour(&self) -> HOUR_R {
        HOUR_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - RTC_HOUR, 0~23."]
    #[inline(always)]
    #[must_use]
    pub fn hour(&mut self) -> HOUR_W<0> {
        HOUR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC hour register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hour](index.html) module"]
pub struct HOUR_SPEC;
impl crate::RegisterSpec for HOUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hour::R](R) reader structure"]
impl crate::Readable for HOUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hour::W](W) writer structure"]
impl crate::Writable for HOUR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
