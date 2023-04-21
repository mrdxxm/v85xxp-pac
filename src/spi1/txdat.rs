#[doc = "Register `TXDAT` reader"]
pub struct R(crate::R<TXDAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXDAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXDAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXDAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXDAT` writer"]
pub struct W(crate::W<TXDAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXDAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TXDAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXDAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXD` reader - Write data to SPI Transmit FIFO."]
pub type TXD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXD` writer - Write data to SPI Transmit FIFO."]
pub type TXD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXDAT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Write data to SPI Transmit FIFO."]
    #[inline(always)]
    pub fn txd(&self) -> TXD_R {
        TXD_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Write data to SPI Transmit FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn txd(&mut self) -> TXD_W<0> {
        TXD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Transmit FIFO register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdat](index.html) module"]
pub struct TXDAT_SPEC;
impl crate::RegisterSpec for TXDAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txdat::R](R) reader structure"]
impl crate::Readable for TXDAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txdat::W](W) writer structure"]
impl crate::Writable for TXDAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXDAT to value 0"]
impl crate::Resettable for TXDAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
