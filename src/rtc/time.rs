#[doc = "Register `TIME` reader"]
pub struct R(crate::R<TIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIME` writer"]
pub struct W(crate::W<TIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIME_SPEC>;
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
impl From<crate::W<TIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIME` reader - This register is used to control the accurate second/millisecond function"]
pub type TIME_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TIME` writer - This register is used to control the accurate second/millisecond function"]
pub type TIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIME_SPEC, u32, u32, 22, O>;
impl R {
    #[doc = "Bits 0:21 - This register is used to control the accurate second/millisecond function"]
    #[inline(always)]
    pub fn time(&self) -> TIME_R {
        TIME_R::new(self.bits & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:21 - This register is used to control the accurate second/millisecond function"]
    #[inline(always)]
    #[must_use]
    pub fn time(&mut self) -> TIME_W<0> {
        TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC accurate second/millisecond register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [time](index.html) module"]
pub struct TIME_SPEC;
impl crate::RegisterSpec for TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [time::R](R) reader structure"]
impl crate::Readable for TIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [time::W](W) writer structure"]
impl crate::Writable for TIME_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
