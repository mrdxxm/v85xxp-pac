#[doc = "Register `CTRL2` reader"]
pub struct R(crate::R<CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL2` writer"]
pub struct W(crate::W<CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL2_SPEC>;
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
impl From<crate::W<CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BKFILL` reader - Fill value at blank period."]
pub type BKFILL_R = crate::BitReader<bool>;
#[doc = "Field `BKFILL` writer - Fill value at blank period."]
pub type BKFILL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL2_SPEC, bool, O>;
#[doc = "Field `FBMODE` reader - LCD frame buffer switch mode control register."]
pub type FBMODE_R = crate::FieldReader<u8, FBMODE_A>;
#[doc = "LCD frame buffer switch mode control register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FBMODE_A {
    #[doc = "0: `0`"]
    BUFFER_A = 0,
    #[doc = "1: `1`"]
    BUFFER_A_AND_BUFFER_B = 1,
    #[doc = "2: `10`"]
    BUFFER_A_AND_BLANK = 2,
}
impl From<FBMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: FBMODE_A) -> Self {
        variant as _
    }
}
impl FBMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FBMODE_A {
        match self.bits {
            0 => FBMODE_A::BUFFER_A,
            1 => FBMODE_A::BUFFER_A_AND_BUFFER_B,
            2 => FBMODE_A::BUFFER_A_AND_BLANK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BUFFER_A`"]
    #[inline(always)]
    pub fn is_buffer_a(&self) -> bool {
        *self == FBMODE_A::BUFFER_A
    }
    #[doc = "Checks if the value of the field is `BUFFER_A_AND_BUFFER_B`"]
    #[inline(always)]
    pub fn is_buffer_a_and_buffer_b(&self) -> bool {
        *self == FBMODE_A::BUFFER_A_AND_BUFFER_B
    }
    #[doc = "Checks if the value of the field is `BUFFER_A_AND_BLANK`"]
    #[inline(always)]
    pub fn is_buffer_a_and_blank(&self) -> bool {
        *self == FBMODE_A::BUFFER_A_AND_BLANK
    }
}
#[doc = "Field `FBMODE` writer - LCD frame buffer switch mode control register."]
pub type FBMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL2_SPEC, u8, FBMODE_A, 2, O>;
impl<'a, const O: u8> FBMODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn buffer_a(self) -> &'a mut W {
        self.variant(FBMODE_A::BUFFER_A)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn buffer_a_and_buffer_b(self) -> &'a mut W {
        self.variant(FBMODE_A::BUFFER_A_AND_BUFFER_B)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn buffer_a_and_blank(self) -> &'a mut W {
        self.variant(FBMODE_A::BUFFER_A_AND_BLANK)
    }
}
#[doc = "Field `SWPR` reader - Frame buffer switch period."]
pub type SWPR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SWPR` writer - Frame buffer switch period."]
pub type SWPR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 4 - Fill value at blank period."]
    #[inline(always)]
    pub fn bkfill(&self) -> BKFILL_R {
        BKFILL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:7 - LCD frame buffer switch mode control register."]
    #[inline(always)]
    pub fn fbmode(&self) -> FBMODE_R {
        FBMODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - Frame buffer switch period."]
    #[inline(always)]
    pub fn swpr(&self) -> SWPR_R {
        SWPR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - Fill value at blank period."]
    #[inline(always)]
    #[must_use]
    pub fn bkfill(&mut self) -> BKFILL_W<4> {
        BKFILL_W::new(self)
    }
    #[doc = "Bits 6:7 - LCD frame buffer switch mode control register."]
    #[inline(always)]
    #[must_use]
    pub fn fbmode(&mut self) -> FBMODE_W<6> {
        FBMODE_W::new(self)
    }
    #[doc = "Bits 8:15 - Frame buffer switch period."]
    #[inline(always)]
    #[must_use]
    pub fn swpr(&mut self) -> SWPR_W<8> {
        SWPR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD control register2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl2](index.html) module"]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl2::R](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl2::W](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
