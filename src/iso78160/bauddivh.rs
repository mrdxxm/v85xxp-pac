#[doc = "Register `BAUDDIVH` reader"]
pub struct R(crate::R<BAUDDIVH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BAUDDIVH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BAUDDIVH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BAUDDIVH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BAUDDIVH` writer"]
pub struct W(crate::W<BAUDDIVH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BAUDDIVH_SPEC>;
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
impl From<crate::W<BAUDDIVH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BAUDDIVH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BAUDDIVH` reader - High byte of baud_rate divider."]
pub type BAUDDIVH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BAUDDIVH` writer - High byte of baud_rate divider."]
pub type BAUDDIVH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BAUDDIVH_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - High byte of baud_rate divider."]
    #[inline(always)]
    pub fn bauddivh(&self) -> BAUDDIVH_R {
        BAUDDIVH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - High byte of baud_rate divider."]
    #[inline(always)]
    #[must_use]
    pub fn bauddivh(&mut self) -> BAUDDIVH_W<0> {
        BAUDDIVH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISO7816 baud-rate high byte register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bauddivh](index.html) module"]
pub struct BAUDDIVH_SPEC;
impl crate::RegisterSpec for BAUDDIVH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bauddivh::R](R) reader structure"]
impl crate::Readable for BAUDDIVH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bauddivh::W](W) writer structure"]
impl crate::Writable for BAUDDIVH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BAUDDIVH to value 0"]
impl crate::Resettable for BAUDDIVH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
