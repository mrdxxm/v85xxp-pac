#[doc = "Register `MON` reader"]
pub struct R(crate::R<MON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MON` writer"]
pub struct W(crate::W<MON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MON_SPEC>;
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
impl From<crate::W<MON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MON` reader - RTC_MON, 1~12."]
pub type MON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MON` writer - RTC_MON, 1~12."]
pub type MON_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MON_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - RTC_MON, 1~12."]
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - RTC_MON, 1~12."]
    #[inline(always)]
    #[must_use]
    pub fn mon(&mut self) -> MON_W<0> {
        MON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC mon register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mon](index.html) module"]
pub struct MON_SPEC;
impl crate::RegisterSpec for MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mon::R](R) reader structure"]
impl crate::Readable for MON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mon::W](W) writer structure"]
impl crate::Writable for MON_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
