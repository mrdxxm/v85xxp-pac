#[doc = "Register `SEGCTRL0` reader"]
pub struct R(crate::R<SEGCTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEGCTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEGCTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEGCTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEGCTRL0` writer"]
pub struct W(crate::W<SEGCTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEGCTRL0_SPEC>;
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
impl From<crate::W<SEGCTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEGCTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEGCTRL` reader - Each bit control the SEG0~SEG31 LCD signal enable."]
pub type SEGCTRL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SEGCTRL` writer - Each bit control the SEG0~SEG31 LCD signal enable."]
pub type SEGCTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEGCTRL0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Each bit control the SEG0~SEG31 LCD signal enable."]
    #[inline(always)]
    pub fn segctrl(&self) -> SEGCTRL_R {
        SEGCTRL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Each bit control the SEG0~SEG31 LCD signal enable."]
    #[inline(always)]
    #[must_use]
    pub fn segctrl(&mut self) -> SEGCTRL_W<0> {
        SEGCTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD segment enable control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [segctrl0](index.html) module"]
pub struct SEGCTRL0_SPEC;
impl crate::RegisterSpec for SEGCTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [segctrl0::R](R) reader structure"]
impl crate::Readable for SEGCTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [segctrl0::W](W) writer structure"]
impl crate::Writable for SEGCTRL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEGCTRL0 to value 0"]
impl crate::Resettable for SEGCTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
