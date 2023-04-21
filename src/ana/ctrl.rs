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
#[doc = "Field `CMP1SEL` reader - This register is used to control the interrupt and wake-up signal generation of CMP1."]
pub type CMP1SEL_R = crate::FieldReader<u8, CMP1SEL_A>;
#[doc = "This register is used to control the interrupt and wake-up signal generation of CMP1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMP1SEL_A {
    #[doc = "0: `0`"]
    OFF = 0,
    #[doc = "1: `1`"]
    RISING = 1,
    #[doc = "2: `10`"]
    FALLING = 2,
    #[doc = "3: `11`"]
    BOTH = 3,
}
impl From<CMP1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CMP1SEL_A) -> Self {
        variant as _
    }
}
impl CMP1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP1SEL_A {
        match self.bits {
            0 => CMP1SEL_A::OFF,
            1 => CMP1SEL_A::RISING,
            2 => CMP1SEL_A::FALLING,
            3 => CMP1SEL_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CMP1SEL_A::OFF
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == CMP1SEL_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == CMP1SEL_A::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == CMP1SEL_A::BOTH
    }
}
#[doc = "Field `CMP1SEL` writer - This register is used to control the interrupt and wake-up signal generation of CMP1."]
pub type CMP1SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, CMP1SEL_A, 2, O>;
impl<'a, const O: u8> CMP1SEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CMP1SEL_A::OFF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(CMP1SEL_A::RISING)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(CMP1SEL_A::FALLING)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(CMP1SEL_A::BOTH)
    }
}
#[doc = "Field `CMP2SEL` reader - This register is used to control the interrupt and wake-up signal generation of CMP2."]
pub type CMP2SEL_R = crate::FieldReader<u8, CMP2SEL_A>;
#[doc = "This register is used to control the interrupt and wake-up signal generation of CMP2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMP2SEL_A {
    #[doc = "0: `0`"]
    OFF = 0,
    #[doc = "1: `1`"]
    RISING = 1,
    #[doc = "2: `10`"]
    FALLING = 2,
    #[doc = "3: `11`"]
    BOTH = 3,
}
impl From<CMP2SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CMP2SEL_A) -> Self {
        variant as _
    }
}
impl CMP2SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP2SEL_A {
        match self.bits {
            0 => CMP2SEL_A::OFF,
            1 => CMP2SEL_A::RISING,
            2 => CMP2SEL_A::FALLING,
            3 => CMP2SEL_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CMP2SEL_A::OFF
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == CMP2SEL_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == CMP2SEL_A::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == CMP2SEL_A::BOTH
    }
}
#[doc = "Field `CMP2SEL` writer - This register is used to control the interrupt and wake-up signal generation of CMP2."]
pub type CMP2SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, CMP2SEL_A, 2, O>;
impl<'a, const O: u8> CMP2SEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CMP2SEL_A::OFF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(CMP2SEL_A::RISING)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(CMP2SEL_A::FALLING)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(CMP2SEL_A::BOTH)
    }
}
#[doc = "Field `PDNS` reader - This register is used to set the deep sleep behavior when PWRDROP is 0(Still need to consider the PDNS setting)."]
pub type PDNS_R = crate::BitReader<bool>;
#[doc = "Field `PDNS` writer - This register is used to set the deep sleep behavior when PWRDROP is 0(Still need to consider the PDNS setting)."]
pub type PDNS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `RCHTGT` reader - RCH auto calibration target register."]
pub type RCHTGT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RCHTGT` writer - RCH auto calibration target register."]
pub type RCHTGT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `CMP1DEB` reader - Comparator 1 de-bounce control register."]
pub type CMP1DEB_R = crate::FieldReader<u8, CMP1DEB_A>;
#[doc = "Comparator 1 de-bounce control register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMP1DEB_A {
    #[doc = "0: `0`"]
    NO_DEBOUNCE = 0,
    #[doc = "1: `1`"]
    _2_32KHZ_DEBOUNCE = 1,
    #[doc = "2: `10`"]
    _3_32KHZ_DEBOUNCE = 2,
    #[doc = "3: `11`"]
    _4_32KHZ_DEBOUNCE = 3,
}
impl From<CMP1DEB_A> for u8 {
    #[inline(always)]
    fn from(variant: CMP1DEB_A) -> Self {
        variant as _
    }
}
impl CMP1DEB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP1DEB_A {
        match self.bits {
            0 => CMP1DEB_A::NO_DEBOUNCE,
            1 => CMP1DEB_A::_2_32KHZ_DEBOUNCE,
            2 => CMP1DEB_A::_3_32KHZ_DEBOUNCE,
            3 => CMP1DEB_A::_4_32KHZ_DEBOUNCE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_DEBOUNCE`"]
    #[inline(always)]
    pub fn is_no_debounce(&self) -> bool {
        *self == CMP1DEB_A::NO_DEBOUNCE
    }
    #[doc = "Checks if the value of the field is `_2_32KHZ_DEBOUNCE`"]
    #[inline(always)]
    pub fn is_2_32khz_debounce(&self) -> bool {
        *self == CMP1DEB_A::_2_32KHZ_DEBOUNCE
    }
    #[doc = "Checks if the value of the field is `_3_32KHZ_DEBOUNCE`"]
    #[inline(always)]
    pub fn is_3_32khz_debounce(&self) -> bool {
        *self == CMP1DEB_A::_3_32KHZ_DEBOUNCE
    }
    #[doc = "Checks if the value of the field is `_4_32KHZ_DEBOUNCE`"]
    #[inline(always)]
    pub fn is_4_32khz_debounce(&self) -> bool {
        *self == CMP1DEB_A::_4_32KHZ_DEBOUNCE
    }
}
#[doc = "Field `CMP1DEB` writer - Comparator 1 de-bounce control register."]
pub type CMP1DEB_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, CMP1DEB_A, 2, O>;
impl<'a, const O: u8> CMP1DEB_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_debounce(self) -> &'a mut W {
        self.variant(CMP1DEB_A::NO_DEBOUNCE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _2_32khz_debounce(self) -> &'a mut W {
        self.variant(CMP1DEB_A::_2_32KHZ_DEBOUNCE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _3_32khz_debounce(self) -> &'a mut W {
        self.variant(CMP1DEB_A::_3_32KHZ_DEBOUNCE)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn _4_32khz_debounce(self) -> &'a mut W {
        self.variant(CMP1DEB_A::_4_32KHZ_DEBOUNCE)
    }
}
#[doc = "Field `CMP2DEB` reader - Comparator 2 de-bounce control register."]
pub type CMP2DEB_R = crate::FieldReader<u8, CMP2DEB_A>;
#[doc = "Comparator 2 de-bounce control register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMP2DEB_A {
    #[doc = "0: `0`"]
    NO_DEBOUNCE = 0,
    #[doc = "1: `1`"]
    _2_32KHZ_DEBOUNCE = 1,
    #[doc = "2: `10`"]
    _3_32KHZ_DEBOUNCE = 2,
    #[doc = "3: `11`"]
    _4_32KHZ_DEBOUNCE = 3,
}
impl From<CMP2DEB_A> for u8 {
    #[inline(always)]
    fn from(variant: CMP2DEB_A) -> Self {
        variant as _
    }
}
impl CMP2DEB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP2DEB_A {
        match self.bits {
            0 => CMP2DEB_A::NO_DEBOUNCE,
            1 => CMP2DEB_A::_2_32KHZ_DEBOUNCE,
            2 => CMP2DEB_A::_3_32KHZ_DEBOUNCE,
            3 => CMP2DEB_A::_4_32KHZ_DEBOUNCE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_DEBOUNCE`"]
    #[inline(always)]
    pub fn is_no_debounce(&self) -> bool {
        *self == CMP2DEB_A::NO_DEBOUNCE
    }
    #[doc = "Checks if the value of the field is `_2_32KHZ_DEBOUNCE`"]
    #[inline(always)]
    pub fn is_2_32khz_debounce(&self) -> bool {
        *self == CMP2DEB_A::_2_32KHZ_DEBOUNCE
    }
    #[doc = "Checks if the value of the field is `_3_32KHZ_DEBOUNCE`"]
    #[inline(always)]
    pub fn is_3_32khz_debounce(&self) -> bool {
        *self == CMP2DEB_A::_3_32KHZ_DEBOUNCE
    }
    #[doc = "Checks if the value of the field is `_4_32KHZ_DEBOUNCE`"]
    #[inline(always)]
    pub fn is_4_32khz_debounce(&self) -> bool {
        *self == CMP2DEB_A::_4_32KHZ_DEBOUNCE
    }
}
#[doc = "Field `CMP2DEB` writer - Comparator 2 de-bounce control register."]
pub type CMP2DEB_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, CMP2DEB_A, 2, O>;
impl<'a, const O: u8> CMP2DEB_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_debounce(self) -> &'a mut W {
        self.variant(CMP2DEB_A::NO_DEBOUNCE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _2_32khz_debounce(self) -> &'a mut W {
        self.variant(CMP2DEB_A::_2_32KHZ_DEBOUNCE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _3_32khz_debounce(self) -> &'a mut W {
        self.variant(CMP2DEB_A::_3_32KHZ_DEBOUNCE)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn _4_32khz_debounce(self) -> &'a mut W {
        self.variant(CMP2DEB_A::_4_32KHZ_DEBOUNCE)
    }
}
#[doc = "Field `PDNS2` reader - This register is used to set the deep sleep behavior when PWRALRAM is 0. (Still need to consider the PDNS setting)."]
pub type PDNS2_R = crate::BitReader<bool>;
#[doc = "Field `PDNS2` writer - This register is used to set the deep sleep behavior when PWRALRAM is 0. (Still need to consider the PDNS setting)."]
pub type PDNS2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - This register is used to control the interrupt and wake-up signal generation of CMP1."]
    #[inline(always)]
    pub fn cmp1sel(&self) -> CMP1SEL_R {
        CMP1SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - This register is used to control the interrupt and wake-up signal generation of CMP2."]
    #[inline(always)]
    pub fn cmp2sel(&self) -> CMP2SEL_R {
        CMP2SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 6 - This register is used to set the deep sleep behavior when PWRDROP is 0(Still need to consider the PDNS setting)."]
    #[inline(always)]
    pub fn pdns(&self) -> PDNS_R {
        PDNS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:15 - RCH auto calibration target register."]
    #[inline(always)]
    pub fn rchtgt(&self) -> RCHTGT_R {
        RCHTGT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 20:21 - Comparator 1 de-bounce control register."]
    #[inline(always)]
    pub fn cmp1deb(&self) -> CMP1DEB_R {
        CMP1DEB_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Comparator 2 de-bounce control register."]
    #[inline(always)]
    pub fn cmp2deb(&self) -> CMP2DEB_R {
        CMP2DEB_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 26 - This register is used to set the deep sleep behavior when PWRALRAM is 0. (Still need to consider the PDNS setting)."]
    #[inline(always)]
    pub fn pdns2(&self) -> PDNS2_R {
        PDNS2_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - This register is used to control the interrupt and wake-up signal generation of CMP1."]
    #[inline(always)]
    #[must_use]
    pub fn cmp1sel(&mut self) -> CMP1SEL_W<0> {
        CMP1SEL_W::new(self)
    }
    #[doc = "Bits 2:3 - This register is used to control the interrupt and wake-up signal generation of CMP2."]
    #[inline(always)]
    #[must_use]
    pub fn cmp2sel(&mut self) -> CMP2SEL_W<2> {
        CMP2SEL_W::new(self)
    }
    #[doc = "Bit 6 - This register is used to set the deep sleep behavior when PWRDROP is 0(Still need to consider the PDNS setting)."]
    #[inline(always)]
    #[must_use]
    pub fn pdns(&mut self) -> PDNS_W<6> {
        PDNS_W::new(self)
    }
    #[doc = "Bits 8:15 - RCH auto calibration target register."]
    #[inline(always)]
    #[must_use]
    pub fn rchtgt(&mut self) -> RCHTGT_W<8> {
        RCHTGT_W::new(self)
    }
    #[doc = "Bits 20:21 - Comparator 1 de-bounce control register."]
    #[inline(always)]
    #[must_use]
    pub fn cmp1deb(&mut self) -> CMP1DEB_W<20> {
        CMP1DEB_W::new(self)
    }
    #[doc = "Bits 22:23 - Comparator 2 de-bounce control register."]
    #[inline(always)]
    #[must_use]
    pub fn cmp2deb(&mut self) -> CMP2DEB_W<22> {
        CMP2DEB_W::new(self)
    }
    #[doc = "Bit 26 - This register is used to set the deep sleep behavior when PWRALRAM is 0. (Still need to consider the PDNS setting)."]
    #[inline(always)]
    #[must_use]
    pub fn pdns2(&mut self) -> PDNS2_W<26> {
        PDNS2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
