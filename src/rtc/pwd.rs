#[doc = "Register `PWD` reader"]
pub struct R(crate::R<PWD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWD` writer"]
pub struct W(crate::W<PWD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWD_SPEC>;
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
impl From<crate::W<PWD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWDEN` reader - This register is used to protect the RTC_CE port accessible."]
pub type PWDEN_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - This register is used to protect the RTC_CE port accessible."]
    #[inline(always)]
    pub fn pwden(&self) -> PWDEN_R {
        PWDEN_R::new((self.bits & 1) != 0)
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
#[doc = "RTC password control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwd](index.html) module"]
pub struct PWD_SPEC;
impl crate::RegisterSpec for PWD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwd::R](R) reader structure"]
impl crate::Readable for PWD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwd::W](W) writer structure"]
impl crate::Writable for PWD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWD to value 0"]
impl crate::Resettable for PWD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
