#[doc = "Register `DAY` reader"]
pub struct R(crate::R<DAY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAY` writer"]
pub struct W(crate::W<DAY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAY_SPEC>;
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
impl From<crate::W<DAY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAY` reader - RTC_DAY, 1~31."]
pub type DAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAY` writer - RTC_DAY, 1~31."]
pub type DAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAY_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - RTC_DAY, 1~31."]
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - RTC_DAY, 1~31."]
    #[inline(always)]
    #[must_use]
    pub fn day(&mut self) -> DAY_W<0> {
        DAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC day register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [day](index.html) module"]
pub struct DAY_SPEC;
impl crate::RegisterSpec for DAY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [day::R](R) reader structure"]
impl crate::Readable for DAY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [day::W](W) writer structure"]
impl crate::Writable for DAY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
