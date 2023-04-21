#[doc = "Register `CSSADDR` reader"]
pub struct R(crate::R<CSSADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSSADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSSADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSSADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSSADDR` writer"]
pub struct W(crate::W<CSSADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSSADDR_SPEC>;
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
impl From<crate::W<CSSADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSSADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSSADDR` reader - Checksum start address register."]
pub type CSSADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CSSADDR` writer - Checksum start address register."]
pub type CSSADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSSADDR_SPEC, u32, u32, 19, O>;
impl R {
    #[doc = "Bits 0:18 - Checksum start address register."]
    #[inline(always)]
    pub fn cssaddr(&self) -> CSSADDR_R {
        CSSADDR_R::new(self.bits & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:18 - Checksum start address register."]
    #[inline(always)]
    #[must_use]
    pub fn cssaddr(&mut self) -> CSSADDR_W<0> {
        CSSADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH Checksum start address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cssaddr](index.html) module"]
pub struct CSSADDR_SPEC;
impl crate::RegisterSpec for CSSADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cssaddr::R](R) reader structure"]
impl crate::Readable for CSSADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cssaddr::W](W) writer structure"]
impl crate::Writable for CSSADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSSADDR to value 0"]
impl crate::Resettable for CSSADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
