#[doc = "Register `IOB_SEL` reader"]
pub struct R(crate::R<IOB_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOB_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOB_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOB_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOB_SEL` writer"]
pub struct W(crate::W<IOB_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOB_SEL_SPEC>;
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
impl From<crate::W<IOB_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOB_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL1` reader - IOB1 special function select register."]
pub type SEL1_R = crate::BitReader<bool>;
#[doc = "Field `SEL1` writer - IOB1 special function select register."]
pub type SEL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOB_SEL_SPEC, bool, O>;
#[doc = "Field `SEL2` reader - IOB2 special function select register."]
pub type SEL2_R = crate::BitReader<bool>;
#[doc = "Field `SEL2` writer - IOB2 special function select register."]
pub type SEL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOB_SEL_SPEC, bool, O>;
#[doc = "Field `SEL6` reader - IOB6 special function select register."]
pub type SEL6_R = crate::BitReader<bool>;
#[doc = "Field `SEL6` writer - IOB6 special function select register."]
pub type SEL6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOB_SEL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - IOB1 special function select register."]
    #[inline(always)]
    pub fn sel1(&self) -> SEL1_R {
        SEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IOB2 special function select register."]
    #[inline(always)]
    pub fn sel2(&self) -> SEL2_R {
        SEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - IOB6 special function select register."]
    #[inline(always)]
    pub fn sel6(&self) -> SEL6_R {
        SEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - IOB1 special function select register."]
    #[inline(always)]
    #[must_use]
    pub fn sel1(&mut self) -> SEL1_W<1> {
        SEL1_W::new(self)
    }
    #[doc = "Bit 2 - IOB2 special function select register."]
    #[inline(always)]
    #[must_use]
    pub fn sel2(&mut self) -> SEL2_W<2> {
        SEL2_W::new(self)
    }
    #[doc = "Bit 6 - IOB6 special function select register."]
    #[inline(always)]
    #[must_use]
    pub fn sel6(&mut self) -> SEL6_W<6> {
        SEL6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IOB special function select register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iob_sel](index.html) module"]
pub struct IOB_SEL_SPEC;
impl crate::RegisterSpec for IOB_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iob_sel::R](R) reader structure"]
impl crate::Readable for IOB_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iob_sel::W](W) writer structure"]
impl crate::Writable for IOB_SEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IOB_SEL to value 0"]
impl crate::Resettable for IOB_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
