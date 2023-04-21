#[doc = "Register `WDTEN` reader"]
pub struct R(crate::R<WDTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTEN` writer"]
pub struct W(crate::W<WDTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTEN_SPEC>;
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
impl From<crate::W<WDTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTEN` reader - This bit indicates the watch dog timer is enable."]
pub type WDTEN_R = crate::BitReader<bool>;
#[doc = "Field `WDTEN` writer - This bit indicates the watch dog timer is enable."]
pub type WDTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WDTEN_SPEC, bool, O>;
#[doc = "Field `WDTSEL` reader - This register is used to control the WDT counting period."]
pub type WDTSEL_R = crate::FieldReader<u8, WDTSEL_A>;
#[doc = "This register is used to control the WDT counting period.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDTSEL_A {
    #[doc = "0: `0`"]
    _2_SECOND = 0,
    #[doc = "1: `1`"]
    _4_SECOND = 1,
    #[doc = "2: `10`"]
    _8_SECOND = 2,
    #[doc = "3: `11`"]
    _16_SECOND = 3,
}
impl From<WDTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: WDTSEL_A) -> Self {
        variant as _
    }
}
impl WDTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTSEL_A {
        match self.bits {
            0 => WDTSEL_A::_2_SECOND,
            1 => WDTSEL_A::_4_SECOND,
            2 => WDTSEL_A::_8_SECOND,
            3 => WDTSEL_A::_16_SECOND,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_2_SECOND`"]
    #[inline(always)]
    pub fn is_2_second(&self) -> bool {
        *self == WDTSEL_A::_2_SECOND
    }
    #[doc = "Checks if the value of the field is `_4_SECOND`"]
    #[inline(always)]
    pub fn is_4_second(&self) -> bool {
        *self == WDTSEL_A::_4_SECOND
    }
    #[doc = "Checks if the value of the field is `_8_SECOND`"]
    #[inline(always)]
    pub fn is_8_second(&self) -> bool {
        *self == WDTSEL_A::_8_SECOND
    }
    #[doc = "Checks if the value of the field is `_16_SECOND`"]
    #[inline(always)]
    pub fn is_16_second(&self) -> bool {
        *self == WDTSEL_A::_16_SECOND
    }
}
#[doc = "Field `WDTSEL` writer - This register is used to control the WDT counting period."]
pub type WDTSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, WDTEN_SPEC, u8, WDTSEL_A, 2, O>;
impl<'a, const O: u8> WDTSEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _2_second(self) -> &'a mut W {
        self.variant(WDTSEL_A::_2_SECOND)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _4_second(self) -> &'a mut W {
        self.variant(WDTSEL_A::_4_SECOND)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _8_second(self) -> &'a mut W {
        self.variant(WDTSEL_A::_8_SECOND)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn _16_second(self) -> &'a mut W {
        self.variant(WDTSEL_A::_16_SECOND)
    }
}
impl R {
    #[doc = "Bit 0 - This bit indicates the watch dog timer is enable."]
    #[inline(always)]
    pub fn wdten(&self) -> WDTEN_R {
        WDTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - This register is used to control the WDT counting period."]
    #[inline(always)]
    pub fn wdtsel(&self) -> WDTSEL_R {
        WDTSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - This bit indicates the watch dog timer is enable."]
    #[inline(always)]
    #[must_use]
    pub fn wdten(&mut self) -> WDTEN_W<0> {
        WDTEN_W::new(self)
    }
    #[doc = "Bits 2:3 - This register is used to control the WDT counting period."]
    #[inline(always)]
    #[must_use]
    pub fn wdtsel(&mut self) -> WDTSEL_W<2> {
        WDTSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watch dog timer enable register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdten](index.html) module"]
pub struct WDTEN_SPEC;
impl crate::RegisterSpec for WDTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdten::R](R) reader structure"]
impl crate::Readable for WDTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdten::W](W) writer structure"]
impl crate::Writable for WDTEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDTEN to value 0x01"]
impl crate::Resettable for WDTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
