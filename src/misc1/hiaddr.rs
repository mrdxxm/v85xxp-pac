#[doc = "Register `HIADDR` reader"]
pub struct R(crate::R<HIADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HIADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HIADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HIADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HIADDR` reader - AHB invalid access address."]
pub type HIADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - AHB invalid access address."]
    #[inline(always)]
    pub fn hiaddr(&self) -> HIADDR_R {
        HIADDR_R::new(self.bits)
    }
}
#[doc = "AHB invalid access address.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hiaddr](index.html) module"]
pub struct HIADDR_SPEC;
impl crate::RegisterSpec for HIADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hiaddr::R](R) reader structure"]
impl crate::Readable for HIADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HIADDR to value 0"]
impl crate::Resettable for HIADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
