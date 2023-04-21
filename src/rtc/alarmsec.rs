#[doc = "Register `ALARMSEC` reader"]
pub struct R(crate::R<ALARMSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALARMSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALARMSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALARMSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALARMSEC` writer"]
pub struct W(crate::W<ALARMSEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALARMSEC_SPEC>;
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
impl From<crate::W<ALARMSEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALARMSEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALARMSEC` reader - This register is used to control the inaccurate second alarm function"]
pub type ALARMSEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALARMSEC` writer - This register is used to control the inaccurate second alarm function"]
pub type ALARMSEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALARMSEC_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - This register is used to control the inaccurate second alarm function"]
    #[inline(always)]
    pub fn alarmsec(&self) -> ALARMSEC_R {
        ALARMSEC_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - This register is used to control the inaccurate second alarm function"]
    #[inline(always)]
    #[must_use]
    pub fn alarmsec(&mut self) -> ALARMSEC_W<0> {
        ALARMSEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC alarm inaccurate second\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alarmsec](index.html) module"]
pub struct ALARMSEC_SPEC;
impl crate::RegisterSpec for ALARMSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alarmsec::R](R) reader structure"]
impl crate::Readable for ALARMSEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alarmsec::W](W) writer structure"]
impl crate::Writable for ALARMSEC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALARMSEC to value 0"]
impl crate::Resettable for ALARMSEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
