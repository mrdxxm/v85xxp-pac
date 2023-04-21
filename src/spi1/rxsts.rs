#[doc = "Register `RXSTS` reader"]
pub struct R(crate::R<RXSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXSTS` writer"]
pub struct W(crate::W<RXSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXSTS_SPEC>;
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
impl From<crate::W<RXSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXFFLAG` reader - Receive FIFO Data Level."]
pub type RXFFLAG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXFLEV` reader - Receive FIFO interrupt level register."]
pub type RXFLEV_R = crate::FieldReader<u8, RXFLEV_A>;
#[doc = "Receive FIFO interrupt level register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXFLEV_A {
    #[doc = "0: `0`"]
    GREAT_EQUAL_1 = 0,
    #[doc = "1: `1`"]
    GREAT_EQUAL_2 = 1,
    #[doc = "2: `10`"]
    GREAT_EQUAL_3 = 2,
    #[doc = "3: `11`"]
    GREAT_EQUAL_4 = 3,
    #[doc = "4: `100`"]
    GREAT_EQUAL_5 = 4,
    #[doc = "5: `101`"]
    GREAT_EQUAL_6 = 5,
    #[doc = "6: `110`"]
    GREAT_EQUAL_7 = 6,
    #[doc = "7: `111`"]
    GREAT_EQUAL_8 = 7,
}
impl From<RXFLEV_A> for u8 {
    #[inline(always)]
    fn from(variant: RXFLEV_A) -> Self {
        variant as _
    }
}
impl RXFLEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFLEV_A {
        match self.bits {
            0 => RXFLEV_A::GREAT_EQUAL_1,
            1 => RXFLEV_A::GREAT_EQUAL_2,
            2 => RXFLEV_A::GREAT_EQUAL_3,
            3 => RXFLEV_A::GREAT_EQUAL_4,
            4 => RXFLEV_A::GREAT_EQUAL_5,
            5 => RXFLEV_A::GREAT_EQUAL_6,
            6 => RXFLEV_A::GREAT_EQUAL_7,
            7 => RXFLEV_A::GREAT_EQUAL_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GREAT_EQUAL_1`"]
    #[inline(always)]
    pub fn is_great_equal_1(&self) -> bool {
        *self == RXFLEV_A::GREAT_EQUAL_1
    }
    #[doc = "Checks if the value of the field is `GREAT_EQUAL_2`"]
    #[inline(always)]
    pub fn is_great_equal_2(&self) -> bool {
        *self == RXFLEV_A::GREAT_EQUAL_2
    }
    #[doc = "Checks if the value of the field is `GREAT_EQUAL_3`"]
    #[inline(always)]
    pub fn is_great_equal_3(&self) -> bool {
        *self == RXFLEV_A::GREAT_EQUAL_3
    }
    #[doc = "Checks if the value of the field is `GREAT_EQUAL_4`"]
    #[inline(always)]
    pub fn is_great_equal_4(&self) -> bool {
        *self == RXFLEV_A::GREAT_EQUAL_4
    }
    #[doc = "Checks if the value of the field is `GREAT_EQUAL_5`"]
    #[inline(always)]
    pub fn is_great_equal_5(&self) -> bool {
        *self == RXFLEV_A::GREAT_EQUAL_5
    }
    #[doc = "Checks if the value of the field is `GREAT_EQUAL_6`"]
    #[inline(always)]
    pub fn is_great_equal_6(&self) -> bool {
        *self == RXFLEV_A::GREAT_EQUAL_6
    }
    #[doc = "Checks if the value of the field is `GREAT_EQUAL_7`"]
    #[inline(always)]
    pub fn is_great_equal_7(&self) -> bool {
        *self == RXFLEV_A::GREAT_EQUAL_7
    }
    #[doc = "Checks if the value of the field is `GREAT_EQUAL_8`"]
    #[inline(always)]
    pub fn is_great_equal_8(&self) -> bool {
        *self == RXFLEV_A::GREAT_EQUAL_8
    }
}
#[doc = "Field `RXFLEV` writer - Receive FIFO interrupt level register."]
pub type RXFLEV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, RXSTS_SPEC, u8, RXFLEV_A, 3, O>;
impl<'a, const O: u8> RXFLEV_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn great_equal_1(self) -> &'a mut W {
        self.variant(RXFLEV_A::GREAT_EQUAL_1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn great_equal_2(self) -> &'a mut W {
        self.variant(RXFLEV_A::GREAT_EQUAL_2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn great_equal_3(self) -> &'a mut W {
        self.variant(RXFLEV_A::GREAT_EQUAL_3)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn great_equal_4(self) -> &'a mut W {
        self.variant(RXFLEV_A::GREAT_EQUAL_4)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn great_equal_5(self) -> &'a mut W {
        self.variant(RXFLEV_A::GREAT_EQUAL_5)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn great_equal_6(self) -> &'a mut W {
        self.variant(RXFLEV_A::GREAT_EQUAL_6)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn great_equal_7(self) -> &'a mut W {
        self.variant(RXFLEV_A::GREAT_EQUAL_7)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn great_equal_8(self) -> &'a mut W {
        self.variant(RXFLEV_A::GREAT_EQUAL_8)
    }
}
#[doc = "Field `RXFOV` reader - Receive FIFO over run register."]
pub type RXFOV_R = crate::BitReader<bool>;
#[doc = "Field `RXFULL` reader - Receive FIFO full register."]
pub type RXFULL_R = crate::BitReader<bool>;
#[doc = "Field `RXIEN` reader - SPI Receive Interrupt Enable."]
pub type RXIEN_R = crate::BitReader<bool>;
#[doc = "Field `RXIEN` writer - SPI Receive Interrupt Enable."]
pub type RXIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXSTS_SPEC, bool, O>;
#[doc = "Field `RXIF` reader - SPI Receive Interrupt flag."]
pub type RXIF_R = crate::BitReader<bool>;
#[doc = "Field `RXIF` writer - SPI Receive Interrupt flag."]
pub type RXIF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, RXSTS_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Receive FIFO Data Level."]
    #[inline(always)]
    pub fn rxfflag(&self) -> RXFFLAG_R {
        RXFFLAG_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Receive FIFO interrupt level register."]
    #[inline(always)]
    pub fn rxflev(&self) -> RXFLEV_R {
        RXFLEV_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Receive FIFO over run register."]
    #[inline(always)]
    pub fn rxfov(&self) -> RXFOV_R {
        RXFOV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive FIFO full register."]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI Receive Interrupt Enable."]
    #[inline(always)]
    pub fn rxien(&self) -> RXIEN_R {
        RXIEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI Receive Interrupt flag."]
    #[inline(always)]
    pub fn rxif(&self) -> RXIF_R {
        RXIF_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:6 - Receive FIFO interrupt level register."]
    #[inline(always)]
    #[must_use]
    pub fn rxflev(&mut self) -> RXFLEV_W<4> {
        RXFLEV_W::new(self)
    }
    #[doc = "Bit 14 - SPI Receive Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn rxien(&mut self) -> RXIEN_W<14> {
        RXIEN_W::new(self)
    }
    #[doc = "Bit 15 - SPI Receive Interrupt flag."]
    #[inline(always)]
    #[must_use]
    pub fn rxif(&mut self) -> RXIF_W<15> {
        RXIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Receive Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxsts](index.html) module"]
pub struct RXSTS_SPEC;
impl crate::RegisterSpec for RXSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxsts::R](R) reader structure"]
impl crate::Readable for RXSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxsts::W](W) writer structure"]
impl crate::Writable for RXSTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x8000;
}
#[doc = "`reset()` method sets RXSTS to value 0"]
impl crate::Resettable for RXSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
