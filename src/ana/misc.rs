#[doc = "Register `MISC` reader"]
pub struct R(crate::R<MISC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISC` writer"]
pub struct W(crate::W<MISC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISC_SPEC>;
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
impl From<crate::W<MISC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TADCTH` reader - TADC threshold setting."]
pub type TADCTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TADCTH` writer - TADC threshold setting."]
pub type TADCTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MISC_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 4:5 - TADC threshold setting."]
    #[inline(always)]
    pub fn tadcth(&self) -> TADCTH_R {
        TADCTH_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - TADC threshold setting."]
    #[inline(always)]
    #[must_use]
    pub fn tadcth(&mut self) -> TADCTH_W<4> {
        TADCTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog MISC control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc](index.html) module"]
pub struct MISC_SPEC;
impl crate::RegisterSpec for MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misc::R](R) reader structure"]
impl crate::Readable for MISC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [misc::W](W) writer structure"]
impl crate::Writable for MISC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MISC to value 0"]
impl crate::Resettable for MISC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
