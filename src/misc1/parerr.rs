#[doc = "Register `PARERR` reader"]
pub struct R(crate::R<PARERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PARERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PARERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PARERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PEADDR` reader - Parity error address."]
pub type PEADDR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:13 - Parity error address."]
    #[inline(always)]
    pub fn peaddr(&self) -> PEADDR_R {
        PEADDR_R::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "SRAM Parity Error address register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [parerr](index.html) module"]
pub struct PARERR_SPEC;
impl crate::RegisterSpec for PARERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [parerr::R](R) reader structure"]
impl crate::Readable for PARERR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PARERR to value 0"]
impl crate::Resettable for PARERR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
