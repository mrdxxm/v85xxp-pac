#[doc = "Register `RXDAT` reader"]
pub struct R(crate::R<RXDAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXD` reader - Read data from SPI Receive FIFO."]
pub type RXD_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Read data from SPI Receive FIFO."]
    #[inline(always)]
    pub fn rxd(&self) -> RXD_R {
        RXD_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "SPI Receive FIFO Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdat](index.html) module"]
pub struct RXDAT_SPEC;
impl crate::RegisterSpec for RXDAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxdat::R](R) reader structure"]
impl crate::Readable for RXDAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXDAT to value 0"]
impl crate::Resettable for RXDAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
