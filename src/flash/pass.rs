#[doc = "Register `PASS` reader"]
pub struct R(crate::R<PASS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PASS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PASS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PASS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PASS` writer"]
pub struct W(crate::W<PASS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PASS_SPEC>;
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
impl From<crate::W<PASS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PASS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UNLOCK` reader - The UNLOCK bit is used to indicate the flash program has been unlocked or not."]
pub type UNLOCK_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - The UNLOCK bit is used to indicate the flash program has been unlocked or not."]
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
#[doc = "FLASH password register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pass](index.html) module"]
pub struct PASS_SPEC;
impl crate::RegisterSpec for PASS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pass::R](R) reader structure"]
impl crate::Readable for PASS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pass::W](W) writer structure"]
impl crate::Writable for PASS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PASS to value 0"]
impl crate::Resettable for PASS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
