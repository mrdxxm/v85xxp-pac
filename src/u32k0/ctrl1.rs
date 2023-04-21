#[doc = "Register `CTRL1` reader"]
pub struct R(crate::R<CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL1` writer"]
pub struct W(crate::W<CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL1_SPEC>;
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
impl From<crate::W<CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXIE` reader - Receive interrupt/wake-up enable register."]
pub type RXIE_R = crate::BitReader<bool>;
#[doc = "Field `RXIE` writer - Receive interrupt/wake-up enable register."]
pub type RXIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL1_SPEC, bool, O>;
#[doc = "Field `RXPEIE` reader - Receive parity error interrupt/wake-up enable register."]
pub type RXPEIE_R = crate::BitReader<bool>;
#[doc = "Field `RXPEIE` writer - Receive parity error interrupt/wake-up enable register."]
pub type RXPEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL1_SPEC, bool, O>;
#[doc = "Field `RXOVIE` reader - Receive overrun interrupt/wake-up enable register."]
pub type RXOVIE_R = crate::BitReader<bool>;
#[doc = "Field `RXOVIE` writer - Receive overrun interrupt/wake-up enable register."]
pub type RXOVIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL1_SPEC, bool, O>;
#[doc = "Field `RXSEL` reader - Receive data select register."]
pub type RXSEL_R = crate::FieldReader<u8, RXSEL_A>;
#[doc = "Receive data select register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXSEL_A {
    #[doc = "0: `0`"]
    FROM_RX0 = 0,
    #[doc = "1: `1`"]
    FROM_RX1 = 1,
    #[doc = "2: `10`"]
    FROM_RX2 = 2,
    #[doc = "3: `11`"]
    FROM_RX3 = 3,
}
impl From<RXSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RXSEL_A) -> Self {
        variant as _
    }
}
impl RXSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXSEL_A {
        match self.bits {
            0 => RXSEL_A::FROM_RX0,
            1 => RXSEL_A::FROM_RX1,
            2 => RXSEL_A::FROM_RX2,
            3 => RXSEL_A::FROM_RX3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FROM_RX0`"]
    #[inline(always)]
    pub fn is_from_rx0(&self) -> bool {
        *self == RXSEL_A::FROM_RX0
    }
    #[doc = "Checks if the value of the field is `FROM_RX1`"]
    #[inline(always)]
    pub fn is_from_rx1(&self) -> bool {
        *self == RXSEL_A::FROM_RX1
    }
    #[doc = "Checks if the value of the field is `FROM_RX2`"]
    #[inline(always)]
    pub fn is_from_rx2(&self) -> bool {
        *self == RXSEL_A::FROM_RX2
    }
    #[doc = "Checks if the value of the field is `FROM_RX3`"]
    #[inline(always)]
    pub fn is_from_rx3(&self) -> bool {
        *self == RXSEL_A::FROM_RX3
    }
}
#[doc = "Field `RXSEL` writer - Receive data select register."]
pub type RXSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CTRL1_SPEC, u8, RXSEL_A, 2, O>;
impl<'a, const O: u8> RXSEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn from_rx0(self) -> &'a mut W {
        self.variant(RXSEL_A::FROM_RX0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn from_rx1(self) -> &'a mut W {
        self.variant(RXSEL_A::FROM_RX1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn from_rx2(self) -> &'a mut W {
        self.variant(RXSEL_A::FROM_RX2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn from_rx3(self) -> &'a mut W {
        self.variant(RXSEL_A::FROM_RX3)
    }
}
impl R {
    #[doc = "Bit 0 - Receive interrupt/wake-up enable register."]
    #[inline(always)]
    pub fn rxie(&self) -> RXIE_R {
        RXIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive parity error interrupt/wake-up enable register."]
    #[inline(always)]
    pub fn rxpeie(&self) -> RXPEIE_R {
        RXPEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive overrun interrupt/wake-up enable register."]
    #[inline(always)]
    pub fn rxovie(&self) -> RXOVIE_R {
        RXOVIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Receive data select register."]
    #[inline(always)]
    pub fn rxsel(&self) -> RXSEL_R {
        RXSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Receive interrupt/wake-up enable register."]
    #[inline(always)]
    #[must_use]
    pub fn rxie(&mut self) -> RXIE_W<0> {
        RXIE_W::new(self)
    }
    #[doc = "Bit 1 - Receive parity error interrupt/wake-up enable register."]
    #[inline(always)]
    #[must_use]
    pub fn rxpeie(&mut self) -> RXPEIE_W<1> {
        RXPEIE_W::new(self)
    }
    #[doc = "Bit 2 - Receive overrun interrupt/wake-up enable register."]
    #[inline(always)]
    #[must_use]
    pub fn rxovie(&mut self) -> RXOVIE_W<2> {
        RXOVIE_W::new(self)
    }
    #[doc = "Bits 4:5 - Receive data select register."]
    #[inline(always)]
    #[must_use]
    pub fn rxsel(&mut self) -> RXSEL_W<4> {
        RXSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART 32K x control register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl1](index.html) module"]
pub struct CTRL1_SPEC;
impl crate::RegisterSpec for CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl1::R](R) reader structure"]
impl crate::Readable for CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl1::W](W) writer structure"]
impl crate::Writable for CTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
