#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - ISO7816 enable register."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - ISO7816 enable register."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `CHKP` reader - Parity mode control register."]
pub type CHKP_R = crate::BitReader<bool>;
#[doc = "Field `CHKP` writer - Parity mode control register."]
pub type CHKP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `RXERRIE` reader - Received data error interrupt enable register."]
pub type RXERRIE_R = crate::BitReader<bool>;
#[doc = "Field `RXERRIE` writer - Received data error interrupt enable register."]
pub type RXERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `ACKLEN` reader - ACK low period when receive an error data."]
pub type ACKLEN_R = crate::BitReader<bool>;
#[doc = "Field `ACKLEN` writer - ACK low period when receive an error data."]
pub type ACKLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `RXIE` reader - Receive interrupt enable register."]
pub type RXIE_R = crate::BitReader<bool>;
#[doc = "Field `RXIE` writer - Receive interrupt enable register."]
pub type RXIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `TXDONEIE` reader - Transmit interrupt enable register."]
pub type TXDONEIE_R = crate::BitReader<bool>;
#[doc = "Field `TXDONEIE` writer - Transmit interrupt enable register."]
pub type TXDONEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `RXOVIE` reader - Receive overrun interrupt enable register."]
pub type RXOVIE_R = crate::BitReader<bool>;
#[doc = "Field `RXOVIE` writer - Receive overrun interrupt enable register."]
pub type RXOVIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `TXRTYERRIE` reader - Transmit retry error interrupt enable register"]
pub type TXRTYERRIE_R = crate::BitReader<bool>;
#[doc = "Field `TXRTYERRIE` writer - Transmit retry error interrupt enable register"]
pub type TXRTYERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `AUTORXACK` reader - Automatic response ACK as 0 when receive an error data to let transmitter re-send the data."]
pub type AUTORXACK_R = crate::BitReader<bool>;
#[doc = "Field `AUTORXACK` writer - Automatic response ACK as 0 when receive an error data to let transmitter re-send the data."]
pub type AUTORXACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `LSB` reader - MSB/LSB transmit order control register."]
pub type LSB_R = crate::BitReader<bool>;
#[doc = "Field `LSB` writer - MSB/LSB transmit order control register."]
pub type LSB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `TXRTYCNT` reader - Transmit retry number control register."]
pub type TXRTYCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXRTYCNT` writer - Transmit retry number control register."]
pub type TXRTYCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `RXACKSET` reader - RXACK length."]
pub type RXACKSET_R = crate::BitReader<bool>;
#[doc = "Field `RXACKSET` writer - RXACK length."]
pub type RXACKSET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ISO7816 enable register."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity mode control register."]
    #[inline(always)]
    pub fn chkp(&self) -> CHKP_R {
        CHKP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Received data error interrupt enable register."]
    #[inline(always)]
    pub fn rxerrie(&self) -> RXERRIE_R {
        RXERRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - ACK low period when receive an error data."]
    #[inline(always)]
    pub fn acklen(&self) -> ACKLEN_R {
        ACKLEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive interrupt enable register."]
    #[inline(always)]
    pub fn rxie(&self) -> RXIE_R {
        RXIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit interrupt enable register."]
    #[inline(always)]
    pub fn txdoneie(&self) -> TXDONEIE_R {
        TXDONEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive overrun interrupt enable register."]
    #[inline(always)]
    pub fn rxovie(&self) -> RXOVIE_R {
        RXOVIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit retry error interrupt enable register"]
    #[inline(always)]
    pub fn txrtyerrie(&self) -> TXRTYERRIE_R {
        TXRTYERRIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Automatic response ACK as 0 when receive an error data to let transmitter re-send the data."]
    #[inline(always)]
    pub fn autorxack(&self) -> AUTORXACK_R {
        AUTORXACK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - MSB/LSB transmit order control register."]
    #[inline(always)]
    pub fn lsb(&self) -> LSB_R {
        LSB_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Transmit retry number control register."]
    #[inline(always)]
    pub fn txrtycnt(&self) -> TXRTYCNT_R {
        TXRTYCNT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - RXACK length."]
    #[inline(always)]
    pub fn rxackset(&self) -> RXACKSET_R {
        RXACKSET_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ISO7816 enable register."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Parity mode control register."]
    #[inline(always)]
    #[must_use]
    pub fn chkp(&mut self) -> CHKP_W<1> {
        CHKP_W::new(self)
    }
    #[doc = "Bit 2 - Received data error interrupt enable register."]
    #[inline(always)]
    #[must_use]
    pub fn rxerrie(&mut self) -> RXERRIE_W<2> {
        RXERRIE_W::new(self)
    }
    #[doc = "Bit 4 - ACK low period when receive an error data."]
    #[inline(always)]
    #[must_use]
    pub fn acklen(&mut self) -> ACKLEN_W<4> {
        ACKLEN_W::new(self)
    }
    #[doc = "Bit 5 - Receive interrupt enable register."]
    #[inline(always)]
    #[must_use]
    pub fn rxie(&mut self) -> RXIE_W<5> {
        RXIE_W::new(self)
    }
    #[doc = "Bit 6 - Transmit interrupt enable register."]
    #[inline(always)]
    #[must_use]
    pub fn txdoneie(&mut self) -> TXDONEIE_W<6> {
        TXDONEIE_W::new(self)
    }
    #[doc = "Bit 7 - Receive overrun interrupt enable register."]
    #[inline(always)]
    #[must_use]
    pub fn rxovie(&mut self) -> RXOVIE_W<7> {
        RXOVIE_W::new(self)
    }
    #[doc = "Bit 8 - Transmit retry error interrupt enable register"]
    #[inline(always)]
    #[must_use]
    pub fn txrtyerrie(&mut self) -> TXRTYERRIE_W<8> {
        TXRTYERRIE_W::new(self)
    }
    #[doc = "Bit 9 - Automatic response ACK as 0 when receive an error data to let transmitter re-send the data."]
    #[inline(always)]
    #[must_use]
    pub fn autorxack(&mut self) -> AUTORXACK_W<9> {
        AUTORXACK_W::new(self)
    }
    #[doc = "Bit 11 - MSB/LSB transmit order control register."]
    #[inline(always)]
    #[must_use]
    pub fn lsb(&mut self) -> LSB_W<11> {
        LSB_W::new(self)
    }
    #[doc = "Bits 12:15 - Transmit retry number control register."]
    #[inline(always)]
    #[must_use]
    pub fn txrtycnt(&mut self) -> TXRTYCNT_W<12> {
        TXRTYCNT_W::new(self)
    }
    #[doc = "Bit 16 - RXACK length."]
    #[inline(always)]
    #[must_use]
    pub fn rxackset(&mut self) -> RXACKSET_W<16> {
        RXACKSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISO7816 control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
