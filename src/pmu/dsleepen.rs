#[doc = "Register `DSLEEPEN` reader"]
pub struct R(crate::R<DSLEEPEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSLEEPEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSLEEPEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSLEEPEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSLEEPEN` writer"]
pub struct W(crate::W<DSLEEPEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSLEEPEN_SPEC>;
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
impl From<crate::W<DSLEEPEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSLEEPEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WKU` reader - Current wake-up signal status."]
pub type WKU_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 31 - Current wake-up signal status."]
    #[inline(always)]
    pub fn wku(&self) -> WKU_R {
        WKU_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMU deep sleep enable register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsleepen](index.html) module"]
pub struct DSLEEPEN_SPEC;
impl crate::RegisterSpec for DSLEEPEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsleepen::R](R) reader structure"]
impl crate::Readable for DSLEEPEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsleepen::W](W) writer structure"]
impl crate::Writable for DSLEEPEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSLEEPEN to value 0"]
impl crate::Resettable for DSLEEPEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
