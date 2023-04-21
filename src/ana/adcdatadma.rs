#[doc = "Register `ADCDATADMA` reader"]
pub struct R(crate::R<ADCDATADMA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCDATADMA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCDATADMA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCDATADMA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADCDATA_DMA` reader - ADC data source for DMA."]
pub type ADCDATA_DMA_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - ADC data source for DMA."]
    #[inline(always)]
    pub fn adcdata_dma(&self) -> ADCDATA_DMA_R {
        ADCDATA_DMA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "ANA_ADCDATADMA.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcdatadma](index.html) module"]
pub struct ADCDATADMA_SPEC;
impl crate::RegisterSpec for ADCDATADMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcdatadma::R](R) reader structure"]
impl crate::Readable for ADCDATADMA_SPEC {
    type Reader = R;
}
