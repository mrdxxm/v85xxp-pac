#[doc = "Register `WKUCNTR` reader"]
pub struct R(crate::R<WKUCNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WKUCNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WKUCNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WKUCNTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WKUCNTR` reader - This register is used to represent the current WKUCNT value."]
pub type WKUCNTR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - This register is used to represent the current WKUCNT value."]
    #[inline(always)]
    pub fn wkucntr(&self) -> WKUCNTR_R {
        WKUCNTR_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "This register is used to represent the current WKUCNT value.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wkucntr](index.html) module"]
pub struct WKUCNTR_SPEC;
impl crate::RegisterSpec for WKUCNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wkucntr::R](R) reader structure"]
impl crate::Readable for WKUCNTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WKUCNTR to value 0"]
impl crate::Resettable for WKUCNTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
