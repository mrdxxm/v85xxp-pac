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
#[doc = "Field `ACT` reader - Write 1 to this bit will start an operation specified in the MODE register."]
pub type ACT_R = crate::BitReader<bool>;
#[doc = "Field `ACT` writer - Write 1 to this bit will start an operation specified in the MODE register."]
pub type ACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `MODE` reader - This trgister controls the operation mode of crypt engine ."]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "This trgister controls the operation mode of crypt engine .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: `0`"]
    MULTIPLY_MODE = 0,
    #[doc = "1: `1`"]
    ADD_MODE = 1,
    #[doc = "2: `10`"]
    SUB_MODE = 2,
    #[doc = "3: `11`"]
    RSHIFT1_MODE = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::MULTIPLY_MODE),
            1 => Some(MODE_A::ADD_MODE),
            2 => Some(MODE_A::SUB_MODE),
            3 => Some(MODE_A::RSHIFT1_MODE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MULTIPLY_MODE`"]
    #[inline(always)]
    pub fn is_multiply_mode(&self) -> bool {
        *self == MODE_A::MULTIPLY_MODE
    }
    #[doc = "Checks if the value of the field is `ADD_MODE`"]
    #[inline(always)]
    pub fn is_add_mode(&self) -> bool {
        *self == MODE_A::ADD_MODE
    }
    #[doc = "Checks if the value of the field is `SUB_MODE`"]
    #[inline(always)]
    pub fn is_sub_mode(&self) -> bool {
        *self == MODE_A::SUB_MODE
    }
    #[doc = "Checks if the value of the field is `RSHIFT1_MODE`"]
    #[inline(always)]
    pub fn is_rshift1_mode(&self) -> bool {
        *self == MODE_A::RSHIFT1_MODE
    }
}
#[doc = "Field `MODE` writer - This trgister controls the operation mode of crypt engine ."]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, MODE_A, 3, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn multiply_mode(self) -> &'a mut W {
        self.variant(MODE_A::MULTIPLY_MODE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn add_mode(self) -> &'a mut W {
        self.variant(MODE_A::ADD_MODE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sub_mode(self) -> &'a mut W {
        self.variant(MODE_A::SUB_MODE)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn rshift1_mode(self) -> &'a mut W {
        self.variant(MODE_A::RSHIFT1_MODE)
    }
}
#[doc = "Field `LENGTH` reader - This bit is used to control the VLI length of current operation."]
pub type LENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LENGTH` writer - This bit is used to control the VLI length of current operation."]
pub type LENGTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `NOSTOP` reader - This register is used to control if the cpu will be stop by CRYPT engine when the CRYPT engine is busy and CPU read or write CRPYT engine register."]
pub type NOSTOP_R = crate::BitReader<bool>;
#[doc = "Field `NOSTOP` writer - This register is used to control if the cpu will be stop by CRYPT engine when the CRYPT engine is busy and CPU read or write CRPYT engine register."]
pub type NOSTOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Write 1 to this bit will start an operation specified in the MODE register."]
    #[inline(always)]
    pub fn act(&self) -> ACT_R {
        ACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - This trgister controls the operation mode of crypt engine ."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:11 - This bit is used to control the VLI length of current operation."]
    #[inline(always)]
    pub fn length(&self) -> LENGTH_R {
        LENGTH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - This register is used to control if the cpu will be stop by CRYPT engine when the CRYPT engine is busy and CPU read or write CRPYT engine register."]
    #[inline(always)]
    pub fn nostop(&self) -> NOSTOP_R {
        NOSTOP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to this bit will start an operation specified in the MODE register."]
    #[inline(always)]
    #[must_use]
    pub fn act(&mut self) -> ACT_W<0> {
        ACT_W::new(self)
    }
    #[doc = "Bits 4:6 - This trgister controls the operation mode of crypt engine ."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<4> {
        MODE_W::new(self)
    }
    #[doc = "Bits 8:11 - This bit is used to control the VLI length of current operation."]
    #[inline(always)]
    #[must_use]
    pub fn length(&mut self) -> LENGTH_W<8> {
        LENGTH_W::new(self)
    }
    #[doc = "Bit 15 - This register is used to control if the cpu will be stop by CRYPT engine when the CRYPT engine is busy and CPU read or write CRPYT engine register."]
    #[inline(always)]
    #[must_use]
    pub fn nostop(&mut self) -> NOSTOP_W<15> {
        NOSTOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRYPT control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
