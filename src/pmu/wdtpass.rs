#[doc = "Register `WDTPASS` reader"]
pub struct R(crate::R<WDTPASS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTPASS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTPASS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTPASS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTPASS` writer"]
pub struct W(crate::W<WDTPASS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTPASS_SPEC>;
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
impl From<crate::W<WDTPASS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTPASS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UNLOCK` reader - This bit indicates the watch dog timer enable register has been unlocked and is ready to change the watch dog enable control register."]
pub type UNLOCK_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - This bit indicates the watch dog timer enable register has been unlocked and is ready to change the watch dog enable control register."]
    #[inline(always)]
    pub fn unlock(&self) -> UNLOCK_R {
        UNLOCK_R::new((self.bits & 1) != 0)
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
#[doc = "Watch dog timing unlock register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtpass](index.html) module"]
pub struct WDTPASS_SPEC;
impl crate::RegisterSpec for WDTPASS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdtpass::R](R) reader structure"]
impl crate::Readable for WDTPASS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtpass::W](W) writer structure"]
impl crate::Writable for WDTPASS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDTPASS to value 0"]
impl crate::Resettable for WDTPASS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
