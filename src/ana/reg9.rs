#[doc = "Register `REG9` reader"]
pub struct R(crate::R<REG9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG9` writer"]
pub struct W(crate::W<REG9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG9_SPEC>;
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
impl From<crate::W<REG9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLLSEL` reader - Frequency selection of PLLL"]
pub type PLLLSEL_R = crate::FieldReader<u8, PLLLSEL_A>;
#[doc = "Frequency selection of PLLL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLLSEL_A {
    #[doc = "0: `0`"]
    _26_2144MHZ = 0,
    #[doc = "1: `1`"]
    _13_1072MHZ = 1,
    #[doc = "2: `10`"]
    _6_5536MHZ = 2,
    #[doc = "3: `11`"]
    _3_2768MHZ = 3,
    #[doc = "4: `100`"]
    _1_6384MHZ = 4,
    #[doc = "5: `101`"]
    _0_8192MHZ = 5,
    #[doc = "6: `110`"]
    _0_4096MHZ = 6,
    #[doc = "7: `111`"]
    _0_2048MHZ = 7,
}
impl From<PLLLSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLLSEL_A) -> Self {
        variant as _
    }
}
impl PLLLSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLLSEL_A {
        match self.bits {
            0 => PLLLSEL_A::_26_2144MHZ,
            1 => PLLLSEL_A::_13_1072MHZ,
            2 => PLLLSEL_A::_6_5536MHZ,
            3 => PLLLSEL_A::_3_2768MHZ,
            4 => PLLLSEL_A::_1_6384MHZ,
            5 => PLLLSEL_A::_0_8192MHZ,
            6 => PLLLSEL_A::_0_4096MHZ,
            7 => PLLLSEL_A::_0_2048MHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_26_2144MHZ`"]
    #[inline(always)]
    pub fn is_26_2144mhz(&self) -> bool {
        *self == PLLLSEL_A::_26_2144MHZ
    }
    #[doc = "Checks if the value of the field is `_13_1072MHZ`"]
    #[inline(always)]
    pub fn is_13_1072mhz(&self) -> bool {
        *self == PLLLSEL_A::_13_1072MHZ
    }
    #[doc = "Checks if the value of the field is `_6_5536MHZ`"]
    #[inline(always)]
    pub fn is_6_5536mhz(&self) -> bool {
        *self == PLLLSEL_A::_6_5536MHZ
    }
    #[doc = "Checks if the value of the field is `_3_2768MHZ`"]
    #[inline(always)]
    pub fn is_3_2768mhz(&self) -> bool {
        *self == PLLLSEL_A::_3_2768MHZ
    }
    #[doc = "Checks if the value of the field is `_1_6384MHZ`"]
    #[inline(always)]
    pub fn is_1_6384mhz(&self) -> bool {
        *self == PLLLSEL_A::_1_6384MHZ
    }
    #[doc = "Checks if the value of the field is `_0_8192MHZ`"]
    #[inline(always)]
    pub fn is_0_8192mhz(&self) -> bool {
        *self == PLLLSEL_A::_0_8192MHZ
    }
    #[doc = "Checks if the value of the field is `_0_4096MHZ`"]
    #[inline(always)]
    pub fn is_0_4096mhz(&self) -> bool {
        *self == PLLLSEL_A::_0_4096MHZ
    }
    #[doc = "Checks if the value of the field is `_0_2048MHZ`"]
    #[inline(always)]
    pub fn is_0_2048mhz(&self) -> bool {
        *self == PLLLSEL_A::_0_2048MHZ
    }
}
#[doc = "Field `PLLLSEL` writer - Frequency selection of PLLL"]
pub type PLLLSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, REG9_SPEC, u8, PLLLSEL_A, 3, O>;
impl<'a, const O: u8> PLLLSEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _26_2144mhz(self) -> &'a mut W {
        self.variant(PLLLSEL_A::_26_2144MHZ)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _13_1072mhz(self) -> &'a mut W {
        self.variant(PLLLSEL_A::_13_1072MHZ)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _6_5536mhz(self) -> &'a mut W {
        self.variant(PLLLSEL_A::_6_5536MHZ)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn _3_2768mhz(self) -> &'a mut W {
        self.variant(PLLLSEL_A::_3_2768MHZ)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn _1_6384mhz(self) -> &'a mut W {
        self.variant(PLLLSEL_A::_1_6384MHZ)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn _0_8192mhz(self) -> &'a mut W {
        self.variant(PLLLSEL_A::_0_8192MHZ)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn _0_4096mhz(self) -> &'a mut W {
        self.variant(PLLLSEL_A::_0_4096MHZ)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn _0_2048mhz(self) -> &'a mut W {
        self.variant(PLLLSEL_A::_0_2048MHZ)
    }
}
#[doc = "Field `PLLHSEL` reader - Frequency selection of PLLH."]
pub type PLLHSEL_R = crate::FieldReader<u8, PLLHSEL_A>;
#[doc = "Frequency selection of PLLH.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLHSEL_A {
    #[doc = "0: `0`"]
    _4X_INPUT_CLOCK = 0,
    #[doc = "1: `1`"]
    _4_5X_INPUT_CLOCK = 1,
    #[doc = "2: `10`"]
    _5X_INPUT_CLOCK = 2,
    #[doc = "3: `11`"]
    _5_5X_INPUT_CLOCK = 3,
    #[doc = "4: `100`"]
    _6X_INPUT_CLOCK = 4,
    #[doc = "5: `101`"]
    _6_5X_INPUT_CLOCK = 5,
    #[doc = "6: `110`"]
    _7X_INPUT_CLOCK = 6,
    #[doc = "7: `111`"]
    _7_5X_INPUT_CLOCK = 7,
    #[doc = "12: `1100`"]
    _2X_INPUT_CLOCK = 12,
    #[doc = "13: `1101`"]
    _2_5X_INPUT_CLOCK = 13,
    #[doc = "14: `1110`"]
    _3X_INPUT_CLOCK = 14,
    #[doc = "15: `1111`"]
    _3_5X_INPUT_CLOCK = 15,
}
impl From<PLLHSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLHSEL_A) -> Self {
        variant as _
    }
}
impl PLLHSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PLLHSEL_A> {
        match self.bits {
            0 => Some(PLLHSEL_A::_4X_INPUT_CLOCK),
            1 => Some(PLLHSEL_A::_4_5X_INPUT_CLOCK),
            2 => Some(PLLHSEL_A::_5X_INPUT_CLOCK),
            3 => Some(PLLHSEL_A::_5_5X_INPUT_CLOCK),
            4 => Some(PLLHSEL_A::_6X_INPUT_CLOCK),
            5 => Some(PLLHSEL_A::_6_5X_INPUT_CLOCK),
            6 => Some(PLLHSEL_A::_7X_INPUT_CLOCK),
            7 => Some(PLLHSEL_A::_7_5X_INPUT_CLOCK),
            12 => Some(PLLHSEL_A::_2X_INPUT_CLOCK),
            13 => Some(PLLHSEL_A::_2_5X_INPUT_CLOCK),
            14 => Some(PLLHSEL_A::_3X_INPUT_CLOCK),
            15 => Some(PLLHSEL_A::_3_5X_INPUT_CLOCK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_4X_INPUT_CLOCK`"]
    #[inline(always)]
    pub fn is_4x_input_clock(&self) -> bool {
        *self == PLLHSEL_A::_4X_INPUT_CLOCK
    }
    #[doc = "Checks if the value of the field is `_4_5X_INPUT_CLOCK`"]
    #[inline(always)]
    pub fn is_4_5x_input_clock(&self) -> bool {
        *self == PLLHSEL_A::_4_5X_INPUT_CLOCK
    }
    #[doc = "Checks if the value of the field is `_5X_INPUT_CLOCK`"]
    #[inline(always)]
    pub fn is_5x_input_clock(&self) -> bool {
        *self == PLLHSEL_A::_5X_INPUT_CLOCK
    }
    #[doc = "Checks if the value of the field is `_5_5X_INPUT_CLOCK`"]
    #[inline(always)]
    pub fn is_5_5x_input_clock(&self) -> bool {
        *self == PLLHSEL_A::_5_5X_INPUT_CLOCK
    }
    #[doc = "Checks if the value of the field is `_6X_INPUT_CLOCK`"]
    #[inline(always)]
    pub fn is_6x_input_clock(&self) -> bool {
        *self == PLLHSEL_A::_6X_INPUT_CLOCK
    }
    #[doc = "Checks if the value of the field is `_6_5X_INPUT_CLOCK`"]
    #[inline(always)]
    pub fn is_6_5x_input_clock(&self) -> bool {
        *self == PLLHSEL_A::_6_5X_INPUT_CLOCK
    }
    #[doc = "Checks if the value of the field is `_7X_INPUT_CLOCK`"]
    #[inline(always)]
    pub fn is_7x_input_clock(&self) -> bool {
        *self == PLLHSEL_A::_7X_INPUT_CLOCK
    }
    #[doc = "Checks if the value of the field is `_7_5X_INPUT_CLOCK`"]
    #[inline(always)]
    pub fn is_7_5x_input_clock(&self) -> bool {
        *self == PLLHSEL_A::_7_5X_INPUT_CLOCK
    }
    #[doc = "Checks if the value of the field is `_2X_INPUT_CLOCK`"]
    #[inline(always)]
    pub fn is_2x_input_clock(&self) -> bool {
        *self == PLLHSEL_A::_2X_INPUT_CLOCK
    }
    #[doc = "Checks if the value of the field is `_2_5X_INPUT_CLOCK`"]
    #[inline(always)]
    pub fn is_2_5x_input_clock(&self) -> bool {
        *self == PLLHSEL_A::_2_5X_INPUT_CLOCK
    }
    #[doc = "Checks if the value of the field is `_3X_INPUT_CLOCK`"]
    #[inline(always)]
    pub fn is_3x_input_clock(&self) -> bool {
        *self == PLLHSEL_A::_3X_INPUT_CLOCK
    }
    #[doc = "Checks if the value of the field is `_3_5X_INPUT_CLOCK`"]
    #[inline(always)]
    pub fn is_3_5x_input_clock(&self) -> bool {
        *self == PLLHSEL_A::_3_5X_INPUT_CLOCK
    }
}
#[doc = "Field `PLLHSEL` writer - Frequency selection of PLLH."]
pub type PLLHSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG9_SPEC, u8, PLLHSEL_A, 4, O>;
impl<'a, const O: u8> PLLHSEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _4x_input_clock(self) -> &'a mut W {
        self.variant(PLLHSEL_A::_4X_INPUT_CLOCK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _4_5x_input_clock(self) -> &'a mut W {
        self.variant(PLLHSEL_A::_4_5X_INPUT_CLOCK)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _5x_input_clock(self) -> &'a mut W {
        self.variant(PLLHSEL_A::_5X_INPUT_CLOCK)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn _5_5x_input_clock(self) -> &'a mut W {
        self.variant(PLLHSEL_A::_5_5X_INPUT_CLOCK)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn _6x_input_clock(self) -> &'a mut W {
        self.variant(PLLHSEL_A::_6X_INPUT_CLOCK)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn _6_5x_input_clock(self) -> &'a mut W {
        self.variant(PLLHSEL_A::_6_5X_INPUT_CLOCK)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn _7x_input_clock(self) -> &'a mut W {
        self.variant(PLLHSEL_A::_7X_INPUT_CLOCK)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn _7_5x_input_clock(self) -> &'a mut W {
        self.variant(PLLHSEL_A::_7_5X_INPUT_CLOCK)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn _2x_input_clock(self) -> &'a mut W {
        self.variant(PLLHSEL_A::_2X_INPUT_CLOCK)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn _2_5x_input_clock(self) -> &'a mut W {
        self.variant(PLLHSEL_A::_2_5X_INPUT_CLOCK)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn _3x_input_clock(self) -> &'a mut W {
        self.variant(PLLHSEL_A::_3X_INPUT_CLOCK)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn _3_5x_input_clock(self) -> &'a mut W {
        self.variant(PLLHSEL_A::_3_5X_INPUT_CLOCK)
    }
}
#[doc = "Field `VDDDETPD` reader - Power down VDD input POWALARM detector."]
pub type VDDDETPD_R = crate::BitReader<bool>;
#[doc = "Field `VDDDETPD` writer - Power down VDD input POWALARM detector."]
pub type VDDDETPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG9_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Frequency selection of PLLL"]
    #[inline(always)]
    pub fn plllsel(&self) -> PLLLSEL_R {
        PLLLSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:6 - Frequency selection of PLLH."]
    #[inline(always)]
    pub fn pllhsel(&self) -> PLLHSEL_R {
        PLLHSEL_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Power down VDD input POWALARM detector."]
    #[inline(always)]
    pub fn vdddetpd(&self) -> VDDDETPD_R {
        VDDDETPD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Frequency selection of PLLL"]
    #[inline(always)]
    #[must_use]
    pub fn plllsel(&mut self) -> PLLLSEL_W<0> {
        PLLLSEL_W::new(self)
    }
    #[doc = "Bits 3:6 - Frequency selection of PLLH."]
    #[inline(always)]
    #[must_use]
    pub fn pllhsel(&mut self) -> PLLHSEL_W<3> {
        PLLHSEL_W::new(self)
    }
    #[doc = "Bit 7 - Power down VDD input POWALARM detector."]
    #[inline(always)]
    #[must_use]
    pub fn vdddetpd(&mut self) -> VDDDETPD_W<7> {
        VDDDETPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog register 9.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg9](index.html) module"]
pub struct REG9_SPEC;
impl crate::RegisterSpec for REG9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg9::R](R) reader structure"]
impl crate::Readable for REG9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg9::W](W) writer structure"]
impl crate::Writable for REG9_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REG9 to value 0"]
impl crate::Resettable for REG9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
