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
#[doc = "Field `SCKSEL` reader - Master mode clock selection"]
pub type SCKSEL_R = crate::FieldReader<u8, SCKSEL_A>;
#[doc = "Master mode clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCKSEL_A {
    #[doc = "0: `0`"]
    APBCLK_DIV_2 = 0,
    #[doc = "1: `1`"]
    APBCLK_DIV_4 = 1,
    #[doc = "2: `10`"]
    APBCLK_DIV_8 = 2,
    #[doc = "3: `11`"]
    APBCLK_DIV_16 = 3,
    #[doc = "4: `100`"]
    APBCLK_DIV_32 = 4,
    #[doc = "5: `101`"]
    APBCLK_DIV_64 = 5,
    #[doc = "6: `110`"]
    APBCLK_DIV_128 = 6,
}
impl From<SCKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SCKSEL_A) -> Self {
        variant as _
    }
}
impl SCKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCKSEL_A {
        match self.bits {
            0 => SCKSEL_A::APBCLK_DIV_2,
            1 => SCKSEL_A::APBCLK_DIV_4,
            2 => SCKSEL_A::APBCLK_DIV_8,
            3 => SCKSEL_A::APBCLK_DIV_16,
            4 => SCKSEL_A::APBCLK_DIV_32,
            5 => SCKSEL_A::APBCLK_DIV_64,
            6 => SCKSEL_A::APBCLK_DIV_128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `APBCLK_DIV_2`"]
    #[inline(always)]
    pub fn is_apbclk_div_2(&self) -> bool {
        *self == SCKSEL_A::APBCLK_DIV_2
    }
    #[doc = "Checks if the value of the field is `APBCLK_DIV_4`"]
    #[inline(always)]
    pub fn is_apbclk_div_4(&self) -> bool {
        *self == SCKSEL_A::APBCLK_DIV_4
    }
    #[doc = "Checks if the value of the field is `APBCLK_DIV_8`"]
    #[inline(always)]
    pub fn is_apbclk_div_8(&self) -> bool {
        *self == SCKSEL_A::APBCLK_DIV_8
    }
    #[doc = "Checks if the value of the field is `APBCLK_DIV_16`"]
    #[inline(always)]
    pub fn is_apbclk_div_16(&self) -> bool {
        *self == SCKSEL_A::APBCLK_DIV_16
    }
    #[doc = "Checks if the value of the field is `APBCLK_DIV_32`"]
    #[inline(always)]
    pub fn is_apbclk_div_32(&self) -> bool {
        *self == SCKSEL_A::APBCLK_DIV_32
    }
    #[doc = "Checks if the value of the field is `APBCLK_DIV_64`"]
    #[inline(always)]
    pub fn is_apbclk_div_64(&self) -> bool {
        *self == SCKSEL_A::APBCLK_DIV_64
    }
    #[doc = "Checks if the value of the field is `APBCLK_DIV_128`"]
    #[inline(always)]
    pub fn is_apbclk_div_128(&self) -> bool {
        *self == SCKSEL_A::APBCLK_DIV_128
    }
}
#[doc = "Field `SCKSEL` writer - Master mode clock selection"]
pub type SCKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, SCKSEL_A, 3, O>;
impl<'a, const O: u8> SCKSEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn apbclk_div_2(self) -> &'a mut W {
        self.variant(SCKSEL_A::APBCLK_DIV_2)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn apbclk_div_4(self) -> &'a mut W {
        self.variant(SCKSEL_A::APBCLK_DIV_4)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn apbclk_div_8(self) -> &'a mut W {
        self.variant(SCKSEL_A::APBCLK_DIV_8)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn apbclk_div_16(self) -> &'a mut W {
        self.variant(SCKSEL_A::APBCLK_DIV_16)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn apbclk_div_32(self) -> &'a mut W {
        self.variant(SCKSEL_A::APBCLK_DIV_32)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn apbclk_div_64(self) -> &'a mut W {
        self.variant(SCKSEL_A::APBCLK_DIV_64)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn apbclk_div_128(self) -> &'a mut W {
        self.variant(SCKSEL_A::APBCLK_DIV_128)
    }
}
#[doc = "Field `SCKPOL` reader - SPI clock polarity."]
pub type SCKPOL_R = crate::BitReader<bool>;
#[doc = "Field `SCKPOL` writer - SPI clock polarity."]
pub type SCKPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SCKPHA` reader - SPI clock phase."]
pub type SCKPHA_R = crate::BitReader<bool>;
#[doc = "Field `SCKPHA` writer - SPI clock phase."]
pub type SCKPHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `MOD` reader - SPI Mode Selection register."]
pub type MOD_R = crate::BitReader<bool>;
#[doc = "Field `MOD` writer - SPI Mode Selection register."]
pub type MOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SWAP` reader - SPI MISO/MOSI swap control register."]
pub type SWAP_R = crate::BitReader<bool>;
#[doc = "Field `SWAP` writer - SPI MISO/MOSI swap control register."]
pub type SWAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CSGPIO` reader - SPI CS pin is controlled by GPIO or H/W."]
pub type CSGPIO_R = crate::BitReader<bool>;
#[doc = "Field `CSGPIO` writer - SPI CS pin is controlled by GPIO or H/W."]
pub type CSGPIO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `RST` reader - SPI Soft Reset."]
pub type RST_R = crate::BitReader<bool>;
#[doc = "Field `RST` writer - SPI Soft Reset."]
pub type RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `LSBF` reader - SPI LSBF."]
pub type LSBF_R = crate::BitReader<bool>;
#[doc = "Field `LSBF` writer - SPI LSBF."]
pub type LSBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `EN` reader - SPI enable."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - SPI enable."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Master mode clock selection"]
    #[inline(always)]
    pub fn scksel(&self) -> SCKSEL_R {
        SCKSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - SPI clock polarity."]
    #[inline(always)]
    pub fn sckpol(&self) -> SCKPOL_R {
        SCKPOL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI clock phase."]
    #[inline(always)]
    pub fn sckpha(&self) -> SCKPHA_R {
        SCKPHA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - SPI Mode Selection register."]
    #[inline(always)]
    pub fn mod_(&self) -> MOD_R {
        MOD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SPI MISO/MOSI swap control register."]
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SPI CS pin is controlled by GPIO or H/W."]
    #[inline(always)]
    pub fn csgpio(&self) -> CSGPIO_R {
        CSGPIO_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SPI Soft Reset."]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI LSBF."]
    #[inline(always)]
    pub fn lsbf(&self) -> LSBF_R {
        LSBF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI enable."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Master mode clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn scksel(&mut self) -> SCKSEL_W<0> {
        SCKSEL_W::new(self)
    }
    #[doc = "Bit 4 - SPI clock polarity."]
    #[inline(always)]
    #[must_use]
    pub fn sckpol(&mut self) -> SCKPOL_W<4> {
        SCKPOL_W::new(self)
    }
    #[doc = "Bit 5 - SPI clock phase."]
    #[inline(always)]
    #[must_use]
    pub fn sckpha(&mut self) -> SCKPHA_W<5> {
        SCKPHA_W::new(self)
    }
    #[doc = "Bit 8 - SPI Mode Selection register."]
    #[inline(always)]
    #[must_use]
    pub fn mod_(&mut self) -> MOD_W<8> {
        MOD_W::new(self)
    }
    #[doc = "Bit 9 - SPI MISO/MOSI swap control register."]
    #[inline(always)]
    #[must_use]
    pub fn swap(&mut self) -> SWAP_W<9> {
        SWAP_W::new(self)
    }
    #[doc = "Bit 10 - SPI CS pin is controlled by GPIO or H/W."]
    #[inline(always)]
    #[must_use]
    pub fn csgpio(&mut self) -> CSGPIO_W<10> {
        CSGPIO_W::new(self)
    }
    #[doc = "Bit 11 - SPI Soft Reset."]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RST_W<11> {
        RST_W::new(self)
    }
    #[doc = "Bit 12 - SPI LSBF."]
    #[inline(always)]
    #[must_use]
    pub fn lsbf(&mut self) -> LSBF_W<12> {
        LSBF_W::new(self)
    }
    #[doc = "Bit 15 - SPI enable."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<15> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
