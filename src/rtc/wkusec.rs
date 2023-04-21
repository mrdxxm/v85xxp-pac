#[doc = "Register `WKUSEC` reader"]
pub struct R(crate::R<WKUSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WKUSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WKUSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WKUSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WKUSEC` writer"]
pub struct W(crate::W<WKUSEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WKUSEC_SPEC>;
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
impl From<crate::W<WKUSEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WKUSEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WKUSEC` reader - This register is used to control the multi-second wake-up function."]
pub type WKUSEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WKUSEC` writer - This register is used to control the multi-second wake-up function."]
pub type WKUSEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WKUSEC_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - This register is used to control the multi-second wake-up function."]
    #[inline(always)]
    pub fn wkusec(&self) -> WKUSEC_R {
        WKUSEC_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - This register is used to control the multi-second wake-up function."]
    #[inline(always)]
    #[must_use]
    pub fn wkusec(&mut self) -> WKUSEC_W<0> {
        WKUSEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC wake-up second register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wkusec](index.html) module"]
pub struct WKUSEC_SPEC;
impl crate::RegisterSpec for WKUSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wkusec::R](R) reader structure"]
impl crate::Readable for WKUSEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wkusec::W](W) writer structure"]
impl crate::Writable for WKUSEC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WKUSEC to value 0"]
impl crate::Resettable for WKUSEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
