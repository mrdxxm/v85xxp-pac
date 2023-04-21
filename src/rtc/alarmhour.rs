#[doc = "Register `ALARMHOUR` reader"]
pub struct R(crate::R<ALARMHOUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALARMHOUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALARMHOUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALARMHOUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALARMHOUR` writer"]
pub struct W(crate::W<ALARMHOUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALARMHOUR_SPEC>;
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
impl From<crate::W<ALARMHOUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALARMHOUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALARMHOUR` reader - This register is used to control the hour alarm function"]
pub type ALARMHOUR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALARMHOUR` writer - This register is used to control the hour alarm function"]
pub type ALARMHOUR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALARMHOUR_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - This register is used to control the hour alarm function"]
    #[inline(always)]
    pub fn alarmhour(&self) -> ALARMHOUR_R {
        ALARMHOUR_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - This register is used to control the hour alarm function"]
    #[inline(always)]
    #[must_use]
    pub fn alarmhour(&mut self) -> ALARMHOUR_W<0> {
        ALARMHOUR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC alarm hour\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alarmhour](index.html) module"]
pub struct ALARMHOUR_SPEC;
impl crate::RegisterSpec for ALARMHOUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alarmhour::R](R) reader structure"]
impl crate::Readable for ALARMHOUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alarmhour::W](W) writer structure"]
impl crate::Writable for ALARMHOUR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALARMHOUR to value 0"]
impl crate::Resettable for ALARMHOUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
