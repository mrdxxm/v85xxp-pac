#[doc = "Register `WDTCLR` reader"]
pub struct R(crate::R<WDTCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTCLR` writer"]
pub struct W(crate::W<WDTCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTCLR_SPEC>;
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
impl From<crate::W<WDTCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTCNT` reader - This register shows the current counter value of wat dog timer."]
pub type WDTCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WDTCNT` writer - This register shows the current counter value of wat dog timer."]
pub type WDTCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WDTCLR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - This register shows the current counter value of wat dog timer."]
    #[inline(always)]
    pub fn wdtcnt(&self) -> WDTCNT_R {
        WDTCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register shows the current counter value of wat dog timer."]
    #[inline(always)]
    #[must_use]
    pub fn wdtcnt(&mut self) -> WDTCNT_W<0> {
        WDTCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watch dog timer clear register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtclr](index.html) module"]
pub struct WDTCLR_SPEC;
impl crate::RegisterSpec for WDTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdtclr::R](R) reader structure"]
impl crate::Readable for WDTCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtclr::W](W) writer structure"]
impl crate::Writable for WDTCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDTCLR to value 0"]
impl crate::Resettable for WDTCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
