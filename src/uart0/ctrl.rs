#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXEN` reader - Transmit engine enable register."]
pub type TXEN_R = crate::BitReader<bool>;
#[doc = "Field `TXEN` writer - Transmit engine enable register."]
pub type TXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `RXEN` reader - Receive engine enable register."]
pub type RXEN_R = crate::BitReader<bool>;
#[doc = "Field `RXEN` writer - Receive engine enable register."]
pub type RXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `RXIE` reader - Receive interrupt enable register."]
pub type RXIE_R = crate::BitReader<bool>;
#[doc = "Field `RXIE` writer - Receive interrupt enable register."]
pub type RXIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `TXOVIE` reader - Transmit overrun interrupt enable register."]
pub type TXOVIE_R = crate::BitReader<bool>;
#[doc = "Field `TXOVIE` writer - Transmit overrun interrupt enable register."]
pub type TXOVIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `RXOVIE` reader - Receive overrun interrupt enable register."]
pub type RXOVIE_R = crate::BitReader<bool>;
#[doc = "Field `RXOVIE` writer - Receive overrun interrupt enable register."]
pub type RXOVIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `RXPEIE` reader - Receive parity error interrupt enable register."]
pub type RXPEIE_R = crate::BitReader<bool>;
#[doc = "Field `RXPEIE` writer - Receive parity error interrupt enable register."]
pub type RXPEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `TXDONEIE` reader - Transmit done interrupt enable register."]
pub type TXDONEIE_R = crate::BitReader<bool>;
#[doc = "Field `TXDONEIE` writer - Transmit done interrupt enable register."]
pub type TXDONEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transmit engine enable register."]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive engine enable register."]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive interrupt enable register."]
    #[inline(always)]
    pub fn rxie(&self) -> RXIE_R {
        RXIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit overrun interrupt enable register."]
    #[inline(always)]
    pub fn txovie(&self) -> TXOVIE_R {
        TXOVIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive overrun interrupt enable register."]
    #[inline(always)]
    pub fn rxovie(&self) -> RXOVIE_R {
        RXOVIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive parity error interrupt enable register."]
    #[inline(always)]
    pub fn rxpeie(&self) -> RXPEIE_R {
        RXPEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit done interrupt enable register."]
    #[inline(always)]
    pub fn txdoneie(&self) -> TXDONEIE_R {
        TXDONEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit engine enable register."]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TXEN_W<0> {
        TXEN_W::new(self)
    }
    #[doc = "Bit 1 - Receive engine enable register."]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RXEN_W<1> {
        RXEN_W::new(self)
    }
    #[doc = "Bit 3 - Receive interrupt enable register."]
    #[inline(always)]
    #[must_use]
    pub fn rxie(&mut self) -> RXIE_W<3> {
        RXIE_W::new(self)
    }
    #[doc = "Bit 4 - Transmit overrun interrupt enable register."]
    #[inline(always)]
    #[must_use]
    pub fn txovie(&mut self) -> TXOVIE_W<4> {
        TXOVIE_W::new(self)
    }
    #[doc = "Bit 5 - Receive overrun interrupt enable register."]
    #[inline(always)]
    #[must_use]
    pub fn rxovie(&mut self) -> RXOVIE_W<5> {
        RXOVIE_W::new(self)
    }
    #[doc = "Bit 7 - Receive parity error interrupt enable register."]
    #[inline(always)]
    #[must_use]
    pub fn rxpeie(&mut self) -> RXPEIE_W<7> {
        RXPEIE_W::new(self)
    }
    #[doc = "Bit 8 - Transmit done interrupt enable register."]
    #[inline(always)]
    #[must_use]
    pub fn txdoneie(&mut self) -> TXDONEIE_W<8> {
        TXDONEIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
