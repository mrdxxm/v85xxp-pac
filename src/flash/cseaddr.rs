#[doc = "Register `CSEADDR` reader"]
pub struct R(crate::R<CSEADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSEADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSEADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSEADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSEADDR` writer"]
pub struct W(crate::W<CSEADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSEADDR_SPEC>;
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
impl From<crate::W<CSEADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSEADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSEADDR` reader - Checksum end address register."]
pub type CSEADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CSEADDR` writer - Checksum end address register."]
pub type CSEADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSEADDR_SPEC, u32, u32, 19, O>;
impl R {
    #[doc = "Bits 0:18 - Checksum end address register."]
    #[inline(always)]
    pub fn cseaddr(&self) -> CSEADDR_R {
        CSEADDR_R::new(self.bits & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:18 - Checksum end address register."]
    #[inline(always)]
    #[must_use]
    pub fn cseaddr(&mut self) -> CSEADDR_W<0> {
        CSEADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH Checksum end address.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cseaddr](index.html) module"]
pub struct CSEADDR_SPEC;
impl crate::RegisterSpec for CSEADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cseaddr::R](R) reader structure"]
impl crate::Readable for CSEADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cseaddr::W](W) writer structure"]
impl crate::Writable for CSEADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSEADDR to value 0x0007_fffc"]
impl crate::Resettable for CSEADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0007_fffc;
}
