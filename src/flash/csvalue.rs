#[doc = "Register `CSVALUE` reader"]
pub struct R(crate::R<CSVALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSVALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSVALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSVALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CSVALUE` reader - Checksum latched value register."]
pub type CSVALUE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Checksum latched value register."]
    #[inline(always)]
    pub fn csvalue(&self) -> CSVALUE_R {
        CSVALUE_R::new(self.bits)
    }
}
#[doc = "FLASH Checksum value register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csvalue](index.html) module"]
pub struct CSVALUE_SPEC;
impl crate::RegisterSpec for CSVALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csvalue::R](R) reader structure"]
impl crate::Readable for CSVALUE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CSVALUE to value 0"]
impl crate::Resettable for CSVALUE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
