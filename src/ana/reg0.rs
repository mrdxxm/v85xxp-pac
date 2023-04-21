#[doc = "Register `REG0` reader"]
pub struct R(crate::R<REG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG0` writer"]
pub struct W(crate::W<REG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG0_SPEC>;
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
impl From<crate::W<REG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG0_SPEC>) -> Self {
        W(writer)
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
#[doc = "Analog register 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg0](index.html) module"]
pub struct REG0_SPEC;
impl crate::RegisterSpec for REG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg0::R](R) reader structure"]
impl crate::Readable for REG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg0::W](W) writer structure"]
impl crate::Writable for REG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REG0 to value 0"]
impl crate::Resettable for REG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
