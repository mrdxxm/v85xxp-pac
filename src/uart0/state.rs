#[doc = "Register `STATE` reader"]
pub struct R(crate::R<STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATE` writer"]
pub struct W(crate::W<STATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATE_SPEC>;
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
impl From<crate::W<STATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXFULL` reader - Receive buffer full register."]
pub type RXFULL_R = crate::BitReader<bool>;
#[doc = "Field `RXFULL` writer - Receive buffer full register."]
pub type RXFULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATE_SPEC, bool, O>;
#[doc = "Field `TXOV` reader - Transmit buffer overrun flag."]
pub type TXOV_R = crate::BitReader<bool>;
#[doc = "Field `TXOV` writer - Transmit buffer overrun flag."]
pub type TXOV_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STATE_SPEC, bool, O>;
#[doc = "Field `RXOV` reader - Receive buffer overrun flag."]
pub type RXOV_R = crate::BitReader<bool>;
#[doc = "Field `RXOV` writer - Receive buffer overrun flag."]
pub type RXOV_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STATE_SPEC, bool, O>;
#[doc = "Field `RXPE` reader - Receive parity error flag."]
pub type RXPE_R = crate::BitReader<bool>;
#[doc = "Field `RXPE` writer - Receive parity error flag."]
pub type RXPE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STATE_SPEC, bool, O>;
#[doc = "Field `TXDONE` reader - Transmit done flag."]
pub type TXDONE_R = crate::BitReader<bool>;
#[doc = "Field `TXDONE` writer - Transmit done flag."]
pub type TXDONE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STATE_SPEC, bool, O>;
#[doc = "Field `RXPSTS` reader - Receive parity data flag."]
pub type RXPSTS_R = crate::BitReader<bool>;
#[doc = "Field `RXPSTS` writer - Receive parity data flag."]
pub type RXPSTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATE_SPEC, bool, O>;
#[doc = "Field `DMATXDONE` reader - DMA Transmit finish flag."]
pub type DMATXDONE_R = crate::BitReader<bool>;
#[doc = "Field `DMATXDONE` writer - DMA Transmit finish flag."]
pub type DMATXDONE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STATE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Receive buffer full register."]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit buffer overrun flag."]
    #[inline(always)]
    pub fn txov(&self) -> TXOV_R {
        TXOV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive buffer overrun flag."]
    #[inline(always)]
    pub fn rxov(&self) -> RXOV_R {
        RXOV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive parity error flag."]
    #[inline(always)]
    pub fn rxpe(&self) -> RXPE_R {
        RXPE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit done flag."]
    #[inline(always)]
    pub fn txdone(&self) -> TXDONE_R {
        TXDONE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive parity data flag."]
    #[inline(always)]
    pub fn rxpsts(&self) -> RXPSTS_R {
        RXPSTS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA Transmit finish flag."]
    #[inline(always)]
    pub fn dmatxdone(&self) -> DMATXDONE_R {
        DMATXDONE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Receive buffer full register."]
    #[inline(always)]
    #[must_use]
    pub fn rxfull(&mut self) -> RXFULL_W<1> {
        RXFULL_W::new(self)
    }
    #[doc = "Bit 2 - Transmit buffer overrun flag."]
    #[inline(always)]
    #[must_use]
    pub fn txov(&mut self) -> TXOV_W<2> {
        TXOV_W::new(self)
    }
    #[doc = "Bit 3 - Receive buffer overrun flag."]
    #[inline(always)]
    #[must_use]
    pub fn rxov(&mut self) -> RXOV_W<3> {
        RXOV_W::new(self)
    }
    #[doc = "Bit 4 - Receive parity error flag."]
    #[inline(always)]
    #[must_use]
    pub fn rxpe(&mut self) -> RXPE_W<4> {
        RXPE_W::new(self)
    }
    #[doc = "Bit 5 - Transmit done flag."]
    #[inline(always)]
    #[must_use]
    pub fn txdone(&mut self) -> TXDONE_W<5> {
        TXDONE_W::new(self)
    }
    #[doc = "Bit 6 - Receive parity data flag."]
    #[inline(always)]
    #[must_use]
    pub fn rxpsts(&mut self) -> RXPSTS_W<6> {
        RXPSTS_W::new(self)
    }
    #[doc = "Bit 7 - DMA Transmit finish flag."]
    #[inline(always)]
    #[must_use]
    pub fn dmatxdone(&mut self) -> DMATXDONE_W<7> {
        DMATXDONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART status register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [state](index.html) module"]
pub struct STATE_SPEC;
impl crate::RegisterSpec for STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [state::R](R) reader structure"]
impl crate::Readable for STATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [state::W](W) writer structure"]
impl crate::Writable for STATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xbc;
}
#[doc = "`reset()` method sets STATE to value 0"]
impl crate::Resettable for STATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
