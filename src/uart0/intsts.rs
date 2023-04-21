#[doc = "Register `INTSTS` reader"]
pub struct R(crate::R<INTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTSTS` writer"]
pub struct W(crate::W<INTSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTSTS_SPEC>;
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
impl From<crate::W<INTSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXIF` reader - Receive interrupt flag."]
pub type RXIF_R = crate::BitReader<bool>;
#[doc = "Field `RXIF` writer - Receive interrupt flag."]
pub type RXIF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `TXOVIF` reader - Transmit buffer overrun flag."]
pub type TXOVIF_R = crate::BitReader<bool>;
#[doc = "Field `TXOVIF` writer - Transmit buffer overrun flag."]
pub type TXOVIF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `RXOVIF` reader - Receive buffer overrun flag."]
pub type RXOVIF_R = crate::BitReader<bool>;
#[doc = "Field `RXOVIF` writer - Receive buffer overrun flag."]
pub type RXOVIF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `RXPEIF` reader - Receive parity error flag."]
pub type RXPEIF_R = crate::BitReader<bool>;
#[doc = "Field `RXPEIF` writer - Receive parity error flag."]
pub type RXPEIF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `TXDONEIF` reader - Transmit done flag."]
pub type TXDONEIF_R = crate::BitReader<bool>;
#[doc = "Field `TXDONEIF` writer - Transmit done flag."]
pub type TXDONEIF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTSTS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Receive interrupt flag."]
    #[inline(always)]
    pub fn rxif(&self) -> RXIF_R {
        RXIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit buffer overrun flag."]
    #[inline(always)]
    pub fn txovif(&self) -> TXOVIF_R {
        TXOVIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive buffer overrun flag."]
    #[inline(always)]
    pub fn rxovif(&self) -> RXOVIF_R {
        RXOVIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive parity error flag."]
    #[inline(always)]
    pub fn rxpeif(&self) -> RXPEIF_R {
        RXPEIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit done flag."]
    #[inline(always)]
    pub fn txdoneif(&self) -> TXDONEIF_R {
        TXDONEIF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Receive interrupt flag."]
    #[inline(always)]
    #[must_use]
    pub fn rxif(&mut self) -> RXIF_W<1> {
        RXIF_W::new(self)
    }
    #[doc = "Bit 2 - Transmit buffer overrun flag."]
    #[inline(always)]
    #[must_use]
    pub fn txovif(&mut self) -> TXOVIF_W<2> {
        TXOVIF_W::new(self)
    }
    #[doc = "Bit 3 - Receive buffer overrun flag."]
    #[inline(always)]
    #[must_use]
    pub fn rxovif(&mut self) -> RXOVIF_W<3> {
        RXOVIF_W::new(self)
    }
    #[doc = "Bit 4 - Receive parity error flag."]
    #[inline(always)]
    #[must_use]
    pub fn rxpeif(&mut self) -> RXPEIF_W<4> {
        RXPEIF_W::new(self)
    }
    #[doc = "Bit 5 - Transmit done flag."]
    #[inline(always)]
    #[must_use]
    pub fn txdoneif(&mut self) -> TXDONEIF_W<5> {
        TXDONEIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART interrupt status register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intsts](index.html) module"]
pub struct INTSTS_SPEC;
impl crate::RegisterSpec for INTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intsts::R](R) reader structure"]
impl crate::Readable for INTSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intsts::W](W) writer structure"]
impl crate::Writable for INTSTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x3e;
}
#[doc = "`reset()` method sets INTSTS to value 0"]
impl crate::Resettable for INTSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
