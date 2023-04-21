#[doc = "Register `INFO` reader"]
pub struct R(crate::R<INFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INFO` writer"]
pub struct W(crate::W<INFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INFO_SPEC>;
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
impl From<crate::W<INFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXACK` reader - The received ACK at the end of transmit. This bit can be read/write by CPU, by the original message will be lost."]
pub type RXACK_R = crate::BitReader<bool>;
#[doc = "Field `RXACK` writer - The received ACK at the end of transmit. This bit can be read/write by CPU, by the original message will be lost."]
pub type RXACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INFO_SPEC, bool, O>;
#[doc = "Field `CHKSUM` reader - The transmitted or received data check sum bit. This bit can be read/write by CPU, by the original message will be lost."]
pub type CHKSUM_R = crate::BitReader<bool>;
#[doc = "Field `CHKSUM` writer - The transmitted or received data check sum bit. This bit can be read/write by CPU, by the original message will be lost."]
pub type CHKSUM_W<'a, const O: u8> = crate::BitWriter<'a, u32, INFO_SPEC, bool, O>;
#[doc = "Field `RXERRIF` reader - When received data have check sum error, this bit will be set to 1."]
pub type RXERRIF_R = crate::BitReader<bool>;
#[doc = "Field `RXERRIF` writer - When received data have check sum error, this bit will be set to 1."]
pub type RXERRIF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INFO_SPEC, bool, O>;
#[doc = "Field `RXIF` reader - Receive interrupt flag."]
pub type RXIF_R = crate::BitReader<bool>;
#[doc = "Field `RXIF` writer - Receive interrupt flag."]
pub type RXIF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INFO_SPEC, bool, O>;
#[doc = "Field `TXDONEIF` reader - Transmit interrupt flag."]
pub type TXDONEIF_R = crate::BitReader<bool>;
#[doc = "Field `TXDONEIF` writer - Transmit interrupt flag."]
pub type TXDONEIF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INFO_SPEC, bool, O>;
#[doc = "Field `RXOVIF` reader - Receive overflow flag."]
pub type RXOVIF_R = crate::BitReader<bool>;
#[doc = "Field `RXOVIF` writer - Receive overflow flag."]
pub type RXOVIF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INFO_SPEC, bool, O>;
#[doc = "Field `TXRTYERRIF` reader - Transmit Retry counter reach flag."]
pub type TXRTYERRIF_R = crate::BitReader<bool>;
#[doc = "Field `TXRTYERRIF` writer - Transmit Retry counter reach flag."]
pub type TXRTYERRIF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INFO_SPEC, bool, O>;
#[doc = "Field `DMATXDONE` reader - Transmit finish flag."]
pub type DMATXDONE_R = crate::BitReader<bool>;
#[doc = "Field `DMATXDONE` writer - Transmit finish flag."]
pub type DMATXDONE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INFO_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - The received ACK at the end of transmit. This bit can be read/write by CPU, by the original message will be lost."]
    #[inline(always)]
    pub fn rxack(&self) -> RXACK_R {
        RXACK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The transmitted or received data check sum bit. This bit can be read/write by CPU, by the original message will be lost."]
    #[inline(always)]
    pub fn chksum(&self) -> CHKSUM_R {
        CHKSUM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When received data have check sum error, this bit will be set to 1."]
    #[inline(always)]
    pub fn rxerrif(&self) -> RXERRIF_R {
        RXERRIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive interrupt flag."]
    #[inline(always)]
    pub fn rxif(&self) -> RXIF_R {
        RXIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit interrupt flag."]
    #[inline(always)]
    pub fn txdoneif(&self) -> TXDONEIF_R {
        TXDONEIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive overflow flag."]
    #[inline(always)]
    pub fn rxovif(&self) -> RXOVIF_R {
        RXOVIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit Retry counter reach flag."]
    #[inline(always)]
    pub fn txrtyerrif(&self) -> TXRTYERRIF_R {
        TXRTYERRIF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmit finish flag."]
    #[inline(always)]
    pub fn dmatxdone(&self) -> DMATXDONE_R {
        DMATXDONE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The received ACK at the end of transmit. This bit can be read/write by CPU, by the original message will be lost."]
    #[inline(always)]
    #[must_use]
    pub fn rxack(&mut self) -> RXACK_W<0> {
        RXACK_W::new(self)
    }
    #[doc = "Bit 1 - The transmitted or received data check sum bit. This bit can be read/write by CPU, by the original message will be lost."]
    #[inline(always)]
    #[must_use]
    pub fn chksum(&mut self) -> CHKSUM_W<1> {
        CHKSUM_W::new(self)
    }
    #[doc = "Bit 2 - When received data have check sum error, this bit will be set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn rxerrif(&mut self) -> RXERRIF_W<2> {
        RXERRIF_W::new(self)
    }
    #[doc = "Bit 5 - Receive interrupt flag."]
    #[inline(always)]
    #[must_use]
    pub fn rxif(&mut self) -> RXIF_W<5> {
        RXIF_W::new(self)
    }
    #[doc = "Bit 6 - Transmit interrupt flag."]
    #[inline(always)]
    #[must_use]
    pub fn txdoneif(&mut self) -> TXDONEIF_W<6> {
        TXDONEIF_W::new(self)
    }
    #[doc = "Bit 7 - Receive overflow flag."]
    #[inline(always)]
    #[must_use]
    pub fn rxovif(&mut self) -> RXOVIF_W<7> {
        RXOVIF_W::new(self)
    }
    #[doc = "Bit 8 - Transmit Retry counter reach flag."]
    #[inline(always)]
    #[must_use]
    pub fn txrtyerrif(&mut self) -> TXRTYERRIF_W<8> {
        TXRTYERRIF_W::new(self)
    }
    #[doc = "Bit 9 - Transmit finish flag."]
    #[inline(always)]
    #[must_use]
    pub fn dmatxdone(&mut self) -> DMATXDONE_W<9> {
        DMATXDONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISO7816 information register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [info](index.html) module"]
pub struct INFO_SPEC;
impl crate::RegisterSpec for INFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [info::R](R) reader structure"]
impl crate::Readable for INFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [info::W](W) writer structure"]
impl crate::Writable for INFO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x03e4;
}
#[doc = "`reset()` method sets INFO to value 0"]
impl crate::Resettable for INFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
