#[doc = "Register `WKUMIN` reader"]
pub struct R(crate::R<WKUMIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WKUMIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WKUMIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WKUMIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WKUMIN` writer"]
pub struct W(crate::W<WKUMIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WKUMIN_SPEC>;
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
impl From<crate::W<WKUMIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WKUMIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WKUMIN` reader - This register is used to control the multi-minute wake-up function."]
pub type WKUMIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WKUMIN` writer - This register is used to control the multi-minute wake-up function."]
pub type WKUMIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WKUMIN_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - This register is used to control the multi-minute wake-up function."]
    #[inline(always)]
    pub fn wkumin(&self) -> WKUMIN_R {
        WKUMIN_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - This register is used to control the multi-minute wake-up function."]
    #[inline(always)]
    #[must_use]
    pub fn wkumin(&mut self) -> WKUMIN_W<0> {
        WKUMIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC wake-up minute register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wkumin](index.html) module"]
pub struct WKUMIN_SPEC;
impl crate::RegisterSpec for WKUMIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wkumin::R](R) reader structure"]
impl crate::Readable for WKUMIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wkumin::W](W) writer structure"]
impl crate::Writable for WKUMIN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WKUMIN to value 0"]
impl crate::Resettable for WKUMIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
