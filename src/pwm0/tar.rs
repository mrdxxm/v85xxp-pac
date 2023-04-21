#[doc = "Register `TAR` reader"]
pub struct R(crate::R<TAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TAR` reader - Current count register of PWM Timer x."]
pub type TAR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Current count register of PWM Timer x."]
    #[inline(always)]
    pub fn tar(&self) -> TAR_R {
        TAR_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Current count register of PWM Timer x.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tar](index.html) module"]
pub struct TAR_SPEC;
impl crate::RegisterSpec for TAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tar::R](R) reader structure"]
impl crate::Readable for TAR_SPEC {
    type Reader = R;
}
