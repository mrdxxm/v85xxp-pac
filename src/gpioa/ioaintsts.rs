#[doc = "Register `IOAINTSTS` reader"]
pub struct R(crate::R<IOAINTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOAINTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOAINTSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOAINTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOAINTSTS` writer"]
pub struct W(crate::W<IOAINTSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOAINTSTS_SPEC>;
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
impl From<crate::W<IOAINTSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOAINTSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTSTS` reader - Each bit represents the IOA' s interrupt status."]
pub type INTSTS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INTSTS` writer - Each bit represents the IOA' s interrupt status."]
pub type INTSTS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOAINTSTS_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Each bit represents the IOA' s interrupt status."]
    #[inline(always)]
    pub fn intsts(&self) -> INTSTS_R {
        INTSTS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Each bit represents the IOA' s interrupt status."]
    #[inline(always)]
    #[must_use]
    pub fn intsts(&mut self) -> INTSTS_W<0> {
        INTSTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IOA interrupt status register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioaintsts](index.html) module"]
pub struct IOAINTSTS_SPEC;
impl crate::RegisterSpec for IOAINTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ioaintsts::R](R) reader structure"]
impl crate::Readable for IOAINTSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ioaintsts::W](W) writer structure"]
impl crate::Writable for IOAINTSTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xffff;
}
#[doc = "`reset()` method sets IOAINTSTS to value 0"]
impl crate::Resettable for IOAINTSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
