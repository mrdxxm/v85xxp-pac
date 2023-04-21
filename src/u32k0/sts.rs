#[doc = "Register `STS` reader"]
pub struct R(crate::R<STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STS` writer"]
pub struct W(crate::W<STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STS_SPEC>;
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
impl From<crate::W<STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXIF` reader - Receive interrupt flag."]
pub type RXIF_R = crate::BitReader<bool>;
#[doc = "Field `RXIF` writer - Receive interrupt flag."]
pub type RXIF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `RXPE` reader - Receive parity error flag."]
pub type RXPE_R = crate::BitReader<bool>;
#[doc = "Field `RXPE` writer - Receive parity error flag."]
pub type RXPE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `RXOV` reader - Receive buffer overrun flag."]
pub type RXOV_R = crate::BitReader<bool>;
#[doc = "Field `RXOV` writer - Receive buffer overrun flag."]
pub type RXOV_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Receive interrupt flag."]
    #[inline(always)]
    pub fn rxif(&self) -> RXIF_R {
        RXIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive parity error flag."]
    #[inline(always)]
    pub fn rxpe(&self) -> RXPE_R {
        RXPE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive buffer overrun flag."]
    #[inline(always)]
    pub fn rxov(&self) -> RXOV_R {
        RXOV_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive interrupt flag."]
    #[inline(always)]
    #[must_use]
    pub fn rxif(&mut self) -> RXIF_W<0> {
        RXIF_W::new(self)
    }
    #[doc = "Bit 1 - Receive parity error flag."]
    #[inline(always)]
    #[must_use]
    pub fn rxpe(&mut self) -> RXPE_W<1> {
        RXPE_W::new(self)
    }
    #[doc = "Bit 2 - Receive buffer overrun flag."]
    #[inline(always)]
    #[must_use]
    pub fn rxov(&mut self) -> RXOV_W<2> {
        RXOV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART 32K x interrupt status register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts](index.html) module"]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sts::R](R) reader structure"]
impl crate::Readable for STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sts::W](W) writer structure"]
impl crate::Writable for STS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x07;
}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for STS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
