#[doc = "Register `ACF200` reader"]
pub struct R(crate::R<ACF200_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACF200_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACF200_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACF200_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACF200` writer"]
pub struct W(crate::W<ACF200_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACF200_SPEC>;
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
impl From<crate::W<ACF200_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACF200_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `F200` reader - Auto-calibration F200 control register."]
pub type F200_R = crate::FieldReader<u32, u32>;
#[doc = "Field `F200` writer - Auto-calibration F200 control register."]
pub type F200_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACF200_SPEC, u32, u32, 26, O>;
impl R {
    #[doc = "Bits 0:25 - Auto-calibration F200 control register."]
    #[inline(always)]
    pub fn f200(&self) -> F200_R {
        F200_R::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - Auto-calibration F200 control register."]
    #[inline(always)]
    #[must_use]
    pub fn f200(&mut self) -> F200_W<0> {
        F200_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC auto-calibration 200*frequency control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acf200](index.html) module"]
pub struct ACF200_SPEC;
impl crate::RegisterSpec for ACF200_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acf200::R](R) reader structure"]
impl crate::Readable for ACF200_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acf200::W](W) writer structure"]
impl crate::Writable for ACF200_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACF200 to value 0x0064_0000"]
impl crate::Resettable for ACF200_SPEC {
    const RESET_VALUE: Self::Ux = 0x0064_0000;
}
