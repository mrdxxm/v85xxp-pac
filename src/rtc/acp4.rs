#[doc = "Register `ACP4` reader"]
pub struct R(crate::R<ACP4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACP4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACP4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACP4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACP4` writer"]
pub struct W(crate::W<ACP4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACP4_SPEC>;
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
impl From<crate::W<ACP4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACP4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P4` reader - The P4 register is the crystal temperature offset."]
pub type P4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `P4` writer - The P4 register is the crystal temperature offset."]
pub type P4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACP4_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - The P4 register is the crystal temperature offset."]
    #[inline(always)]
    pub fn p4(&self) -> P4_R {
        P4_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The P4 register is the crystal temperature offset."]
    #[inline(always)]
    #[must_use]
    pub fn p4(&mut self) -> P4_W<0> {
        P4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC parameter P4 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acp4](index.html) module"]
pub struct ACP4_SPEC;
impl crate::RegisterSpec for ACP4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acp4::R](R) reader structure"]
impl crate::Readable for ACP4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acp4::W](W) writer structure"]
impl crate::Writable for ACP4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACP4 to value 0"]
impl crate::Resettable for ACP4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
