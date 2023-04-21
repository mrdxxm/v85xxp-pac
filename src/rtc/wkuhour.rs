#[doc = "Register `WKUHOUR` reader"]
pub struct R(crate::R<WKUHOUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WKUHOUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WKUHOUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WKUHOUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WKUHOUR` writer"]
pub struct W(crate::W<WKUHOUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WKUHOUR_SPEC>;
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
impl From<crate::W<WKUHOUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WKUHOUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WKUHOUR` reader - This register is used to control the multi-hour wake-up function."]
pub type WKUHOUR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WKUHOUR` writer - This register is used to control the multi-hour wake-up function."]
pub type WKUHOUR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WKUHOUR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - This register is used to control the multi-hour wake-up function."]
    #[inline(always)]
    pub fn wkuhour(&self) -> WKUHOUR_R {
        WKUHOUR_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - This register is used to control the multi-hour wake-up function."]
    #[inline(always)]
    #[must_use]
    pub fn wkuhour(&mut self) -> WKUHOUR_W<0> {
        WKUHOUR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC wake-up hour register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wkuhour](index.html) module"]
pub struct WKUHOUR_SPEC;
impl crate::RegisterSpec for WKUHOUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wkuhour::R](R) reader structure"]
impl crate::Readable for WKUHOUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wkuhour::W](W) writer structure"]
impl crate::Writable for WKUHOUR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WKUHOUR to value 0"]
impl crate::Resettable for WKUHOUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
