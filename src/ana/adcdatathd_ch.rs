#[doc = "Register `ADCDATATHD_CH` reader"]
pub struct R(crate::R<ADCDATATHD_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCDATATHD_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCDATATHD_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCDATATHD_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCDATATHD_CH` writer"]
pub struct W(crate::W<ADCDATATHD_CH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCDATATHD_CH_SPEC>;
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
impl From<crate::W<ADCDATATHD_CH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCDATATHD_CH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THD0_CH` reader - The channel number(0~15) to enable LOWER_THD0 and UPPER_THD0."]
pub type THD0_CH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `THD0_CH` writer - The channel number(0~15) to enable LOWER_THD0 and UPPER_THD0."]
pub type THD0_CH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCDATATHD_CH_SPEC, u8, u8, 4, O>;
#[doc = "Field `THD1_CH` reader - The channel number(0~15) to enable LOWER_THD1 and UPPER_THD1."]
pub type THD1_CH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `THD1_CH` writer - The channel number(0~15) to enable LOWER_THD1 and UPPER_THD1."]
pub type THD1_CH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCDATATHD_CH_SPEC, u8, u8, 4, O>;
#[doc = "Field `THD2_CH` reader - The channel number(0~15) to enable LOWER_THD2 and UPPER_THD2."]
pub type THD2_CH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `THD2_CH` writer - The channel number(0~15) to enable LOWER_THD2 and UPPER_THD2."]
pub type THD2_CH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCDATATHD_CH_SPEC, u8, u8, 4, O>;
#[doc = "Field `THD3_CH` reader - The channel number(0~15) to enable LOWER_THD3 and UPPER_THD3."]
pub type THD3_CH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `THD3_CH` writer - The channel number(0~15) to enable LOWER_THD3 and UPPER_THD3."]
pub type THD3_CH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCDATATHD_CH_SPEC, u8, u8, 4, O>;
#[doc = "Field `THD0_SEL` reader - This register is used to control the interrupt signal generation of UPPER_THD0 and LOWER_THD0."]
pub type THD0_SEL_R = crate::FieldReader<u8, THD0_SEL_A>;
#[doc = "This register is used to control the interrupt signal generation of UPPER_THD0 and LOWER_THD0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum THD0_SEL_A {
    #[doc = "0: `0`"]
    HIGH_LEVEL = 0,
    #[doc = "1: `1`"]
    RISING_EDGE = 1,
    #[doc = "2: `10`"]
    FALLING_EDGE = 2,
    #[doc = "3: `11`"]
    BOTH_OF_EDGE = 3,
}
impl From<THD0_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: THD0_SEL_A) -> Self {
        variant as _
    }
}
impl THD0_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THD0_SEL_A {
        match self.bits {
            0 => THD0_SEL_A::HIGH_LEVEL,
            1 => THD0_SEL_A::RISING_EDGE,
            2 => THD0_SEL_A::FALLING_EDGE,
            3 => THD0_SEL_A::BOTH_OF_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == THD0_SEL_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == THD0_SEL_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == THD0_SEL_A::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `BOTH_OF_EDGE`"]
    #[inline(always)]
    pub fn is_both_of_edge(&self) -> bool {
        *self == THD0_SEL_A::BOTH_OF_EDGE
    }
}
#[doc = "Field `THD0_SEL` writer - This register is used to control the interrupt signal generation of UPPER_THD0 and LOWER_THD0."]
pub type THD0_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ADCDATATHD_CH_SPEC, u8, THD0_SEL_A, 2, O>;
impl<'a, const O: u8> THD0_SEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(THD0_SEL_A::HIGH_LEVEL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(THD0_SEL_A::RISING_EDGE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(THD0_SEL_A::FALLING_EDGE)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn both_of_edge(self) -> &'a mut W {
        self.variant(THD0_SEL_A::BOTH_OF_EDGE)
    }
}
#[doc = "Field `THD1_SEL` reader - This register is used to control the interrupt signal generation of UPPER_THD1 and LOWER_THD1."]
pub type THD1_SEL_R = crate::FieldReader<u8, THD1_SEL_A>;
#[doc = "This register is used to control the interrupt signal generation of UPPER_THD1 and LOWER_THD1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum THD1_SEL_A {
    #[doc = "0: `0`"]
    HIGH_LEVEL = 0,
    #[doc = "1: `1`"]
    RISING_EDGE = 1,
    #[doc = "2: `10`"]
    FALLING_EDGE = 2,
    #[doc = "3: `11`"]
    BOTH_OF_EDGE = 3,
}
impl From<THD1_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: THD1_SEL_A) -> Self {
        variant as _
    }
}
impl THD1_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THD1_SEL_A {
        match self.bits {
            0 => THD1_SEL_A::HIGH_LEVEL,
            1 => THD1_SEL_A::RISING_EDGE,
            2 => THD1_SEL_A::FALLING_EDGE,
            3 => THD1_SEL_A::BOTH_OF_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == THD1_SEL_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == THD1_SEL_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == THD1_SEL_A::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `BOTH_OF_EDGE`"]
    #[inline(always)]
    pub fn is_both_of_edge(&self) -> bool {
        *self == THD1_SEL_A::BOTH_OF_EDGE
    }
}
#[doc = "Field `THD1_SEL` writer - This register is used to control the interrupt signal generation of UPPER_THD1 and LOWER_THD1."]
pub type THD1_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ADCDATATHD_CH_SPEC, u8, THD1_SEL_A, 2, O>;
impl<'a, const O: u8> THD1_SEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(THD1_SEL_A::HIGH_LEVEL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(THD1_SEL_A::RISING_EDGE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(THD1_SEL_A::FALLING_EDGE)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn both_of_edge(self) -> &'a mut W {
        self.variant(THD1_SEL_A::BOTH_OF_EDGE)
    }
}
#[doc = "Field `THD2_SEL` reader - This register is used to control the interrupt signal generation of UPPER_THD2 and LOWER_THD2."]
pub type THD2_SEL_R = crate::FieldReader<u8, THD2_SEL_A>;
#[doc = "This register is used to control the interrupt signal generation of UPPER_THD2 and LOWER_THD2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum THD2_SEL_A {
    #[doc = "0: `0`"]
    HIGH_LEVEL = 0,
    #[doc = "1: `1`"]
    RISING_EDGE = 1,
    #[doc = "2: `10`"]
    FALLING_EDGE = 2,
    #[doc = "3: `11`"]
    BOTH_OF_EDGE = 3,
}
impl From<THD2_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: THD2_SEL_A) -> Self {
        variant as _
    }
}
impl THD2_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THD2_SEL_A {
        match self.bits {
            0 => THD2_SEL_A::HIGH_LEVEL,
            1 => THD2_SEL_A::RISING_EDGE,
            2 => THD2_SEL_A::FALLING_EDGE,
            3 => THD2_SEL_A::BOTH_OF_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == THD2_SEL_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == THD2_SEL_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == THD2_SEL_A::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `BOTH_OF_EDGE`"]
    #[inline(always)]
    pub fn is_both_of_edge(&self) -> bool {
        *self == THD2_SEL_A::BOTH_OF_EDGE
    }
}
#[doc = "Field `THD2_SEL` writer - This register is used to control the interrupt signal generation of UPPER_THD2 and LOWER_THD2."]
pub type THD2_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ADCDATATHD_CH_SPEC, u8, THD2_SEL_A, 2, O>;
impl<'a, const O: u8> THD2_SEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(THD2_SEL_A::HIGH_LEVEL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(THD2_SEL_A::RISING_EDGE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(THD2_SEL_A::FALLING_EDGE)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn both_of_edge(self) -> &'a mut W {
        self.variant(THD2_SEL_A::BOTH_OF_EDGE)
    }
}
#[doc = "Field `THD3_SEL` reader - This register is used to control the interrupt signal generation of UPPER_THD3 and LOWER_THD3."]
pub type THD3_SEL_R = crate::FieldReader<u8, THD3_SEL_A>;
#[doc = "This register is used to control the interrupt signal generation of UPPER_THD3 and LOWER_THD3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum THD3_SEL_A {
    #[doc = "0: `0`"]
    HIGH_LEVEL = 0,
    #[doc = "1: `1`"]
    RISING_EDGE = 1,
    #[doc = "2: `10`"]
    FALLING_EDGE = 2,
    #[doc = "3: `11`"]
    BOTH_OF_EDGE = 3,
}
impl From<THD3_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: THD3_SEL_A) -> Self {
        variant as _
    }
}
impl THD3_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THD3_SEL_A {
        match self.bits {
            0 => THD3_SEL_A::HIGH_LEVEL,
            1 => THD3_SEL_A::RISING_EDGE,
            2 => THD3_SEL_A::FALLING_EDGE,
            3 => THD3_SEL_A::BOTH_OF_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == THD3_SEL_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == THD3_SEL_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == THD3_SEL_A::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `BOTH_OF_EDGE`"]
    #[inline(always)]
    pub fn is_both_of_edge(&self) -> bool {
        *self == THD3_SEL_A::BOTH_OF_EDGE
    }
}
#[doc = "Field `THD3_SEL` writer - This register is used to control the interrupt signal generation of UPPER_THD3 and LOWER_THD3."]
pub type THD3_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ADCDATATHD_CH_SPEC, u8, THD3_SEL_A, 2, O>;
impl<'a, const O: u8> THD3_SEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(THD3_SEL_A::HIGH_LEVEL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(THD3_SEL_A::RISING_EDGE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(THD3_SEL_A::FALLING_EDGE)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn both_of_edge(self) -> &'a mut W {
        self.variant(THD3_SEL_A::BOTH_OF_EDGE)
    }
}
#[doc = "Field `LOWER_THD0_TRGED` reader - This register is to reflect LOWER_THD0 triggered status."]
pub type LOWER_THD0_TRGED_R = crate::BitReader<bool>;
#[doc = "Field `LOWER_THD0_TRGED` writer - This register is to reflect LOWER_THD0 triggered status."]
pub type LOWER_THD0_TRGED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADCDATATHD_CH_SPEC, bool, O>;
#[doc = "Field `UPPER_THD0_TRGED` reader - This register is to reflect UPPER_THD0 triggered status."]
pub type UPPER_THD0_TRGED_R = crate::BitReader<bool>;
#[doc = "Field `UPPER_THD0_TRGED` writer - This register is to reflect UPPER_THD0 triggered status."]
pub type UPPER_THD0_TRGED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADCDATATHD_CH_SPEC, bool, O>;
#[doc = "Field `LOWER_THD1_TRGED` reader - This register is to reflect LOWER_THD1 triggered status."]
pub type LOWER_THD1_TRGED_R = crate::BitReader<bool>;
#[doc = "Field `LOWER_THD1_TRGED` writer - This register is to reflect LOWER_THD1 triggered status."]
pub type LOWER_THD1_TRGED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADCDATATHD_CH_SPEC, bool, O>;
#[doc = "Field `UPPER_THD1_TRGED` reader - This register is to reflect UPPER_THD1 triggered status."]
pub type UPPER_THD1_TRGED_R = crate::BitReader<bool>;
#[doc = "Field `UPPER_THD1_TRGED` writer - This register is to reflect UPPER_THD1 triggered status."]
pub type UPPER_THD1_TRGED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADCDATATHD_CH_SPEC, bool, O>;
#[doc = "Field `LOWER_THD2_TRGED` reader - This register is to reflect LOWER_THD2 triggered status."]
pub type LOWER_THD2_TRGED_R = crate::BitReader<bool>;
#[doc = "Field `LOWER_THD2_TRGED` writer - This register is to reflect LOWER_THD2 triggered status."]
pub type LOWER_THD2_TRGED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADCDATATHD_CH_SPEC, bool, O>;
#[doc = "Field `UPPER_THD2_TRGED` reader - This register is to reflect UPPER_THD2 triggered status."]
pub type UPPER_THD2_TRGED_R = crate::BitReader<bool>;
#[doc = "Field `UPPER_THD2_TRGED` writer - This register is to reflect UPPER_THD2 triggered status."]
pub type UPPER_THD2_TRGED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADCDATATHD_CH_SPEC, bool, O>;
#[doc = "Field `LOWER_THD3_TRGED` reader - This register is to reflect LOWER_THD3 triggered status."]
pub type LOWER_THD3_TRGED_R = crate::BitReader<bool>;
#[doc = "Field `LOWER_THD3_TRGED` writer - This register is to reflect LOWER_THD3 triggered status."]
pub type LOWER_THD3_TRGED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADCDATATHD_CH_SPEC, bool, O>;
#[doc = "Field `UPPER_THD3_TRGED` reader - This register is to reflect UPPER_THD3 triggered status."]
pub type UPPER_THD3_TRGED_R = crate::BitReader<bool>;
#[doc = "Field `UPPER_THD3_TRGED` writer - This register is to reflect UPPER_THD3 triggered status."]
pub type UPPER_THD3_TRGED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADCDATATHD_CH_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - The channel number(0~15) to enable LOWER_THD0 and UPPER_THD0."]
    #[inline(always)]
    pub fn thd0_ch(&self) -> THD0_CH_R {
        THD0_CH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - The channel number(0~15) to enable LOWER_THD1 and UPPER_THD1."]
    #[inline(always)]
    pub fn thd1_ch(&self) -> THD1_CH_R {
        THD1_CH_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - The channel number(0~15) to enable LOWER_THD2 and UPPER_THD2."]
    #[inline(always)]
    pub fn thd2_ch(&self) -> THD2_CH_R {
        THD2_CH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - The channel number(0~15) to enable LOWER_THD3 and UPPER_THD3."]
    #[inline(always)]
    pub fn thd3_ch(&self) -> THD3_CH_R {
        THD3_CH_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - This register is used to control the interrupt signal generation of UPPER_THD0 and LOWER_THD0."]
    #[inline(always)]
    pub fn thd0_sel(&self) -> THD0_SEL_R {
        THD0_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - This register is used to control the interrupt signal generation of UPPER_THD1 and LOWER_THD1."]
    #[inline(always)]
    pub fn thd1_sel(&self) -> THD1_SEL_R {
        THD1_SEL_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - This register is used to control the interrupt signal generation of UPPER_THD2 and LOWER_THD2."]
    #[inline(always)]
    pub fn thd2_sel(&self) -> THD2_SEL_R {
        THD2_SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - This register is used to control the interrupt signal generation of UPPER_THD3 and LOWER_THD3."]
    #[inline(always)]
    pub fn thd3_sel(&self) -> THD3_SEL_R {
        THD3_SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - This register is to reflect LOWER_THD0 triggered status."]
    #[inline(always)]
    pub fn lower_thd0_trged(&self) -> LOWER_THD0_TRGED_R {
        LOWER_THD0_TRGED_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - This register is to reflect UPPER_THD0 triggered status."]
    #[inline(always)]
    pub fn upper_thd0_trged(&self) -> UPPER_THD0_TRGED_R {
        UPPER_THD0_TRGED_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - This register is to reflect LOWER_THD1 triggered status."]
    #[inline(always)]
    pub fn lower_thd1_trged(&self) -> LOWER_THD1_TRGED_R {
        LOWER_THD1_TRGED_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - This register is to reflect UPPER_THD1 triggered status."]
    #[inline(always)]
    pub fn upper_thd1_trged(&self) -> UPPER_THD1_TRGED_R {
        UPPER_THD1_TRGED_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - This register is to reflect LOWER_THD2 triggered status."]
    #[inline(always)]
    pub fn lower_thd2_trged(&self) -> LOWER_THD2_TRGED_R {
        LOWER_THD2_TRGED_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - This register is to reflect UPPER_THD2 triggered status."]
    #[inline(always)]
    pub fn upper_thd2_trged(&self) -> UPPER_THD2_TRGED_R {
        UPPER_THD2_TRGED_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - This register is to reflect LOWER_THD3 triggered status."]
    #[inline(always)]
    pub fn lower_thd3_trged(&self) -> LOWER_THD3_TRGED_R {
        LOWER_THD3_TRGED_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This register is to reflect UPPER_THD3 triggered status."]
    #[inline(always)]
    pub fn upper_thd3_trged(&self) -> UPPER_THD3_TRGED_R {
        UPPER_THD3_TRGED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - The channel number(0~15) to enable LOWER_THD0 and UPPER_THD0."]
    #[inline(always)]
    #[must_use]
    pub fn thd0_ch(&mut self) -> THD0_CH_W<0> {
        THD0_CH_W::new(self)
    }
    #[doc = "Bits 4:7 - The channel number(0~15) to enable LOWER_THD1 and UPPER_THD1."]
    #[inline(always)]
    #[must_use]
    pub fn thd1_ch(&mut self) -> THD1_CH_W<4> {
        THD1_CH_W::new(self)
    }
    #[doc = "Bits 8:11 - The channel number(0~15) to enable LOWER_THD2 and UPPER_THD2."]
    #[inline(always)]
    #[must_use]
    pub fn thd2_ch(&mut self) -> THD2_CH_W<8> {
        THD2_CH_W::new(self)
    }
    #[doc = "Bits 12:15 - The channel number(0~15) to enable LOWER_THD3 and UPPER_THD3."]
    #[inline(always)]
    #[must_use]
    pub fn thd3_ch(&mut self) -> THD3_CH_W<12> {
        THD3_CH_W::new(self)
    }
    #[doc = "Bits 16:17 - This register is used to control the interrupt signal generation of UPPER_THD0 and LOWER_THD0."]
    #[inline(always)]
    #[must_use]
    pub fn thd0_sel(&mut self) -> THD0_SEL_W<16> {
        THD0_SEL_W::new(self)
    }
    #[doc = "Bits 18:19 - This register is used to control the interrupt signal generation of UPPER_THD1 and LOWER_THD1."]
    #[inline(always)]
    #[must_use]
    pub fn thd1_sel(&mut self) -> THD1_SEL_W<18> {
        THD1_SEL_W::new(self)
    }
    #[doc = "Bits 20:21 - This register is used to control the interrupt signal generation of UPPER_THD2 and LOWER_THD2."]
    #[inline(always)]
    #[must_use]
    pub fn thd2_sel(&mut self) -> THD2_SEL_W<20> {
        THD2_SEL_W::new(self)
    }
    #[doc = "Bits 22:23 - This register is used to control the interrupt signal generation of UPPER_THD3 and LOWER_THD3."]
    #[inline(always)]
    #[must_use]
    pub fn thd3_sel(&mut self) -> THD3_SEL_W<22> {
        THD3_SEL_W::new(self)
    }
    #[doc = "Bit 24 - This register is to reflect LOWER_THD0 triggered status."]
    #[inline(always)]
    #[must_use]
    pub fn lower_thd0_trged(&mut self) -> LOWER_THD0_TRGED_W<24> {
        LOWER_THD0_TRGED_W::new(self)
    }
    #[doc = "Bit 25 - This register is to reflect UPPER_THD0 triggered status."]
    #[inline(always)]
    #[must_use]
    pub fn upper_thd0_trged(&mut self) -> UPPER_THD0_TRGED_W<25> {
        UPPER_THD0_TRGED_W::new(self)
    }
    #[doc = "Bit 26 - This register is to reflect LOWER_THD1 triggered status."]
    #[inline(always)]
    #[must_use]
    pub fn lower_thd1_trged(&mut self) -> LOWER_THD1_TRGED_W<26> {
        LOWER_THD1_TRGED_W::new(self)
    }
    #[doc = "Bit 27 - This register is to reflect UPPER_THD1 triggered status."]
    #[inline(always)]
    #[must_use]
    pub fn upper_thd1_trged(&mut self) -> UPPER_THD1_TRGED_W<27> {
        UPPER_THD1_TRGED_W::new(self)
    }
    #[doc = "Bit 28 - This register is to reflect LOWER_THD2 triggered status."]
    #[inline(always)]
    #[must_use]
    pub fn lower_thd2_trged(&mut self) -> LOWER_THD2_TRGED_W<28> {
        LOWER_THD2_TRGED_W::new(self)
    }
    #[doc = "Bit 29 - This register is to reflect UPPER_THD2 triggered status."]
    #[inline(always)]
    #[must_use]
    pub fn upper_thd2_trged(&mut self) -> UPPER_THD2_TRGED_W<29> {
        UPPER_THD2_TRGED_W::new(self)
    }
    #[doc = "Bit 30 - This register is to reflect LOWER_THD3 triggered status."]
    #[inline(always)]
    #[must_use]
    pub fn lower_thd3_trged(&mut self) -> LOWER_THD3_TRGED_W<30> {
        LOWER_THD3_TRGED_W::new(self)
    }
    #[doc = "Bit 31 - This register is to reflect UPPER_THD3 triggered status."]
    #[inline(always)]
    #[must_use]
    pub fn upper_thd3_trged(&mut self) -> UPPER_THD3_TRGED_W<31> {
        UPPER_THD3_TRGED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ANA_ADCDATATHD_CH.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcdatathd_ch](index.html) module"]
pub struct ADCDATATHD_CH_SPEC;
impl crate::RegisterSpec for ADCDATATHD_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcdatathd_ch::R](R) reader structure"]
impl crate::Readable for ADCDATATHD_CH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcdatathd_ch::W](W) writer structure"]
impl crate::Writable for ADCDATATHD_CH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCDATATHD_CH to value 0"]
impl crate::Resettable for ADCDATATHD_CH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
