#[doc = "Register `SEL` reader"]
pub struct R(crate::R<SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEL` writer"]
pub struct W(crate::W<SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEL_SPEC>;
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
impl From<crate::W<SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL3` reader - IOA3 special function select register."]
pub type SEL3_R = crate::BitReader<bool>;
#[doc = "Field `SEL3` writer - IOA3 special function select register."]
pub type SEL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEL_SPEC, bool, O>;
#[doc = "Field `SEL6` reader - IOA6 special function select register."]
pub type SEL6_R = crate::BitReader<bool>;
#[doc = "Field `SEL6` writer - IOA6 special function select register."]
pub type SEL6_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEL_SPEC, bool, O>;
#[doc = "Field `SEL7` reader - IOA7 special function select register."]
pub type SEL7_R = crate::BitReader<bool>;
#[doc = "Field `SEL7` writer - IOA7 special function select register."]
pub type SEL7_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - IOA3 special function select register."]
    #[inline(always)]
    pub fn sel3(&self) -> SEL3_R {
        SEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - IOA6 special function select register."]
    #[inline(always)]
    pub fn sel6(&self) -> SEL6_R {
        SEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IOA7 special function select register."]
    #[inline(always)]
    pub fn sel7(&self) -> SEL7_R {
        SEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - IOA3 special function select register."]
    #[inline(always)]
    #[must_use]
    pub fn sel3(&mut self) -> SEL3_W<3> {
        SEL3_W::new(self)
    }
    #[doc = "Bit 6 - IOA6 special function select register."]
    #[inline(always)]
    #[must_use]
    pub fn sel6(&mut self) -> SEL6_W<6> {
        SEL6_W::new(self)
    }
    #[doc = "Bit 7 - IOA7 special function select register."]
    #[inline(always)]
    #[must_use]
    pub fn sel7(&mut self) -> SEL7_W<7> {
        SEL7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IOA special function select register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel](index.html) module"]
pub struct SEL_SPEC;
impl crate::RegisterSpec for SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sel::R](R) reader structure"]
impl crate::Readable for SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sel::W](W) writer structure"]
impl crate::Writable for SEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEL to value 0"]
impl crate::Resettable for SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
