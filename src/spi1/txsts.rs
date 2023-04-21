#[doc = "Register `TXSTS` reader"]
pub struct R(crate::R<TXSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXSTS` writer"]
pub struct W(crate::W<TXSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXSTS_SPEC>;
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
impl From<crate::W<TXSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXFFLAG` reader - Transmit FIFO Data Level."]
pub type TXFFLAG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMATXDONE` reader - DMA Transmit finish flag."]
pub type DMATXDONE_R = crate::BitReader<bool>;
#[doc = "Field `DMATXDONE` writer - DMA Transmit finish flag."]
pub type DMATXDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXSTS_SPEC, bool, O>;
#[doc = "Field `TXFLEV` reader - Transmit FIFO interrupt level register."]
pub type TXFLEV_R = crate::FieldReader<u8, TXFLEV_A>;
#[doc = "Transmit FIFO interrupt level register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXFLEV_A {
    #[doc = "0: `0`"]
    LESS_THAN_1 = 0,
    #[doc = "1: `1`"]
    LESS_THAN_2 = 1,
    #[doc = "2: `10`"]
    LESS_THAN_3 = 2,
    #[doc = "3: `11`"]
    LESS_THAN_4 = 3,
    #[doc = "4: `100`"]
    LESS_THAN_5 = 4,
    #[doc = "5: `101`"]
    LESS_THAN_6 = 5,
    #[doc = "6: `110`"]
    LESS_THAN_7 = 6,
    #[doc = "7: `111`"]
    LESS_THAN_8 = 7,
}
impl From<TXFLEV_A> for u8 {
    #[inline(always)]
    fn from(variant: TXFLEV_A) -> Self {
        variant as _
    }
}
impl TXFLEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFLEV_A {
        match self.bits {
            0 => TXFLEV_A::LESS_THAN_1,
            1 => TXFLEV_A::LESS_THAN_2,
            2 => TXFLEV_A::LESS_THAN_3,
            3 => TXFLEV_A::LESS_THAN_4,
            4 => TXFLEV_A::LESS_THAN_5,
            5 => TXFLEV_A::LESS_THAN_6,
            6 => TXFLEV_A::LESS_THAN_7,
            7 => TXFLEV_A::LESS_THAN_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LESS_THAN_1`"]
    #[inline(always)]
    pub fn is_less_than_1(&self) -> bool {
        *self == TXFLEV_A::LESS_THAN_1
    }
    #[doc = "Checks if the value of the field is `LESS_THAN_2`"]
    #[inline(always)]
    pub fn is_less_than_2(&self) -> bool {
        *self == TXFLEV_A::LESS_THAN_2
    }
    #[doc = "Checks if the value of the field is `LESS_THAN_3`"]
    #[inline(always)]
    pub fn is_less_than_3(&self) -> bool {
        *self == TXFLEV_A::LESS_THAN_3
    }
    #[doc = "Checks if the value of the field is `LESS_THAN_4`"]
    #[inline(always)]
    pub fn is_less_than_4(&self) -> bool {
        *self == TXFLEV_A::LESS_THAN_4
    }
    #[doc = "Checks if the value of the field is `LESS_THAN_5`"]
    #[inline(always)]
    pub fn is_less_than_5(&self) -> bool {
        *self == TXFLEV_A::LESS_THAN_5
    }
    #[doc = "Checks if the value of the field is `LESS_THAN_6`"]
    #[inline(always)]
    pub fn is_less_than_6(&self) -> bool {
        *self == TXFLEV_A::LESS_THAN_6
    }
    #[doc = "Checks if the value of the field is `LESS_THAN_7`"]
    #[inline(always)]
    pub fn is_less_than_7(&self) -> bool {
        *self == TXFLEV_A::LESS_THAN_7
    }
    #[doc = "Checks if the value of the field is `LESS_THAN_8`"]
    #[inline(always)]
    pub fn is_less_than_8(&self) -> bool {
        *self == TXFLEV_A::LESS_THAN_8
    }
}
#[doc = "Field `TXFLEV` writer - Transmit FIFO interrupt level register."]
pub type TXFLEV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TXSTS_SPEC, u8, TXFLEV_A, 3, O>;
impl<'a, const O: u8> TXFLEV_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn less_than_1(self) -> &'a mut W {
        self.variant(TXFLEV_A::LESS_THAN_1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn less_than_2(self) -> &'a mut W {
        self.variant(TXFLEV_A::LESS_THAN_2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn less_than_3(self) -> &'a mut W {
        self.variant(TXFLEV_A::LESS_THAN_3)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn less_than_4(self) -> &'a mut W {
        self.variant(TXFLEV_A::LESS_THAN_4)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn less_than_5(self) -> &'a mut W {
        self.variant(TXFLEV_A::LESS_THAN_5)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn less_than_6(self) -> &'a mut W {
        self.variant(TXFLEV_A::LESS_THAN_6)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn less_than_7(self) -> &'a mut W {
        self.variant(TXFLEV_A::LESS_THAN_7)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn less_than_8(self) -> &'a mut W {
        self.variant(TXFLEV_A::LESS_THAN_8)
    }
}
#[doc = "Field `TXFUR` reader - Transmit FIFO under run register."]
pub type TXFUR_R = crate::BitReader<bool>;
#[doc = "Field `TXEMPTY` reader - Transmit FIFO empty register."]
pub type TXEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `TXIEN` reader - SPI Transmit Interrupt Enable."]
pub type TXIEN_R = crate::BitReader<bool>;
#[doc = "Field `TXIEN` writer - SPI Transmit Interrupt Enable."]
pub type TXIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXSTS_SPEC, bool, O>;
#[doc = "Field `TXIF` reader - SPI Transmit Interrupt flag."]
pub type TXIF_R = crate::BitReader<bool>;
#[doc = "Field `TXIF` writer - SPI Transmit Interrupt flag."]
pub type TXIF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, TXSTS_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Transmit FIFO Data Level."]
    #[inline(always)]
    pub fn txfflag(&self) -> TXFFLAG_R {
        TXFFLAG_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - DMA Transmit finish flag."]
    #[inline(always)]
    pub fn dmatxdone(&self) -> DMATXDONE_R {
        DMATXDONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Transmit FIFO interrupt level register."]
    #[inline(always)]
    pub fn txflev(&self) -> TXFLEV_R {
        TXFLEV_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Transmit FIFO under run register."]
    #[inline(always)]
    pub fn txfur(&self) -> TXFUR_R {
        TXFUR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmit FIFO empty register."]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI Transmit Interrupt Enable."]
    #[inline(always)]
    pub fn txien(&self) -> TXIEN_R {
        TXIEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI Transmit Interrupt flag."]
    #[inline(always)]
    pub fn txif(&self) -> TXIF_R {
        TXIF_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - DMA Transmit finish flag."]
    #[inline(always)]
    #[must_use]
    pub fn dmatxdone(&mut self) -> DMATXDONE_W<3> {
        DMATXDONE_W::new(self)
    }
    #[doc = "Bits 4:6 - Transmit FIFO interrupt level register."]
    #[inline(always)]
    #[must_use]
    pub fn txflev(&mut self) -> TXFLEV_W<4> {
        TXFLEV_W::new(self)
    }
    #[doc = "Bit 14 - SPI Transmit Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn txien(&mut self) -> TXIEN_W<14> {
        TXIEN_W::new(self)
    }
    #[doc = "Bit 15 - SPI Transmit Interrupt flag."]
    #[inline(always)]
    #[must_use]
    pub fn txif(&mut self) -> TXIF_W<15> {
        TXIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Transmit Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txsts](index.html) module"]
pub struct TXSTS_SPEC;
impl crate::RegisterSpec for TXSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txsts::R](R) reader structure"]
impl crate::Readable for TXSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txsts::W](W) writer structure"]
impl crate::Writable for TXSTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x8000;
}
#[doc = "`reset()` method sets TXSTS to value 0x8200"]
impl crate::Resettable for TXSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0x8200;
}
