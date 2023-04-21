#[doc = "Register `ACP3` reader"]
pub struct R(crate::R<ACP3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACP3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACP3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACP3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `P3` reader - The P3 register is used for frquency adjustment."]
pub type P3_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The P3 register is used for frquency adjustment."]
    #[inline(always)]
    pub fn p3(&self) -> P3_R {
        P3_R::new(self.bits)
    }
}
#[doc = "RTC parameter P3 register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acp3](index.html) module"]
pub struct ACP3_SPEC;
impl crate::RegisterSpec for ACP3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acp3::R](R) reader structure"]
impl crate::Readable for ACP3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ACP3 to value 0"]
impl crate::Resettable for ACP3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
