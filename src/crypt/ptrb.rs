#[doc = "Register `PTRB` reader"]
pub struct R(crate::R<PTRB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTRB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTRB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTRB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTRB` writer"]
pub struct W(crate::W<PTRB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTRB_SPEC>;
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
impl From<crate::W<PTRB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTRB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PTRB` reader - This is the PTRB register of CRYPT controller."]
pub type PTRB_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PTRB` writer - This is the PTRB register of CRYPT controller."]
pub type PTRB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTRB_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - This is the PTRB register of CRYPT controller."]
    #[inline(always)]
    pub fn ptrb(&self) -> PTRB_R {
        PTRB_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This is the PTRB register of CRYPT controller."]
    #[inline(always)]
    #[must_use]
    pub fn ptrb(&mut self) -> PTRB_W<0> {
        PTRB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRYPT pointer B.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptrb](index.html) module"]
pub struct PTRB_SPEC;
impl crate::RegisterSpec for PTRB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptrb::R](R) reader structure"]
impl crate::Readable for PTRB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptrb::W](W) writer structure"]
impl crate::Writable for PTRB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTRB to value 0"]
impl crate::Resettable for PTRB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
