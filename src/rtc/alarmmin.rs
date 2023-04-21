#[doc = "Register `ALARMMIN` reader"]
pub struct R(crate::R<ALARMMIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALARMMIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALARMMIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALARMMIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALARMMIN` writer"]
pub struct W(crate::W<ALARMMIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALARMMIN_SPEC>;
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
impl From<crate::W<ALARMMIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALARMMIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALARMMIN` reader - This register is used to control the minute alarm function"]
pub type ALARMMIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALARMMIN` writer - This register is used to control the minute alarm function"]
pub type ALARMMIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALARMMIN_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - This register is used to control the minute alarm function"]
    #[inline(always)]
    pub fn alarmmin(&self) -> ALARMMIN_R {
        ALARMMIN_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - This register is used to control the minute alarm function"]
    #[inline(always)]
    #[must_use]
    pub fn alarmmin(&mut self) -> ALARMMIN_W<0> {
        ALARMMIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC alarm minute\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alarmmin](index.html) module"]
pub struct ALARMMIN_SPEC;
impl crate::RegisterSpec for ALARMMIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alarmmin::R](R) reader structure"]
impl crate::Readable for ALARMMIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alarmmin::W](W) writer structure"]
impl crate::Writable for ALARMMIN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALARMMIN to value 0"]
impl crate::Resettable for ALARMMIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
