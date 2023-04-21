#[doc = "Register `ACK[%s]` reader"]
pub struct R(crate::R<ACK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACK[%s]` writer"]
pub struct W(crate::W<ACK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACK_SPEC>;
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
impl From<crate::W<ACK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `K` reader - The registers are used for auto-calibration."]
pub type K_R = crate::FieldReader<u16, u16>;
#[doc = "Field `K` writer - The registers are used for auto-calibration."]
pub type K_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACK_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - The registers are used for auto-calibration."]
    #[inline(always)]
    pub fn k(&self) -> K_R {
        K_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The registers are used for auto-calibration."]
    #[inline(always)]
    #[must_use]
    pub fn k(&mut self) -> K_W<0> {
        K_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC auto-calibration parameter Kx control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ack](index.html) module"]
pub struct ACK_SPEC;
impl crate::RegisterSpec for ACK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ack::R](R) reader structure"]
impl crate::Readable for ACK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ack::W](W) writer structure"]
impl crate::Writable for ACK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACK[%s]
to value 0"]
impl crate::Resettable for ACK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
