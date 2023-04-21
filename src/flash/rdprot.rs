#[doc = "Register `RDPROT` reader"]
pub struct R(crate::R<RDPROT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDPROT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDPROT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDPROT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDPORT` reader - This register is used to indicate if the specific region has been protected from read operation during ICE mode (MODE is 0)."]
pub type RDPORT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register is used to indicate if the specific region has been protected from read operation during ICE mode (MODE is 0)."]
    #[inline(always)]
    pub fn rdport(&self) -> RDPORT_R {
        RDPORT_R::new(self.bits)
    }
}
#[doc = "Flash read protect status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdprot](index.html) module"]
pub struct RDPROT_SPEC;
impl crate::RegisterSpec for RDPROT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rdprot::R](R) reader structure"]
impl crate::Readable for RDPROT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RDPROT to value 0"]
impl crate::Resettable for RDPROT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
