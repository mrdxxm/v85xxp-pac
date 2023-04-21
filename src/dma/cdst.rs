#[doc = "Register `C%sDST` reader"]
pub struct R(crate::R<CDST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C%sDST` writer"]
pub struct W(crate::W<CDST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CDST_SPEC>;
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
impl From<crate::W<CDST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CDST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DST` reader - Destination address."]
pub type DST_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DST` writer - Destination address."]
pub type DST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CDST_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Destination address."]
    #[inline(always)]
    pub fn dst(&self) -> DST_R {
        DST_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination address."]
    #[inline(always)]
    #[must_use]
    pub fn dst(&mut self) -> DST_W<0> {
        DST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA channel x destination register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdst](index.html) module"]
pub struct CDST_SPEC;
impl crate::RegisterSpec for CDST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cdst::R](R) reader structure"]
impl crate::Readable for CDST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cdst::W](W) writer structure"]
impl crate::Writable for CDST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C%sDST to value 0"]
impl crate::Resettable for CDST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
