#[doc = "Register `REGC` reader"]
pub struct R(crate::R<REGC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REGC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REGC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REGC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REGC` writer"]
pub struct W(crate::W<REGC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REGC_SPEC>;
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
impl From<crate::W<REGC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REGC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCHTRIM` reader - Trimming of 6.5536MHz RC. 000000~011111: increased by 1.25% for each step; 100000~111111: decreased by 1.25% for each step;"]
pub type RCHTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RCHTRIM` writer - Trimming of 6.5536MHz RC. 000000~011111: increased by 1.25% for each step; 100000~111111: decreased by 1.25% for each step;"]
pub type RCHTRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REGC_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Trimming of 6.5536MHz RC. 000000~011111: increased by 1.25% for each step; 100000~111111: decreased by 1.25% for each step;"]
    #[inline(always)]
    pub fn rchtrim(&self) -> RCHTRIM_R {
        RCHTRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Trimming of 6.5536MHz RC. 000000~011111: increased by 1.25% for each step; 100000~111111: decreased by 1.25% for each step;"]
    #[inline(always)]
    #[must_use]
    pub fn rchtrim(&mut self) -> RCHTRIM_W<0> {
        RCHTRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog register 12.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [regc](index.html) module"]
pub struct REGC_SPEC;
impl crate::RegisterSpec for REGC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [regc::R](R) reader structure"]
impl crate::Readable for REGC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [regc::W](W) writer structure"]
impl crate::Writable for REGC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REGC to value 0"]
impl crate::Resettable for REGC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
