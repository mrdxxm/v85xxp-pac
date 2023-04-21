#[doc = "Register `ALARMTIME` reader"]
pub struct R(crate::R<ALARMTIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALARMTIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALARMTIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALARMTIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALARMTIME` writer"]
pub struct W(crate::W<ALARMTIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALARMTIME_SPEC>;
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
impl From<crate::W<ALARMTIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALARMTIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALARMTIME` reader - This register is used to control the accurate second and millisecond alarm function."]
pub type ALARMTIME_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ALARMTIME` writer - This register is used to control the accurate second and millisecond alarm function."]
pub type ALARMTIME_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ALARMTIME_SPEC, u32, u32, 22, O>;
impl R {
    #[doc = "Bits 0:21 - This register is used to control the accurate second and millisecond alarm function."]
    #[inline(always)]
    pub fn alarmtime(&self) -> ALARMTIME_R {
        ALARMTIME_R::new(self.bits & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:21 - This register is used to control the accurate second and millisecond alarm function."]
    #[inline(always)]
    #[must_use]
    pub fn alarmtime(&mut self) -> ALARMTIME_W<0> {
        ALARMTIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC alarm accurate second/millisecond.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alarmtime](index.html) module"]
pub struct ALARMTIME_SPEC;
impl crate::RegisterSpec for ALARMTIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alarmtime::R](R) reader structure"]
impl crate::Readable for ALARMTIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alarmtime::W](W) writer structure"]
impl crate::Writable for ALARMTIME_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALARMTIME to value 0"]
impl crate::Resettable for ALARMTIME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
