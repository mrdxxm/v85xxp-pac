#[doc = "Register `ATT` reader"]
pub struct R(crate::R<ATT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ATT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ATT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ATT` writer"]
pub struct W(crate::W<ATT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ATT_SPEC>;
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
impl From<crate::W<ATT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ATT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IOXATT` reader - Each bit control the IOX attribute."]
pub type IOXATT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `IOXATT` writer - Each bit control the IOX attribute."]
pub type IOXATT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Each bit control the IOX attribute."]
    #[inline(always)]
    pub fn ioxatt(&self) -> IOXATT_R {
        IOXATT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Each bit control the IOX attribute."]
    #[inline(always)]
    #[must_use]
    pub fn ioxatt(&mut self) -> IOXATT_W<0> {
        IOXATT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IO attribute register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [att](index.html) module"]
pub struct ATT_SPEC;
impl crate::RegisterSpec for ATT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [att::R](R) reader structure"]
impl crate::Readable for ATT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [att::W](W) writer structure"]
impl crate::Writable for ATT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ATT to value 0"]
impl crate::Resettable for ATT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
