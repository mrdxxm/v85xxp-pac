#[doc = "Register `YEAR` reader"]
pub struct R(crate::R<YEAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<YEAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<YEAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<YEAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `YEAR` writer"]
pub struct W(crate::W<YEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<YEAR_SPEC>;
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
impl From<crate::W<YEAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<YEAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `YEAR` reader - RTC_YEAR, 00~99."]
pub type YEAR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `YEAR` writer - RTC_YEAR, 00~99."]
pub type YEAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, YEAR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - RTC_YEAR, 00~99."]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RTC_YEAR, 00~99."]
    #[inline(always)]
    #[must_use]
    pub fn year(&mut self) -> YEAR_W<0> {
        YEAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC year register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [year](index.html) module"]
pub struct YEAR_SPEC;
impl crate::RegisterSpec for YEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [year::R](R) reader structure"]
impl crate::Readable for YEAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [year::W](W) writer structure"]
impl crate::Writable for YEAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
