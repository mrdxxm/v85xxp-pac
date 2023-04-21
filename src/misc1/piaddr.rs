#[doc = "Register `PIADDR` reader"]
pub struct R(crate::R<PIADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PIADDR` reader - APB invalid access address."]
pub type PIADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - APB invalid access address."]
    #[inline(always)]
    pub fn piaddr(&self) -> PIADDR_R {
        PIADDR_R::new(self.bits)
    }
}
#[doc = "APB invalid access address.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [piaddr](index.html) module"]
pub struct PIADDR_SPEC;
impl crate::RegisterSpec for PIADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [piaddr::R](R) reader structure"]
impl crate::Readable for PIADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PIADDR to value 0"]
impl crate::Resettable for PIADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
