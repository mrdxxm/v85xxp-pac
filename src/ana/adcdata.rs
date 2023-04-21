#[doc = "Register `ADCDATA[%s]` reader"]
pub struct R(crate::R<ADCDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADCDATA` reader - The result of ADC conversion will be stored in these registers.x=0~15."]
pub type ADCDATA_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - The result of ADC conversion will be stored in these registers.x=0~15."]
    #[inline(always)]
    pub fn adcdata(&self) -> ADCDATA_R {
        ADCDATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "ADC channel x data register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcdata](index.html) module"]
pub struct ADCDATA_SPEC;
impl crate::RegisterSpec for ADCDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcdata::R](R) reader structure"]
impl crate::Readable for ADCDATA_SPEC {
    type Reader = R;
}
