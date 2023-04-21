#[doc = "Register `C%sLEN` reader"]
pub struct R(crate::R<CLEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C%sLEN` writer"]
pub struct W(crate::W<CLEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLEN_SPEC>;
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
impl From<crate::W<CLEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPLEN` reader - Current package transferred length."]
pub type CPLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPLEN` writer - Current package transferred length."]
pub type CPLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLEN_SPEC, u8, u8, 8, O>;
#[doc = "Field `CFLEN` reader - Current frame transferred length."]
pub type CFLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFLEN` writer - Current frame transferred length."]
pub type CFLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLEN_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Current package transferred length."]
    #[inline(always)]
    pub fn cplen(&self) -> CPLEN_R {
        CPLEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Current frame transferred length."]
    #[inline(always)]
    pub fn cflen(&self) -> CFLEN_R {
        CFLEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Current package transferred length."]
    #[inline(always)]
    #[must_use]
    pub fn cplen(&mut self) -> CPLEN_W<0> {
        CPLEN_W::new(self)
    }
    #[doc = "Bits 8:15 - Current frame transferred length."]
    #[inline(always)]
    #[must_use]
    pub fn cflen(&mut self) -> CFLEN_W<8> {
        CFLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA channel x transfer length register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clen](index.html) module"]
pub struct CLEN_SPEC;
impl crate::RegisterSpec for CLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clen::R](R) reader structure"]
impl crate::Readable for CLEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clen::W](W) writer structure"]
impl crate::Writable for CLEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C%sLEN to value 0"]
impl crate::Resettable for CLEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
