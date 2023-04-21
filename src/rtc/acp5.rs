#[doc = "Register `ACP5` reader"]
pub struct R(crate::R<ACP5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACP5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACP5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACP5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACP5` writer"]
pub struct W(crate::W<ACP5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACP5_SPEC>;
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
impl From<crate::W<ACP5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACP5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P5` reader - The P5 register is used for RTC adjustment."]
pub type P5_R = crate::FieldReader<u16, u16>;
#[doc = "Field `P5` writer - The P5 register is used for RTC adjustment."]
pub type P5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACP5_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - The P5 register is used for RTC adjustment."]
    #[inline(always)]
    pub fn p5(&self) -> P5_R {
        P5_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The P5 register is used for RTC adjustment."]
    #[inline(always)]
    #[must_use]
    pub fn p5(&mut self) -> P5_W<0> {
        P5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC parameter P5 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acp5](index.html) module"]
pub struct ACP5_SPEC;
impl crate::RegisterSpec for ACP5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acp5::R](R) reader structure"]
impl crate::Readable for ACP5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acp5::W](W) writer structure"]
impl crate::Writable for ACP5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACP5 to value 0"]
impl crate::Resettable for ACP5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
