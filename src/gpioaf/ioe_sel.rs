#[doc = "Register `IOE_SEL` reader"]
pub struct R(crate::R<IOE_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOE_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOE_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOE_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOE_SEL` writer"]
pub struct W(crate::W<IOE_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOE_SEL_SPEC>;
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
impl From<crate::W<IOE_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOE_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL7` reader - IOE7 special function select register."]
pub type SEL7_R = crate::BitReader<bool>;
#[doc = "Field `SEL7` writer - IOE7 special function select register."]
pub type SEL7_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOE_SEL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 7 - IOE7 special function select register."]
    #[inline(always)]
    pub fn sel7(&self) -> SEL7_R {
        SEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - IOE7 special function select register."]
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
#[doc = "IOE special function select register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioe_sel](index.html) module"]
pub struct IOE_SEL_SPEC;
impl crate::RegisterSpec for IOE_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ioe_sel::R](R) reader structure"]
impl crate::Readable for IOE_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ioe_sel::W](W) writer structure"]
impl crate::Writable for IOE_SEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IOE_SEL to value 0"]
impl crate::Resettable for IOE_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
