#[doc = "Register `ACP6` reader"]
pub struct R(crate::R<ACP6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACP6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACP6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACP6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACP6` writer"]
pub struct W(crate::W<ACP6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACP6_SPEC>;
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
impl From<crate::W<ACP6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACP6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P6` reader - The P6 register is used for RTC_DIV calibration."]
pub type P6_R = crate::FieldReader<u16, u16>;
#[doc = "Field `P6` writer - The P6 register is used for RTC_DIV calibration."]
pub type P6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACP6_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - The P6 register is used for RTC_DIV calibration."]
    #[inline(always)]
    pub fn p6(&self) -> P6_R {
        P6_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The P6 register is used for RTC_DIV calibration."]
    #[inline(always)]
    #[must_use]
    pub fn p6(&mut self) -> P6_W<0> {
        P6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC parameter P6 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acp6](index.html) module"]
pub struct ACP6_SPEC;
impl crate::RegisterSpec for ACP6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acp6::R](R) reader structure"]
impl crate::Readable for ACP6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acp6::W](W) writer structure"]
impl crate::Writable for ACP6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACP6 to value 0"]
impl crate::Resettable for ACP6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
