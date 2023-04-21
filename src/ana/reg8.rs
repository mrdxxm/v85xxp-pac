#[doc = "Register `REG8` reader"]
pub struct R(crate::R<REG8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG8` writer"]
pub struct W(crate::W<REG8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG8_SPEC>;
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
impl From<crate::W<REG8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DVCCSEL` reader - DVCC Voltage adjustment."]
pub type DVCCSEL_R = crate::FieldReader<u8, DVCCSEL_A>;
#[doc = "DVCC Voltage adjustment.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DVCCSEL_A {
    #[doc = "0: `0`"]
    ADD_0_0V = 0,
    #[doc = "1: `1`"]
    ADD_0_1V = 1,
    #[doc = "2: `10`"]
    SUB_0_1V = 2,
    #[doc = "3: `11`"]
    ADD_0_2V = 3,
}
impl From<DVCCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DVCCSEL_A) -> Self {
        variant as _
    }
}
impl DVCCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DVCCSEL_A {
        match self.bits {
            0 => DVCCSEL_A::ADD_0_0V,
            1 => DVCCSEL_A::ADD_0_1V,
            2 => DVCCSEL_A::SUB_0_1V,
            3 => DVCCSEL_A::ADD_0_2V,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADD_0_0V`"]
    #[inline(always)]
    pub fn is_add_0_0v(&self) -> bool {
        *self == DVCCSEL_A::ADD_0_0V
    }
    #[doc = "Checks if the value of the field is `ADD_0_1V`"]
    #[inline(always)]
    pub fn is_add_0_1v(&self) -> bool {
        *self == DVCCSEL_A::ADD_0_1V
    }
    #[doc = "Checks if the value of the field is `SUB_0_1V`"]
    #[inline(always)]
    pub fn is_sub_0_1v(&self) -> bool {
        *self == DVCCSEL_A::SUB_0_1V
    }
    #[doc = "Checks if the value of the field is `ADD_0_2V`"]
    #[inline(always)]
    pub fn is_add_0_2v(&self) -> bool {
        *self == DVCCSEL_A::ADD_0_2V
    }
}
#[doc = "Field `DVCCSEL` writer - DVCC Voltage adjustment."]
pub type DVCCSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, REG8_SPEC, u8, DVCCSEL_A, 2, O>;
impl<'a, const O: u8> DVCCSEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn add_0_0v(self) -> &'a mut W {
        self.variant(DVCCSEL_A::ADD_0_0V)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn add_0_1v(self) -> &'a mut W {
        self.variant(DVCCSEL_A::ADD_0_1V)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sub_0_1v(self) -> &'a mut W {
        self.variant(DVCCSEL_A::SUB_0_1V)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn add_0_2v(self) -> &'a mut W {
        self.variant(DVCCSEL_A::ADD_0_2V)
    }
}
#[doc = "Field `VDDPVDSEL` reader - Voltage selection of VDD power detector."]
pub type VDDPVDSEL_R = crate::FieldReader<u8, VDDPVDSEL_A>;
#[doc = "Voltage selection of VDD power detector.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VDDPVDSEL_A {
    #[doc = "0: `0`"]
    _4_5V = 0,
    #[doc = "1: `1`"]
    _4_2V = 1,
    #[doc = "2: `10`"]
    _3_9V = 2,
    #[doc = "3: `11`"]
    _3_6V = 3,
    #[doc = "4: `100`"]
    _3_2V = 4,
    #[doc = "5: `101`"]
    _2_9V = 5,
    #[doc = "6: `110`"]
    _2_6V = 6,
    #[doc = "7: `111`"]
    _2_3V = 7,
}
impl From<VDDPVDSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: VDDPVDSEL_A) -> Self {
        variant as _
    }
}
impl VDDPVDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDPVDSEL_A {
        match self.bits {
            0 => VDDPVDSEL_A::_4_5V,
            1 => VDDPVDSEL_A::_4_2V,
            2 => VDDPVDSEL_A::_3_9V,
            3 => VDDPVDSEL_A::_3_6V,
            4 => VDDPVDSEL_A::_3_2V,
            5 => VDDPVDSEL_A::_2_9V,
            6 => VDDPVDSEL_A::_2_6V,
            7 => VDDPVDSEL_A::_2_3V,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_4_5V`"]
    #[inline(always)]
    pub fn is_4_5v(&self) -> bool {
        *self == VDDPVDSEL_A::_4_5V
    }
    #[doc = "Checks if the value of the field is `_4_2V`"]
    #[inline(always)]
    pub fn is_4_2v(&self) -> bool {
        *self == VDDPVDSEL_A::_4_2V
    }
    #[doc = "Checks if the value of the field is `_3_9V`"]
    #[inline(always)]
    pub fn is_3_9v(&self) -> bool {
        *self == VDDPVDSEL_A::_3_9V
    }
    #[doc = "Checks if the value of the field is `_3_6V`"]
    #[inline(always)]
    pub fn is_3_6v(&self) -> bool {
        *self == VDDPVDSEL_A::_3_6V
    }
    #[doc = "Checks if the value of the field is `_3_2V`"]
    #[inline(always)]
    pub fn is_3_2v(&self) -> bool {
        *self == VDDPVDSEL_A::_3_2V
    }
    #[doc = "Checks if the value of the field is `_2_9V`"]
    #[inline(always)]
    pub fn is_2_9v(&self) -> bool {
        *self == VDDPVDSEL_A::_2_9V
    }
    #[doc = "Checks if the value of the field is `_2_6V`"]
    #[inline(always)]
    pub fn is_2_6v(&self) -> bool {
        *self == VDDPVDSEL_A::_2_6V
    }
    #[doc = "Checks if the value of the field is `_2_3V`"]
    #[inline(always)]
    pub fn is_2_3v(&self) -> bool {
        *self == VDDPVDSEL_A::_2_3V
    }
}
#[doc = "Field `VDDPVDSEL` writer - Voltage selection of VDD power detector."]
pub type VDDPVDSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, REG8_SPEC, u8, VDDPVDSEL_A, 3, O>;
impl<'a, const O: u8> VDDPVDSEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _4_5v(self) -> &'a mut W {
        self.variant(VDDPVDSEL_A::_4_5V)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _4_2v(self) -> &'a mut W {
        self.variant(VDDPVDSEL_A::_4_2V)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _3_9v(self) -> &'a mut W {
        self.variant(VDDPVDSEL_A::_3_9V)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn _3_6v(self) -> &'a mut W {
        self.variant(VDDPVDSEL_A::_3_6V)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn _3_2v(self) -> &'a mut W {
        self.variant(VDDPVDSEL_A::_3_2V)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn _2_9v(self) -> &'a mut W {
        self.variant(VDDPVDSEL_A::_2_9V)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn _2_6v(self) -> &'a mut W {
        self.variant(VDDPVDSEL_A::_2_6V)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn _2_3v(self) -> &'a mut W {
        self.variant(VDDPVDSEL_A::_2_3V)
    }
}
#[doc = "Field `AVCCLDOPD` reader - AVCC Power-down control signal."]
pub type AVCCLDOPD_R = crate::BitReader<bool>;
#[doc = "Field `AVCCLDOPD` writer - AVCC Power-down control signal."]
pub type AVCCLDOPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG8_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - DVCC Voltage adjustment."]
    #[inline(always)]
    pub fn dvccsel(&self) -> DVCCSEL_R {
        DVCCSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6 - Voltage selection of VDD power detector."]
    #[inline(always)]
    pub fn vddpvdsel(&self) -> VDDPVDSEL_R {
        VDDPVDSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - AVCC Power-down control signal."]
    #[inline(always)]
    pub fn avccldopd(&self) -> AVCCLDOPD_R {
        AVCCLDOPD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - DVCC Voltage adjustment."]
    #[inline(always)]
    #[must_use]
    pub fn dvccsel(&mut self) -> DVCCSEL_W<0> {
        DVCCSEL_W::new(self)
    }
    #[doc = "Bits 4:6 - Voltage selection of VDD power detector."]
    #[inline(always)]
    #[must_use]
    pub fn vddpvdsel(&mut self) -> VDDPVDSEL_W<4> {
        VDDPVDSEL_W::new(self)
    }
    #[doc = "Bit 7 - AVCC Power-down control signal."]
    #[inline(always)]
    #[must_use]
    pub fn avccldopd(&mut self) -> AVCCLDOPD_W<7> {
        AVCCLDOPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog register 8.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg8](index.html) module"]
pub struct REG8_SPEC;
impl crate::RegisterSpec for REG8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg8::R](R) reader structure"]
impl crate::Readable for REG8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg8::W](W) writer structure"]
impl crate::Writable for REG8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REG8 to value 0"]
impl crate::Resettable for REG8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
