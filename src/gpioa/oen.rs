#[doc = "Register `OEN` reader"]
pub struct R(crate::R<OEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OEN` writer"]
pub struct W(crate::W<OEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OEN_SPEC>;
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
impl From<crate::W<OEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IOAOEN` reader - Each bit control the IOA output enable signal."]
pub type IOAOEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `IOAOEN` writer - Each bit control the IOA output enable signal."]
pub type IOAOEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OEN_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Each bit control the IOA output enable signal."]
    #[inline(always)]
    pub fn ioaoen(&self) -> IOAOEN_R {
        IOAOEN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Each bit control the IOA output enable signal."]
    #[inline(always)]
    #[must_use]
    pub fn ioaoen(&mut self) -> IOAOEN_W<0> {
        IOAOEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IOA output enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oen](index.html) module"]
pub struct OEN_SPEC;
impl crate::RegisterSpec for OEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oen::R](R) reader structure"]
impl crate::Readable for OEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oen::W](W) writer structure"]
impl crate::Writable for OEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OEN to value 0xffff"]
impl crate::Resettable for OEN_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
