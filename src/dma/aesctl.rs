#[doc = "Register `AESCTL` reader"]
pub struct R(crate::R<AESCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AESCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AESCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AESCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AESCTL` writer"]
pub struct W(crate::W<AESCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AESCTL_SPEC>;
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
impl From<crate::W<AESCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AESCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENC` reader - AES encode/decode selection register."]
pub type ENC_R = crate::BitReader<bool>;
#[doc = "Field `ENC` writer - AES encode/decode selection register."]
pub type ENC_W<'a, const O: u8> = crate::BitWriter<'a, u32, AESCTL_SPEC, bool, O>;
#[doc = "Field `MODE` reader - AES mode selection register."]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "AES mode selection register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: `0`"]
    AES128 = 0,
    #[doc = "1: `1`"]
    AES192 = 1,
    #[doc = "2: `10`"]
    AES256 = 2,
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
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::AES128,
            1 => MODE_A::AES192,
            2 => MODE_A::AES256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AES128`"]
    #[inline(always)]
    pub fn is_aes128(&self) -> bool {
        *self == MODE_A::AES128
    }
    #[doc = "Checks if the value of the field is `AES192`"]
    #[inline(always)]
    pub fn is_aes192(&self) -> bool {
        *self == MODE_A::AES192
    }
    #[doc = "Checks if the value of the field is `AES256`"]
    #[inline(always)]
    pub fn is_aes256(&self) -> bool {
        *self == MODE_A::AES256
    }
}
#[doc = "Field `MODE` writer - AES mode selection register."]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AESCTL_SPEC, u8, MODE_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn aes128(self) -> &'a mut W {
        self.variant(MODE_A::AES128)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn aes192(self) -> &'a mut W {
        self.variant(MODE_A::AES192)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn aes256(self) -> &'a mut W {
        self.variant(MODE_A::AES256)
    }
}
impl R {
    #[doc = "Bit 0 - AES encode/decode selection register."]
    #[inline(always)]
    pub fn enc(&self) -> ENC_R {
        ENC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - AES mode selection register."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - AES encode/decode selection register."]
    #[inline(always)]
    #[must_use]
    pub fn enc(&mut self) -> ENC_W<0> {
        ENC_W::new(self)
    }
    #[doc = "Bits 2:3 - AES mode selection register."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<2> {
        MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA AES control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesctl](index.html) module"]
pub struct AESCTL_SPEC;
impl crate::RegisterSpec for AESCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aesctl::R](R) reader structure"]
impl crate::Readable for AESCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aesctl::W](W) writer structure"]
impl crate::Writable for AESCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AESCTL to value 0"]
impl crate::Resettable for AESCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
