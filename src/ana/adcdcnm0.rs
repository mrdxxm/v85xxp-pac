#[doc = "Register `ADCDCNM0` reader"]
pub struct R(crate::R<ADCDCNM0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCDCNM0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCDCNM0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCDCNM0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NM0` reader - ADC NM0."]
pub type NM0_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:8 - ADC NM0."]
    #[inline(always)]
    pub fn nm0(&self) -> NM0_R {
        NM0_R::new((self.bits & 0x01ff) as u16)
    }
}
#[doc = "ANA_ADCDCNM0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcdcnm0](index.html) module"]
pub struct ADCDCNM0_SPEC;
impl crate::RegisterSpec for ADCDCNM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcdcnm0::R](R) reader structure"]
impl crate::Readable for ADCDCNM0_SPEC {
    type Reader = R;
}
