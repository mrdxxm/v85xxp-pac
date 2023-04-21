#[doc = "Register `DUTYH` reader"]
pub struct R(crate::R<DUTYH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DUTYH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DUTYH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DUTYH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DUTYH` writer"]
pub struct W(crate::W<DUTYH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DUTYH_SPEC>;
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
impl From<crate::W<DUTYH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DUTYH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUTYH` reader - IR high pulse width control register."]
pub type DUTYH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DUTYH` writer - IR high pulse width control register."]
pub type DUTYH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DUTYH_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - IR high pulse width control register."]
    #[inline(always)]
    pub fn dutyh(&self) -> DUTYH_R {
        DUTYH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IR high pulse width control register."]
    #[inline(always)]
    #[must_use]
    pub fn dutyh(&mut self) -> DUTYH_W<0> {
        DUTYH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IR Duty high pulse control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dutyh](index.html) module"]
pub struct DUTYH_SPEC;
impl crate::RegisterSpec for DUTYH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dutyh::R](R) reader structure"]
impl crate::Readable for DUTYH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dutyh::W](W) writer structure"]
impl crate::Writable for DUTYH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DUTYH to value 0"]
impl crate::Resettable for DUTYH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
