#[doc = "Register `ADCDOS` reader"]
pub struct R(crate::R<ADCDOS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCDOS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCDOS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCDOS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DOS` reader - ADC DOS."]
pub type DOS_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:8 - ADC DOS."]
    #[inline(always)]
    pub fn dos(&self) -> DOS_R {
        DOS_R::new((self.bits & 0x01ff) as u16)
    }
}
#[doc = "ANA_ADCDOS.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcdos](index.html) module"]
pub struct ADCDOS_SPEC;
impl crate::RegisterSpec for ADCDOS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcdos::R](R) reader structure"]
impl crate::Readable for ADCDOS_SPEC {
    type Reader = R;
}
